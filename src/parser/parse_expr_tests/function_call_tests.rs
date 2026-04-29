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
    fn test_function_call_multiple_args() {
        match parse("add(1, 2, 3)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 3);
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_expression_args() {
        match parse("add(x + 1, y)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(&args[0], Expr::BinOp { op: BinOpKind::Add, .. }));
                assert!(matches!(&args[1], Expr::Var { .. }));
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_nested() {
        match parse("outer(inner(1))").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "outer");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::Call { name, .. } if name == "inner"));
            }
            other => panic!("expected Call, got {:?}", other),
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




}
