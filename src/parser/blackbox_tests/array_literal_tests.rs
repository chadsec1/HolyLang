use super::*;

#[cfg(test)]
mod array_literal_in_function_tests {
    use super::*;

    #[test]
    fn array_literal_expr() {
        let literals = get_all_literals_edge_cases();

        for l in &literals {
            let stmts = parse_body(&format!("[{}, {}, {}]", l, l, l));
            assert_eq!(stmts.len(), 1);

            if let Stmt::Expr(e) = &stmts[0] {
                if let Expr::ArrayLiteral { .. } = &e {
                } else {
                    panic!("Expected ArrayLiteral, instead we got {:?}", &e);
                }
            } else { panic!("Expected Expr, instead we got {:?}", &stmts[0]) }
        }
    }

    #[test]
    fn array_literal_in_var_decl_non_array_type() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("own x {} = [{}, {}, {}]", t, l, l, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_some());

                    if let Expr::ArrayLiteral { elements, .. } = &v.value.clone().unwrap() {
                        assert_eq!(elements.len(), 3);
                    } else {
                        panic!("Expected ArrayLiteral, instead we got {:?}", &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }

    #[test]
    fn array_literal_in_var_decl_dyn_array_type() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("own x []{} = [{}, {}, {}]", t, l, l, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
                    assert!(v.value.is_some());

                    if let Expr::ArrayLiteral { elements, .. } = &v.value.clone().unwrap() {
                        assert_eq!(elements.len(), 3);
                    } else {
                        panic!("Expected ArrayLiteral, instead we got {:?}", &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }

    #[test]
    fn array_literal_in_var_decl_fixed_array_type() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            for t in ALL_TYPES_NO_ARR {
                for i in 0usize..10usize {
                    let stmts = parse_body(&format!("own x [{}]{} = [{}, {}, {}]", i, t, l, l, l));
                    assert_eq!(stmts.len(), 1);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)));
                        assert!(v.value.is_some());

                        if let Expr::ArrayLiteral { elements, .. } = &v.value.clone().unwrap() {
                            assert_eq!(elements.len(), 3);
                        } else {
                            panic!("Expected ArrayLiteral, instead we got {:?}", &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }

    #[test]
    fn array_literal_edge_cases_errors() {
        let literals = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                assert_parse_err(&wrap(&format!("[{}, {}, {}]", t, t, t)));
                assert_parse_err(&wrap(&format!("{}, {}, {}", l, l, l)));
                assert_parse_err(&wrap(&format!("[{}[{}, {}, {}]]", t, l, l, l)));
                assert_parse_err(&wrap(&format!("{}[[{}, {}, {}]]", t, l, l, l)));
                assert_parse_err(&wrap(&format!("{}[{}, {}, {}]", t, l, l, l)));

                
                assert_parse_err(&wrap(&format!("[]{}[{}, {}, {}]", t, l, l, l)));
                assert_parse_err(&wrap(&format!("[]{} {}, {}, {}", t, l, l, l)));

                assert_parse_err(&wrap(&format!("own x {} = {}, {}, {}", t, l, l, l)));
                assert_parse_err(&wrap(&format!("own x {} = [{}[{}, {}, {}]]", t, t, l, l, l)));
                assert_parse_err(&wrap(&format!("own x {} = {}[[{}, {}, {}]]", t, t, l, l, l)));
                assert_parse_err(&wrap(&format!("own x = [{}, {}, {}]", l, l, l)));
                assert_parse_err(&wrap(&format!("own x []{} = {}, {}, {}", t, l, l, l)));
                assert_parse_err(&wrap(&format!("own x []{} [{}, {}, {}]", t, l, l, l)));
            }
        }
    }
}
