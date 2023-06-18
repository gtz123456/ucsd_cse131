use std::env;

use std::collections::HashSet;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, buffer: *mut u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    match errcode {
        0 => eprintln!("Runtime error: invalid argument"),
        1 => eprintln!("Runtime error: overflow"),
        2 => eprintln!("Runtime error: access the index of an non-tuple val"),
        3 => eprintln!("Runtime error: index is out-of-bound"),
        4 => eprintln!("Runtime error: index is not non-negative number"),
        _ => eprintln!("Runtime error ocurred {errcode}"),
    }
    std::process::exit(1);
}


#[export_name = "\x01snek_structural_equal"]
pub extern "C" fn snek_structural_equal(val1: i64, val2: i64) -> i64 {
  unsafe{snek_structural_equal_helper(val1, val2, &mut HashSet::new())}
}

unsafe fn snek_structural_equal_helper(val1: i64, val2: i64, seen: &mut HashSet<i64>) -> i64 {
  // println!("comparing {val1} {val2}");
  if val1 == val2 {return 7}
  if (val1 & 3) != 1 || (val2 & 3) != 1 {return 3}

  if seen.contains(&val1) || seen.contains(&val2) {
    println!("Can't resolve recursive structural equality {val1} {val2}.");
    return 3;
  }

  // both val1 and val2 are tuple
  let mut address1: *const i64 = (val1 >> 2) as *const i64;
  let length1 = *address1 / 8;
  let mut address2: *const i64 = (val2 >> 2) as *const i64;
  let length2 = *address2 / 8;
  if length1 != length2 {return 3}

  for _i in 1..=length1 {
      address1 = (address1 as i64 + 8) as *const i64;
      address2 = (address2 as i64 + 8) as *const i64;
      seen.insert(*address1);
      seen.insert(*address2);
      if snek_structural_equal_helper(*address1, *address2, seen) == 3 {return 3}
      seen.remove(&*address1);
      seen.remove(&*address2);
  }
  return 7
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(content: i64) {
    let t = get_real_content(content, &mut HashSet::new());
    println!("{t}");
}

fn get_real_content(content: i64, seen: &mut HashSet<i64>) -> String {
    // println!("{content}");
    let num = (content >> 1).to_string();
    let t = match content {
        3 => "false",
        7 => "true",
        _ if content & 3 == 1 => "tuple",
        _ => &num,
    };
    
    if t == "tuple".to_string() {
      if seen.contains(&content) {return "[...]".to_string()}
      seen.insert(content);
      let mut s = "(tuple".to_string();
      unsafe {
        let mut address: *const i64 = (content >> 2) as *const i64;
        let length = *address / 8;
        // println!("length{length}");
        for _i in 1..=length {
            address = (address as i64 + 8) as *const i64;
            let val = *address;
            s += &" ";
            s += &get_real_content(val, seen);
        }
      }
      seen.remove(&content);
      s += &")";
      s.to_string()
    }
    else {
      t.to_string()
    } 
}

fn parse_input(input: &str) -> u64 {
    // TODO: parse the input string into internal value representation
    match input {
        "true" => 7,
        "false" => 3,
        "" => 3,
        _ => {
            let t = i64::from_str_radix(&input, 10);
            match t {
              Ok(t) => {
                if t <= 4611686018427387903 && t >= -4611686018427387904 {(t << 1) as u64}
                else {panic!("invalid argument")}
              },
              Err(_) => panic!("invalid argument"),
            }
        },
    }
  }

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::with_capacity(100000);
    let buffer: *mut u64 = memory.as_mut_ptr();

    let i: i64 = unsafe { our_code_starts_here(input, buffer) as i64};
    let t = get_real_content(i, &mut HashSet::new());
    println!("{t}");
}
