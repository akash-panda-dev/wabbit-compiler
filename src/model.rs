#![allow(unused)]
// TODO: Remove this

use wabbit_proc_macros::{Expression, Statement};

pub struct Program {
    pub stmts: Vec<Box<dyn Statement>>,
}

// pub trait Visitable<T> {
//     fn accept(&self, visitor: &dyn Visitor<T>) -> T;
// }

pub trait Expression {
    fn accept_transform(&self, transformer: &dyn Transformer) -> Box<dyn Expression>;
}
pub trait Statement {
    fn accept_transform(&self, transformer: &dyn Transformer) -> Box<dyn Statement>;
}

pub trait Transformer {
    fn visit_binop(&self, binop: &BinOp) -> Box<dyn Expression>
    {
        let left = binop.left.accept_transform(self);
        let right = binop.right.accept_transform(self);

        Box::new(BinOp {
            op: binop.op,
            left,
            right,
        })
    }

    // fn visit_number(&self, number: &Number) -> T;
    // fn visit_print(&self, print: &Print<T>) -> T;
    // fn visit_return(&self, ret: &Return<T>) -> T;
    // fn visit_if(&self, r#if: &If<T>) -> T;
    // fn visit_while(&self, r#while: &While<T>) -> T;
    // fn visit_relop(&self, relop: &RelOp<T>) -> T;
    // fn visit_funccall(&self, funccall: &FuncCall<T>) -> T;
    // fn visit_variable(&self, variable: &Variable) -> T;
    // fn visit_func(&self, variable: &Func<T>) -> T;
}

// #[derive(Statement)]
// pub struct Print<T> {
//     pub value: Box<dyn Expression<T>>,
// }

// #[derive(Statement)]
// pub struct Return<T> {
//     expr: Box<dyn Expression<T>>,
// }

// #[derive(Statement)]
// pub struct Func<T> {
//     name: String,
//     args: Vec<Variable>,
//     body: Vec<Box<dyn Statement<T>>>,
// }

// #[derive(Expression)]
// pub struct FuncCall<T> {
//     name: String,
//     args: Vec<Box<dyn Expression<T>>>,
// }

// #[derive(Expression)]
// pub struct Variable {
//     var: String,
// }

// #[derive(Statement)]
// pub struct If<T> {
//     condition: RelOp<T>,
//     consequence: Vec<Box<dyn Statement<T>>>,
//     alternative: Vec<Box<dyn Statement<T>>>,
// }

// #[derive(Statement)]
// pub struct While<T> {
//     condition: RelOp<T>,
//     body: Vec<Box<dyn Statement<T>>>,
// }

// #[derive(Expression)]
// pub enum Number {
//     Int(isize),
// }

// #[derive(Expression)]
pub struct BinOp {
    pub op: Operation,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for BinOp {
    fn accept_transform(&self, visitor: &dyn Transformer) -> Box<dyn Expression> {
        visitor.visit_binop(self)
    }
}

// #[derive(Expression)]
// pub struct RelOp<T> {
//     pub op: RelOperation,
//     pub left: Box<dyn Expression<T>>,
//     pub right: Box<dyn Expression<T>>,
// }

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
