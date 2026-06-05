use super::*;

#[cfg(test)]
mod unary_op_tests {
    use super::*;

    #[test]
    fn test_unary_logical_not_whitespace_only_after_errors() {
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {  
            assert_parse_err(&format!("!{}", spaces));

            spaces.push(' ');
        }
    }

    #[test]
    fn test_logical_not_all_literals_invalid_expr_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            if l.starts_with('-') {
                continue
            }
            assert_parse_err(&format!("!{} {}", l, l));
        }
    }



    #[test]
    fn test_logical_not_all_literals() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            match parse(&format!("!{}", l)).unwrap() {
                Expr::UnaryOp { op: UnaryOpKind::Not,.. } => {}
                other => panic!("expected UnaryOp NOT, got {:?}", other)
            }
        }
    }


    #[test]
    fn test_bitwise_not_all_literals_invalid_expr_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            if l.starts_with('-') {
                continue
            }
            assert_parse_err(&format!("~{} {}", l, l));
        }
    }




    #[test]
    fn test_bitwise_not_all_literals() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            match parse(&format!("~{}", l)).unwrap() {
                Expr::UnaryOp { op: UnaryOpKind::BitwiseNot,.. } => {}
                other => panic!("expected UnaryOp Bitwise NOT, got {:?}", other)
            }
        }
    }

    #[test]
    fn test_unary_bitwise_not_whitespace_only_after_errors() {
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {  
            assert_parse_err(&format!("~{}", spaces));

            spaces.push(' ');
        }
    }



    #[test]
    fn test_negate_all_literals() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            assert!(parse(&format!("-{}", l)).is_ok());
        }
    }

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
    fn test_unary_negate_string() {
        match parse("-\"hi\"").unwrap() {
            Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                match *expr {
                    Expr::StringLiteral { value, .. } => {
                        assert_eq!(value, "hi");
                    }
                    other => panic!("expected StringLiteral, got {:?}", other),
                }
            }
            other => panic!("expected UnaryOp, got {:?}", other),
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
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {  
            assert_parse_err(&format!("-{}", spaces));

            spaces.push(' ');
        }
    }
    
    #[test]
    fn test_negate_function_result() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals {
            match parse(&format!("-foo({})", l)).unwrap() {
                Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                    assert!(matches!(*expr, Expr::Call { name, .. } if name == "foo"));
                }
                other => panic!("expected UnaryOp, got {:?}", other),
            }
        }
    }


}
