use super::*;

#[cfg(test)]
mod var_decl_tests {
    use super::*;


    #[test]
    fn var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x = {}", l)));
        }
    }

    #[test]
    fn var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("own x {} = {}", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_some());
                } else {
                    panic!("Expected VarDecl");
                }
            }
        }
    }

    #[test]
    fn var_decl_no_value() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {}", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert!(v.value.is_none());
            } else {
                panic!("Expected VarDecl");
            }
        }
    }


    // Even though we do test all these types declarations, we never tested them in whole with their
    // respective literals. So it's worth double checking here again.
    #[test]
    fn var_decl_float64_type() {
        let stmts = parse_body("own y float64 = 1.0");
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "y");
            assert_eq!(v.type_name, Type::Float64);

            if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                assert_eq!(*value, 1.0);
            } else { panic!("Expected Float64Literal"); }
        } else { panic!("Expected VarDecl"); }    

    }

    #[test]
    fn var_decl_bool_type() {
        let stmts = parse_body("own x bool = true");
        assert_eq!(stmts.len(), 1);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::Bool);
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_string_type() {
        let stmts = parse_body(r#"own x string = "hello""#);
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::String);
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_array() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x []{} = [1, 2, 3]", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 3);
                } else {
                    panic!("Expected ArrayLiteral");
                }

            } else { panic!("Expected VarDecl");}
        }
    }

    #[test]
    fn var_decl_array_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x = [{}, {}, {}]", l, l, l)));
        }
    }

    #[test]
    fn var_decl_empty_array() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = []", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert!(elements.is_empty());
                } else {
                    panic!("Expected ArrayLiteral");
                }
            } else { panic!("Expected VarDecl");}
        }
    }

    #[test]
    fn var_decl_nested_array() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x []{} = [[{},{},{}], [{},{},{},{}]]", t, l, l, l, l, l, l, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));

                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert_eq!(elements.len(), 2);
                        assert!(matches!(elements[0], Expr::ArrayLiteral { .. }));
                        assert!(matches!(elements[1], Expr::ArrayLiteral { .. }));
                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                } else { panic!("Expected VarDecl");}
            }
        }
    }

    #[test]
    fn var_decl_nested_array_empty() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = []", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }
            } else { panic!("Expected VarDecl");}
        }
    }


    #[test]
    fn var_decl_deeply_nested_array() {
        for t in ALL_TYPES_NO_ARR {
            let mut s1 = String::with_capacity(100);
            let mut s2 = String::with_capacity(100);

            for _ in 1..100 {
                s1.push_str("[");
                s2.push_str("]");
                let stmts = parse_body(&format!("own x {} = [{}{}]", t, s1, s2 ));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert_eq!(elements.len(), 1);

                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                }
            }
        }
    }


    #[test]
    fn var_decl_multi() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                for t3 in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own x {}, y {}, z {} = give_3_numbers()", t1, t2, t3));
                    assert!(matches!(stmts[0], Stmt::VarDeclMulti(_, _)));
                    if let Stmt::VarDeclMulti(vars, _) = &stmts[0] {
                        assert_eq!(vars.len(), 3);
                        assert_eq!(vars[0].name, "x");
                        assert_eq!(vars[0].type_name, t1.clone());
                        assert_eq!(vars[1].name, "y");
                        assert_eq!(vars[1].type_name, t2.clone());
                        assert_eq!(vars[2].name, "z");
                        assert_eq!(vars[2].type_name, t3.clone());
                    } else { panic!("Expected VarDeclMulti"); }
                }
            }
        }
    }

    #[test]
    fn var_decl_unknown_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("own x badtype = 1"));
        assert_parse_err(&wrap("own x badtype"));
        assert_parse_err(&wrap("own x x = 1"));
        assert_parse_err(&wrap("own x x"));

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x {}", l)));
        }
    }

    #[test]
    fn var_decl_no_type_no_value_errors() {
        assert_parse_err(&wrap("own x"));
    }


    #[test]
    fn var_decl_keyword_name_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for t in ALL_TYPES_NO_ARR {
                for l in &literals_edge_cases {
                    assert_parse_err(&wrap(&format!("own {} {}", kw, t)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}", kw, t, l)));
                }
            }
        }
    }

    // Not allowed in semantics phase, but,  this is **syntaxally** correct
    #[test]
    fn variable_shadowing_allowed() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1\nown x {} = 2", t, t));
            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::IntLiteral { .. })));
            } else { panic!("Expected VarDecl"); }


            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::IntLiteral { .. })));
            } else { panic!("Expected VarDecl"); }
        }
    }
}
