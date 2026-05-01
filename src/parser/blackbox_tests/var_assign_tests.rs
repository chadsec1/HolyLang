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
    fn var_assign_with_var_decl() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                for lit in &literals_edge_cases {
                    let stmts = parse_body(&format!("own {} {}\n{} = {}", l, t, l, lit));
                    assert_eq!(stmts.len(), 2);

                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_none());
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
                        assert!(v.value.is_some());
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
