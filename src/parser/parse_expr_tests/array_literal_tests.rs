use super::*;

#[cfg(test)]
mod array_literal_tests {
    use super::*;


    #[test]
    fn test_empty_array_literal() {
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=1000 {  
            match parse(&format!("[{}]", spaces)).unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 0);
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }


            spaces.push(' ');
        }
    }

    #[test]
    fn test_array_literals_non_separated_errors() {
        let literals = get_all_literals_edge_cases();

        let mut last_l: String = literals[1].clone();

        for l in literals {
            if l.starts_with("-") {
                assert_parse_err(&format!("[{} {} {}]", l, last_l, last_l));
                continue
            }
            assert_parse_err(&format!("[{} {} {}]", l, l, l));
            last_l = l;
        }
    }

    #[test]
    fn test_array_unclosed_strings_errors() {
        assert_parse_err("[\"hi]");
        assert_parse_err("[\"hi, \"lol\"]");
    }

    #[test]
    fn test_array_binop_all_literals_binop() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                match parse(&format!("x[{}] {} y[{}]", l, s, l)).unwrap() {
                    Expr::BinOp { left, right, op, .. } => {
                        assert_eq!(op, b.clone());

                        match *left {
                            Expr::ArrayAccess{ array, .. } => {
                                assert!(matches!(*array, Expr::Var { name, .. } if name == "x"));

                            }
                            other => panic!("expected ArrayAccess, got {:?}", other),
                        }


                        match *right {
                            Expr::ArrayAccess{ array, .. } => {
                                assert!(matches!(*array, Expr::Var { name, .. } if name == "y"));

                            }
                            other => panic!("expected ArrayAccess, got {:?}", other),
                        }
                    }
                    other => panic!("expected {:?}, got {:?}", b, other),
                }
            }
        }
    }




    #[test]
    fn test_array_literals() {
        let literals = get_all_literals_edge_cases();
        for l in literals {
            match parse(&format!("[{}, {}, {}]", l, l, l)).unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
    }


    #[test]
    fn test_array_literal_nested() {
        // array of arrays
        //
        let literals = get_all_literals_edge_cases();

        for l in literals {
            match parse(&format!("[[{},{}], [{},{}]]", l, l, l, l)).unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 2);
                    for elem in &elements {
                        assert!(matches!(elem, Expr::ArrayLiteral { .. }));
                    }
                }
                other => panic!("expected nested ArrayLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_array_literal_with_expressions() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            match parse(&format!("[a + {}, b * {}]", l, l)).unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 2);
                    assert!(matches!(&elements[0], Expr::BinOp { op: BinOpKind::Add, .. }));
                    assert!(matches!(&elements[1], Expr::BinOp { op: BinOpKind::Multiply, .. }));
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
    }



}
