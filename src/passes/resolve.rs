use crate::model::{
    expressions::{Expr, Variable},
    statements::{Assignment, Declaration, Func, Stmt},
};
use scope_ctx::{GlobalScopeCtx, LocalScopeCtx};

use super::pass::Pass;


/// This Resolve Pass is used to assign scopes to all variables. There are 2 scopes
/// Global and Local. All variables in the top level of the code are assigned a global scope
/// and the variables inside functions or any kind of scope are assigned a local scope.
/// Since we recursively walk the tree, we need to pass some context to each AST node for it to be able to know
/// which context it is in.
/// 
/// So we do exactly that, declaration and function args are used to define the scopes of the variables 
/// and assignment or variable usage is used to get the scope from the context which is a HashMap of the
/// variable name in String to the Variable AST.
/// Local scopes can have parents since we can have nested scopes. Global scope doesn't have a parent so its written 
/// as a separate struct.
/// 
/// If a variable is not present in the LocalScope, then we check it's parent.



pub struct Resolve;

#[derive(Clone)]
pub enum ScopeCtx {
    Global(GlobalScopeCtx),
    Local(LocalScopeCtx),
}

impl ScopeCtx {
    pub fn new_global() -> Self {
        ScopeCtx::Global(GlobalScopeCtx::new())
    }

    pub fn new_local(parent: Option<ScopeCtx>) -> Self {
        ScopeCtx::Local(LocalScopeCtx::new(parent))
    }

    pub fn add_to_scope(&mut self, variable_name: &str) {
        match self {
            ScopeCtx::Global(ctx) => ctx.add_to_scope(variable_name),
            ScopeCtx::Local(ctx) => ctx.add_to_scope(variable_name),
        }
    }

    pub fn get_from_scope(&self, variable_name: &str) -> Variable {
        match self {
            ScopeCtx::Global(ctx) => ctx.get_from_scope(variable_name),
            ScopeCtx::Local(ctx) => ctx.get_from_scope(variable_name),
        }
    }
}
mod scope_ctx {
    use super::ScopeCtx;
    use crate::model::expressions::Variable;
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct GlobalScopeCtx {
        variables: HashMap<String, Variable>,
    }

    impl GlobalScopeCtx {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            GlobalScopeCtx {
                variables: HashMap::new(),
            }
        }

        pub fn add_to_scope(&mut self, variable_name: &str) {
            self.variables.insert(
                variable_name.to_string(),
                Variable::new_global(variable_name),
            );
        }

        pub fn get_from_scope(&self, variable_name: &str) -> Variable {
            match self.variables.get(variable_name) {
                Some(variable) => variable.clone(),
                None => panic!("Unadd_to_scoped variable: {}", variable_name),
            }
        }
    }

    #[derive(Clone)]
    pub struct LocalScopeCtx {
        variables: HashMap<String, Variable>,
        parent: Box<Option<ScopeCtx>>,
    }

    impl LocalScopeCtx {
        pub fn new(parent: Option<ScopeCtx>) -> Self {
            LocalScopeCtx {
                variables: HashMap::new(),
                parent: Box::new(parent),
            }
        }

        pub fn add_to_scope(&mut self, variable_name: &str) {
            self.variables.insert(
                variable_name.to_string(),
                Variable::new_local(variable_name),
            );
        }

        pub fn get_from_scope(&self, variable_name: &str) -> Variable {
            match self.variables.get(variable_name) {
                Some(variable) => variable.clone(),
                None => match *self.parent {
                    Some(ref parent_ctx) => parent_ctx.get_from_scope(variable_name),
                    None => panic!("Unadd_to_scoped variable: {}", variable_name),
                },
            }
        }
    }
}

impl Pass<ScopeCtx> for Resolve {
    fn get_context() -> ScopeCtx {
        ScopeCtx::new_global()
    }

    fn transform_func(func: Func, ctx: &mut ScopeCtx) -> Stmt {
        func.args.iter().for_each(|arg| ctx.add_to_scope(&arg.name));

        let mut func_ctx = ScopeCtx::new_local(Some(ctx.clone()));

        let body: Vec<Stmt> = func
            .body
            .into_iter()
            .map(|stmt| Self::transform_stmt(stmt, &mut func_ctx))
            .collect();

        Func::new(&func.name, func.args, body).into()
    }

    fn transform_declaration(declaration: Declaration, ctx: &mut ScopeCtx) -> Stmt {
        let var_name = declaration.var.name;
        ctx.add_to_scope(&var_name);
        Declaration::new(ctx.get_from_scope(&var_name)).into()
    }

    fn transform_variable(variable: Variable, ctx: &mut ScopeCtx) -> Expr {
        ctx.get_from_scope(&variable.name).into()
    }

    #[allow(clippy::bool_comparison)]
    fn transform_assignment(assignment: Assignment, ctx: &mut ScopeCtx) -> Stmt {
        assert!(assignment.is_decl_and_assign != true);

        let var_name = assignment.var.name;
        Assignment::assign_var(
            ctx.get_from_scope(&var_name),
            Self::transform_expr(assignment.value, ctx),
        )
        .into()
    }
}
