#![allow(unused)]
#![allow(unused_variables)]
use crate::model::{
    expressions::{BinOp, Expr, FuncCall, Number, RelOp, Variable},
    statements::{Assignment, Declaration, Func, If, Print, Return, Stmt, While},
    Program,
};

/// The Context generic type allows pass implementors to pass custom context data
/// while traversing the AST. Since the context is implementation-specific, it is
/// specified as a type parameter in the Pass trait definition which is defaulted to . This enables
/// each pass to define and use its own context type as needed.
///
/// The default type for Context is the unit type so that Passes which don't need a context can
/// just set it as ()
pub trait Pass<Context = ()> {
    fn get_context() -> Context;

    fn transform_program(program: Program) -> Program {
        let mut _ctx = Self::get_context();
        let transformed_stmts: Vec<Stmt> = Self::transform_stmts_body(program.stmts, &mut _ctx);

        Program {
            stmts: transformed_stmts,
        }
    }

    fn transform_stmt(stmt: Stmt, _ctx: &mut Context) -> Stmt {
        match stmt {
            Stmt::Func(func) => Self::transform_func(func, _ctx),
            Stmt::If(r#if) => Self::transform_if(r#if, _ctx),
            Stmt::While(r#while) => Self::transform_while(r#while, _ctx),
            Stmt::Print(print) => Self::transform_print(print, _ctx),
            Stmt::Assignment(varcreate) => Self::transform_assignment(varcreate, _ctx),
            Stmt::Declaration(declaration) => Self::transform_declaration(declaration, _ctx),
            Stmt::Return(r#return) => Self::transform_return(r#return, _ctx),
        }
    }

    fn transform_expr(expr: Expr, _ctx: &mut Context) -> Expr {
        match expr {
            Expr::FuncCall(func_call) => Self::transform_func_call(func_call, _ctx),
            Expr::Variable(variable) => Self::transform_variable(variable, _ctx),
            Expr::BinOp(binop) => Self::transform_binop(binop, _ctx),
            Expr::RelOp(relop) => Self::transform_relop(relop, _ctx).into(),
            Expr::Number(number) => Self::transform_number(number, _ctx),
        }
    }

    fn transform_func(func: Func, _ctx: &mut Context) -> Stmt {
        let args = func
            .args
            .into_iter()
            .map(|arg| Self::transform_variable(arg, _ctx))
            .map(|expr| match expr {
                Expr::Variable(v) => v,
                _ => panic!("Expected Expr::Name, but got a different variant"),
            })
            .collect();
        let body = Self::transform_stmts_body(func.body, _ctx);

        Func::new(&func.name, args, body).into()
    }

    fn transform_while(r#while: While, _ctx: &mut Context) -> Stmt {
        let condition = Self::transform_relop(r#while.condition, _ctx);
        let body = Self::transform_stmts_body(r#while.body, _ctx);
        While::new(condition, body).into()
    }

    fn transform_if(r#if: If, _ctx: &mut Context) -> Stmt {
        let condition = Self::transform_relop(r#if.condition, _ctx);
        let consequence = Self::transform_stmts_body(r#if.consequence, _ctx);
        let alternative = Self::transform_stmts_body(r#if.alternative, _ctx);

        If::new(condition, consequence, alternative).into()
    }

    fn transform_stmts_body(stmts: Vec<Stmt>, _ctx: &mut Context) -> Vec<Stmt> {
        stmts
            .into_iter()
            .map(|stmt| Self::transform_stmt(stmt, _ctx))
            .collect()
    }

    fn transform_print(print: Print, _ctx: &mut Context) -> Stmt {
        let value = Self::transform_expr(print.value, _ctx);

        Print { value }.into()
    }

    fn transform_return(r#return: Return, _ctx: &mut Context) -> Stmt {
        let expr = Self::transform_expr(r#return.value, _ctx);
        Return::new(expr).into()
    }

    fn transform_assignment(assignment: Assignment, _ctx: &mut Context) -> Stmt {
        let value = Self::transform_expr(assignment.value, _ctx);
        Assignment {
            var: assignment.var,
            value,
            is_decl_and_assign: assignment.is_decl_and_assign,
        }
        .into()
    }

    fn transform_declaration(declaration: Declaration, _ctx: &mut Context) -> Stmt {
        Declaration {
            var: declaration.var,
        }
        .into()
    }

    fn transform_binop(binop: BinOp, _ctx: &mut Context) -> Expr {
        let left = Self::transform_expr(*binop.left, _ctx);
        let right = Self::transform_expr(*binop.right, _ctx);
        BinOp::new(binop.op, left, right).into()
    }

    fn transform_func_call(func_call: FuncCall, _ctx: &mut Context) -> Expr {
        let args = func_call
            .args
            .into_iter()
            .map(|arg| Self::transform_expr(arg, _ctx))
            .collect::<Vec<Expr>>();

        FuncCall {
            name: func_call.name,
            args,
        }
        .into()
    }

    fn transform_variable(variable: Variable, _ctx: &mut Context) -> Expr {
        variable.into()
    }

    fn transform_relop(relop: RelOp, _ctx: &mut Context) -> RelOp {
        let left = Self::transform_expr(*relop.left, _ctx);
        let right = Self::transform_expr(*relop.right, _ctx);

        RelOp::new(relop.op, left, right)
    }

    fn transform_number(number: Number, _ctx: &mut Context) -> Expr {
        number.into()
    }
}
