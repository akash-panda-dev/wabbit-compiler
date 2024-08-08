use wabbit_compiler::{
    ast_printer::format_program,
    model::Program,
    passes::{
        constant_folding::ConstantFolding, deinit::DeInit, pass::Pass, resolve::Resolve,
        returns::Returns, unscript::Unscript,
    },
};

#[allow(dead_code)]
type CompilerPassFn = fn(Program) -> Program;

#[allow(dead_code)]
fn compile(program: Program) -> String {
    let passes: Vec<(&str, CompilerPassFn)> = vec![
        ("Constant Folding", ConstantFolding::transform_program),
        ("DeInitialization", DeInit::transform_program),
        ("Resolve Scopes", Resolve::transform_program),
        ("Unscript", Unscript::transform_program),
        ("Returns", Returns::transform_program),
    ];
    let mut program = program;
    let mut compiled_code = String::new();

    println!("Source code:\n{}", format_program(&program));

    for (pass_name, pass) in passes {
        program = pass(program);
        compiled_code = format_program(&program);
        println!("After pass: {}", pass_name);
        println!("{}", compiled_code)
    }

    compiled_code
}

fn main() {}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use wabbit_compiler::{
        exprs,
        model::{
            expressions::{BinOp, FuncCall, Number, RelOp, Variable},
            statements::{Assignment, Func, If, Print, Return, While},
            BinOperator, Program, RelOperator,
        },
        stmts, vars,
    };

    use crate::compile;

    #[test]
    fn test_compile() {
        /*
        var x = 10;
        x = x * 1;
        print (23 * 45) + x;
        */
        let program1 = Program {
            stmts: stmts![
                Assignment::new_var("x".into(), Number::Int(10)),
                Assignment::assign_var(
                    "x".into(),
                    BinOp::new("*".parse().unwrap(), Variable::from("x"), Number::Int(1))
                ),
                Print::new(BinOp::new(
                    "+".parse().unwrap(),
                    BinOp::new("*".parse().unwrap(), Number::Int(23), Number::Int(45)),
                    Variable::from("x")
                ))
            ],
        };

        /*
        var x = 3;
        var y = 4;
        var min = 0;
        if x < y {
            min = x;
        } else {
            min = y;
        }
        print min;   // -> 3
        */
        let program2 = Program {
            stmts: stmts![
                Assignment::new_var("x".into(), Number::Int(3)),
                Assignment::new_var("y".into(), Number::Int(4)),
                Assignment::new_var("min".into(), Number::Int(0)),
                If::new(
                    RelOp::new(
                        RelOperator::LessThan,
                        Variable::from("x"),
                        Variable::from("y")
                    ),
                    stmts![Assignment::assign_var("min".into(), Variable::from("x"))],
                    stmts![Assignment::assign_var("min".into(), Variable::from("y"))]
                ),
                Print::new(Variable::from("min"))
            ],
        };

        /*
        var result = 1;
        var x = 1;
        while x < 10 {
            result = result * x;
            x = x + 1;
        }
        print result;   // -> 362880
        */
        let program3 = Program {
            stmts: stmts![
                Assignment::new_var("result".into(), Number::Int(1)),
                Assignment::new_var("x".into(), Number::Int(1)),
                While::new(
                    RelOp::new(RelOperator::LessThan, Variable::from("x"), Number::Int(10)),
                    stmts![
                        Assignment::assign_var(
                            "result".into(),
                            BinOp::new(
                                BinOperator::Mul,
                                Variable::from("result"),
                                Variable::from("x")
                            )
                        ),
                        Assignment::assign_var(
                            "x".into(),
                            BinOp::new(BinOperator::Add, Variable::from("x"), Number::Int(1))
                        )
                    ]
                ),
                Print::new(Variable::from("result"))
            ],
        };

        /*
        func add1(x) {
            x = x + 1;
            return x;
        }

        var x = 10;
        print (23 * 45) + add1(x);   // -> 1046
        print x;                     // -> 10
        */
        let program4 = Program {
            stmts: stmts![
                Func::new(
                    "add1",
                    vars!("x"),
                    stmts![
                        Assignment::assign_var(
                            "x".into(),
                            BinOp::new(BinOperator::Add, Variable::from("x"), Number::Int(1))
                        ),
                        Return::new(Variable::from("x"))
                    ]
                ),
                Assignment::new_var("x".into(), Number::Int(10)),
                Print::new(BinOp::new(
                    BinOperator::Add,
                    BinOp::new(BinOperator::Mul, Number::Int(23), Number::Int(45)),
                    FuncCall {
                        name: "add1".to_string(),
                        args: exprs![Variable::from("x")]
                    }
                )),
                Print::new(Variable::from("x"))
            ],
        };

        insta::with_settings!({snapshot_suffix => "program1"}, {
            assert_snapshot!(compile(program1));
        });

        insta::with_settings!({snapshot_suffix => "program2"}, {
            assert_snapshot!(compile(program2))
        });

        insta::with_settings!({snapshot_suffix => "program3"}, {
            assert_snapshot!(compile(program3))
        });

        insta::with_settings!({snapshot_suffix => "program4"}, {
            assert_snapshot!(compile(program4))
        });
    }
}
