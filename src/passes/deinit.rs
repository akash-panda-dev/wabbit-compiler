use crate::model::statements::{Assignment, Declaration, Stmt};

use super::pass::Pass;

pub struct DeInit;

impl Pass for DeInit {
    fn get_context() {}

    fn transform_stmts_body(stmts: Vec<Stmt>, _ctx: &mut ()) -> Vec<Stmt> {
        let mut new_stmts: Vec<Stmt> = vec![];

        for stmt in stmts {
            let new_stmt = Self::transform_stmt(stmt, &mut ());

            if let Stmt::Assignment(Assignment {
                var,
                value,
                is_decl_and_assign: true,
            }) = new_stmt
            {
                // This is a declaration and initialization, so we need to split it
                let declaration = Declaration::new(var.clone()).into();
                let assignment = Assignment::assign_var(var, value).into();
                new_stmts.push(declaration);
                new_stmts.push(assignment);
            } else {
                new_stmts.push(new_stmt);
            }
        }

        new_stmts
    }
}
