use crate::model::statements::{Func, Stmt};

use super::pass::Pass;

pub struct Unscript;

impl Pass for Unscript {
    fn get_context() {}

    fn transform_stmts_body(stmts: Vec<Stmt>, _ctx: &mut ()) -> Vec<Stmt> {
        let mut main_stmts: Vec<Stmt> = Vec::new();
        let mut new_stmts: Vec<Stmt> = Vec::new();

        for stmt in stmts {
            match stmt {
                Stmt::Declaration(_) | Stmt::Func(_) => new_stmts.push(stmt),
                _ => main_stmts.push(stmt),
            }
        }

        new_stmts.push(Func::new("main", vec![], main_stmts).into());

        new_stmts
    }
}
