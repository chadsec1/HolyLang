use super::*;

#[cfg(test)]
mod infinite_stmt_in_function_tests {
    use super::*;


    #[test]
    fn infinite_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        assert_parse_err(&wrap("infinite x {\n\n}"));    
        assert_parse_err(&wrap("infinite range(1, 10) {\n\n}"));    
        assert_parse_err(&wrap("infinite range() {\n\n}"));    
        assert_parse_err(&wrap("infinite range {\n\n}"));    
        assert_parse_err(&wrap("infinite infinite {\n\n}"));    
        assert_parse_err(&wrap("infinite i in x {\n\n}"));    
        assert_parse_err(&wrap("infinite in x {\n\n}"));    
        assert_parse_err(&wrap("infinite i in {\n\n}"));
        assert_parse_err(&wrap("infinite true {\n\n}"));
        assert_parse_err(&wrap("infinite false {\n\n}"));    
        assert_parse_err(&wrap("infinite 1 {\n\n}")); 
        assert_parse_err(&wrap("infinite 1.0 {\n\n}")); 
        assert_parse_err(&wrap("infinite \"\" {\n\n}"));    
        assert_parse_err(&wrap("infinite {\n\n"));    
        assert_parse_err(&wrap("infinite {}"));    
        assert_parse_err(&wrap("infinite \n\n}"));    

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", kw, kw)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", kw, kw)));    
        }

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", l, l)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", t, t)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", t, t)));    
        }
    }

    #[test]
    fn infinite_statements_spaces_after_passes() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            let stmts = parse_body(&format!("infinite {} {{\n\n}}", spaces));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }

    #[test]
    fn infinite_statements_spaces_before_passes() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            let stmts = parse_body(&format!("{} infinite {{\n\n}}", spaces));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }


    #[test]
    fn infinite_statements_below_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own {} {} = {}\ninfinite {{\n\n}}", l, t, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::Infinite(inf) = &stmts[1] {
                        assert_eq!(inf.branch.len(), 0);
                    } else {
                        panic!("Expected infinite statement");
                    }
                }
            }
        }
    }

    #[test]
    fn infinite_statements_below_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("own {} {}\ninfinite {{\n\n}}", l, t));
                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, l.to_string());
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_none());
                } else { panic!("Expected VarDecl"); }

                if let Stmt::Infinite(inf) = &stmts[1] {
                    assert_eq!(inf.branch.len(), 0);
                } else {
                    panic!("Expected infinite statement");
                }
            }
        }
    }




    #[test]
    fn infinite_statements_after_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("infinite {{\n\n}}\nown {} {} = {}", l, t, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::Infinite(inf) = &stmts[0] {
                        assert_eq!(inf.branch.len(), 0);
                    } else {
                        panic!("Expected infinite statement");
                    }

                    if let Stmt::VarDecl(v) = &stmts[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                }
            }
        }
    }

    #[test]
    fn infinite_statements_after_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("infinite {{\n\n}}\nown {} {}", l, t));
                assert_eq!(stmts.len(), 2);

                if let Stmt::Infinite(inf) = &stmts[0] {
                    assert_eq!(inf.branch.len(), 0);
                } else {
                    panic!("Expected infinite statement");
                }

                if let Stmt::VarDecl(v) = &stmts[1] {
                    assert_eq!(v.name, l.to_string());
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_none());
                } else { panic!("Expected VarDecl"); }
            }
        }
    }





    #[test]
    fn infinite_statements_with_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("infinite {{\nown {} {} = {}\n}}", l, t, lit));
                    assert_eq!(stmts.len(), 1);

                    if let Stmt::Infinite(inf) = &stmts[0] {
                        assert_eq!(inf.branch.len(), 1);

                        if let Stmt::VarDecl(v) = &inf.branch[0] {
                            assert_eq!(v.name, l.to_string());
                            assert_eq!(v.type_name, t.clone());
                            assert!(v.value.is_some());
                        } else { panic!("Expected VarDecl"); }
                    } else {
                        panic!("Expected infinite statement");
                    }
                }
            }
        }
    }


    #[test]
    fn infinite_statements_with_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("infinite {{\nown {} {}\n}}", l, t));
                assert_eq!(stmts.len(), 1);

                if let Stmt::Infinite(inf) = &stmts[0] {
                    assert_eq!(inf.branch.len(), 1);

                    if let Stmt::VarDecl(v) = &inf.branch[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }
                } else {
                    panic!("Expected infinite statement");
                }
            }
        }
    }

    #[test]
    fn infinite_statements_with_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let stmts = parse_body(&format!("infinite {{\n{}\n}}", l));
            assert_eq!(stmts.len(), 1);

            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 1);
                assert!(matches!(inf.branch[0], Stmt::Expr(_)));
            } else {
                panic!("Expected infinite statement");
            }
        }
    }

    #[test]
    fn infinite_statements_before_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let stmts = parse_body(&format!("{}\ninfinite {{\n\n}}", l));
            assert_eq!(stmts.len(), 2);
            assert!(matches!(stmts[0], Stmt::Expr(_)));

            if let Stmt::Infinite(inf) = &stmts[1] {
                assert_eq!(inf.branch.len(), 0);
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn infinite_statements_after_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let stmts = parse_body(&format!("infinite {{\n\n}}\n{}", l));
            assert_eq!(stmts.len(), 2);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);
            } else {panic!("Expected infinite statement"); }
            
            assert!(matches!(stmts[1], Stmt::Expr(_)));
        }
    }


    #[test]
    fn infinite_statements_nested() {
        let mut s = format!("infinite {{\n\n}}");

        for _ in 0..100 {
            s = format!("infinite {{\n{}\n}}", s);
            let stmts = parse_body(&s);
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 1);
            } else {panic!("Expected infinite statement"); }
        }
    }
}










//
//
//

#[cfg(test)]
mod infinite_stmt_in_globals_tests {
    use super::*;


    #[test]
    fn infinite_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        assert_parse_err("infinite x {\n\n}");
        assert_parse_err("infinite range(1, 10) {\n\n}");    
        assert_parse_err("infinite range() {\n\n}");    
        assert_parse_err("infinite range {\n\n}");    
        assert_parse_err("infinite infinite {\n\n}");    
        assert_parse_err("infinite i in x {\n\n}");    
        assert_parse_err("infinite in x {\n\n}");    
        assert_parse_err("infinite i in {\n\n}");
        assert_parse_err("infinite true {\n\n}");
        assert_parse_err("infinite false {\n\n}");    
        assert_parse_err("infinite 1 {\n\n}"); 
        assert_parse_err("infinite 1.0 {\n\n}"); 
        assert_parse_err("infinite \"\" {\n\n}");    
        assert_parse_err("infinite {\n\n");    
        assert_parse_err("infinite {}");  
        assert_parse_err("infinite \n\n}");

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("infinite {} {{\n\n}}", kw));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", kw));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", kw));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", kw));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", kw, kw));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", kw, kw));    
        }

        for l in &literals_edge_cases {
            assert_parse_err(&format!("infinite {} {{\n\n}}", l));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", l));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", l));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", l));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", l, l));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", l, l));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("infinite {} {{\n\n}}", t));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", t));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", t));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", t));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", t, t));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", t, t));    
        }
    }

    #[test]
    fn infinite_statements_spaces_after_passes() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            let ast = parse(&format!("infinite {} {{\n\n}}", spaces)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::Infinite(inf) = &ast.globals[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }

    #[test]
    fn infinite_statements_spaces_before_passes() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            let ast = parse(&format!("{} infinite {{\n\n}}", spaces)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::Infinite(inf) = &ast.globals[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }


    #[test]
    fn infinite_statements_below_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own {} {} = {}\ninfinite {{\n\n}}", l, t, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                    if let Stmt::Infinite(inf) = &ast.globals[1] {
                        assert_eq!(inf.branch.len(), 0);
                    } else {
                        panic!("Expected infinite statement");
                    }
                }
            }
        }
    }

    #[test]
    fn infinite_statements_below_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("own {} {}\ninfinite {{\n\n}}", l, t)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 2);

                if let Stmt::VarDecl(v) = &ast.globals[0] {
                    assert_eq!(v.name, l.to_string());
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_none());
                } else { panic!("Expected VarDecl"); }

                if let Stmt::Infinite(inf) = &ast.globals[1] {
                    assert_eq!(inf.branch.len(), 0);
                } else {
                    panic!("Expected infinite statement");
                }
            }
        }
    }




    #[test]
    fn infinite_statements_after_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("infinite {{\n\n}}\nown {} {} = {}", l, t, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 2);

                    if let Stmt::Infinite(inf) = &ast.globals[0] {
                        assert_eq!(inf.branch.len(), 0);
                    } else {
                        panic!("Expected infinite statement");
                    }

                    if let Stmt::VarDecl(v) = &ast.globals[1] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());
                    } else { panic!("Expected VarDecl"); }

                }
            }
        }
    }

    #[test]
    fn infinite_statements_after_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("infinite {{\n\n}}\nown {} {}", l, t)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 2);

                if let Stmt::Infinite(inf) = &ast.globals[0] {
                    assert_eq!(inf.branch.len(), 0);
                } else {
                    panic!("Expected infinite statement");
                }

                if let Stmt::VarDecl(v) = &ast.globals[1] {
                    assert_eq!(v.name, l.to_string());
                    assert_eq!(v.type_name, t.clone());
                    assert!(v.value.is_none());
                } else { panic!("Expected VarDecl"); }
            }
        }
    }





    #[test]
    fn infinite_statements_with_var_decl_with_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("infinite {{\nown {} {} = {}\n}}", l, t, lit)).unwrap();
                    assert_eq!(ast.functions.len(), 0);
                    assert_eq!(ast.globals.len(), 1);

                    if let Stmt::Infinite(inf) = &ast.globals[0] {
                        assert_eq!(inf.branch.len(), 1);

                        if let Stmt::VarDecl(v) = &inf.branch[0] {
                            assert_eq!(v.name, l.to_string());
                            assert_eq!(v.type_name, t.clone());
                            assert!(v.value.is_some());
                        } else { panic!("Expected VarDecl"); }
                    } else {
                        panic!("Expected infinite statement");
                    }
                }
            }
        }
    }


    #[test]
    fn infinite_statements_with_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("infinite {{\nown {} {}\n}}", l, t)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let Stmt::Infinite(inf) = &ast.globals[0] {
                    assert_eq!(inf.branch.len(), 1);

                    if let Stmt::VarDecl(v) = &inf.branch[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
                    } else { panic!("Expected VarDecl"); }
                } else {
                    panic!("Expected infinite statement");
                }
            }
        }
    }

    #[test]
    fn infinite_statements_with_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let ast = parse(&format!("infinite {{\n{}\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::Infinite(inf) = &ast.globals[0] {
                assert_eq!(inf.branch.len(), 1);
                assert!(matches!(inf.branch[0], Stmt::Expr(_)));
            } else {
                panic!("Expected infinite statement");
            }
        }
    }

    #[test]
    fn infinite_statements_before_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let ast = parse(&format!("{}\ninfinite {{\n\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 2);

            assert!(matches!(ast.globals[0], Stmt::Expr(_)));

            if let Stmt::Infinite(inf) = &ast.globals[1] {
                assert_eq!(inf.branch.len(), 0);
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn infinite_statements_after_expr_stmt() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            let ast = parse(&format!("infinite {{\n\n}}\n{}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 2);

            if let Stmt::Infinite(inf) = &ast.globals[0] {
                assert_eq!(inf.branch.len(), 0);
            } else {panic!("Expected infinite statement"); }
            
            assert!(matches!(ast.globals[1], Stmt::Expr(_)));
        }
    }


    #[test]
    fn infinite_statements_nested() {
        let mut s = format!("infinite {{\n\n}}");

        for _ in 0..100 {
            s = format!("infinite {{\n{}\n}}", s);
            let ast = parse(&s).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::Infinite(inf) = &ast.globals[0] {
                assert_eq!(inf.branch.len(), 1);
            } else {panic!("Expected infinite statement"); }
        }
    }

}


