use super::*;

#[cfg(test)]
mod parentheses_grouping_tests {
    use super::*;

    #[test]
    fn test_parens_simple() {
        assert_int_literal("(5)", IntLiteralValue::Int8(5));
    }

    #[test]
    fn test_parens_nested() {
        assert_int_literal("((5))", IntLiteralValue::Int8(5));
    }

    #[test]
    fn test_parens_wrapping_binop() {
        match parse("(1 + 2)").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, .. } => {}
            other => panic!("expected BinOp Add, got {:?}", other),
        }
    }

    #[test]
    fn test_parens_partial_wrap_not_treated_as_group() {
        // (1 + 2) * 3
        // outer parens don't wrap the whole expression
        match parse("(1 + 2) * 3").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, .. } => {}
            other => panic!("expected BinOp Multiply at top level, got {:?}", other),
        }
    }

    #[test]
    fn test_deeply_nested_parens() {
        // (((x))) should resolve to a Var
        match parse("(((x)))").unwrap() {
            Expr::Var { name, .. } => assert_eq!(name, "x"),
            other => panic!("expected Var, got {:?}", other),
        }
    }

}
