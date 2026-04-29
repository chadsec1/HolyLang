use super::*;

#[cfg(test)]
mod copy_call_tests {
    use super::*;

    #[test]
    fn test_copy_call_valid() {
        match parse("copy(x)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::Var { name, .. } if name == "x"));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    #[test]
    fn test_copy_call_no_args_errors() {
        assert_parse_err("copy()");
    }

    #[test]
    fn test_copy_call_too_many_args_errors() {
        assert_parse_err("copy(x, y)");
    }

    #[test]
    fn test_copy_call_expression_arg() {
        match parse("copy(a + b)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    #[test]
    fn test_copy_of_array_single_access() {
        let literals = get_all_literals_edge_cases();
        for l in literals {
            match parse(&format!("copy(arr[{}])", l)).unwrap() {
                Expr::CopyCall { expr, .. } => {
                    assert!(matches!(*expr, Expr::ArrayAccess { .. }));
                }
                other => panic!("expected CopyCall, got {:?}", other),
            }
        }
    }


    #[test]
    fn test_copy_of_array_multiple_access() {
        let literals = get_all_literals_edge_cases();
        for l in literals {
            match parse(&format!("copy(arr[{}:{}])", l, l)).unwrap() {
                Expr::CopyCall { expr, .. } => {
                    match *expr {
                        Expr::ArraySlicing{ array, .. } => {
                            if let Expr::Var{ name, ..} = *array {
                                assert_eq!(name, "arr");
                            } else { panic!("expected Var, got {:?}", array); }
                        }
                        
                        other => panic!("expected nested ArraySlicing, got {:?}", other),
                    }

                }
                other => panic!("expected CopyCall, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_copy_call_with_binop() {
        match parse("copy(x + 1)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }



}
