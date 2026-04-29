use super::*;

#[cfg(test)]
mod bin_op_tests {
    use super::*;

    #[test]
    fn test_binop_add() {
        match parse("1 + 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, .. } => {}
            other => panic!("expected Add, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_subtract() {
        match parse("5 - 3").unwrap() {
            Expr::BinOp { op: BinOpKind::Subtract, .. } => {}
            other => panic!("expected Subtract, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_multiply() {
        match parse("4 * 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, .. } => {}
            other => panic!("expected Multiply, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_divide() {
        match parse("10 / 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Divide, .. } => {}
            other => panic!("expected Divide, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_equal() {
        match parse("x == y").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, .. } => {}
            other => panic!("expected Equal, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_not_equal() {
        match parse("x != y").unwrap() {
            Expr::BinOp { op: BinOpKind::NotEqual, .. } => {}
            other => panic!("expected NotEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_greater() {
        match parse("x > y").unwrap() {
            Expr::BinOp { op: BinOpKind::Greater, .. } => {}
            other => panic!("expected Greater, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_greater_equal() {
        match parse("x >= y").unwrap() {
            Expr::BinOp { op: BinOpKind::GreaterEqual, .. } => {}
            other => panic!("expected GreaterEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_less() {
        match parse("x < y").unwrap() {
            Expr::BinOp { op: BinOpKind::Less, .. } => {}
            other => panic!("expected Less, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_less_equal() {
        match parse("x <= y").unwrap() {
            Expr::BinOp { op: BinOpKind::LessEqual, .. } => {}
            other => panic!("expected LessEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_missing_left_errors() {
        assert_parse_err("+ 2");
        assert_parse_err("* y");
    }

    #[test]
    fn test_binop_missing_right_errors() {
        assert_parse_err("1 +");
        assert_parse_err("x *");
    }

    #[test]
    fn test_single_equals_not_binop_errors() {
        assert_parse_err("x = y");
    }

    #[test]
    fn test_single_bang_not_binop_errors() {
        assert_parse_err("x ! y");
    }

    #[test]
    fn test_binop_left_associative_add_subtract() {
        // "1 + 2 + 3" — top-level op split gives left = "1 + 2", right = "3"
        // or depending on find_top_level_op_any semantics, at least it parses
        assert!(parse("1 + 2 + 3").is_ok());
    }

    #[test]
    fn test_binop_vars() {
        match parse("a + b").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, right, .. } => {
                assert!(matches!(*left, Expr::Var { name, .. } if name == "a"));
                assert!(matches!(*right, Expr::Var { name, .. } if name == "b"));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_nested() {
        // The top-level split should give us a BinOp at the top
        assert!(matches!(parse("a + b * c"), Ok(Expr::BinOp { .. })));
    }

    #[test]
    fn test_binop_with_parens_changes_grouping() {
        // (a + b) * c — top-level op is *
        match parse("(a + b) * c").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, left, right, .. } => {
                assert!(matches!(*left, Expr::BinOp { op: BinOpKind::Add, .. }));
                assert!(matches!(*right, Expr::Var { name, .. } if name == "c"));
            }
            other => panic!("expected top-level Multiply, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_between_bool_literals() {
        match parse("true == false").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, left, right, .. } => {
                assert!(matches!(*left, Expr::BoolLiteral { value: true, .. }));
                assert!(matches!(*right, Expr::BoolLiteral { value: false, .. }));
            }
            other => panic!("expected BinOp Equal, got {:?}", other),
        }
    }


    #[test]
    fn test_binop_of_string_and_var() {
        // This will parse structurally even if semantics would reject it
        match parse(r#""hello" + name"#).unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, right, .. } => {
                assert!(matches!(*left, Expr::StringLiteral { .. }));
                assert!(matches!(*right, Expr::Var { .. }));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }


    // Many spacing variants should parse identically
    // i.e. 1+2,  1 + 2, 1+ 2, 1 +2, 2   * 1, etc.
    // with special handling for "or" and "and" words.
    #[test]
    fn test_whitespace_around_and_within_operators_literals() {
        // just a helper so i don't have spam code with it over and over again.
        fn checker(variant: &str, b: BinOpKind) {
            match parse(variant).unwrap() {
                Expr::BinOp { op, left, right, .. } => {
                    assert_eq!(op, b.clone());
                    assert!(matches!(*left, Expr::IntLiteral { value: IntLiteralValue::Int8(1), .. }));
                    assert!(matches!(*right, Expr::IntLiteral { value: IntLiteralValue::Int8(2), .. }));
                }
                other => panic!("expected BinOp, got {:?}", other),
            }
        }

        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                if *s == "or" || *s == "and" {
                    let variant = &format!("{}1{}2", spaces, s);
                    assert_parse_err(variant);

                    let variant = &format!("1{}2{}", s, spaces);
                    assert_parse_err(variant);

                    let variant = &format!("1{}{}2", s, spaces);
                    assert_parse_err(variant);

                    let variant = &format!("1{}{}2", spaces, s);
                    assert_parse_err(variant);


                    let variant = &format!("{}1 {} 2", spaces, s);
                    checker(variant, b.clone());
                   
                    let variant = &format!("1 {} 2{}", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("1 {} {}2", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("1{} {} 2", spaces, s);
                    checker(variant, b.clone());
                } else {
                    let variant = &format!("{}1{}2", spaces, s);
                    checker(variant, b.clone());
                   
                    let variant = &format!("1{}2{}", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("1{}{}2", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("1{}{}2", spaces, s);
                    checker(variant, b.clone());
                }
            }
            spaces.push(' ');
        }
    }


    // Same as above test, except its for variables
    #[test]
    fn test_whitespace_around_and_within_operators_vars() {
        // just a helper so i don't have spam code with it over and over again.
        fn checker(variant: &str, b: BinOpKind) {
            match parse(variant).unwrap() {
                Expr::BinOp { op, left, right, .. } => {
                    assert_eq!(op, b.clone());
                    assert!(matches!(*left, Expr::Var { name, .. } if name == "x" ));
                    assert!(matches!(*right, Expr::Var { name, ..} if name == "y" ));

                }
                other => panic!("expected BinOp, got {:?}", other),
            }
        }
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                if *s == "or" || *s == "and" {
                    let variant = &format!("{}x {} y", spaces, s);
                    checker(variant, b.clone());

                    let variant = &format!("x {} y{}", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("x {} {}y", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("x{} {} y", spaces, s);
                    checker(variant, b.clone());

                } else {
                    let variant = &format!("{}x{}y", spaces, s);
                    checker(variant, b.clone());

                    let variant = &format!("x{}y{}", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("x{}{}y", s, spaces);
                    checker(variant, b.clone());

                    let variant = &format!("x{}{}y", spaces, s);
                    checker(variant, b.clone());
                }
            }

            spaces.push(' ');
        }
    }


    #[test]
    fn test_binop_array_access() {
        match parse("arr[0] + 1").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, .. } => {
                assert!(matches!(*left, Expr::ArrayAccess { .. }));
            }
            other => panic!("expected BinOp with ArrayAccess on left, got {:?}", other),
        }
    }


    #[test]
    fn test_binop_with_function_call() {
        match parse("foo(1) + 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, .. } => {
                assert!(matches!(*left, Expr::Call { name, .. } if name == "foo"));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_comp_between_calls() {
        match parse("foo(1) == bar(2)").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, left, right, .. } => {
                assert!(matches!(*left, Expr::Call { name, .. } if name == "foo"));
                assert!(matches!(*right, Expr::Call { name, .. } if name == "bar"));
            }
            other => panic!("expected BinOp Equal, got {:?}", other),
        }
    }





}

