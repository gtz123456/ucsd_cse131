pub mod compiler;
pub mod parser;
pub mod types;
pub mod util;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use compiler::compile;
use parser::parse_expr;
use sexp::parse;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Doing all panic error handling in the main function.
    // Note that sexpr itself is a Sexp object containing a bunch of other
    // Sexpr objects. If the lifetime of sexpr is 'a, then when we call parse_expr
    // and compile, their lifetimes are also associated with sexpr!
    //
    // The lifetime of sexpr ends at the end of the main function.
    let sexpr = match parse(&in_contents) {
        Ok(o) => o,
        Err(e) => panic!("[Parse Error] Invalid s-expression: {e}"),
    };

    let expr = match parse_expr(&sexpr) {
        Ok(o) => o,
        Err(e) => panic!("[Parse Error] {e}"),
    };

    let result = match compile(&expr) {
        Ok(o) => o,
        Err(e) => panic!("[Compile Error] {e}"),
    };

    let asm_program = format!(
        "section .text
extern snek_error
global our_code_starts_here

throw_error:
    mov rdi, rax
    push rsp
    call snek_error
    pop rsp

our_code_starts_here:
{}
    ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
