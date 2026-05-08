use super::*;

#[cfg(test)]
mod function_call_tests {
    use super::*;

    #[test]
    fn test_function_call_no_args() {
        match parse("foo()").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "foo");
                assert!(args.is_empty());
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_one_arg() {
        match parse("foo(1)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "foo");
                assert_eq!(args.len(), 1);
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_one_arg_all_literals() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            match parse(&format!("foo({})", l)).unwrap() {
                Expr::Call { name, args, .. } => {
                    assert_eq!(name, "foo");
                    assert_eq!(args.len(), 1);
                }
                other => panic!("expected Call, got {:?}", other),
            }
        }
    }


    #[test]
    fn test_function_call_multiple_args() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            match parse(&format!("add({}, {}, {})", l, l, l)).unwrap() {
                Expr::Call { name, args, .. } => {
                    assert_eq!(name, "add");
                    assert_eq!(args.len(), 3);
                }
                other => panic!("expected Call, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_function_call_expression_args() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {    
                match parse(&format!("add({} {} {}, {})", l, s, l, l)).unwrap() {
                    Expr::Call { name, args, .. } => {
                        assert_eq!(name, "add");
                        assert_eq!(args.len(), 2);
                        match &args[0] {
                            Expr::BinOp { op, .. } => {
                                assert_eq!(op, b);
                            }
                            other => panic!("expected BinOp, got {:?}", other),
                        }
                    }
                    other => panic!("expected Call, got {:?}", other),
                }
            }
        }
    }

    #[test]
    fn test_function_call_nested() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals {
            match parse(&format!("outer(inner({}))", l)).unwrap() {
                Expr::Call { name, args, .. } => {
                    assert_eq!(name, "outer");
                    assert_eq!(args.len(), 1);
                    assert!(matches!(&args[0], Expr::Call { name, .. } if name == "inner"));
                }
                other => panic!("expected Call, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_function_call_with_array_literal_arg() {
        let literals = get_all_literals_edge_cases();
        for l in literals {
            match parse(&format!("foo([{}, {}])", l, l)).unwrap() {
                Expr::Call { name, args, .. } => {
                    assert_eq!(name, "foo");
                    assert_eq!(args.len(), 1);

                    match &args[0] {
                        Expr::ArrayLiteral { elements, .. } => {
                            assert_eq!(elements.len(), 2);
                        }
                        other => panic!("expected nested ArrayLiteral, got {:?}", other),
                    }
                }
                other => panic!("expected Call, got {:?}", other),
            }
        }
    }


    #[test]
    fn test_function_call_invalid_args() {
        let literals = get_all_literals_edge_cases();
        for l in literals {
            assert_parse_err(&format!("foo({}, \"hi)", l));
            assert_parse_err(&format!("foo(\"hi {})", l));
            assert_parse_err(&format!("foo(\"{} hi)", l));
            assert_parse_err(&format!("foo(\"hi\", {}, \"lol)", l));
        }
        assert_parse_err("foo(\"hi)");
    }






}
