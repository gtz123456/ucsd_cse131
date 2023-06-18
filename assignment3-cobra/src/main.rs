use std::{env, vec};
use std::fs::File;
use std::io::prelude::*;
use sexp::Atom::*;
use sexp::*;

use im::HashMap;

static ERROR_INVALID_ARGUMENT: u64 = 0;
static ERROR_OVERFLOW: u64 = 1;

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm(u64),
    RegOffset(Reg, i64),
    Mark(String),
}

#[derive(Debug)]
enum Reg {
    RAX,
    RBX,
    RSP,
    RDI,
}

#[derive(Debug)]
enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    Idiv(Val),
    IAnd(Val, Val),
    IOr(Val, Val),
    IXor(Val, Val),
    ICmp(Val, Val),
    IJmp(Val),
    IJe(Val),
    IJo(Val),
    IJne(Val),
    //IJz(Val),
    //IJnz(Val),
    ICmove(Val, Val),
    ICmovne(Val, Val),
    ICmovg(Val, Val),
    ICmovge(Val, Val),
    ICmovl(Val, Val),
    ICmovle(Val, Val),
    Imark(Val),
    Isar(Val, Val),
}

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
enum Expr {
  Number(i64),
  Boolean(bool),
  Id(String),
  Let(Vec<(String, Expr)>, Box<Expr>),
  UnOp(Op1, Box<Expr>),
  BinOp(Op2, Box<Expr>, Box<Expr>),
  If(Box<Expr>, Box<Expr>, Box<Expr>),
  Loop(Box<Expr>),
  Break(Box<Expr>),
  Set(String, Box<Expr>),
  Block(Vec<Expr>),
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    // You will make result hold the result of actually compiling
    //let result = "mov rax, 131";

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    
    let expr = match parse(&in_contents) {
        Ok(parsed) => parse_expr(&parsed),
        Err(_) => panic!("Invalid sexp1"),
    };

    let result = compile(&expr);

    let asm_program = format!(
        "
section .text
extern snek_error
global our_code_starts_here
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

fn parse_expr(s: &Sexp) -> Expr {
  match s {
      Sexp::Atom(I(n)) => {
        if n > &4611686018427387903 || n < &-4611686018427387904 {panic!("Invalid overflow")}
        else {Expr::Number(*n)}
      }
      Sexp::Atom(S(s)) if s == "true" => Expr::Boolean(true),
      Sexp::Atom(S(s)) if s == "false" => Expr::Boolean(false),
      Sexp::Atom(S(s)) => Expr::Id(s.to_string()),
      Sexp::List(vec) => {
        if vec.len() > 0 && vec[0].to_string() == "block" {
          let mut exps = vec![];
          for i in &vec[1..] {
            exps.push(parse_expr(i));
          }
          if exps.len() == 0 {panic!("Invalid block")}
          Expr::Block(exps)
        }
        else {
          match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), Sexp::List(vec), e] if op == "let" => {
                if vec.len() == 0 {panic!("Invalid no-binding let sexp")}
                let mut pairs = Vec::new();
                for i in vec {
                    pairs.push(parse_bind(i));
                }
                Expr::Let(pairs, Box::new(parse_expr(e)))
            },
            [Sexp::Atom(S(op)), e1, e2] if op == "set!" => Expr::Set(e1.to_string(), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Expr::If(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)), Box::new(parse_expr(e3))),
            /*[Sexp::List(codes)] if &codes.len() > &0 && codes[0].to_string() == "block".to_string()=> {
              let mut exps = vec![];
              
              for i in &codes[1..] {
                exps.push(parse_expr(i));
              }
              Expr::Block(exps)
            }*/
            [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] => panic!("Invalid op{op}"),
            _ => panic!("Invalid sexp2"),
          }
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


fn compile_to_instrs(e: &Expr, si: i64, env: &HashMap<String, i64>, ifnum: &mut i64, loopnum: &mut i64, breaknum: i64) -> Vec<Instr> {
match e {
    Expr::Number(num) => {
        let mut t = vec![];
        t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm((*num << 1) as u64)));
        t
    },
    Expr::Id(s) => {
        if s == "input" {
          let mut t = vec![];
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 16)));
            t
        }
        else if !env.contains_key(s) {
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
            if &i.0 == "add1" ||&i.0 == "sub1" || &i.0 == "isnum" || &i.0 == "isbool" || &i.0 == "+" || &i.0 == "-" || &i.0 == "*" || &i.0 == "=" || &i.0 == ">" || &i.0 == ">=" || &i.0 == "<" || &i.0 == "<=" || &i.0 == "let" || &i.0 == "set!" || &i.0 == "if" || &i.0 == "block" || &i.0 == "loop" || &i.0 == "break" || &i.0 == "true" || &i.0 == "false" || &i.0 == "input" {
              panic!("Invalid keyword binding")
            }
            t.append(&mut compile_to_instrs(&i.1, si, &newenv.clone().union(env.clone()),ifnum, loopnum, breaknum));
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
        t.append(&mut compile_to_instrs(e, si, &newenv.union(env.clone()), ifnum, loopnum, breaknum));
        t
    },
    Expr::UnOp(op, e) => {
        let mut t = compile_to_instrs(e, si, env, ifnum, loopnum, breaknum);
        match op {
          Op1::Add1 => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(0)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJne(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(2)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_OVERFLOW)));
            t.push(Instr::IJo(Val::Mark("snek_error".to_string())));
          },
          Op1::Sub1 => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(0)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJne(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(2)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_OVERFLOW)));
            t.push(Instr::IJo(Val::Mark("snek_error".to_string())));
          },
          Op1::IsNum => {
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(0)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovne(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
          Op1::IsBool => {
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(0xfffffffffffffffd)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovne(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
        }
        t
    },
    Expr::BinOp(op, e1, e2) => {
        let mut t = compile_to_instrs(e2, si, env, ifnum, loopnum, breaknum);
        t.push(Instr::IMov(Val::RegOffset(Reg::RSP, si * 8), Val::Reg(Reg::RAX)));
        t.append(&mut compile_to_instrs(e1, si + 1, env, ifnum, loopnum, breaknum));
        match op {
          Op2::Minus => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ISub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_OVERFLOW)));
            t.push(Instr::IJo(Val::Mark("snek_error".to_string())));
          },
          Op2::Plus => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_OVERFLOW)));
            t.push(Instr::IJo(Val::Mark("snek_error".to_string())));
          },
          Op2::Times => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::Isar(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_OVERFLOW)));
            t.push(Instr::IJo(Val::Mark("snek_error".to_string())));
          },
          Op2::Equal => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IXor(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmove(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovne(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
          Op2::Greater => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmovg(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovle(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
          Op2::GreaterEqual => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmovge(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
          Op2::Less => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovge(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
          Op2::LessEqual => {
            t.push(Instr::IMov(Val::RegOffset(Reg::RSP, (si + 1) * 8), Val::Reg(Reg::RAX)));
            t.push(Instr::IOr(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
            t.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Imm(ERROR_INVALID_ARGUMENT)));
            t.push(Instr::IJe(Val::Mark("snek_error".to_string())));
            t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, (si + 1) * 8)));
            t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si * 8)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
            t.push(Instr::ICmovle(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
            t.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
            t.push(Instr::ICmovg(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          },
        }
        t
    },
    Expr::Boolean(boolvar) => {
      let mut t = vec![];
      if *boolvar {
        t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)));
      }
      else {
        t.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
      }
      t
    },
    Expr::If(condition, con_true, con_false) => {
      *ifnum += 1;
      let currentifnum = ifnum.clone();
      let mut t = compile_to_instrs(condition, si, env, ifnum, loopnum, breaknum);
      t.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
      t.push(Instr::IJe(Val::Mark(format!("elsestart{currentifnum}"))));
      t.append(&mut compile_to_instrs(con_true, si, env, ifnum, loopnum, breaknum));
      t.push(Instr::IJmp(Val::Mark(format!("elseend{currentifnum}"))));
      t.push(Instr::Imark(Val::Mark(format!("elsestart{currentifnum}"))));
      t.append(&mut compile_to_instrs(con_false, si, env, ifnum, loopnum, breaknum));
      t.push(Instr::Imark(Val::Mark(format!("elseend{currentifnum}"))));
      t
    },
    Expr::Loop(inloop) => {
      let mut t = vec![];
      *loopnum += 1;
      let currentloopnum = loopnum.clone();
      t.push(Instr::Imark(Val::Mark(format!("loopstart{currentloopnum}"))));
      t.append(&mut compile_to_instrs(inloop, si, env, ifnum, loopnum, currentloopnum));
      t.push(Instr::IJmp(Val::Mark(format!("loopstart{currentloopnum}"))));
      t.push(Instr::Imark(Val::Mark(format!("loopend{currentloopnum}"))));
      t
    },
    Expr::Break(loopresult) => {
      if breaknum == 0 {panic!("break outside loop")}
      let mut t = vec![];
      t.append(&mut compile_to_instrs(loopresult, si, env, ifnum, loopnum, breaknum));
      t.push(Instr::IJmp(Val::Mark(format!("loopend{breaknum}"))));
      t
    },
    Expr::Set(identifier, exp) => {
      let mut t = vec![];
      t.append(&mut compile_to_instrs(exp, si, env, ifnum, loopnum, breaknum));
      let identifier_offset = env.get(identifier);
      
      match identifier_offset {
        Some(identifier_offset) => 
          t.push(Instr::IMov(Val::RegOffset(Reg::RSP, identifier_offset * 8), Val::Reg(Reg::RAX))),
        None => panic!("Unbound variable identifier {identifier}"),
      }
      t
    },

    Expr::Block(blocks) => {
      let mut t = vec![];
      for i in blocks {
        t.append(&mut compile_to_instrs(i, si, env, ifnum, loopnum, breaknum))
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
    Instr::IAnd(val1, val2) => "\nand ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::IXor(val1, val2) => "\nxor ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmp(val1, val2) => "\ncmp ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::Idiv(val) => "\nidiv ".to_owned() + &val_to_str(val),
    Instr::IJmp(val) => "\njmp ".to_owned() + &val_to_str(val),
    Instr::IJe(val) => "\nje ".to_owned() + &val_to_str(val),
    Instr::IJne(val) => "\njne ".to_owned() + &val_to_str(val),
    //Instr::IJz(val) => "\njz ".to_owned() + &val_to_str(val),
    Instr::IJo(val) => "\njo ".to_owned() + &val_to_str(val),
    Instr::ICmove(val1, val2) => "\ncmove ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmovne(val1, val2) => "\ncmovne ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmovg(val1, val2) => "\ncmovg ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmovge(val1, val2) => "\ncmovge ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmovl(val1, val2) => "\ncmovl ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::ICmovle(val1, val2) => "\ncmovle ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::Imark(Val::Mark(val)) => "\n".to_owned() + val + ":",
    Instr::IOr(val1, val2) => "\nor ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    Instr::Isar(val1, val2) => "\nsar ".to_owned() + &val_to_str(val1) + ", " + &val_to_str(val2),
    _ => panic!("instr_to_str error"),
  }
}

fn val_to_str(v: &Val) -> String {
  match v {
      Val::Reg(Reg::RAX) => "rax".to_string(),
      Val::Reg(Reg::RBX) => "rbx".to_string(),
      Val::Reg(Reg::RSP) => "rsp".to_string(),
      Val::Reg(Reg::RDI) => "rdi".to_string(),
      Val::Imm(i) => i.to_string(),
      Val::RegOffset(Reg::RSP, offset) => "[rsp-".to_owned() + &offset.to_string() + "]",
      Val::Mark(s) => s.to_string(),
      _ => panic!("val_to_str error"),
  }
}

fn compile(e: &Expr) -> String {
  let instrs = compile_to_instrs(&e, 3, &HashMap::new(), &mut 0, &mut 0,  0);

  let mut result = "\nmov [rsp-16], rdi".to_string();

  for i in instrs {
      result += &instr_to_str(&i);
  }
  result
}