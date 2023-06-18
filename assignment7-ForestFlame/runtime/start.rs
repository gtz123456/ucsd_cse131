use std::{collections::HashMap, collections::HashSet, env};

type SnekVal = u64;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum ErrCode {
    InvalidArgument = 1,
    Overflow = 2,
    IndexOutOfBounds = 3,
    InvalidVecSize = 4,
    OutOfMemory = 5,
}

const TRUE: u64 = 7;
const FALSE: u64 = 3;

static mut HEAP_START: *const u64 = std::ptr::null();
static mut HEAP_END: *const u64 = std::ptr::null();

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, heap_start: *const u64, heap_end: *const u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    if errcode == ErrCode::InvalidArgument as i64 {
        eprintln!("invalid argument");
    } else if errcode == ErrCode::Overflow as i64 {
        eprintln!("overflow");
    } else if errcode == ErrCode::IndexOutOfBounds as i64 {
        eprintln!("index out of bounds");
    } else if errcode == ErrCode::InvalidVecSize as i64 {
        eprintln!("vector size must be non-negative");
    } else {
        eprintln!("an error ocurred {}", errcode);
    }
    std::process::exit(errcode as i32);
}

#[export_name = "\x01snek_print"]
pub unsafe extern "C" fn snek_print(val: SnekVal) -> SnekVal {
    println!("{}", snek_str(val, &mut HashSet::new()));
    val
}

/// This function is called when the program needs to allocate `count` words of memory and there's no
/// space left. The function should try to clean up space by triggering a garbage collection. If there's
/// not enough space to hold `count` words after running the garbage collector, the program should terminate
/// with an `out of memory` error.
///
/// Args:
///     * `count`: The number of words the program is trying to allocate, including an extra word for
///       the size of the vector and an extra word to store metadata for the garbage collector, e.g.,
///       to allocate a vector of size 5, `count` will be 7.
///     * `heap_ptr`: The current position of the heap pointer (i.e., the value stored in `%r15`). It
///       is guaranteed that `heap_ptr + 8 * count > HEAP_END`, i.e., this function is only called if
///       there's not enough space to allocate `count` words.
///     * `stack_base`: A pointer to the "base" of the stack.
///     * `curr_rbp`: The value of `%rbp` in the stack frame that triggered the allocation.
///     * `curr_rsp`: The value of `%rsp` in the stack frame that triggered the allocation.
///
/// Returns:
///
/// The new heap pointer where the program should allocate the vector (i.e., the new value of `%r15`)
///
#[export_name = "\x01snek_try_gc"]
pub unsafe fn snek_try_gc(
    count: isize,
    heap_ptr: *const u64,
    stack_base: *const u64,
    curr_rbp: *const u64,
    curr_rsp: *const u64,
) -> *const u64 {
    // println!("heap before gc");
    // snek_print_heap(heap_ptr);
    let new_heap_ptr = snek_gc(heap_ptr, stack_base, curr_rbp, curr_rsp);
    // println!("heap after gc");
    // snek_print_heap(new_heap_ptr);
    if HEAP_END < new_heap_ptr.add(count as usize) {
        eprintln!("out of memory");
        std::process::exit(ErrCode::OutOfMemory as i32)
    }
    new_heap_ptr
}

/// This function should trigger garbage collection and return the updated heap pointer (i.e., the new
/// value of `%r15`). See [`snek_try_gc`] for a description of the meaning of the arguments.
#[export_name = "\x01snek_gc"]
pub unsafe fn snek_gc(
    heap_ptr: *const u64,
    stack_base: *const u64,
    _curr_rbp: *const u64,
    curr_rsp: *const u64,
) -> *const u64 {
    // println!("-----------------");
    // println!("starting gc: HEAP_START:{:#0x} HEAP_END:{:#0x}, heap_ptr:{:#0x}", HEAP_START as u64, HEAP_END as u64, heap_ptr as u64);

    let mut marked: HashSet<*const u64> = HashSet::new();
    let root = snek_gc_get_root(stack_base, curr_rsp);

    for i in root.keys() {
        snek_gc_mark(i.clone(), &mut marked);
    }
    snek_gc_compute_forwarding_addresses(heap_ptr);
    // println!("compute_forwarding_addresses finish");
    snek_gc_update_references(heap_ptr);
    // println!("update_references finish");
    snek_gc_update_stack(root);
    // println!("update_stack finish");
    snek_gc_move_objects(heap_ptr)
}

unsafe fn snek_gc_get_root(stack_base: *const u64, curr_rsp: *const u64) -> HashMap<*const u64, Vec<*const u64>> {
    let mut root_temp: HashMap<*const u64, Vec<*const u64>> = HashMap::new();
    let mut ptr = stack_base;
    while ptr >= curr_rsp {
        let val = *ptr;
        if val & 3 == 1 && val >= HEAP_START as u64 && val < HEAP_END as u64 {
            if let Some(t) = root_temp.get_mut(&((val - 1) as *const u64)) {
                t.push(ptr);
            } else {
                root_temp.insert((val - 1) as *const u64, vec![ptr]);
            }
            
            // println!("vec {:#0x} added to root", val);
        }
        ptr = ptr.sub(1);
    }
    root_temp
}

unsafe fn snek_gc_mark(vec_start: *const u64, marked: &mut HashSet<*const u64>) {
    if marked.contains(&vec_start) {return}
    marked.insert(vec_start);
    // println!("marking{:#0x}", vec_start as u64);
    let mutable_ptr = vec_start as *mut u64;
    *mutable_ptr = 1;
    let ptr = vec_start.add(1);
    let vec_length = *ptr;
    for i in 1..=vec_length {
        let val = *ptr.add(i as usize);
        if val & 1 == 1 && val >= HEAP_START as u64 && val < HEAP_END as u64 {
            snek_gc_mark((val - 1) as *const u64, marked);
        }
    }
}

unsafe fn snek_gc_compute_forwarding_addresses(heap_ptr: *const u64) {
    let mut ptr = HEAP_START as *mut u64;
    let mut new_heap_ptr = HEAP_START as *mut u64;
    while (ptr as *const u64) < heap_ptr {
        let length = *(ptr.add(1)) as usize;
        if *ptr != 0 {
            // println!("{:#0x} move to {:#0x}", ptr as u64, new_heap_ptr as u64);
            *ptr = new_heap_ptr as u64;
            new_heap_ptr = new_heap_ptr.add(length + 2);
        }
        ptr = ptr.add(length + 2);
    }
}

unsafe fn snek_gc_update_references(heap_ptr: *const u64) {
    let mut ptr = HEAP_START as *mut u64;
    while (ptr as *const u64) < heap_ptr {
        // println!("updating vec {:#0x}", ptr as u64);
        let length = *(ptr.add(1)) as usize;
        if *ptr != 0 {
            ptr = ptr.add(2);
            for i in 0..length {
                if *ptr & 1 == 1 && *ptr != 1 {
                    // println!("updating reference {:#0x} index {:#0x}, from {:#0x} to {:#0x}", ptr as u64, i, *ptr, *((*ptr - 1) as *const u64) + 1);
                    *ptr = *((*ptr - 1) as *const u64) + 1;
                }
                ptr = ptr.add(1);
            }
        }
        else {
            ptr = ptr.add(length + 2);
        }
    }
}

unsafe fn snek_gc_move_objects(heap_ptr: *const u64) -> *const u64 {
    let mut ptr = HEAP_START as *mut u64;
    let mut new_heap_ptr = HEAP_START as *mut u64;
    while (ptr as *const u64) < heap_ptr {
        let length = *(ptr.add(1)) as usize;
        if *ptr != 0 {
            // println!("moving {:#0x} to {:#0x}", ptr as u64, new_heap_ptr as u64);
            *new_heap_ptr = 0;
            new_heap_ptr = new_heap_ptr.add(1);
            ptr = ptr.add(1);
            for _i in 0..length + 1 {
                *new_heap_ptr = *ptr;
                new_heap_ptr = new_heap_ptr.add(1);
                ptr = ptr.add(1);
            }
        }
        else {
            ptr = ptr.add(length + 2);
        }
    }
    // println!("---------------------------------------------current heap ptr{:#0x}", new_heap_ptr as u64);
    new_heap_ptr
}

unsafe fn snek_gc_update_stack(dic: HashMap<*const u64, Vec<*const u64>>) {
    for (key, value) in dic {
        for ptr in value {
            let mut_ptr = ptr as *mut u64;
            *mut_ptr = *key + 1;
        }
    }
}

unsafe fn snek_print_heap(heap_ptr: *const u64) {
    let mut ptr = HEAP_START;
    println!("-------------snek_print_heap---------------");
    while ptr < heap_ptr {
        let val = *ptr;
        // println!("ptr:{:#0x}: {:#0x}", ptr as u64, val);
        ptr = ptr.add(1);
    }
    println!("----------snek_print_heap_end--------------");
}

/// A helper function that can called with the `(snek-printstack)` snek function. It prints the stack
/// See [`snek_try_gc`] for a description of the meaning of the arguments.
#[export_name = "\x01snek_print_stack"]
pub unsafe fn snek_print_stack(stack_base: *const u64, curr_rbp: *const u64, curr_rsp: *const u64) {
    let mut ptr = stack_base;
    println!("-----------------------------------------");
    while ptr >= curr_rsp {
        let val = *ptr;
        println!("{ptr:?}: {:#0x}", val);
        ptr = ptr.sub(1);
    }
    println!("-----------------------------------------");
}

unsafe fn snek_str(val: SnekVal, seen: &mut HashSet<SnekVal>) -> String {
    if val == TRUE {
        format!("true")
    } else if val == FALSE {
        format!("false")
    } else if val & 1 == 0 {
        format!("{}", (val as i64) >> 1)
    } else if val == 1 {
        format!("nil")
    } else if val & 1 == 1 {
        if !seen.insert(val) {
            return "[...]".to_string();
        }
        let addr = (val - 1) as *const u64;
        let size = addr.add(1).read() as usize;
        let mut res = "[".to_string();
        for i in 0..size {
            let elem = addr.add(2 + i).read();
            res = res + &snek_str(elem, seen);
            if i < size - 1 {
                res = res + ", ";
            }
        }
        seen.remove(&val);
        res + "]"
    } else {
        format!("unknown value: {val}")
    }
}

fn parse_input(input: &str) -> u64 {
    match input {
        "true" => TRUE,
        "false" => FALSE,
        _ => (input.parse::<i64>().unwrap() << 1) as u64,
    }
}

fn parse_heap_size(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() >= 2 { &args[1] } else { "false" };
    let heap_size = if args.len() >= 3 { &args[2] } else { "10000" };
    let input = parse_input(&input);
    let heap_size = parse_heap_size(&heap_size);

    // Initialize heap
    let mut heap: Vec<u64> = Vec::with_capacity(heap_size);
    unsafe {
        HEAP_START = heap.as_mut_ptr();
        HEAP_END = HEAP_START.add(heap_size);
    }

    let i: u64 = unsafe { our_code_starts_here(input, HEAP_START, HEAP_END) };
    unsafe { snek_print(i) };
}
