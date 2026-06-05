use super::*;

#[cfg(test)]
mod const_decl_in_function_tests {
    use super::*; 

    #[test]
    fn const_decl_multi_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("const x {}, y {} = {}", t, t, l)));
                assert_parse_err(&wrap(&format!("const x {}, x {} = {}", t, t, l)));
            }
        }
    }
    
    #[test]
    fn const_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("const {} = {}", l, lit)));
            }
        }
    }


    #[test]
    fn const_decl_invalid_name_errors() {
        let int_literals_edge_cases = get_all_ints_literals_edge_cases(); 

        for l in &int_literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("const {} {} = {}", l, t, l)));
                assert_parse_err(&wrap(&format!("const {} {} = {}", t, l, l)));
            }
        }
    }


    #[test]
    fn const_decl_invalid_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("const {} {} = {}", l, lit, lit)));
                assert_parse_err(&wrap(&format!("const {} {} = {}", l, l, lit)));
            }
        }
    }

    #[test]
    fn const_decl_no_value_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("const x {}", t)));
        }

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("const x {}", l)));
        }
    }

    #[test]
    fn const_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("const x {} = {}", t, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::Const(c) = &stmts[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else { panic!("Expected Const Declaration"); }
            }
        }
    }


    #[test]
    fn const_decl_in_infinite_loop() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("infinite {{\nconst x {} = {}\n}}", t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::Infinite(inf) = &stmts[0] {
                    assert_eq!(inf.branch.len(), 1);

                    if let Stmt::Const(c) = &inf.branch[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected infinite statement"); }
            }
        }
    }

    #[test]
    fn const_decl_in_while_loop() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("while {} {{\nconst x {} = {}\n}}", l, t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::While(w) = &stmts[0] {
                    assert_eq!(w.branch.len(), 1);

                    if let Stmt::Const(c) = &w.branch[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected while statement"); }
            }
        }
    }


    #[test]
    fn const_decl_in_if_main_branch() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("if {} {{\nconst x {} = {}\n}}", l, t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::If(i) = &stmts[0] {
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());

                    if let Stmt::Const(c) = &i.if_branch[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected if statement, got {:?}", stmts[0]); }
            }
        }
    }

    #[test]
    fn const_decl_in_if_else_branch() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("if {} {{\n\n}} else {{\nconst x {} = {}\n}}", l, t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::If(i) = &stmts[0] {
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_some());

                    let else_branch = i.else_branch.clone().unwrap();

                    if let Stmt::Const(c) = &else_branch[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected if statement, got {:?}", stmts[0]); }
            }
        }
    }


    #[test]
    fn const_decl_in_if_elif_branch() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("if {} {{\n\n}} elif {} {{\nconst x {} = {}\n}}", l, l, t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::If(i) = &stmts[0] {
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 1);
                    assert!(i.else_branch.is_none());

                    if let Stmt::Const(c) = &i.elif_branches[0].1[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected if statement, got {:?}", stmts[0]); }
            }
        }
    }


    #[test]
    fn const_decl_in_for_branch() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("for x in {} {{\nconst x {} = {}\n}}", l, t, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::For(f) = &stmts[0] {
                    assert_eq!(f.branch.len(), 1);

                    if let Stmt::Const(c) = &f.branch[0] {
                        assert_eq!(c.name, "x");
                        assert_eq!(c.type_name, t.clone());
                    } else { panic!("Expected Const Declaration"); }
                } else { panic!("Expected for statement, got {:?}", stmts[0]); }
            }
        }
    }


    // This is semantically not allowed, but it is syntaxally valid.
    //
    #[test]
    fn const_decl_and_assign() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("const x {} = {}\nx = {}", t, l, l));
                assert_eq!(stmts.len(), 2);
                if let Stmt::Const(c) = &stmts[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else {
                    panic!("Expected VarDecl");
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


#[cfg(test)]
mod const_decl_in_global_tests {
    use super::*; 
    
    #[test]
    fn const_decl_multi_decl_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("const x {}, y {} = {}", t, t, l));
                assert_parse_err(&format!("const x {}, x {} = {}", t, t, l));
            }
        }
    }
    
    #[test]
    fn const_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("const {} = {}", l, lit));
            }
        }
    }


    #[test]
    fn const_decl_invalid_name_errors() {
        let int_literals_edge_cases = get_all_ints_literals_edge_cases(); 

        for l in &int_literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("const {} {} = {}", l, t, l));
                assert_parse_err(&format!("const {} {} = {}", t, l, l));
            }
        }
    }


    #[test]
    fn const_decl_invalid_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            for lit in &literals_edge_cases {
                assert_parse_err(&format!("const {} {} = {}", l, lit, lit));
                assert_parse_err(&format!("const {} {} = {}", l, l, lit));
            }
        }
    }

    #[test]
    fn const_decl_no_value_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("const x {}", t));
        }

        for l in &literals_edge_cases {
            assert_parse_err(&format!("const x {}", l));
        }
    }

    #[test]
    fn const_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("const x {} = {}", t, l)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let GlobalStmt::Const(c) = &ast.globals[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else { panic!("Expected Const Declaration"); }
            }
        }
    }


    #[test]
    fn const_decl_in_infinite_loop_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("infinite {{\nconst x {} = {}\n}}", t, l));
            }
        }
    }

    #[test]
    fn const_decl_in_while_loop_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("while {} {{\nconst x {} = {}\n}}", l, t, l));
            }
        }
    }


    #[test]
    fn const_decl_in_if_main_branch_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("if {} {{\nconst x {} = {}\n}}", l, t, l));
            }
        }
    }

    #[test]
    fn const_decl_in_if_else_branch_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("if {} {{\n\n}} else {{\nconst x {} = {}\n}}", l, t, l));
            }
        }
    }


    #[test]
    fn const_decl_in_if_elif_branch_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("if {} {{\n\n}} elif {} {{\nconst x {} = {}\n}}", l, l, t, l));
            }
        }
    }


    #[test]
    fn const_decl_in_for_branch_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("for x in {} {{\nconst x {} = {}\n}}", l, t, l));
            }
        }
    }


    // This is semantically not allowed, but it is syntaxally valid.
    //
    #[test]
    fn const_decl_and_assign_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("const x {} = {}\nx = {}", t, l, l));
            }
        }
    }



}
