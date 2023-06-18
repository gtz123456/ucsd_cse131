use std::env;

const OVERFLOW_ERROR: i64 = 1;
const TYPE_MISMATCH: i64 = 2;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    match errcode {
        OVERFLOW_ERROR => eprintln!("[Runtime Error] overflow error."),
        TYPE_MISMATCH => eprintln!("[Runtime Error] type mismatch error (invalid argument)."),
        _ => eprintln!("[Runtime Error] Unknown error code: {errcode}"),
    };

    std::process::exit(1);
}

fn parse_input(input: &str) -> u64 {
    if input == "true" {
        0b11
    } else if input == "false" {
        0b01
    } else if let Ok(val) = input.parse::<i64>() {
        if val > (i64::MAX >> 1) {
            panic!("[Input Error] Invalid input, {} overflow/overflow", val)
        } else if val < (i64::MIN >> 1) {
            panic!("[Input Error] Invalid input, {} overflow/underflow", val)
        } else {
            (val << 1) as u64
        }
    } else {
        panic!("[Input Error] Unsupported input: `{}`", input);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: i64 = unsafe { our_code_starts_here(input) } as i64;
    // If the LSB of i is 0, then we have a number
    // If the LSB of i is 1, then we have a boolean
    if i & 1 == 0 {
        // Number
        println!("{}", i >> 1);
    } else {
        // Boolean
        println!("{}", i >> 1 == 1);
    }
}
