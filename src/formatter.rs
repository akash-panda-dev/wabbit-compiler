// #![allow(unused)]
// #![allow(unused_variables)]
// use std::ops::Add;

// use crate::model::*;

// struct Formatter;

// impl Formatter {
//     fn format(&self, program: Program<String>) -> String {
//         program
//             .stmts
//             .into_iter()
//             .map(|s| s.accept(self))
//             .collect::<Vec<String>>()
//             .join("\n")
//     }
// }

// impl Visitor<String> for Formatter {
//     fn visit_binop(&self, binop: &BinOp<String>) -> String {
//         match binop.op {
//             Operation::Add => {
//                 format!("{} + {}", binop.left.accept(self), binop.right.accept(self))
//             }
//             Operation::Mul => {
//                 format!("{} * {}", binop.left.accept(self), binop.right.accept(self))
//             }
//         }
//     }

//     fn visit_func(&self, func: &Func<String>) -> String {
//         String::new()
//     }

//     fn visit_if(&self, if_stmt: &If<String>) -> String {
//         String::new()
//     }

//     fn visit_number(&self, number: &Number) -> String {
//         match number {
//             Number::Int(val) => val.to_string(),
//         }
//     }

//     fn visit_print(&self, print: &Print<String>) -> String {
//         format!("print {}", print.value.accept(self))
//     }

//     fn visit_return(&self, ret: &Return<String>) -> String {
//         String::new()
//     }

//     fn visit_while(&self, while_stmt: &While<String>) -> String {
//         String::new()
//     }

//     fn visit_funccall(&self, funccall: &FuncCall<String>) -> String {
//         String::new()
//     }

//     fn visit_relop(&self, relop: &RelOp<String>) -> String {
//         String::new()
//     }

//     fn visit_variable(&self, variable: &Variable) -> String {
//         String::new()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::model::*;

//     #[cfg(test)]
//     #[test]
//     fn test_format_binop_program() {
//         let program = Program {
//             stmts: vec![Box::new(Print {
//                 value: Box::new(BinOp {
//                     op: Operation::Add,
//                     left: Box::new(Number::Int(2)),
//                     right: Box::new(Number::Int(3)),
//                 }),
//             })],
//         };
//         let formatter = Formatter;
//         assert_eq!(formatter.format(program), "print 2 + 3");
//     }
// }
