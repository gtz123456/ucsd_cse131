use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::Atom::*;
use sexp::*;

use im::HashMap;


#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i32),
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
    ICmp(Val, Val),
    ICMov(Val, Val),
    ICMovg(Val, Val),
    ICMovge(Val, Val),
    ICmovl(Val, Val),
    ICmovle(Val, Val),
    IXor(Val, Val),
    IAnd(Val, Val),
    ISar(Val, Val),
    ITest(Val, Val),
    IJe(String),
    IJo(),
    IJne(String),
    IJmp(String),
    ILabel(String),
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

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => {
          match i64::try_from(*n) {
            Ok(result) => {
              if result > 4611686018427387903 || result < -4611686018427387904 {
                panic!("Invalid, overflow!")
              }
              Expr::Number(result)
            }
            Err(error) => {
                panic!("Invalid, overflow!")
            }
          }               
        },
        Sexp::Atom(S(n)) => {
            println!("{}", n);
            if n.eq("let") || n.eq("add1") || n.eq("sub1") || n.eq("block") || n.eq("break") || n.eq("set!")
            || n.eq("loop") || n.eq("isnum") || n.eq("isbool") || n.eq("if") {
                panic!("keyword");
            }
            if n.eq("true") {
              Expr::Boolean(true)
            } else if n.eq("false") {
              Expr::Boolean(false)
            } else {
              Expr::Id(n.clone())
            }   
        },
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), Sexp::Atom(S(name)), e] if op == "set!" => Expr::Set(name.clone(), Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Expr::If(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)), Box::new(parse_expr(e3))),
                [Sexp::Atom(S(op)), rest @ ..] if op == "block" => {
                  let mut v: Vec<Expr> = Vec::new();
                  println!("match block, len = {}", rest.len());
                  for exp in rest {
                      v.push(parse_expr(exp));
                  }
                  if v.len() == 0{
                    panic!("Invalid! Empty block");
                }
                  Expr::Block(v)
                },
                [Sexp::Atom(S(op)), Sexp::List(e1), e2] if op == "let" => {
                    let mut v: Vec<(String, Expr)> = Vec::new();
                    for binding in e1 {
                        v.push(parse_bind(binding))
                    }
                    if v.len() == 0{
                        panic!("Invalid! Empty bindings");
                    }
                    Expr::Let(v, Box::new(parse_expr(e2)))
                }
                _ => panic!("Invalid"),
            }
        },
        _ => panic!("Invalid"),
    }
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(n)), e] => {
                  if n.eq("let") || n.eq("add1") || n.eq("sub1") || n.eq("true") ||n.eq("false") 
                  || n.eq("block") || n.eq("break") || n.eq("set!") || n.eq("loop")
                  || n.eq("isnum") || n.eq("isbool") || n.eq("if") || n.eq("input"){
                    panic!("keyword");
                  }
                  (n.clone(), parse_expr(e))
                },
                _ => panic!("Invalid"),
            }
        }
        _ => panic!("Invalid"),
    }
}

fn new_label(l:&mut i32, s:&str) -> String {
  let current = *l;
  *l += 1;
  format!("{s}_{current}")
}


fn compile_to_instrs(e: &Expr, si:i32, env:&HashMap<String, i32>, l:&mut i32, break_target:&String) -> Vec<Instr> {
    let mut v: Vec<Instr> = Vec::new();
    match e {
        Expr::Number(n) => {
            let result = n << 1;
            v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(result)));
            v
        },
        Expr::Id(s) if s.eq("input")=> {   
            v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI)));
            v
        }    
        Expr::Id(s) => {
            if !env.contains_key(s) {
                panic!("Unbound variable identifier {}", s)
            } else {
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, *env.get(s).unwrap())));
                v
            }
        },
        Expr::Boolean(true) => {
          v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)));
          v
        },
        Expr::Boolean(false) => {
          v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
          v
        },
        Expr::If(cond, thn, els) => {
          let end_label = new_label(l, "ifend");
          let else_label = new_label(l, "ifelse");
          v.append(&mut compile_to_instrs(cond, si, env, l, break_target));
          v.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));
          v.push(Instr::IJe(else_label.clone()));
          v.append(&mut compile_to_instrs(thn, si, env, l, break_target));
          v.push(Instr::IJmp(end_label.clone()));
          v.push(Instr::ILabel(else_label.clone()));
          v.append(&mut compile_to_instrs(els, si, env, l, break_target));
          v.push(Instr::ILabel(end_label.clone()));
          v
        }, 
        Expr::Loop(e) => {
          let start_label = new_label(l, "loopstart");
          let end_label = new_label(l, "loopend");
          v.push(Instr::ILabel(start_label.clone()));
          v.append(&mut compile_to_instrs(e, si, env, l, &end_label));
          v.push(Instr::IJmp(start_label.clone()));
          v.push(Instr::ILabel(end_label));
          v
        },
        Expr::Break(e) => {
          if break_target.eq("") {
            panic!("Using break outside of loop!");
          }
          let new_break_target = format!("");
          v.append(&mut compile_to_instrs(e, si, env, l, &new_break_target));
          v.push(Instr::IJmp(break_target.clone()));
          v
        },
        Expr::Block(e) => {
          for exp  in e {
            v.append(&mut compile_to_instrs(exp, si, env, l, break_target));
          }
          v
        },
        Expr::Set(variable, e) => {
          println!("match set!");
          if !env.contains_key(variable) {
            panic!("Unbound variable identifier {}", variable)
        } 
          v.append(&mut compile_to_instrs(e, si, env, l, break_target));
          let offset:i32 = *env.get(variable).unwrap();
          v.push(Instr::IMov(Val::RegOffset(Reg::RSP, offset), Val::Reg(Reg::RAX)));
          v
        },
        Expr::UnOp(Op1::Add1, subexpr) => {
            v.append(&mut compile_to_instrs(subexpr, si, env, l, break_target));
            v.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Imm(2)));
            v.push(Instr::IJo());
            v
        },
        Expr::UnOp(Op1::Sub1, subexpr) => {
            v.append(&mut compile_to_instrs(subexpr, si, env, l, break_target));
            v.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Imm(2)));
            v.push(Instr::IJo());
            v
        },
        Expr::UnOp(Op1::IsBool, subexpr) => {
          v.append(&mut compile_to_instrs(subexpr, si, env, l, break_target));
          v.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
          v.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));

          v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(1)));
          v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(3)));
          v.push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          v
        },
        Expr::UnOp(Op1::IsNum, subexpr) => {
          v.append(&mut compile_to_instrs(subexpr, si, env, l, break_target));
          v.push(Instr::IAnd(Val::Reg(Reg::RAX), Val::Imm(1)));
          v.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::Imm(1)));

          v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
          v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
          v.push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          v
        },
        Expr::BinOp(Op2::Equal, subexpr1, subexpr2) => {
          v.append(&mut compile_to_instrs(subexpr1, si, env, l, break_target));
          let stack_offset = si * 8;
          v.push(Instr::IMov(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
          v.append(&mut compile_to_instrs(subexpr2, si + 1, env, l, break_target));
          v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));

          v.push(Instr::IXor(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, stack_offset)));
          v.push(Instr::ITest(Val::Reg(Reg::RBX), Val::Imm(1)));
          let error_label = format!("throw_error");
          v.push(Instr::IJne(error_label));

          
          v.push(Instr::ICmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
          v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
          v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
          v.push(Instr::ICMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
          v
        },

        Expr::BinOp(op, subexpr1, subexpr2) => {
            v.append(&mut compile_to_instrs(subexpr1, si, env, l, break_target));
            let stack_offset = si * 8;
            v.push(Instr::IMov(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
            v.append(&mut compile_to_instrs(subexpr2, si + 1, env, l, break_target));

            let error_label = format!("throw_error");
            v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
            v.push(Instr::IXor(Val::Reg(Reg::RBX), Val::Imm(0)));
            v.push(Instr::ITest(Val::Reg(Reg::RBX), Val::Imm(1)));
            v.push(Instr::IJne(error_label.clone()));
  
            v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, stack_offset)));
            v.push(Instr::IXor(Val::Reg(Reg::RBX), Val::Imm(0)));
            v.push(Instr::ITest(Val::Reg(Reg::RBX), Val::Imm(1)));
            v.push(Instr::IJne(error_label.clone()));

            match op {
              Op2::Plus => {
                v.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
                v.push(Instr::IJo());
              },
              Op2::Minus => {            
                v.push(Instr::ISub(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
                v.push(Instr::IJo());
              },
              Op2::Times => {
                v.push(Instr::ISar(Val::Reg(Reg::RAX), Val::Imm(1)));
                v.push(Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, stack_offset)));
                v.push(Instr::IJo());
              },
              Op2::Greater => {
                v.push(Instr::ICmp(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
                v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
                v.push(Instr::ICMovg(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              }
              Op2::GreaterEqual => {
                v.push(Instr::ICmp(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
                v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
                v.push(Instr::ICMovge(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              }
              Op2::Less => {
                v.push(Instr::ICmp(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
                v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
                v.push(Instr::ICmovl(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              }
              Op2::LessEqual => {
                v.push(Instr::ICmp(Val::RegOffset(Reg::RSP, stack_offset), Val::Reg(Reg::RAX)));
                v.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Imm(3)));
                v.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(1)));
                v.push(Instr::ICmovle(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              }
              _ => panic!("invalid"),
            } 
            v
        },

        Expr::Let(bindings, body) => {
            let mut new_env : HashMap<String, i32> = HashMap::new();
            let mut cur_si = si;
            new_env.clone_from(&env);
            let mut cur_layer = Vec::new();
            for (variable, val_exp)  in bindings {
                v.append(&mut compile_to_instrs(val_exp, cur_si, &new_env, l, break_target));
                if cur_layer.contains(variable) {
                    panic!("Duplicate binding")
                }
                new_env.insert(variable.clone(), cur_si*8);
                cur_layer.push(variable.clone());
                v.push(Instr::IMov(Val::RegOffset(Reg::RSP, cur_si*8), Val::Reg(Reg::RAX)));
                cur_si += 1;
            }
            v.append(&mut compile_to_instrs(body, cur_si+1, &new_env, l, break_target));
            v
        },
    }
}

fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(val1, val2) => format!("mov {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICMov(val1, val2) => format!("cmove {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICMovg(val1, val2) => format!("cmovg {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICMovge(val1, val2) => format!("cmovge {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICmovl(val1, val2) => format!("cmovl {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICmovle(val1, val2) => format!("cmovle {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::IAdd(val1, val2) => format!("add {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ISub(val1, val2) => format!("sub {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::IMul(val1, val2) => format!("imul {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ICmp(val1, val2) => format!("cmp {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::IXor(val1, val2) => format!("xor {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::IAnd(val1, val2) => format!("and {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ITest(val1, val2) => format!("test {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::ISar(val1, val2) => format!("sar {}, {}", val_to_str(val1), val_to_str(val2)),
        Instr::IJe(label) => format!("je {}", label),
        Instr::IJo() => format!("jo overflow"),
        Instr::IJne(label) => format!("jne {}", label),
        Instr::IJmp(label) => format!("jmp {}", label),
        Instr::ILabel(label) => format!("{}:", label),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Imm(n) => format!("{}", *n),
        Val::Reg(Reg::RAX) => format!("rax"),
        Val::Reg(Reg::RBX) => format!("rbx"),
        Val::Reg(Reg::RDI) => format!("rdi"),
        Val::RegOffset(Reg::RSP, offset) => format!("[rsp - {}]", *offset),
        _ => panic!("parse error"),
    }
}

fn compile(e: &Expr) -> String {
    let mut res = "".to_string();
    let init_si = 2;
    let mut l: i32 = 0;
    let break_target = format!("");
    let env : HashMap<String, i32> = HashMap::new();
    let instr = compile_to_instrs(e, init_si, &env, &mut l, &break_target);
    for i in &instr {
        res += &instr_to_str(&i);
        res += "\n";
    }
    res
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    // You will make result hold the result of actually compiling
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let parse_res = parse(&in_contents);
    let parse_res_file = match parse_res {
        Ok(file) => file,
        Err(error) => panic!("Invalid!"),
    };
    let expr = parse_expr(&parse_res_file);

    let result = compile(&expr);
 
    let asm_program = format!(
      "
section .text
extern snek_error
global our_code_starts_here
throw_error:
mov rdi, 7
push rsp
call snek_error
overflow:
mov rdi, 5
push rsp
call snek_error
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
