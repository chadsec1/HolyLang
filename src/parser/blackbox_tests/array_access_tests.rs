use super::*;

#[cfg(test)]
mod array_access_tests_in_functions {
    use super::*;

    #[test]
    fn array_single_access() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);

            for lit in &literals_edge_cases {
                let stmts = parse_body(&format!("{}[{}]", arr_name, lit));
                if let Stmt::Expr(e) = &stmts[0] {
                    if let Expr::ArrayAccess{ array, .. } = &e {
                        if let Expr::Var{ name, .. } = &**array {
                            assert_eq!(name, &arr_name.to_string());
                        } else { panic!("Expected Var in ArrayAccess expression 'array' field, instead got {:?}", array); }

                    } else { panic!("Expected ArrayAccess expression, instead got {:?}", e); }
                } else { panic!("Expected Expr, instead got {:?}", stmts); }
            }
        }
    }

    #[test]
    fn array_single_access_in_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own {} {} = {}[{}]", l, t, arr_name, lit));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        if let Expr::ArrayAccess{ array, .. } = &v.value {
                            if let Expr::Var{ name, .. } = &**array {
                                assert_eq!(name, &arr_name.to_string());
                            } else { panic!("Expected Var in ArrayAccess expression 'array' field, instead got {:?}", array); }

                        } else { panic!("Expected ArrayAccess expression, instead got {:?}", v.value); }
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    #[test]
    fn array_single_access_in_var_decl_with_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[{}]", l, arr_name, lit)));
            }
        }
    }
}




// Same tests, but now in globals.
#[cfg(test)]
mod array_access_tests_in_globals {
    use super::*;

    #[test]
    fn array_single_access() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);

            for lit in &literals_edge_cases {
                let ast = parse(&format!("{}[{}]", arr_name, lit)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::Expr(e) = &ast.globals[0] {
                    if let Expr::ArrayAccess{ array, .. } = &e {
                        if let Expr::Var{ name, .. } = &**array {
                            assert_eq!(name, &arr_name.to_string());
                        } else { panic!("Expected Var in ArrayAccess expression 'array' field, instead got {:?}", array); }

                    } else { panic!("Expected ArrayAccess expression, instead got {:?}", e); }
                } else { panic!("Expected Expr, instead got {:?}", ast); }
            }
        }
    }

    #[test]
    fn array_single_access_in_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own {} {} = {}[{}]", l, t, arr_name, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());

                        if let Expr::ArrayAccess{ array, .. } = &v.value {
                            if let Expr::Var{ name, .. } = &**array {
                                assert_eq!(name, &arr_name.to_string());
                            } else { panic!("Expected Var in ArrayAccess expression 'array' field, instead got {:?}", array); }

                        } else { panic!("Expected ArrayAccess expression, instead got {:?}", v.value); }
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    #[test]
    fn array_single_access_in_var_decl_with_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("own {} = {}[{}]", l, arr_name, lit));
            }
        }
    }

}



