use std::env;

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
    eprintln!("an error ocurred {errcode}");
    if errcode == 7 { 
        eprintln!("invalid argument");
    }    
    if errcode == 5 {
        eprintln!("overflow!");
    }    
    std::process::exit(1);
}


fn parse_input(input: &str) -> u64 {
    let mut res:u64 = 1;
    if input.eq("true") {
        res = 3;
    } else if input.eq("false") {
        res = 1;
    } else {
        let my_u64 = input.parse::<u64>();
        match my_u64 {
            Ok(n) => {
                let is_zero_64 = (n & (1 << 63));
                let is_zero_63 = (n & (1 << 62));
                if is_zero_64 != is_zero_63 {
                    eprintln!("invalid argument");
                    std::process::exit(1);
                }
                res = n << 1
            },
            Err(_) => res = 1,
        }

    }
    res
}

fn print_value(i:u64) {
    if i % 2 == 0 {
        let is_negative = (i & (1 << 63)) != 0;
        if is_negative {
            let mut result = !i; 
            result += 1;
            println!("-{}", result/2);
        } else {
            println!("{}", i/2);
        }
    } else if i == 3 {
        println!("true");
    } else if i == 1 {
        println!("false");
    } else {
        println!("Unknown: {}", i);
        eprintln!("invalid argument");
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    print_value(i);
}
