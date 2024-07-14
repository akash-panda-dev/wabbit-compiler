#![allow(unused)]
#![allow(unused_variables)]
use std::env::var;
use std::ops::Add;

use crate::model::expressions::{BinOp, Number, RelOp};
use crate::model::statements::{Assignment, Func, If, Print, Return, While};
use crate::model::RelOperator;
use crate::model::{
    expressions::{Expr, FuncCall, Variable},
    statements::Stmt::{self, *},
    Program,
};

trait Indentable {
    const INDENT: &'static str;
    fn indent(&self) -> String;
}

impl Indentable for str {
    const INDENT: &'static str = "    ";

    fn indent(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let indented_newline = format!("\n{}", Self::INDENT);
        Self::INDENT.to_string() + &self.replace('\n', &indented_newline)
    }
}

pub fn format_program(program: &Program) -> String {
    program
        .stmts
        .iter()
        .map(format_stmt)
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_stmt(stmt: &Stmt) -> String {
    use Stmt::*;
    match stmt {
        Func(func) => format_func(func),
        If(r#if) => format_if(r#if),
        While(r#while) => format_while(r#while),
        Print(print) => format!("print {};", format_expr(&print.value)),
        Return(r#return) => format_return(r#return),
        Assignment(assignment) => {
            if assignment.is_decl_and_init {
                format!(
                    "var {} = {};",
                    assignment.var.name,
                    format_expr(&assignment.value)
                )
            } else {
                format!(
                    "{} = {};",
                    assignment.var.name,
                    format_expr(&assignment.value)
                )
            }
        }
        Declaration(declaration) => {
            format!("var {};", declaration.var.name)
        }
    }
}

fn format_expr(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        FuncCall(func_call) => format_func_call(func_call),
        Variable(variable) => variable.name.clone(),
        BinOp(binop) => format_binop(binop),
        RelOp(relop) => format!(
            "{} {} {}",
            format_expr(&relop.left),
            relop.op,
            format_expr(&relop.right)
        ),
        Number(e) => format_number(e),
    }
}

fn format_number(number: &Number) -> String {
    match number {
        Number::Int(i) => format!("{i}"),
    }
}

fn format_relop(relop: &RelOp) -> String {
    let left = format_expr(&relop.left);
    let right = format_expr(&relop.right);

    format!("{} {} {}", left, relop.op, right)
}

fn format_binop(binop: &BinOp) -> String {
    let left = if let Expr::BinOp(_) = *binop.left {
        format!("({})", format_expr(&binop.left))
    } else {
        format_expr(&binop.left)
    };

    let right = if let Expr::BinOp(_) = *binop.right {
        format!("({})", format_expr(&binop.right))
    } else {
        format_expr(&binop.right)
    };

    format!("{left} {} {right}", binop.op)
}

fn format_func(func: &Func) -> String {
    let args: String = func
        .args
        .iter()
        .map(|a| a.name.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let body: String = func
        .body
        .iter()
        .map(format_stmt)
        .collect::<Vec<_>>()
        .join("\n")
        .indent();

    format!("func {}({}) {{\n{}\n}}", func.name, args, body)
}

fn format_if(r#if: &If) -> String {
    let relop: String = format_relop(&r#if.condition);
    let consequent: String = r#if
        .consequence
        .iter()
        .map(format_stmt)
        .collect::<Vec<_>>()
        .join("\n")
        .indent();
    let alternative: String = r#if
        .alternative
        .iter()
        .map(format_stmt)
        .collect::<Vec<_>>()
        .join("\n")
        .indent();

    format!(
        "if {} {{\n{}\n}} else {{\n{}\n}}",
        relop, consequent, alternative
    )
}

fn format_while(r#while: &While) -> String {
    let relop = format_relop(&r#while.condition);
    let body = r#while
        .body
        .iter()
        .map(format_stmt)
        .collect::<Vec<_>>()
        .join("\n")
        .indent();

    format!("while {} {{\n{}\n}}", relop, body)
}

fn format_return(r#return: &Return) -> String {
    format!("return {}", format_expr(&r#return.expr))
}

fn format_func_call(func_call: &FuncCall) -> String {
    let args = func_call
        .args
        .iter()
        .map(format_expr)
        .collect::<Vec<_>>()
        .join(", ");

    format!("{}({})", func_call.name, args)
}

#[cfg(test)]
mod tests {
    use std::{process::Output, str::FromStr};

    use crate::{model::BinOperator, stmts, vars};

    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_format_bin_op() {
        let binop_case1 = BinOp::new(
            BinOperator::Add,
            Number::Int(2),
            BinOp::new(BinOperator::Mul, Number::Int(3), Number::Int(4)),
        );

        let binop_case2 = BinOp::new(
            BinOperator::Add,
            BinOp::new(BinOperator::Mul, Variable::from("x"), Number::Int(9)),
            BinOp::new(BinOperator::Mul, Number::Int(3), Number::Int(4)),
        );

        insta::with_settings!({snapshot_suffix => "case1"}, {
            assert_snapshot!(format_binop(&binop_case1))
        });

        insta::with_settings!({snapshot_suffix => "case2"}, {
            assert_snapshot!(format_binop(&binop_case2))
        });
    }

    #[test]
    fn test_format_func() {
        let func_abc = Func::new(
            "abc",
            vars!("a", "b", "c"),
            stmts![Print::new(BinOp::new(
                BinOperator::Mul,
                Variable::from("a"),
                BinOp::new(BinOperator::Add, Variable::from("b"), Variable::from("c"))
            ))],
        );

        let func_xyz = Func::new(
            "xyz",
            vars!("x", "y", "z"),
            stmts![While::new(
                RelOp::new(
                    RelOperator::LessThan,
                    Variable::from("x"),
                    Variable::from("y")
                ),
                stmts![Print::new(BinOp::new(
                    BinOperator::Mul,
                    Variable::from("x"),
                    BinOp::new(BinOperator::Add, Variable::from("y"), Variable::from("z"))
                ))]
            )],
        );

        let func_xyz_abc = Func::new(
            "xyz",
            vars!("x", "y", "z"),
            stmts![While::new(
                RelOp::new(
                    RelOperator::LessThan,
                    Variable::from("x"),
                    Variable::from("y")
                ),
                stmts![
                    Print::new(BinOp::new(
                        BinOperator::Mul,
                        Variable::from("x"),
                        BinOp::new(BinOperator::Add, Variable::from("y"), Variable::from("z"))
                    )),
                    Print::new(Number::Int(5))
                ]
            )],
        );

        insta::with_settings!({snapshot_suffix => "abc"}, {
            assert_snapshot!(format_func(&func_abc));
        });

        insta::with_settings!({snapshot_suffix => "xyz"}, {
            assert_snapshot!(format_func(&func_xyz));
        });

        insta::with_settings!({snapshot_suffix => "xyz_abc"}, {
            assert_snapshot!(format_func(&func_xyz_abc))
        });
    }
}
