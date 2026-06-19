use super::*;

#[cfg(test)]
mod var_assign_tests {
    use super::*; 


    #[test]
    fn var_assign() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
         
        for l in letters {
            for lit in &literals_edge_cases {
                let stmts = parse_body(&format!("{} = {}", l, lit));
                assert_eq!(stmts.len(), 1);
                if let Stmt::VarAssign(va) = &stmts[0] {
                    assert_eq!(va.name, l.to_string());
                } else {
                    panic!("Expected VarAssign");
                }
            }
        }
    }

    #[test]
    fn invalid_var_name_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            if l.chars().all(|c| c.is_ascii_alphabetic()) {
                continue
            }

            assert_parse_err(&wrap(&format!("{} = {}", l, l)));
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("{} = {}", t, l)));
                assert_parse_err(&wrap(&format!("{} = {}", l, t)));
                assert_parse_err(&wrap(&format!("{} = {}", t, t)));
            }
        }
    }


    #[test]
    fn var_assign_with_var_decl_no_value() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                if *t == Type::Char {
                    continue
                }

                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {}\n{} = {}", l, t, l, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.type_name.get_default_value(v.span).unwrap(), v.value);

                        if let Stmt::VarAssign(va) = &stmts[1] {
                            assert_eq!(va.name, l.to_string());
                        } else {
                            panic!("Expected VarAssign");
                        }

                    } else { panic!("Expected VarDecl"); }    
                }
            }
        }
    }

    #[test]
    fn var_re_assign_with_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {} = {}\n{} = {}", l, t, lit, l, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                    } else { panic!("Expected VarDecl"); }


                    if let Stmt::VarAssign(va) = &stmts[1] {
                        assert_eq!(va.name, l.to_string());
                    } else {
                        panic!("Expected VarAssign");
                    }
                }
            }
        }
    }


    #[test]
    fn var_assign_multi() {
        let letters_1: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();

        let letters_2: Vec<char> = ('A'..='Z')
            .chain('a'..='z')
            .collect();
        
        for l1 in letters_1 {
            for l2 in &letters_2 {
                let stmts = parse_body(&format!("{}, {} = swap()", l1, l2));
                if let Stmt::VarAssignMulti(ma) = &stmts[0] {
                    assert_eq!(ma.names, vec![l1.to_string(), l2.to_string()]);
                } else {
                    panic!("Expected VarAssignMulti");
                }
            }
        }
    }

}
