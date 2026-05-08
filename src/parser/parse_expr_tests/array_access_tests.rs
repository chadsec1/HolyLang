use super::*;

#[cfg(test)]
mod array_access_tests {
    use super::*;

    
    #[test]
    fn test_array_single_access() {
        match parse("arr[0]").unwrap() {
            Expr::ArrayAccess { array, index, .. } => {
                assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                assert!(matches!(*index, Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. }));
            }
            other => panic!("expected ArrayAccess, got {:?}", other),
        }
    }


    #[test]
    fn test_array_single_access_all_literals() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals {
            match parse(&format!("arr[{}]", l)).unwrap() {
                Expr::ArrayAccess { array, .. } => {
                    assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                }
                other => panic!("expected ArrayAccess, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_array_single_access_expression_all_binop() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                match parse(&format!("arr[{} {} {}]", l, s, l)).unwrap() {
                    Expr::ArrayAccess { array, index, .. } => {
                        assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));

                        match *index {
                            Expr::BinOp { op, ..} => {
                                assert_eq!(op, b.clone())
                            }
                            other => panic!("expected {:?}, got {:?}", b, other),
                        }
                    }
                    other => panic!("expected ArrayAccess, got {:?}", other),
                }
            }
        }
    }


}
