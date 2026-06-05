use super::*;

#[cfg(test)]
mod var_multi_decl_in_functions_tests {
    use super::*;

    #[test]
    fn missing_type_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals { 
            assert_parse_err(&wrap(&format!("own x, y = {}", l)));
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own x {}, y = {}", t, t)));
                assert_parse_err(&wrap(&format!("own x, y {} = {}", l, t)));
            }
        }
    }

    #[test]
    fn unknown_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own x badtype, y {} = {}", t, l)));
                assert_parse_err(&wrap(&format!("own x badtype, y {}", t)));

                assert_parse_err(&wrap(&format!("own x {}, y badtype = {}", t, l)));
                assert_parse_err(&wrap(&format!("own x {}, y badtype", t)));
                
                assert_parse_err(&wrap(&format!("own x {}, y {} = {}", l, t, l)));
                assert_parse_err(&wrap(&format!("own x {}, y {}", l, t)));

                assert_parse_err(&wrap(&format!("own x {}, y {} = {}", t, l, l)));
                assert_parse_err(&wrap(&format!("own x {}, y {}", t, l)));
            }
            assert_parse_err(&wrap(&format!("own x badtype, y badtype = {}", l)));
            assert_parse_err(&wrap(&format!("own x {}, y {}", l, l)));
            assert_parse_err(&wrap(&format!("own x {}, y {} = {}", l, l, l)));
        }
        assert_parse_err(&wrap("own x badtype, y badtype"));

        for l1 in &letters {
            for l2 in &letters {
                assert_parse_err(&wrap(&format!("own {} {}, {} {}", l1, l2, l1, l2)));
            }

            for lit in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own x {}, y {} = {}", l1, l1, lit)));
            }
        }
    }

    #[test]
    fn no_value_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {}, y {}", t, t)));
        }
    }

    #[test]
    fn invalid_var_name_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            if l.chars().all(|c| c.is_ascii_alphabetic()) {
                continue
            }

            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own {} {}, y {} = {}", l, t, t, l)));
                assert_parse_err(&wrap(&format!("own x {}, {} {} = {}", t, l, t, l)));
                assert_parse_err(&wrap(&format!("own {} {}, {} {} = {}", l, t, l, t, l)));

                assert_parse_err(&wrap(&format!("own {} {}, y {}", l, t, t)));
                assert_parse_err(&wrap(&format!("own x {}, {} {}", t, l, t)));
                assert_parse_err(&wrap(&format!("own {} {}, {} {}", l, t, l, t)));
            }
        }
    }


    #[test]
    fn var_decl_multi() {
        let literals = get_all_literals_edge_cases(); 
         
        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                let stmts = parse_body(&format!("own x {}, y {}, z {} = {}", t, t, t, l));
                assert!(matches!(stmts[0], Stmt::VarDeclMulti(_, _)));
                if let Stmt::VarDeclMulti(vars, _) = &stmts[0] {
                    assert_eq!(vars.len(), 3);
                    assert_eq!(vars[0].name, "x");
                    assert_eq!(vars[0].type_name, t.clone());
                    assert_eq!(vars[1].name, "y");
                    assert_eq!(vars[1].type_name, t.clone());
                    assert_eq!(vars[2].name, "z");
                    assert_eq!(vars[2].type_name, t.clone());
                } else { panic!("Expected VarDeclMulti"); }
            }
        }
    }

    #[test]
    fn different_types() {
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
}
