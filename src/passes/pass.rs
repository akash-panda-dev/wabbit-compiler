#![allow(unused)]
#![allow(unused_variables)]
use crate::model::{
    expressions::{BinOp, Expr, FuncCall, Number, RelOp, Variable},
    statements::{Assignment, Declaration, Func, If, Print, Return, Stmt, While},
    Program,
};

pub trait Pass {
    fn transform_program(program: Program) -> Program {
        let transformed_stmts: Vec<Stmt> = program
            .stmts
            .into_iter()
            .map(Self::transform_stmt)
            .collect();

        Program {
            stmts: transformed_stmts,
        }
    }

    fn transform_stmt(stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Func(func) => Self::transform_func(func),
            Stmt::If(r#if) => Self::transform_if(r#if),
            Stmt::While(r#while) => Self::transform_while(r#while),
            Stmt::Print(print) => Self::transform_print(print),
            Stmt::Assignment(varcreate) => Self::transform_assignment(varcreate),
            Stmt::Declaration(declaration) => Self::transform_declaration(declaration),
            Stmt::Return(r#return) => Self::transform_return(r#return),
        }
    }

    fn transform_expr(expr: Expr) -> Expr {
        match expr {
            Expr::FuncCall(func_call) => Self::transform_func_call(func_call),
            Expr::Variable(variable) => Self::transform_variable(variable),
            Expr::BinOp(binop) => Self::transform_binop(binop),
            Expr::RelOp(relop) => Self::transform_relop(relop).into(),
            Expr::Number(number) => Self::transform_number(number),
        }
    }

    fn transform_func(func: Func) -> Stmt {
        let args = func
            .args
            .into_iter()
            .map(Self::transform_variable)
            .map(|expr| match expr {
                Expr::Variable(v) => v,
                _ => panic!("Expected Expr::Name, but got a different variant"),
            })
            .collect();
        let body = func.body.into_iter().map(Self::transform_stmt).collect();

        Func::new(&func.name, args, body).into()
    }

    fn transform_while(r#while: While) -> Stmt {
        let condition = Self::transform_relop(r#while.condition);
        let body = r#while.body.into_iter().map(Self::transform_stmt).collect();
        While::new(condition, body).into()
    }

    fn transform_if(r#if: If) -> Stmt {
        let condition = Self::transform_relop(r#if.condition);
        let consequence = r#if
            .consequence
            .into_iter()
            .map(Self::transform_stmt)
            .collect();
        let alternative = r#if
            .alternative
            .into_iter()
            .map(Self::transform_stmt)
            .collect();

        If::new(condition, consequence, alternative).into()
    }

    fn transform_print(print: Print) -> Stmt {
        let value = Self::transform_expr(print.value);

        Print { value }.into()
    }

    fn transform_return(r#return: Return) -> Stmt {
        let expr = Self::transform_expr(r#return.expr);
        Return::new(expr).into()
    }

    fn transform_assignment(assignment: Assignment) -> Stmt {
        let value = Self::transform_expr(assignment.value);
        Assignment {
            var: assignment.var,
            value,
            is_decl_and_init: assignment.is_decl_and_init,
        }
        .into()
    }

    fn transform_declaration(declaration: Declaration) -> Stmt {
        Declaration {
            var: declaration.var,
        }
        .into()
    }

    fn transform_binop(binop: BinOp) -> Expr {
        let left = Self::transform_expr(*binop.left);
        let right = Self::transform_expr(*binop.right);
        BinOp::new(binop.op, left, right).into()
    }

    fn transform_func_call(func_call: FuncCall) -> Expr {
        let args = func_call
            .args
            .into_iter()
            .map(Self::transform_expr)
            .collect::<Vec<Expr>>();

        FuncCall {
            name: func_call.name,
            args,
        }
        .into()
    }

    fn transform_variable(variable: Variable) -> Expr {
        variable.into()
    }

    fn transform_relop(relop: RelOp) -> RelOp {
        let left = Self::transform_expr(*relop.left);
        let right = Self::transform_expr(*relop.right);

        RelOp::new(relop.op, left, right)
    }

    fn transform_number(number: Number) -> Expr {
        number.into()
    }
}
