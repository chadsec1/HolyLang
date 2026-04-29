use super::*;

#[cfg(test)]
mod array_literal_tests {
    use super::*;


    #[test]
    fn test_empty_array_literal() {
        match parse("[]").unwrap() {
            Expr::ArrayLiteral { elements, .. } => {
                assert_eq!(elements.len(), 0);
            }
            other => panic!("expected ArrayLiteral, got {:?}", other),
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
