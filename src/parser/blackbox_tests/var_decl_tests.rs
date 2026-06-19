use super::*;

#[cfg(test)]
mod var_decl_in_functions_tests {
    use super::*;

    #[test]
    fn var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x = {}", l)));
        }
    }

    #[test]
    fn var_decl_invalid_name_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            if l.chars().all(|c| c.is_ascii_alphabetic()) {
                continue
            }

            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own {} {} = {}", l, t, l)));
                assert_parse_err(&wrap(&format!("own {} {} = {}", t, l, l)));

                assert_parse_err(&wrap(&format!("own {} {}", l, t)));
                assert_parse_err(&wrap(&format!("own {} {}", t, l)));

            }
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
                } else {
                    panic!("Expected VarDecl");
                }
            }
        }
    }

    #[test]
    fn var_decl_no_value() {
        for t in ALL_TYPES_NO_ARR {
            if *t == Type::Char {
                continue
            }

            let stmts = parse_body(&format!("own x {}", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.type_name.get_default_value(v.span).unwrap(), v.value);
            } else {
                panic!("Expected VarDecl");
            }
        }
    }


    // Even though we do test all these types declarations, we never tested them in whole with their
    // respective literals and checked the literal matches. So it's worth double checking here again.
    #[test]
    fn var_decl_float64_type() {
        let stmts = parse_body("own y float64 = 1.0");
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "y");
            assert_eq!(v.type_name, Type::Float64);

            if let Expr::Float64Literal { value, .. } = &v.value {
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
            assert_ne!(Type::Bool.get_default_value(span()).unwrap(), v.value);

            if let Expr::BoolLiteral { value, .. } = &v.value {
                assert_eq!(*value, true);
            } else { panic!("Expected BoolLiteral"); }
    
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_string_type() {
        let stmts = parse_body(r#"own x string = "hello""#);
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::String);
            assert_ne!(Type::String.get_default_value(span()).unwrap(), v.value);
            
            if let Expr::StringLiteral { value, .. } = &v.value {
                assert_eq!(*value, "hello");
            } else { panic!("Expected StringLiteral"); }
    

        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_array() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
         
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x []{} = [{}, {}, {}]", t, l, l, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
            
                    assert_ne!(v.type_name.get_default_value(v.span).unwrap(), v.value);

                    if let Expr::ArrayLiteral { elements, .. } = &v.value {
                        assert_eq!(elements.len(), 3);
                    } else {
                        panic!("Expected ArrayLiteral");
                    }

                } else { panic!("Expected VarDecl");}
            }
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
                if v.type_name != Type::Char {
                    assert_ne!(v.type_name.get_default_value(v.span).unwrap(), v.value);
                }

                if let Expr::ArrayLiteral { elements, .. } = &v.value {
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

                    if let Expr::ArrayLiteral { elements, .. } = &v.value {
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
                
                if v.type_name != Type::Char {
                    assert_ne!(v.type_name.get_default_value(v.span).unwrap(), v.value);
                }

                if let Expr::ArrayLiteral { elements, .. } = &v.value {
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
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.name, "x");
                    if v.type_name != Type::Char {
                        assert_ne!(v.type_name.get_default_value(v.span).unwrap(), v.value);
                    }

                    if let Expr::ArrayLiteral { elements, .. } = &v.value {
                        assert_eq!(elements.len(), 1);

                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn var_decl_unknown_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("own x badtype"));
        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x badtype = {}",  l)));
            assert_parse_err(&wrap(&format!("own x {} = {}", l, l)));
        }

        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l1 in &letters {
            for l2 in &letters {
                assert_parse_err(&wrap(&format!("own {} {}", l1, l2)));
            }

            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own x {} = {}", l1, lit)));
            }
        }

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x {}", l)));
        }
    }

    #[test]
    fn var_decl_no_type_no_value_errors() {
        assert_parse_err(&wrap("own x"));
    }


    #[test]
    fn var_decl_invalid_syntax_errors() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            assert_parse_err(&wrap(&format!("own{} x", l)));
            assert_parse_err(&wrap(&format!("ow{}n x", l)));
            assert_parse_err(&wrap(&format!("o{}wn x", l)));
            assert_parse_err(&wrap(&format!("{}own x", l)));
            assert_parse_err(&wrap(&format!("{}own{} x", l, l)));
        }
    }


    #[test]
    fn var_decl_keyword_name_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for t in ALL_TYPES_NO_ARR {
                for l in &literals_edge_cases {
                    assert_parse_err(&wrap(&format!("own {} {}", kw, t)));
                    assert_parse_err(&wrap(&format!("own {} {}", kw.to_uppercase(), t)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}", kw, t, l)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}", kw.to_uppercase(), t, l)));
                }
            }
        }
    }

    // Not allowed in semantics phase, but, this is **syntactically** correct
    #[test]
    fn variable_redeclaration_with_value_allowed() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own {} {} = {}\nown {} {} = {}", l, t, lit, l, t, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    // Same as above.
    #[test]
    fn variable_redeclaration_without_value_allowed() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    if *t == Type::Char {
                        continue
                    }

                    let stmts = parse_body(&format!("own {} {}\nown {} {} = {}", l, t, l, t, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                
                        assert_eq!(v.type_name.get_default_value(v.span).unwrap(), v.value);
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }
}


//
//
//
//
//
//
//
//
//


#[cfg(test)]
mod var_decl_in_globals_tests {
    use super::*;


    #[test]
    fn var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&format!("own x = {}", l));
        }
    }

    #[test]
    fn var_decl_invalid_name_errors() {
        let ints_literals_edge_cases = get_all_ints_literals_edge_cases(); 

        for l in &ints_literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("own {} {} = {}", l, t, l));
                assert_parse_err(&format!("own {} {} = {}", t, l, l));

                assert_parse_err(&format!("own {} {}", l, t));
                assert_parse_err(&format!("own {} {}", t, l));

            }
        }
    }

    #[test]
    fn var_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("own x {} = {}", t, l));
            }
        }
    }

    #[test]
    fn var_decl_no_value_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("own x {}", t));
        }
    }


    #[test]
    fn var_decl_float64_type_errors() {
        assert_parse_err("own y float64 = 1.0");
    }

    #[test]
    fn var_decl_bool_type_errors() {
        assert_parse_err("own x bool = true");
    }

    #[test]
    fn var_decl_string_type_errors() {
        assert_parse_err(r#"own x string = "hello""#);
    }

    #[test]
    fn var_decl_array_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
         
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                assert_parse_err(&format!("own x []{} = [{}, {}, {}]", t, l, l, l));
            }
        }
    }

    #[test]
    fn var_decl_array_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&format!("own x = [{}, {}, {}]", l, l, l));
        }
    }

    #[test]
    fn var_decl_empty_array_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("own x {} = []", t));
        }
    }

    #[test]
    fn var_decl_nested_array_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                assert_parse_err(&format!("own x []{} = [[{},{},{}], [{},{},{},{}]]", t, l, l, l, l, l, l, l));
            }
        }
    }

    #[test]
    fn var_decl_nested_array_empty_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("own x {} = []", t));
        }
    }


    #[test]
    fn var_decl_deeply_nested_array_errors() {
        for t in ALL_TYPES_NO_ARR {
            let mut s1 = String::with_capacity(100);
            let mut s2 = String::with_capacity(100);

            for _ in 1..100 {
                s1.push_str("[");
                s2.push_str("]");
                assert_parse_err(&format!("own x {} = [{}{}]", t, s1, s2 ));
            }
        }
    }


    #[test]
    fn var_decl_unknown_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err("own x badtype = 1");
        assert_parse_err("own x badtype");

        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l1 in &letters {
            for l2 in &letters {
                assert_parse_err(&format!("own {} {}", l1, l2));
            }

            for lit in &literals_edge_cases {
                assert_parse_err(&format!("own x {} = {}", l1, lit));
            }
        }

        for l in &literals_edge_cases {
            assert_parse_err(&format!("own x {}", l));
        }
    }

    #[test]
    fn var_decl_no_type_no_value_errors() {
        assert_parse_err("own x");
    }


    #[test]
    fn var_decl_invalid_syntax_errors() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            assert_parse_err(&format!("own{} x", l));
            assert_parse_err(&format!("ow{}n x", l));
            assert_parse_err(&format!("o{}wn x", l));
            assert_parse_err(&format!("{}own x", l));
            assert_parse_err(&format!("{}own{} x", l, l));
        }
    }


    #[test]
    fn var_decl_keyword_name_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for t in ALL_TYPES_NO_ARR {
                for l in &literals_edge_cases {
                    assert_parse_err(&format!("own {} {}", kw, t));
                    assert_parse_err(&format!("own {} {}", kw.to_uppercase(), t));
                    assert_parse_err(&format!("own {} {} = {}", kw, t, l));
                    assert_parse_err(&format!("own {} {} = {}", kw.to_uppercase(), t, l));
                }
            }
        }
    }

    // Not allowed in semantics phase, but, this is **syntactically** correct
    #[test]
    fn variable_redeclaration_with_value_allowed_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    assert_parse_err(&format!("own {} {} = {}\nown {} {} = {}", l, t, lit, l, t, lit));
                }
            }
        }
    }

    // Same as above.
    #[test]
    fn variable_redeclaration_without_value_allowed_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    assert_parse_err(&format!("own {} {}\nown {} {} = {}", l, t, l, t, lit));
                }
            }
        }
    }
}
