use super::*;

#[cfg(test)]
mod array_slicing_tests_in_functions {
    use super::*;

    #[test]
    fn array_slice_both_bounds() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                let stmts = parse_body(&format!("{}[{}:{}]", arr_name, lit, lit));
                if let Stmt::Expr(e) = &stmts[0] {
                    if let Expr::ArraySlicing { range, .. } = &e {
                        assert!(matches!(range, ArraySliceRange::FromTo(_, _)))
                    } else { panic!("Expected ArraySlicing"); }
                
                } else { panic!("Expected Expr, instead got {:?}", stmts); }
            }
        }
    }
    
    #[test]
    fn array_slice_both_bounds_in_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {} = {}[{}:{}]", l, t, arr_name, lit, lit));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        if let Expr::ArraySlicing { range, .. } = &v.value {
                            assert!(matches!(range, ArraySliceRange::FromTo(_, _)))
                        } else { panic!("Expected ArraySlicing"); }
                    
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_both_bounds_in_var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[{}:{}]", l, arr_name, lit, lit)));
            }
        }
    }

    #[test]
    fn array_slice_open_start() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                let stmts = parse_body(&format!("{}[:{}]",  arr_name, lit));
                if let Stmt::Expr(e) = &stmts[0] {
                    if let Expr::ArraySlicing { range, .. } = &e {
                        assert!(matches!(range, ArraySliceRange::To(_)))
                    } else { panic!("Expected ArraySlicing"); }
                
                } else { panic!("Expected Expr, instead got {:?}", stmts); }
            }
        }
    }


    #[test]
    fn array_slice_open_start_in_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {} = {}[:{}]", l, t, arr_name, lit));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        if let Expr::ArraySlicing { range, .. } = &v.value {
                            assert!(matches!(range, ArraySliceRange::To(_)))
                        } else { panic!("Expected ArraySlicing"); }
                    
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }




    #[test]
    fn array_slice_open_start_in_var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[:{}]", l, arr_name, lit)));
            }
        }
    }



    #[test]
    fn array_slice_open_end() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                let stmts = parse_body(&format!("{}[{}:]", arr_name, lit));
                if let Stmt::Expr(e) = &stmts[0] {
                    if let Expr::ArraySlicing { range, .. } = &e {
                        assert!(matches!(range, ArraySliceRange::From(_)))
                    } else { panic!("Expected ArraySlicing"); }
                
                } else { panic!("Expected Expr, instead got {:?}", stmts); }
            }
        }
    }



    #[test]
    fn array_slice_open_end_in_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {} = {}[{}:]", l, t, arr_name, lit));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        if let Expr::ArraySlicing { range, .. } = &v.value {
                            assert!(matches!(range, ArraySliceRange::From(_)))
                        } else { panic!("Expected ArraySlicing"); }
                    
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }


    #[test]
    fn array_slice_open_end_in_var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[{}:]", l, arr_name, lit)));
            }
        }
    }



    #[test]
    fn array_slicing_no_start_no_end_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for i in 1..=1000 { 
                assert_parse_err(&wrap(&format!("{}[{}]", arr_name, ":".repeat(i))));
            }

            for t in ALL_TYPES_NO_ARR { 
                assert_parse_err(&wrap(&format!("own {} {} = {}[:]", l, t, arr_name)));
                assert_parse_err(&wrap(&format!("own {} {} = {}[::]", l, t, arr_name)));
                assert_parse_err(&wrap(&format!("own {} {} = {}[]", l, t, arr_name)));

                for lit in &literals_edge_cases {
                    assert_parse_err(&wrap(&format!("own {} {} = {}[{}::]", l, t, arr_name, lit)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}[::{}]", l, t, arr_name, lit)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}[{}::{}]", l, t, arr_name, lit, lit)));
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
//

// Same tests, but now in globals.

#[cfg(test)]
mod array_slicing_tests_in_globals {
    use super::*;
    
    #[test]
    fn array_slice_both_bounds_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("{}[{}:{}]", arr_name, lit, lit));
            }
        }
    }


    #[test]
    fn array_slice_both_bounds_in_var_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    assert_parse_err(&format!("own {} {} = {}[{}:{}]", l, t, arr_name, lit, lit));
                }
            }
        }
    }

    #[test]
    fn array_slice_both_bounds_in_const() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("const {} {} = {}[{}:{}]", l, t, arr_name, lit, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    if let GlobalStmt::Const(c) = &ast.globals[0] {
                        assert_eq!(c.name, l.to_string());
                        assert_eq!(c.type_name, t.clone());

                        if let Expr::ArraySlicing { range, .. } = &c.value {
                            assert!(matches!(range, ArraySliceRange::FromTo(_, _)))
                        } else { panic!("Expected ArraySlicing"); }
                    } else { panic!("Expected Const"); }
                }
            }
        }
    }


    #[test]
    fn array_slice_both_bounds_in_var_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("own {} = {}[{}:{}]", l, arr_name, lit, lit));
            }
        }
    }

    #[test]
    fn array_slice_open_start_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("{}[:{}]", arr_name, lit));
            }
        }
    }


    #[test]
    fn array_slice_open_start_in_var_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    assert_parse_err(&format!("own {} {} = {}[:{}]", l, t, arr_name, lit));
                }
            }
        }
    }

    #[test]
    fn array_slice_open_start_in_const() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("const {} {} = {}[:{}]", l, t, arr_name, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    if let GlobalStmt::Const(c) = &ast.globals[0] {
                        assert_eq!(c.name, l.to_string());
                        assert_eq!(c.type_name, t.clone());

                        if let Expr::ArraySlicing { range, .. } = &c.value {
                            assert!(matches!(range, ArraySliceRange::To(_)))
                        } else { panic!("Expected ArraySlicing"); }
                    } else { panic!("Expected Const"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_open_start_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[:{}]", l, arr_name, lit)));
            }
        }
    }

    #[test]
    fn array_slice_open_end_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("{}[{}:]", arr_name, lit));
            }
        }
    }


    #[test]
    fn array_slice_open_end_in_var_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    assert_parse_err(&format!("own {} {} = {}[{}:]", l, t, arr_name, lit));
                }
            }
        }
    }

    #[test]
    fn array_slice_open_end_in_const() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("const {} {} = {}[{}:]", l, t, arr_name, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    if let GlobalStmt::Const(c) = &ast.globals[0] {
                        assert_eq!(c.name, l.to_string());
                        assert_eq!(c.type_name, t.clone());

                        if let Expr::ArraySlicing { range, .. } = &c.value {
                            assert!(matches!(range, ArraySliceRange::From(_)))
                        } else { panic!("Expected ArraySlicing"); }
                    } else { panic!("Expected Const"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_open_end_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own {} = {}[{}:]", l, arr_name, lit)));
            }
        }
    }




    #[test]
    fn array_slicing_no_start_no_end_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            let arr_name = format!("{}{}", l, l);
            for i in 1..=1000 { 
                assert_parse_err(&format!("{}[{}]", arr_name, ":".repeat(i)));
            }

            for t in ALL_TYPES_NO_ARR { 
                assert_parse_err(&format!("own {} {} = {}[:]", l, t, arr_name));
                assert_parse_err(&format!("own {} {} = {}[::]", l, t, arr_name));
                assert_parse_err(&format!("own {} {} = {}[]", l, t, arr_name));

                for lit in &literals_edge_cases {
                    assert_parse_err(&format!("own {} {} = {}[{}::]", l, t, arr_name, lit));
                    assert_parse_err(&format!("own {} {} = {}[::{}]", l, t, arr_name, lit));
                    assert_parse_err(&format!("own {} {} = {}[{}::{}]", l, t, arr_name, lit, lit));
                }
            }
        }
    }

}
