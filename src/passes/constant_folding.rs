use crate::model::{
    expressions::{BinOp, Expr, Number},
    BinOperator,
};

use super::pass::Pass;

pub struct ConstantFolding;

impl Pass for ConstantFolding {
    fn transform_binop(binop: BinOp) -> Expr {
        match (binop.op, *binop.left, *binop.right) {
            (BinOperator::Add, Expr::Number(Number::Int(n1)), Expr::Number(Number::Int(n2))) => {
                Number::Int(n1 + n2).into()
            }
            (BinOperator::Mul, Expr::Number(Number::Int(n1)), Expr::Number(Number::Int(n2))) => {
                Number::Int(n1 * n2).into()
            }
            (op, left, right) => {
                BinOp::new(op, Self::transform_expr(left), Self::transform_expr(right)).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        model::expressions::{BinOp, Expr, Number, Variable},
        passes::{constant_folding::ConstantFolding, pass::Pass},
    };

    #[test]
    fn test_transform_binop_add_number() {
        let binop = BinOp::new("+".parse().unwrap(), Number::Int(1), Number::Int(3));
        let expected = Expr::from(Number::Int(4));

        let actual = ConstantFolding::transform_binop(binop);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_transform_binop_mul_number() {
        let binop = BinOp::new("*".parse().unwrap(), Number::Int(2), Number::Int(3));
        let expected = Expr::from(Number::Int(6));

        let actual = ConstantFolding::transform_binop(binop);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_transform_binop_add_variable() {
        let binop = BinOp::new("+".parse().unwrap(), Variable::from("a"), Number::Int(3));
        let expected: Expr =
            BinOp::new("+".parse().unwrap(), Variable::from("a"), Number::Int(3)).into();

        let actual = ConstantFolding::transform_binop(binop);

        assert_eq!(expected, actual);
    }
}
