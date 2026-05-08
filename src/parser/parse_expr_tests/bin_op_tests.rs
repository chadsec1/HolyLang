use super::*;

#[cfg(test)]
mod bin_op_tests {
    use super::*;

    #[test]
    fn test_binop_all_literals() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                match parse(&format!("{} {} {}", l, s, l)).unwrap() {
                    Expr::BinOp { op, .. } => {
                        assert_eq!(op, b.clone());
                    }
                    other => panic!("expected {:?}, got {:?}", b, other),
                }
            }
        }
    }

    #[test]
    fn test_binop_missing_left_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for s in BIN_OP_KIND_SYMBOLS {
                // Cuz negate wouldn't error.
                if s == "-" {
                    continue
                }
                assert_parse_err(&format!("{} {}", s, l));
            }
        }
    }


    #[test]
    fn test_binop_invalid_left_expr_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for s in BIN_OP_KIND_SYMBOLS {
                if l.starts_with("-") {
                    continue
                }
                assert_parse_err(&format!("{} {} {} {}", l, l, s, l));
            }
        }
    }

    #[test]
    fn test_binop_missing_right_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for s in BIN_OP_KIND_SYMBOLS {
                assert_parse_err(&format!("{} {}", l, s));
            }
        }
    }


    #[test]
    fn test_binop_invalid_right_expr_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for s in BIN_OP_KIND_SYMBOLS {
                if l.starts_with("-") {
                    continue
                }
                assert_parse_err(&format!("{} {} {} {}", l, s, l, l));
            }
        }
    }


    #[test]
    fn test_single_not_binop_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            assert_parse_err(&format!("{} = {}", l, l));
            assert_parse_err(&format!("{} ! {}", l, l));
            assert_parse_err(&format!("{} ~ {}", l, l));
        }
    }

    #[test]
    fn test_binop_left_associative_add_subtract() {
        // "1 + 2 + 3", top-level op split gives left = "1 + 2", right = "3"
        // TODO: Improve this test.
        assert!(parse("1 + 2 + 3").is_ok());
    }

    #[test]
    fn test_binop_vars() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) { 
            match parse(&format!("a {} b", s)).unwrap() {
                Expr::BinOp { op, left, right, .. } => {
                    assert_eq!(op, b.clone());
                    assert!(matches!(*left, Expr::Var { name, .. } if name == "a"));
                    assert!(matches!(*right, Expr::Var { name, .. } if name == "b"));
                }
                other => panic!("expected BinOp, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_binop_nested() {
        // The top-level split should give us a BinOp at the top
        assert!(matches!(parse("a + b * c"), Ok(Expr::BinOp { .. })));
    }

    #[test]
    fn test_binop_with_parens_changes_grouping() {
        // (a + b) * c,  top-level op is *
        //
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for (b1, s1) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) { 
                for (b2, s2) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) { 
                    match parse(&format!("({} {} {}) {} c", l, s1, l, s2)).unwrap() {
                        Expr::BinOp { op, left, right, .. } => {
                            assert_eq!(op, b2.clone());

                            match *left {
                                Expr::BinOp { op, .. } => {
                                    assert_eq!(op, b1.clone())
                                }

                                other => panic!("expected BinOp got {:?}", other),
                            }
                            assert!(matches!(*right, Expr::Var { name, .. } if name == "c"));
                        }
                        other => panic!("expected BinOp got {:?}", other),
                    }
                }

            }
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

