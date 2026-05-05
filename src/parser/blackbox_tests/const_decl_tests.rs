use super::*;

#[cfg(test)]
mod const_decl_in_function_tests {
    use super::*; 
    
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
                } else {
                    panic!("Expected VarDecl");
                }
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
                    } else { panic!("Expected VarDecl"); }
                } else { panic!("Expected infinite statement"); }
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
    // use super::*; 
    

}
