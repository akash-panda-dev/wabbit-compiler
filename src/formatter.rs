#![allow(unused)]
#![allow(unused_variables)]
use std::ops::Add;

use crate::model::*;

struct Formatter;

impl Formatter {
    fn format(&self, program: Program) -> String {
        program
            .stmts
            .into_iter()
            .map(|s| s.accept(self))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
