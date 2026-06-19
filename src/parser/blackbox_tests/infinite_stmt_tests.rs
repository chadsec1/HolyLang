use super::*;

#[cfg(test)]
mod infinite_stmt_in_function_tests {
    use super::*;

    #[test]
    fn invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n{}}}", l, l)));    
            assert_parse_err(&wrap(&format!("infinite range({}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite range(, {}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite range({}, ) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite range({}, {}) {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("infinite range({}, {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("infinite range{}, {} {{\n\n}}", l, l)));    

            assert_parse_err(&wrap(&format!("infinite {} in {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("infinite in {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite {} in {{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", l, l)));   
        }

        assert_parse_err(&wrap("infinite {\n\n"));    
        assert_parse_err(&wrap("infinite {}"));    
        assert_parse_err(&wrap("infinite {{\n\n}}"));    
        assert_parse_err(&wrap("infinite \n\n}"));    

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", kw, kw)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", kw, kw)));    


            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", kw.to_uppercase(), kw)));    
            
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", t, t)));    
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", t, t)));

            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("infinite{} {{\n\n}}", t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} infinite {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}infinite {{\n\n}}", t.to_string().to_uppercase() )));

            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{}infinite{} {{\n\n}}", t, t.to_string().to_uppercase()))); 

            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} infinite {} {{\n\n}}", t.to_string().to_uppercase(), t)));
        }
    }

    #[test]
    fn trailing_exprs_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals{
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {} {{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{} {{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {}{{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{}{{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n", l)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n}}", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n}}", l)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n", l)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n", l)));
        }
    }

    #[test]
    fn trailing_kw_errors() {
        let literals = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {}{{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{}{{\n\n}}", kw)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n}}", kw)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n", kw)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n}}", kw)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n", kw)));
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n", kw)));

            for l in &literals{
                assert_parse_err(&wrap(&format!("infinite {{\n\n}} {} {} {{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite {{\n\n}}{} {}{{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite {{\n\n}} {}{}{{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite {{\n\n}}{}{}{{\n\n}}", kw, l)));

                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{} {{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{} {{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{}{{\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{}{{\n\n}}", kw, l)));

                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{} {{\n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{} {{\n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{}{{\n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{}{{\n\n", kw, l)));

                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{} \n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{} \n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{}\n\n}}", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{}\n\n}}", kw, l)));

                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{} \n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{} \n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{}\n\n", kw, l)));
                assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{}\n\n", kw, l)));
            }
        }
    }

    #[test]
    fn trailing_types_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite {{\n\n}} {}{{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite {{\n\n}}{}{{\n\n}}", t)));    

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n}}", t)));    

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} {{\n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} {{\n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}{{\n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}{{\n\n", t)));    

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n}}", t)));

            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {} \n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{} \n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}} {}\n\n", t)));    
            assert_parse_err(&wrap(&format!("infinite{{\n\n}}{}\n\n", t)));
        }
    }



    #[test]
    fn spaces_after_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", spaces)));
            spaces.push(' ');
        }
    }

    #[test]
    fn spaces_before_passes() {
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
    fn below_var_decl_with_value() {
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
    fn below_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                if *t == Type::Char {
                    continue
                }
 
                let stmts = parse_body(&format!("own {} {}\ninfinite {{\n\n}}", l, t));
                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, l.to_string());
                    assert_eq!(v.type_name, t.clone());
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
    fn after_var_decl_with_value() {
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
                    } else { panic!("Expected VarDecl"); }

                }
            }
        }
    }

    #[test]
    fn after_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                if *t == Type::Char {
                    continue
                }
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
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn with_var_decl_with_value() {
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
                        } else { panic!("Expected VarDecl"); }
                    } else {
                        panic!("Expected infinite statement");
                    }
                }
            }
        }
    }


    #[test]
    fn with_var_decl_without_value() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                if *t == Type::Char {
                    continue
                }
 
                let stmts = parse_body(&format!("infinite {{\nown {} {}\n}}", l, t));
                assert_eq!(stmts.len(), 1);

                if let Stmt::Infinite(inf) = &stmts[0] {
                    assert_eq!(inf.branch.len(), 1);

                    if let Stmt::VarDecl(v) = &inf.branch[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                    } else { panic!("Expected VarDecl"); }
                } else {
                    panic!("Expected infinite statement");
                }
            }
        }
    }

    #[test]
    fn with_expr_stmt() {
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
    fn before_expr_stmt() {
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
    fn after_expr_stmt() {
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
    fn invalid_branch_stmts_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("infinite {{\n{}\n}}", t)));
        }
    }

    #[test]
    fn nested() {
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
    fn invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&format!("infinite range({}) {{\n\n}}", l));    
            assert_parse_err(&format!("infinite range(, {}) {{\n\n}}", l));    
            assert_parse_err(&format!("infinite range({}, ) {{\n\n}}", l));    
            assert_parse_err(&format!("infinite range({}, {}) {{\n\n}}", l, l));    
            assert_parse_err(&format!("infinite range({}, {} {{\n\n}}", l, l));    
            assert_parse_err(&format!("infinite range{}, {} {{\n\n}}", l, l));    

            assert_parse_err(&format!("infinite {} in {} {{\n\n}}", l, l));    
            assert_parse_err(&format!("infinite in {} {{\n\n}}", l));    
            assert_parse_err(&format!("infinite {} in {{\n\n}}", l));

            assert_parse_err(&format!("infinite {} {{\n\n}}", l));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", l));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", l));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", l));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", l, l));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", l, l));   
        }

        assert_parse_err("infinite {\n\n");    
        assert_parse_err("infinite {}");    
        assert_parse_err("infinite {{\n\n}}");    
        assert_parse_err("infinite \n\n}");    

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("infinite {} {{\n\n}}", kw));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", kw));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", kw));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", kw));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", kw, kw));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", kw, kw));    


            assert_parse_err(&format!("infinite {} {{\n\n}}", kw.to_uppercase()));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", kw.to_uppercase()));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", kw.to_uppercase()));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", kw.to_uppercase()));    

            assert_parse_err(&format!("{}infinite{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", kw, kw.to_uppercase()));
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", kw.to_uppercase(), kw));    
            
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", kw, kw.to_uppercase()));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", kw.to_uppercase(), kw));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("infinite {} {{\n\n}}", t));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", t));    
            assert_parse_err(&format!("{} infinite {{\n\n}}", t));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", t));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", t, t));    
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", t, t));

            assert_parse_err(&format!("infinite {} {{\n\n}}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("infinite{} {{\n\n}}", t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} infinite {{\n\n}}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{}infinite {{\n\n}}", t.to_string().to_uppercase()));

            assert_parse_err(&format!("{}infinite{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{}infinite{} {{\n\n}}", t, t.to_string().to_uppercase())); 

            assert_parse_err(&format!("{} infinite {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} infinite {} {{\n\n}}", t.to_string().to_uppercase(), t));
        }
    }

    #[test]
    fn spaces_after_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            assert_parse_err(&format!("infinite {} {{\n\n}}", spaces));
            spaces.push(' ');
        }
    }

    #[test]
    fn spaces_before_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..=MAX_SPACES {
            assert_parse_err(&format!("{} infinite {{\n\n}}", spaces));
            spaces.push(' ');

        }
    }


    #[test]
    fn below_var_decl_with_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    assert_parse_err(&format!("own {} {} = {}\ninfinite {{\n\n}}", l, t, lit));
                }
            }
        }
    }

    #[test]
    fn below_var_decl_without_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                if *t == Type::Char {
                    continue
                }

                assert_parse_err(&format!("own {} {}\ninfinite {{\n\n}}", l, t));
            }
        }
    }




    #[test]
    fn after_var_decl_with_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    assert_parse_err(&format!("infinite {{\n\n}}\nown {} {} = {}", l, t, lit));
                }
            }
        }
    }

    #[test]
    fn after_var_decl_without_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("infinite {{\n\n}}\nown {} {}", l, t));
            }
        }
    }


    #[test]
    fn with_var_decl_with_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        let literals_edge_cases = get_all_literals_edge_cases();

        for lit in literals_edge_cases {
            for l in &letters {
                for t in ALL_TYPES_NO_ARR {
                    assert_parse_err(&format!("infinite {{\nown {} {} = {}\n}}", l, t, lit));
                }
            }
        }
    }


    #[test]
    fn with_var_decl_without_value_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("infinite {{\nown {} {}\n}}", l, t));
            }
        }
    }

    #[test]
    fn with_expr_stmt_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            assert_parse_err(&format!("infinite {{\n{}\n}}", l));
        }
    }

    #[test]
    fn before_expr_stmt_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            assert_parse_err(&format!("{}\ninfinite {{\n\n}}", l));
        }
    }


    #[test]
    fn after_expr_stmt_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for l in literals_edge_cases {
            assert_parse_err(&format!("infinite {{\n\n}}\n{}", l));
        }
    }

    #[test]
    fn invalid_branch_stmts_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("infinite {{\n{}\n}}", t));
        }
    }

    #[test]
    fn nested() {
        let mut s = format!("infinite {{\n\n}}");

        for _ in 0..100 {
            s = format!("infinite {{\n{}\n}}", s);
            assert_parse_err(&s);
        }
    }

}


