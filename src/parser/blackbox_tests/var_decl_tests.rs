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
        let ints_literals_edge_cases = get_all_ints_literals_edge_cases(); 

        for l in &ints_literals_edge_cases {
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
    // respective literals and checked the literal matches. So it's worth double checking here again.
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

            if let Some(Expr::BoolLiteral { value, .. }) = &v.value {
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
            
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
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

                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
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
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
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
                    let stmts = parse_body(&format!("own {} {}\nown {} {} = {}", l, t, l, t, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
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
    fn var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("own x {} = {}", t, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::VarDecl(v) = &ast.globals[0] {
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
            let ast = parse(&format!("own x {}", t)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::VarDecl(v) = &ast.globals[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert!(v.value.is_none());
            } else {
                panic!("Expected VarDecl");
            }
        }
    }


    // Even though we do test all these types declarations, we never tested them in whole with their
    // respective literals and checked the literal matches. So it's worth double checking here again.
    #[test]
    fn var_decl_float64_type() {
        let ast = parse("own y float64 = 1.0").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        if let Stmt::VarDecl(v) = &ast.globals[0] {
            assert_eq!(v.name, "y");
            assert_eq!(v.type_name, Type::Float64);

            if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                assert_eq!(*value, 1.0);
            } else { panic!("Expected Float64Literal"); }
        } else { panic!("Expected VarDecl"); }    

    }

    #[test]
    fn var_decl_bool_type() {
        let ast = parse("own x bool = true").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        if let Stmt::VarDecl(v) = &ast.globals[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::Bool);

            if let Some(Expr::BoolLiteral { value, .. }) = &v.value {
                assert_eq!(*value, true);
            } else { panic!("Expected BoolLiteral"); }
    
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_string_type() {
        let ast = parse(r#"own x string = "hello""#).unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        if let Stmt::VarDecl(v) = &ast.globals[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::String);
            
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(*value, "hello");
            } else { panic!("Expected StringLiteral"); }
    

        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_array() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
         
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let ast = parse(&format!("own x []{} = [{}, {}, {}]", t, l, l, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::VarDecl(v) = &ast.globals[0] {
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
    }

    #[test]
    fn var_decl_array_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&format!("own x = [{}, {}, {}]", l, l, l));
        }
    }

    #[test]
    fn var_decl_empty_array() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("own x {} = []", t)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::VarDecl(v) = &ast.globals[0] {
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
                let ast = parse(&format!("own x []{} = [[{},{},{}], [{},{},{},{}]]", t, l, l, l, l, l, l, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::VarDecl(v) = &ast.globals[0] {
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
            let ast = parse(&format!("own x {} = []", t)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::VarDecl(v) = &ast.globals[0] {
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
                let ast = parse(&format!("own x {} = [{}{}]", t, s1, s2 )).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::VarDecl(v) = &ast.globals[0] {
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
                    let ast = parse(&format!("own x {}, y {}, z {} = give_3_numbers()", t1, t2, t3)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    assert!(matches!(ast.globals[0], Stmt::VarDeclMulti(_, _)));
                    if let Stmt::VarDeclMulti(vars, _) = &ast.globals[0] {
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
    fn variable_redeclaration_with_value_allowed() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own {} {} = {}\nown {} {} = {}", l, t, lit, l, t, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &ast.globals[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
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
                    let ast = parse(&format!("own {} {}\nown {} {} = {}", l, t, l, t, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarDecl(v) = &ast.globals[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }
}
