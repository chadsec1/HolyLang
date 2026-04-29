use super::*;

#[cfg(test)]
mod for_stmt_tests {
    use super::*; 
    
    // For statements
    #[test]
    fn for_statements_vars() {
        let stmts = parse_body("for i in x {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::Var { name, .. } = &f.value {
                assert_eq!(name, "x"); 
            } else { panic!("Expected Var expression") }
        } else {
            panic!("expected while statement");
        }
    }

    #[test]
    fn for_statements_literal() {
        let stmts = parse_body("for i in [12,\"hi\", true, 6.9, []] {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::ArrayLiteral { elements, .. } = &f.value {
                assert_eq!(elements.len(), 5);

                if let Expr::IntLiteral { value, .. } = &elements[0] {
                    assert!(matches!(value, IntLiteralValue::Int8(12)));
                } else { panic!("Expected IntLiteral"); }

                if let Expr::StringLiteral { value, .. } = &elements[1] {
                    assert_eq!(value, "hi");
                } else { panic!("Expected StringLiteral"); }

                if let Expr::BoolLiteral { value, .. } = &elements[2] {
                    assert_eq!(value, &true);
                } else { panic!("Expected BoolLiteral"); }

                if let Expr::Float64Literal { value, .. } = &elements[3] {
                    assert_eq!(*value, 6.9);
                } else { panic!("Expected Float64Literal"); }

                if let Expr::ArrayLiteral { elements, .. } = &elements[4] {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }


            } else {
                panic!("Expected ArrayLiteral");
            }

        } else {
            panic!("expected while statement");
        }
    }


    #[test]
    fn for_statements_2_holders_errors() {
        assert_parse_err(&wrap("for i v in x {\n\n}"));    
    }


    #[test]
    fn for_statements_2_values_errors() {
        assert_parse_err(&wrap("for i in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_2_holders_and_values_errors() {
        assert_parse_err(&wrap("for i v in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_no_value_errors() {
        assert_parse_err(&wrap("for i in {\n\n}"));    
    }


    #[test]
    fn for_statements_no_holder_errors() {
        assert_parse_err(&wrap("for in x {\n\n}"));    
    }

    #[test]
    fn for_statements_2_in() {
        assert_parse_err(&wrap("for i in in x {\n\n}"));    
        assert_parse_err(&wrap("for in i in x {\n\n}"));    
        assert_parse_err(&wrap("for i in x in {\n\n}"));    
        assert_parse_err(&wrap("for in i x in {\n\n}"));    
        assert_parse_err(&wrap("for i x in {\n\n}"));    
    }

    #[test]
    fn for_statements_no_in() {
        assert_parse_err(&wrap("for i x {\n\n}"));    
    }

    #[test]
    fn for_statements_no_holder_no_value_no_in_errors() {
        assert_parse_err(&wrap("for {\n\n}"));    
    }
}
