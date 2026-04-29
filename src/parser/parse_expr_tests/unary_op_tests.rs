use super::*;

#[cfg(test)]
mod unary_op_tests {
    use super::*;

    #[test]
    fn test_unary_negate_int() {
        // integer literals negated shouldn't produce unary negate, but instead just the literal
        // its self.
        match parse("-5").unwrap() {
            Expr::IntLiteral { value, .. } => {
                assert!(matches!(value, IntLiteralValue::Int8(-5)));
            }
            other => panic!("expected IntLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_float() {
        // float literals negated shouldn't produce unary negate, but instead just the literal
        // its self.
        match parse("-3.14").unwrap() {
            Expr::Float64Literal { value, .. } => {
                assert_eq!(value, -3.14);
            }
            other => panic!("expected Float64Literal, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_variable() {
        match parse("-foo").unwrap() {
            Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                assert!(matches!(*expr, Expr::Var { name, .. } if name == "foo"));
            }
            other => panic!("expected UnaryOp, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_alone_errors() {
        assert_parse_err("-");
    }

    #[test]
    fn test_unary_negate_whitespace_only_after_errors() {
        assert_parse_err("-   ");
    }
    
    #[test]
    fn test_negate_function_result() {
        match parse("-foo(1)").unwrap() {
            Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                assert!(matches!(*expr, Expr::Call { name, .. } if name == "foo"));
            }
            other => panic!("expected UnaryOp, got {:?}", other),
        }
    }



}
