use std::fmt::Display;

use sexp::{Atom, Sexp};
use thiserror::Error;

pub const RESERVED_KEYWORDS: [&str; 13] = [
    "let", "add1", "sub1", "true", "false", "input", "set!", "if", "block", "loop", "break",
    "isnum", "isbool",
];

/// A value represented in assembly.
#[derive(Debug, PartialEq)]
pub enum Val {
    /// A register (e.g., rax or rsp).
    Reg(Reg),
    /// An (immutable) integer (e.g., 10, 8)
    Imm(i64),
    /// A register offset (e.g., [rax - 16]).
    RegOffset(Reg, i32),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Reg(r) => f.write_str(r.to_string().as_str()),
            Val::Imm(val) => f.write_str(val.to_string().as_str()),
            Val::RegOffset(reg, n) => f.write_fmt(format_args!("[{} - {}]", reg, n)),
        }
    }
}

/// A register in x86.
#[derive(Debug, PartialEq)]
pub enum Reg {
    /// The rax register. Also known as the return value.
    Rax,
    /// The r10 register. Also known as a scratch/temporary register.
    R10,
    /// The r11 register. Also known as a scratch/temporary register.
    R11,
    /// The rsp (i.e., stack pointer) register.
    Rsp,
    /// The first register to be used for passing in arguments.
    Rdi,
    /// The second register to be used for passing in arguments.
    Rsi,
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::Rax => f.write_str("rax"),
            Reg::R10 => f.write_str("r10"),
            Reg::R11 => f.write_str("r11"),
            Reg::Rsp => f.write_str("rsp"),
            Reg::Rdi => f.write_str("rdi"),
            Reg::Rsi => f.write_str("rsi"),
        }
    }
}

/// An assembly instruction.
#[derive(Debug, PartialEq)]
pub enum Instr {
    /// The mov instruction.
    IMov(Val, Val),
    /// The add instruction.
    IAdd(Val, Val),
    /// The sub instruction.
    ISub(Val, Val),
    /// The mul instruction.
    IMul(Val, Val),
    /// Compares the two arguments arg1 and arg2. More technically, it computes
    /// <reg> - <val> and sets condition codes.
    Compare(Val, Val),
    /// Compares the two arguments arg1 and arg2. More technically, it computes
    /// <reg> & <val> and sets condition codes.
    Test(Val, Val),
    /// The unconditional jump instruction (jmp). Unconditionally jumps to the specified
    /// label.
    Jump(String),
    /// Jump if equal instruction (je). This will jump to the specified
    /// label if the two compared values are equal.
    JumpEqual(String),
    /// Jump if not equal instruction (jne). This will jump to the specified
    /// label if the two compared values are not equal.
    JumpNotEqual(String),
    /// Jump if less instruction (jl). This will jump to the specified
    /// label if the first value is less than the second value.
    JumpLess(String),
    /// Jmp if less than or equal instruction (jle). This will jump to the specified
    /// label if the first value is less than or equal to the second value.
    JumpLessEqual(String),
    /// Jump if greater instruction (jg). This will jump to the specified
    /// label if the first value is greater than the second value.
    JumpGreater(String),
    /// Jump if greater than or equal instruction (jge). This will jump to the specified
    /// label if the first value is greater than the second value.
    JumpGreaterEqual(String),
    /// Jump if an overflow occurred in the last arithmetic operation.
    JumpOverflow(String),
    /// Jump if no overflow occurred in the last arithmetic operation.
    JumpNoOverflow(String),
    /// A label.
    Label(String),
    /// Shifts <reg> to the left by 1, filling in the least-significant bit with zero.
    ShiftLeft(Val, i32),
    /// Shifts <reg> to the right by 1, filling in the most-significant bit to preserve sign.
    ShiftSignRight(Val, i32),
    /// Shifts <reg> to the right by 1, filling in most-significant bit with zero.
    ShiftRight(Val, i32),
    /// Performs bitwise and on <reg> and <value>
    BitwiseAnd(Val, Val),
    // Performs the bitwise xor on <arg> and <value>
    BitwiseXor(Val, Val),
    /// Calls a Rust function.
    Call(String),
    /// Pushes the register to the stack.
    Push(Val),
    /// Pops the register from the stack.
    Pop(Reg),
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::IAdd(a, b) => f.write_fmt(format_args!("add {a}, {b}")),
            Instr::ISub(a, b) => f.write_fmt(format_args!("sub {a}, {b}")),
            Instr::IMul(a, b) => f.write_fmt(format_args!("imul {a}, {b}")),
            Instr::IMov(a, b) => f.write_fmt(format_args!("mov {a}, {b}")),
            Instr::Jump(lbl) => f.write_fmt(format_args!("jmp {lbl}")),
            Instr::JumpEqual(lbl) => f.write_fmt(format_args!("je {lbl}")),
            Instr::JumpNotEqual(lbl) => f.write_fmt(format_args!("jne {lbl}")),
            Instr::JumpLess(lbl) => f.write_fmt(format_args!("jl {lbl}")),
            Instr::JumpLessEqual(lbl) => f.write_fmt(format_args!("jle {lbl}")),
            Instr::JumpGreater(lbl) => f.write_fmt(format_args!("jg {lbl}")),
            Instr::JumpGreaterEqual(lbl) => f.write_fmt(format_args!("jge {lbl}")),
            Instr::Compare(arg1, arg2) => f.write_fmt(format_args!("cmp {arg1}, {arg2}")),
            Instr::Test(arg1, arg2) => f.write_fmt(format_args!("test {arg1}, {arg2}")),
            Instr::Label(lbl) => f.write_fmt(format_args!("{lbl}:")),
            Instr::ShiftLeft(s, amt) => f.write_fmt(format_args!("shl {s}, {amt}")),
            Instr::ShiftSignRight(s, amt) => f.write_fmt(format_args!("sar {s}, {amt}")),
            Instr::ShiftRight(s, amt) => f.write_fmt(format_args!("shr {s}, {amt}")),
            Instr::BitwiseAnd(a, b) => f.write_fmt(format_args!("and {a}, {b}")),
            Instr::Call(func) => f.write_fmt(format_args!("call {func}")),
            Instr::Push(p) => f.write_fmt(format_args!("push {p}")),
            Instr::Pop(p) => f.write_fmt(format_args!("pop {p}")),
            Instr::BitwiseXor(a, b) => f.write_fmt(format_args!("xor {a}, {b}")),
            Instr::JumpOverflow(lbl) => f.write_fmt(format_args!("jo {lbl}")),
            Instr::JumpNoOverflow(lbl) => f.write_fmt(format_args!("jno {lbl}")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Op1 {
    Arith(ArithmeticUnaryOp),
    Comp(ComparisonUnaryOp),
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticUnaryOp {
    Add1,
    Sub1,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonUnaryOp {
    IsNum,
    IsBool,
}

#[derive(Debug, PartialEq)]
pub enum Op2 {
    Arith(ArithmeticBinaryOp),
    Comp(ComparisonBinaryOp),
}

#[derive(Debug, PartialEq)]
pub enum ArithmeticBinaryOp {
    Plus,
    Minus,
    Times,
}

#[derive(Debug, PartialEq)]
pub enum ComparisonBinaryOp {
    Equality(CompEqualityOp),
    Inequality(CompInequalityOp),
}

#[derive(Debug, PartialEq)]
pub enum CompInequalityOp {
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
}

#[derive(Debug, PartialEq)]
pub enum CompEqualityOp {
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Number(i64),
    Bool(bool),
    Id(&'a str),
    Let(Vec<(&'a str, Expr<'a>)>, Box<Expr<'a>>),
    UnOp(Op1, Box<Expr<'a>>),
    BinOp(Op2, Box<Expr<'a>>, Box<Expr<'a>>),
    If(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    Set(&'a str, Box<Expr<'a>>),
    Block(Vec<Expr<'a>>),
    Loop(Box<Expr<'a>>),
    Break(Box<Expr<'a>>),
    Input,
}

#[derive(Error, Debug)]
pub enum CompileError<'a> {
    #[error("Unbound variable identifier {0}")]
    UnboundIdentifier(&'a str),

    #[error("No associated loop to break from")]
    NoLoopForBreak,
}

#[derive(Error, Debug, PartialEq)]
pub enum ParseError<'a> {
    #[error("Invalid block: Block is empty.")]
    BlockEmpty,

    #[error("Invalid operator: {0} (context: {1:?})")]
    UnknownOperator(&'a str, &'a [Sexp]),

    #[error("Invalid atom type: {0} not supported (or overflow occurred)")]
    UnsupportedAtomType(&'a Atom),

    #[error("Invalid binary operator: {0} (context: {1:?})")]
    UnknownBinaryOperator(&'a str, &'a [Sexp]),

    #[error("Invalid s-expression: {0:?}")]
    InvalidSExpr(&'a [Sexp]),

    #[error("Invalid let-binding, incorrectly formatted: {0}")]
    BadBinding(&'a Sexp),

    #[error("Invalid let-binding, bad binding: {0}")]
    InvalidBinding(&'a Sexp),

    #[error("Invalid identifier: {0} (context: {1:?}). Possible keyword or binary operator use?")]
    InvalidIdentifier(&'a str, &'a [Sexp]),

    #[error("Invalid identifier: {0} (context: {1:?}), keyword was used")]
    ReservedIdentifierUsed(&'a str, &'a [Sexp]),

    #[error("Invalid let-binding: no bindings.")]
    NoLetBindings,

    #[error("Duplicate binding {0}")]
    DuplicateIdentifier(&'a str),

    #[error("Invalid number; number too big: {0} (overflow/overflow)")]
    OverflowNumber(&'a Atom),

    #[error("Invalid number; number too small: {0} (overflow/underflow)")]
    UnderflowNumber(&'a Atom),
}
