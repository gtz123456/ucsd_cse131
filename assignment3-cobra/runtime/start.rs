use std::env;

//static ERROR_INVALID_ARGUMENT: u64 = 0;
//static ERROR_OVERFLOW: u64 = 1;

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
    // TODO: print error message according to writeup
    match errcode {
        0 => eprintln!("invalid argument"),
        1 => eprintln!("overflow"),
        _ => eprintln!("an error ocurred {errcode}"),
    }
    std::process::exit(1);
}

fn parse_input(input: &str) -> u64 {
    // TODO: parse the input string into internal value representation
    match input {
        "true" => 3,
        "false" => 1,
        "" => 1,
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

    let i: i64 = unsafe { our_code_starts_here(input) as i64};
    let t = &(i >> 1).to_string();
    let t = match &i {
        1 => "false",
        3 => "true",
        _ => t,
    };
    println!("{t}");
}
