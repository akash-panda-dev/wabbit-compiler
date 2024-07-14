use crate::model::{
    statements::{Assignment, Declaration, Stmt},
    Program,
};

use super::pass::Pass;

pub struct DeInit;

impl Pass for DeInit {
    fn transform_program(program: Program) -> Program {
        let mut stmts: Vec<Stmt> = vec![];

        for stmt in program.stmts {
            let new_stmt = Self::transform_stmt(stmt);

            if let Stmt::Assignment(Assignment {
                var,
                value,
                is_decl_and_init: true,
            }) = new_stmt
            {
                // This is a declaration and initialization, so we need to split it
                let declaration = Declaration { var: var.clone() }.into();
                let assignment = Stmt::Assignment(Assignment {
                    var,
                    value,
                    is_decl_and_init: false,
                });
                stmts.push(declaration);
                stmts.push(assignment);
            } else {
                stmts.push(new_stmt);
            }
        }

        Program { stmts }
    }
}
