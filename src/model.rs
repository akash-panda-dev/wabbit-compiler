#![allow(unused)]
// TODO: Remove this

use std::{
    error::Error,
    fmt::{write, Display},
    marker::PhantomData,
    str::FromStr,
};

use statements::Stmt;

use crate::passes::{constant_folding::ConstantFolding, pass::Pass};

#[macro_export]
macro_rules! vars {
    ($($x:expr),*) => {
        vec![$(Variable::from($x)), *]
    };
}

// stmts!(Print::new(Number::Int(2)), Print::new(Number::Int(3)))
// vec![Print::new(Number::Int(2)).into(), Print::new(Number::Int(3)).into()]
#[macro_export]
macro_rules! stmts {
    ($($x:expr),*) => {
        vec![$($x.into()), *]
    }
    }

#[macro_export]
macro_rules! exprs {
    ($($x:expr),*) => {
       vec![$($x.into()), *]
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinOperator {
    Add,
    Mul,
}

impl FromStr for BinOperator {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(BinOperator::Add),
            "*" => Ok(BinOperator::Mul),
            _ => Err(format!("Invalid binary operator: {}", s).into()),
        }
    }
}

impl Display for BinOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOperator::Add => write!(f, "+"),
            BinOperator::Mul => write!(f, "*"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelOperator {
    Equal,
    LessThan,
}

impl Display for RelOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelOperator::Equal => write!(f, "=="),
            RelOperator::LessThan => write!(f, "<"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub mod statements {
    use std::marker::PhantomData;

    use wabbit_proc_macros::FromAST;

    use super::expressions::{Expr, RelOp, Variable};

    #[derive(FromAST, PartialEq, Eq, Debug)]
    pub enum Stmt {
        Func(Func),
        If(If),
        While(While),
        Print(Print),
        Return(Return),
        Assignment(Assignment),
        Declaration(Declaration),
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Print {
        pub value: Expr,
    }

    impl Print {
        pub fn new(value: impl Into<Expr>) -> Self {
            Print {
                value: value.into(),
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Assignment {
        pub var: Variable,
        pub value: Expr,
        pub is_decl_and_assign: bool,
    }

    impl Assignment {
        pub fn new_var(var: Variable, value: impl Into<Expr>) -> Self {
            Assignment {
                var,
                value: value.into(),
                is_decl_and_assign: true,
            }
        }

        pub fn assign_var(var: Variable, value: impl Into<Expr>) -> Self {
            Assignment {
                var,
                value: value.into(),
                is_decl_and_assign: false,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Declaration {
        pub var: Variable,
    }

    impl Declaration {
        pub fn new(var: Variable) -> Self {
            Declaration { var }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Return {
        pub value: Expr,
    }

    impl Return {
        pub fn new(value: impl Into<Expr>) -> Self {
            Return { value: value.into() }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Func {
        pub name: String,
        pub args: Vec<Variable>,
        pub body: Vec<Stmt>,
    }

    impl Func {
        pub fn new(name: &str, args: Vec<Variable>, body: Vec<Stmt>) -> Self {
            Func {
                name: name.to_string(),
                args,
                body,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct If {
        pub condition: RelOp,
        pub consequence: Vec<Stmt>,
        pub alternative: Vec<Stmt>,
    }

    impl If {
        pub fn new(condition: RelOp, consequence: Vec<Stmt>, alternative: Vec<Stmt>) -> Self {
            If {
                condition,
                consequence,
                alternative,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct While {
        pub condition: RelOp,
        pub body: Vec<Stmt>,
    }

    impl While {
        pub fn new(condition: RelOp, body: Vec<Stmt>) -> Self {
            While { condition, body }
        }
    }
}

pub mod expressions {
    use std::{error::Error, str::FromStr};

    use wabbit_proc_macros::FromAST;

    use super::{BinOperator, RelOperator};

    #[derive(FromAST, PartialEq, Eq, Debug)]
    pub enum Expr {
        FuncCall(FuncCall),
        Variable(Variable),
        BinOp(BinOp),
        RelOp(RelOp),
        Number(Number),
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct FuncCall {
        pub name: String,
        pub args: Vec<Expr>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Scope {
        LOCAL,
        GLOBAL,
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct Variable {
        pub name: String,
        pub scope: Option<Scope>,
    }

    impl Variable {
        pub fn new_global(value: &str) -> Self {
            Variable {
                name: value.to_string(),
                scope: Some(Scope::GLOBAL),
            }
        }

        pub fn new_local(value: &str) -> Self {
            Variable {
                name: value.to_string(),
                scope: Some(Scope::LOCAL),
            }
        }
    }

    impl From<&str> for Variable {
        fn from(value: &str) -> Self {
            Variable {
                name: value.to_string(),
                scope: None,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum Number {
        Int(isize),
    }

    impl From<isize> for Number {
        fn from(value: isize) -> Self {
            Number::Int(value)
        }
    }

    // Using Box<Expr> for left and right fields to manage the size of recursive Expr variants
    #[derive(PartialEq, Eq, Debug)]
    pub struct BinOp {
        pub op: BinOperator,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }

    impl BinOp {
        pub fn new(op: BinOperator, left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            BinOp {
                op,
                left: Box::new(left.into()),
                right: Box::new(right.into()),
            }
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct RelOp {
        pub op: RelOperator,
        pub left: Box<Expr>,
        pub right: Box<Expr>,
    }

    impl RelOp {
        pub fn new(op: RelOperator, left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
            RelOp {
                op,
                left: Box::new(left.into()),
                right: Box::new(right.into()),
            }
        }
    }
}
