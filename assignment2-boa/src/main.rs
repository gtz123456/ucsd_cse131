use std::{env, vec};
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use im::HashMap;

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm(i32),
    RegOffset(Reg, i32),
}

#[derive(Debug)]
enum Reg {
    RAX,
    RSP,
}

#[derive(Debug)]
enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
}

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
}

#[derive(Debug)]
enum Expr {
    Number(i32),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Number(i32::try_from(*n).unwrap()),
        Sexp::Atom(S(s)) => Expr::Id(s.to_string()),
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), Sexp::List(vec), e] if op == "let" => {
                    if vec.len() == 0 {panic!("Invalid no-binding let sexp")}
                    let mut pairs = Vec::new();
                    for i in vec {
                        pairs.push(parse_bind(i));
                    }
                    Expr::Let(pairs, Box::new(parse_expr(e)))
                },
                _ => panic!("Invalid sexp"),
            }
        },
        _ => panic!("Invalid sexp"),
    }
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(var)), e] => (var.to_string(), parse_expr(e)),
                _ => panic!("Invalid bind"),
            }
        },
        _ => panic!("Invalid bind"),
    }
}


fn compile_to_instrs(e: &Expr, si: i32, env: &HashMap<String, i32>) -> Vec<Instr> {
    match e {
        Expr::Number(num) => {
            let mut t = vec![];
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*num)));
            t
        },
        Expr::Id(s) => {
            if !env.contains_key(s) {
                panic!("Unbound variable identifier {s}")
            }
            else {
                let mut t = vec![];
                t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, env.get(s).unwrap() * 8)));
                t
            }
        },
        Expr::Let(binds, e) => {
            let mut newenv = HashMap::new();
            let mut t = vec![];

            let mut si = si;
            for i in binds {
                if newenv.contains_key(&i.0) {panic!("Duplicate binding")}
                if &i.0 == "let" || &i.0 == "add1" ||&i.0 == "sub1" {panic!("Invalid identifier")}
                t.append(&mut compile_to_instrs(&i.1, si, &newenv.clone().union(env.clone())));
                if !env.contains_key(&i.0) {
                    newenv = newenv.update(i.0.clone(), si);
                    t.push(Instr::IMov(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)));
                    si += 1;
                }
                else {
                    t.push(Instr::IMov(Val::RegOffset(Reg::RSP, env.get(&i.0).unwrap() * 8), Val::Reg(Reg::RAX)));
                    newenv = newenv.update(i.0.clone(), env.get(&i.0).unwrap().to_owned());
                }
            }

            t.append(&mut compile_to_instrs(e, si, &newenv.union(env.clone())));
            t
        },
        Expr::UnOp(op, e) => {
            let mut t = compile_to_instrs(e, si, env);
            match op {
                Op1::Add1 => t.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(1))),
                Op1::Sub1 => t.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(1))),
            }
            t
        },
        Expr::BinOp(op, e1, e2) => {
            let mut t = compile_to_instrs(e2, si, env);
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)));
            t.append(&mut compile_to_instrs(e1, si + 1, env));
            match op {
                Op2::Minus => t.push(Instr::ISub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8))),
                Op2::Plus => t.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8))),
                Op2::Times => t.push(Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8))),
            }
            t
        },
        
    }
}

fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(val1, val2) => "\nmov ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
        Instr::IAdd(val1, val2) => "\nadd ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
        Instr::ISub(val1, val2) => "\nsub ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
        Instr::IMul(val1, val2) => "\nimul ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(Reg::RAX) => "rax".to_string(),
        Val::Reg(Reg::RSP) => "rsp".to_string(),
        Val::Imm(i) => i.to_string(),
        Val::RegOffset(Reg::RSP, offset)=> "[rsp-".to_owned() + &offset.to_string() + "]",
        _ => panic!("val_to_str error"),
    }
}

fn compile(e: &Expr) -> String {
    let instrs = compile_to_instrs(&e, 2, &HashMap::new());

    let mut result = String::new();

    for i in instrs {
        result += &instr_to_str(&i);
    }
    result
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    // You will make result hold the result of actually compiling
    // let result = "mov rax, 131";
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    
    let expr = match parse(&in_contents) {
        Ok(parsed) => parse_expr(&parsed),
        Err(_) => panic!("Invalid sexp"),
    };

    let result = compile(&expr);

    let asm_program = format!(
        "
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",result);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
