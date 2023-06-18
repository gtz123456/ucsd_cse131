use im::{hashmap, HashMap};

use crate::types::{
    ArithmeticBinaryOp, ArithmeticUnaryOp, CompEqualityOp, CompInequalityOp, ComparisonBinaryOp,
    ComparisonUnaryOp, CompileError, Expr, Instr, Op1, Op2, Reg, Val,
};

const OVERFLOW_ERROR: i64 = 1;
const TYPE_MISMATCH: i64 = 2;
const THROW_ERROR_LABEL: &str = "throw_error";

const TRUE: i64 = 0b11;
const FALSE: i64 = 0b01;

/// Compiles an expression into a formatted `String` containing the final assembly
/// code.
///
/// # Parameters
/// - `e`: The expression to evaluate.
///
/// # Returns
/// A string containing the final assembly code as a `String`. If an error occurred while
/// compiling, this will instead return a `CompileError`.
pub fn compile<'a>(e: &'a Expr) -> Result<String, CompileError<'a>> {
    let mut all_instructions = vec![];
    compile_helper(e, 1, &mut all_instructions, &hashmap! {}, &mut 0, None)?;
    Ok(all_instructions
        .into_iter()
        .map(|inst| format!("    {inst}"))
        .collect::<Vec<_>>()
        .join("\n"))
}

/// A helper function that recursive evaluates the given expression, generating assembly instructions.
///
/// # Parameters
/// - `e`: The expression to evaluate.
/// - `si`: The stack index.
/// - `instructions`: The instructions that have been parsed from the given expressions. This is the list
///                   where the result of evaluating the expression goes.
/// - `environment`: The identifier environment. This is represented as a map where the key is the identifier
///                  name and the value is the stack offset (not index).
/// - `counter`: A counter used to create unique labels.
/// - `loop_label`: An option that represents either a `String` if the compiler is working on a subexpression that is
///                 within a loop, or `None` if the expression is not within a loop.
///
/// # Returns
/// Either `Ok(())` if generating the assembly instructions succeeded (the actual instructions are stored in
/// the `instructions` vector), or `Err(CompileError)` if a compilation error occurred.
fn compile_helper<'a>(
    e: &'a Expr,
    si: i32,
    instructions: &mut Vec<Instr>,
    environment: &HashMap<&'a str, i32>,
    counter: &mut u64,
    loop_label: Option<&str>,
) -> Result<(), CompileError<'a>> {
    match e {
        Expr::Number(n) => instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(*n << 1))),
        Expr::BinOp(op_type, e1, e2) => {
            let stack_offset = si * 8;
            // Subtraction is a bit of a weird case. The s-expression, (- 10 7), generates the assembly
            //      mov rax, 10
            //      mov [rsp - 8], rax
            //      mov rax, 7
            //      sub rax, [rsp - 8]
            //      ret
            //
            // What ends up happening is that
            //      [rsp - 8] -> 10
            //      rax       -> 7
            //      rax = rax - [rsp - 8] = 7 - 10 = -3
            // except, the answer should be 10 - 7 = 3.
            //
            // So, it looks like the easiest solution is, for subtraction only, to flip the operands.
            //
            // This applies to all operations except + and *, since order doesn't really matter...
            compile_helper(
                match op_type {
                    Op2::Arith(ArithmeticBinaryOp::Plus)
                    | Op2::Arith(ArithmeticBinaryOp::Times) => e1,
                    _ => e2,
                },
                si,
                instructions,
                environment,
                counter,
                loop_label,
            )?;

            instructions.push(Instr::IMov(
                Val::RegOffset(Reg::Rsp, stack_offset),
                Val::Reg(Reg::Rax),
            ));

            compile_helper(
                match op_type {
                    Op2::Arith(ArithmeticBinaryOp::Plus)
                    | Op2::Arith(ArithmeticBinaryOp::Times) => e2,
                    _ => e1,
                },
                si + 1,
                instructions,
                environment,
                counter,
                loop_label,
            )?;

            let true_label = new_label("true_lbl", counter);
            let done_label = new_label("done_lbl", counter);

            // Before we actually perform the operation, we need to make sure we have
            // the correct types for our operands.
            //
            // Let's suppose our NUMBER type has value 0 and BOOLEAN type has value 1,
            // analogous to how we're representing their tags.
            //
            // We can use r11 to determine whether they have the same types.
            // If op1 has type NUMBER, add 0 to r11; if op1 has type BOOLEAN, add 1 to r11.
            // If op2 has type NUMBER, add 0 to r11; if op2 has type BOOLEAN, add 1 to r11.
            //
            // At the end, check if r11 has value 0 OR 2. If it does, then types match. Otherwise,
            // fail.

            instructions.push(Instr::IMov(Val::Reg(Reg::R11), Val::Imm(0)));

            // Check the value type in RAX
            instructions.push(Instr::IMov(Val::Reg(Reg::R10), Val::Reg(Reg::Rax)));
            instructions.push(Instr::BitwiseAnd(Val::Reg(Reg::R10), Val::Imm(1)));
            instructions.push(Instr::IAdd(Val::Reg(Reg::R11), Val::Reg(Reg::R10)));

            // Check the value type in [rsp - offset]
            instructions.push(Instr::IMov(
                Val::Reg(Reg::R10),
                Val::RegOffset(Reg::Rsp, stack_offset),
            ));
            instructions.push(Instr::BitwiseAnd(Val::Reg(Reg::R10), Val::Imm(1)));
            instructions.push(Instr::IAdd(Val::Reg(Reg::R11), Val::Reg(Reg::R10)));

            // At this point, r11 should either be 0, 1, or 2.
            // Remember that
            // - (number, number) => 0
            // - (number, boolean) => 1
            // - (boolean, number) => 1
            // - (boolean, boolean) => 2
            let type_check_lbl = new_label("type_checker", counter);

            // Before we do anything, know that any comparison operator will not want mixed types,
            // so we can check if r11 contains 1 (implying that we have a type mismatch). If we do,
            // then we can just juimp to the error function.
            instructions.push(Instr::Compare(Val::Reg(Reg::R11), Val::Imm(1)));
            instructions.push(Instr::JumpNotEqual(type_check_lbl.to_owned()));
            call_error_fn(TYPE_MISMATCH, instructions);
            instructions.push(Instr::Label(type_check_lbl));

            // At this point, we know that the types of the operands are the same.
            let type_check_success = new_label("success_check", counter);
            match op_type {
                Op2::Arith(op) => {
                    // For arithmetic operators, we only want (number, number).
                    instructions.push(Instr::Compare(Val::Reg(Reg::R11), Val::Imm(0)));
                    instructions.push(Instr::JumpEqual(type_check_success.to_owned()));
                    call_error_fn(TYPE_MISMATCH, instructions);
                    instructions.push(Instr::Label(type_check_success));

                    match op {
                        ArithmeticBinaryOp::Plus => {
                            instructions.push(Instr::IAdd(
                                Val::Reg(Reg::Rax),
                                Val::RegOffset(Reg::Rsp, stack_offset),
                            ));
                        }
                        ArithmeticBinaryOp::Minus => {
                            instructions.push(Instr::ISub(
                                Val::Reg(Reg::Rax),
                                Val::RegOffset(Reg::Rsp, stack_offset),
                            ));
                        }
                        ArithmeticBinaryOp::Times => {
                            instructions.push(Instr::ShiftSignRight(Val::Reg(Reg::Rax), 1));
                            instructions.push(Instr::IMul(
                                Val::Reg(Reg::Rax),
                                Val::RegOffset(Reg::Rsp, stack_offset),
                            ));
                        }
                    }

                    // After the operation is complete, make sure there wasn't an overflow
                    let no_overflow_label = new_label("no_overflow", counter);
                    instructions.push(Instr::JumpNoOverflow(no_overflow_label.to_owned()));
                    call_error_fn(OVERFLOW_ERROR, instructions);
                    instructions.push(Instr::Label(no_overflow_label));
                }
                Op2::Comp(op_type) => {
                    let mut temp_instructions = vec![Instr::Compare(
                        Val::Reg(Reg::Rax),
                        Val::RegOffset(Reg::Rsp, stack_offset),
                    )];

                    match op_type {
                        // For equalities, we allow both pair types (number, number) and (boolean, boolean)
                        ComparisonBinaryOp::Equality(op) => match op {
                            CompEqualityOp::Equal => {
                                temp_instructions.push(Instr::JumpEqual(true_label.to_owned()));
                            }
                            CompEqualityOp::NotEqual => {
                                temp_instructions.push(Instr::JumpNotEqual(true_label.to_owned()));
                            }
                        },
                        // But, for inequalities, we only want (number, number)
                        ComparisonBinaryOp::Inequality(op) => {
                            instructions.push(Instr::Compare(Val::Reg(Reg::R11), Val::Imm(0)));

                            let second_success_label = new_label("check_success", counter);

                            instructions.push(Instr::JumpEqual(second_success_label.to_owned()));
                            call_error_fn(TYPE_MISMATCH, instructions);
                            instructions.push(Instr::Label(second_success_label));

                            match op {
                                CompInequalityOp::LessEqual => {
                                    temp_instructions
                                        .push(Instr::JumpLessEqual(true_label.to_owned()));
                                }
                                CompInequalityOp::GreaterEqual => {
                                    temp_instructions
                                        .push(Instr::JumpGreaterEqual(true_label.to_owned()));
                                }
                                CompInequalityOp::Less => {
                                    temp_instructions.push(Instr::JumpLess(true_label.to_owned()));
                                }
                                CompInequalityOp::Greater => {
                                    temp_instructions
                                        .push(Instr::JumpGreater(true_label.to_owned()));
                                }
                            }
                        }
                    }

                    instructions.extend(temp_instructions);
                    instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(FALSE)));
                    instructions.push(Instr::Jump(done_label.to_owned()));
                    instructions.push(Instr::Label(true_label));
                    instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(TRUE)));
                    instructions.push(Instr::Label(done_label));
                }
            }
        }
        Expr::UnOp(op_type, e) => {
            compile_helper(e, si, instructions, environment, counter, loop_label)?;
            let unary_success_lbl = new_label("unary_success", counter);

            // First, let's figure out what type is in rax. We can move rax into r10,
            // and then AND r10 with 1 to see if we have 1 (bool) or 0 (num).
            instructions.push(Instr::IMov(Val::Reg(Reg::R10), Val::Reg(Reg::Rax)));
            instructions.push(Instr::BitwiseAnd(Val::Reg(Reg::R10), Val::Imm(1)));
            match op_type {
                // For arithmetic operations, we only want number inputs
                Op1::Arith(op) => {
                    // if r10 is 1, then we have boolean
                    // otherwise, we have number
                    instructions.push(Instr::Compare(Val::Reg(Reg::R10), Val::Imm(0)));
                    instructions.push(Instr::JumpEqual(unary_success_lbl.to_owned()));
                    call_error_fn(TYPE_MISMATCH, instructions);
                    instructions.push(Instr::Label(unary_success_lbl));

                    match op {
                        ArithmeticUnaryOp::Add1 => {
                            instructions.push(Instr::IAdd(Val::Reg(Reg::Rax), Val::Imm(0b10)))
                        }
                        ArithmeticUnaryOp::Sub1 => {
                            instructions.push(Instr::ISub(Val::Reg(Reg::Rax), Val::Imm(0b10)))
                        }
                    }

                    // After the operation is complete, make sure there wasn't an overflow
                    let no_overflow_label = new_label("no_overflow", counter);
                    instructions.push(Instr::JumpNoOverflow(no_overflow_label.to_owned()));
                    call_error_fn(OVERFLOW_ERROR, instructions);
                    instructions.push(Instr::Label(no_overflow_label));
                }
                // For comparison operations, we allow any input types.
                Op1::Comp(op) => {
                    let unary_comp_else_lbl = new_label("unary_comp_else_lbl", counter);
                    let unary_comp_done_lbl = new_label("unary_comp_done_lbl", counter);
                    let (r1, r2) = match op {
                        ComparisonUnaryOp::IsNum => (TRUE, FALSE),
                        ComparisonUnaryOp::IsBool => (FALSE, TRUE),
                    };

                    instructions.push(Instr::Compare(Val::Reg(Reg::R10), Val::Imm(1)));
                    instructions.push(Instr::JumpEqual(unary_comp_else_lbl.to_owned()));
                    instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(r1)));
                    instructions.push(Instr::Jump(unary_comp_done_lbl.to_owned()));
                    instructions.push(Instr::Label(unary_comp_else_lbl));
                    instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(r2)));
                    instructions.push(Instr::Label(unary_comp_done_lbl));
                }
            }
        }
        Expr::Id(identifier) => {
            // If we have an identifier by itself, we need to get the
            // address associated with that identifier, and then
            // get the value located at that address. Once we do that,
            // we store that value in RAX.
            match environment.get(identifier) {
                // If we have an identifier, all we need to do is get its value and put it
                // in RAX so it can be used by whoever called this function.
                Some(s) => instructions.push(Instr::IMov(
                    Val::Reg(Reg::Rax),
                    Val::RegOffset(Reg::Rsp, *s),
                )),
                None => return Err(CompileError::UnboundIdentifier(identifier)),
            };
        }
        Expr::Let(bindings, e) => {
            let mut new_env = environment.clone();
            let mut new_si = si;

            // The idea is as follows:
            //
            // For each binding, (identifier, expression),
            // - Evaluate the given expression and put it into register RAX
            //   (note that each concrete value, i.e., returns a value directly
            //   as opposed to returning recursive call, moves some value into RAX).
            // - Add an instruction to move the value stored in RAX to
            //   some place in the stack (indicated by new_si * 8, the
            //   address, or just the stack offset)
            // - Update the environment (all of our variables) to include
            //   our new identifier
            // - Increment the stack index so we don't accidently overwrite
            //   the stored value
            for (id, exp) in bindings {
                // First, compile the expression associated with the identifier (the
                // binding). After this is done, the last element in `instructions`
                // should update the RAX register with the result of compiling the
                // bound expression
                compile_helper(exp, new_si, instructions, &new_env, counter, loop_label)?;

                // Next, move the value stored in RAX to some place in the stack. We'll
                // use new_si * 8 to indicate the location where this value should be stored
                // in the stack.
                instructions.push(Instr::IMov(
                    Val::RegOffset(Reg::Rsp, new_si * 8),
                    Val::Reg(Reg::Rax),
                ));

                // Update our environments map with the identifier and its stack offset/
                // location.
                new_env = new_env.update(id, new_si * 8);

                // Increment the stack index so we don't accidently overwrite the value
                // we just stored (when we end up calling the compile function again).
                new_si += 1;
            }

            // Finally, we can evaluate the resulting expression (the one
            // following the let bindings).
            compile_helper(e, new_si, instructions, &new_env, counter, loop_label)?;
        }
        Expr::Bool(b) => {
            instructions.push(Instr::IMov(
                Val::Reg(Reg::Rax),
                Val::Imm(if *b { TRUE } else { FALSE }),
            ));
        }
        Expr::If(cond_expr, then_cond, else_cond) => {
            let if_false_label = new_label("false_lbl", counter);
            let done_label = new_label("done_lbl", counter);

            compile_helper(
                cond_expr,
                si,
                instructions,
                environment,
                counter,
                loop_label,
            )?;
            // If Rax does NOT resolves to the value TRUE, then
            // - Jump to the else branch and run that
            // - Otherwise, fall through to the true branch, run that, and then jump to the
            //   done branch.
            instructions.push(Instr::Compare(Val::Reg(Reg::Rax), Val::Imm(FALSE)));
            instructions.push(Instr::JumpEqual(if_false_label.to_owned()));
            compile_helper(
                then_cond,
                si,
                instructions,
                environment,
                counter,
                loop_label,
            )?;
            instructions.push(Instr::Jump(done_label.to_owned()));
            instructions.push(Instr::Label(if_false_label));
            compile_helper(
                else_cond,
                si,
                instructions,
                environment,
                counter,
                loop_label,
            )?;
            instructions.push(Instr::Label(done_label));
        }
        Expr::Set(identifier, expr) => match environment.get(identifier) {
            Some(loc) => {
                compile_helper(expr, si, instructions, environment, counter, loop_label)?;
                instructions.push(Instr::IMov(
                    Val::RegOffset(Reg::Rsp, *loc),
                    Val::Reg(Reg::Rax),
                ));
            }
            None => return Err(CompileError::UnboundIdentifier(identifier)),
        },
        Expr::Block(statements) => {
            for s in statements {
                compile_helper(s, si, instructions, environment, counter, loop_label)?;
            }
        }
        Expr::Loop(expr) => {
            let loop_start = new_label("loop_start", counter);
            let loop_done = format!("loop_break_done#{loop_start}");

            instructions.push(Instr::Label(loop_start.to_owned()));
            compile_helper(
                expr,
                si,
                instructions,
                environment,
                counter,
                Some(&loop_done),
            )?;
            instructions.push(Instr::Jump(loop_start));
            instructions.push(Instr::Label(loop_done));
        }
        Expr::Break(expr) => {
            match loop_label {
                Some(label) => {
                    compile_helper(expr, si, instructions, environment, counter, loop_label)?;
                    instructions.push(Instr::Jump(label.to_owned()));
                }
                None => return Err(CompileError::NoLoopForBreak),
            };
        }
        Expr::Input => instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Reg(Reg::Rdi))),
    };

    Ok(())
}

/// Creates a new label and increments the label counter.
///
/// # Parameters
/// - `s`: The label name.
/// - `label_counter`: The label counter.
///
/// # Returns
/// The name of the label.
pub(crate) fn new_label(s: &str, label_counter: &mut u64) -> String {
    let curr = *label_counter;
    *label_counter += 1;
    format!("{s}_{curr}")
}

/// Generates assembly that just moves the error code into `rax` and then jumps
/// to the `throw_error` label.
///
/// # Parameters
/// - `error_code`: The error code.
/// - `instructions`: The instructions.
#[inline(always)]
pub(crate) fn call_error_fn(error_code: i64, instructions: &mut Vec<Instr>) {
    instructions.push(Instr::IMov(Val::Reg(Reg::Rax), Val::Imm(error_code)));
    instructions.push(Instr::Jump(THROW_ERROR_LABEL.to_owned()));
}
