#![allow(unused)]
// TODO: Remove this

use statements::Stmt;
use wabbit_proc_macros::{Expression, Statement};

pub struct Program {
    pub stmts: Vec<Stmt>,
}

mod statements {
    use super::expressions::{RelOp, Variable};
    pub enum Stmt {
        Func(Func),
        If(If),
        While(While),
    }

    pub struct Func {
        name: String,
        args: Vec<Variable>,
        body: Vec<Stmt>,
    }

    pub struct If {
        condition: RelOp,
        consequence: Vec<Stmt>,
        alternative: Vec<Stmt>,
    }

    pub struct While {
        condition: RelOp,
        body: Vec<Stmt>,
    }
}

mod expressions {
    pub enum Expr {
        Print(Print),
        Return(Return),
        FuncCall(FuncCall),
        Variable(Variable),
        BinOp(BinOp),
        RelOp(RelOp),
        Number(Number),
    }

    pub struct Print {
        pub value: Box<Expr>,
    }

    pub struct Return {
        expr: Box<Expr>,
    }

    pub struct FuncCall {
        name: String,
        args: Vec<Expr>,
    }

    pub struct Variable {
        var: String,
    }

    pub enum Number {
        Int(isize),
    }

    pub struct BinOp {
        pub op: Operation,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }

    pub struct RelOp {
        pub op: RelOperation,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }

    #[derive(Clone, Copy)]
    pub enum Operation {
        Add,
        Mul,
    }

    #[derive(Clone, Copy)]
    pub enum RelOperation {
        Equal,
        LessThan,
    }
}
