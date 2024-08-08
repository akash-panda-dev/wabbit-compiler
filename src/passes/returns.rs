use crate::model::{expressions::Number, statements::*};

use super::pass::Pass;

pub struct Returns;

impl Pass for Returns {
    fn get_context() {}

    fn transform_stmts_body(stmts: Vec<Stmt>, _ctx: &mut ()) -> Vec<Stmt> {
        let mut new_stmts = vec![];

        for stmt in stmts {
            match stmt {
                Stmt::Func(Func { name, args, body })
                    if body.last().map_or(true, |s| !matches!(s, Stmt::Return(_))) =>
                {
                    let mut new_body = body;
                    new_body.push(Return::new(Number::Int(0)).into());
                    new_stmts.push(Stmt::Func(Func {
                        name,
                        args,
                        body: new_body,
                    }));
                }
                _ => new_stmts.push(stmt),
            }
        }

        new_stmts
    }
}
