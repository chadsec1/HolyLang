use super::*;

#[cfg(test)]
mod return_stmt_tests {
    use super::*; 

    #[test]
    fn return_statements_invalid_args_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("return"));
        assert_parse_err(&wrap("return "));
        assert_parse_err(&wrap("return range(,)"));    
        assert_parse_err(&wrap("return range()"));    

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("return {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("return {} in {}", l, l)));    
            assert_parse_err(&wrap(&format!("return range({})", l)));    
            assert_parse_err(&wrap(&format!("return range(, {})", l)));    
            assert_parse_err(&wrap(&format!("return range({}, )", l)));    
            assert_parse_err(&wrap(&format!("return range({}, {})", l, l)));    
            assert_parse_err(&wrap(&format!("return range({}, {}", l, l)));    
            assert_parse_err(&wrap(&format!("return range, {}", l)));    


            assert_parse_err(&wrap(&format!("return {} {} in {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("return {} in {} {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("return in {}", l)));    
            assert_parse_err(&wrap(&format!("return {} in ", l)));

            assert_parse_err(&wrap(&format!("{} return", l)));    
            assert_parse_err(&wrap(&format!("{} return {}", l, l)));    
        }

        assert_parse_err(&wrap("return {\n\n"));    
        assert_parse_err(&wrap("return {{\n\n"));    
        assert_parse_err(&wrap("return \n\n}"));    
        assert_parse_err(&wrap("return \n\n}}"));    
        assert_parse_err(&wrap("return {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "true" || *kw == "false" {
                continue
            }

            assert_parse_err(&wrap(&format!("return {} {{\n\n}}", kw)));    

            assert_parse_err(&wrap(&format!("return {}", kw)));
            assert_parse_err(&wrap(&format!("return {}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("{} return", kw)));    
            assert_parse_err(&wrap(&format!("{} return {}", kw, kw)));    

            assert_parse_err(&wrap(&format!("{} return", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} return {}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} return {}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} return {}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("return {}", t)));    
            assert_parse_err(&wrap(&format!("{} return", t)));    
            assert_parse_err(&wrap(&format!("{} return {}", t, t)));    


            assert_parse_err(&wrap(&format!("return {}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} return", t.to_string().to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} return {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} return {}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} return {}", t.to_string().to_uppercase(), t)));
        }
    }



    #[test]
    fn return_single_value() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals { 
            let stmts = parse_body(&format!("return {}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Return(exprs) = &stmts[0] {
                assert_eq!(exprs.len(), 1);
            } else {
                panic!("Expected Return");
            }
        }
    }

    #[test]
    fn return_single_value_str_has_commas() {
        let stmts = parse_body("return \"hi, lol\"");
        assert_eq!(stmts.len(), 1);
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 1);
            if let Expr::StringLiteral { value, .. } = &exprs[0] {
                assert_eq!(value, "hi, lol");
            } else { panic!("Expcted stringLiteral"); }
        } else { panic!("Expected Return"); }
    }

    #[test]
    fn return_single_value_unterminated_str_errors() {
        let literals = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("return \"hi, lol"));
        for l in literals { 
            assert_parse_err(&wrap(&format!("return \"hi, lol {}", l)));
            assert_parse_err(&wrap(&format!("return {}\"hi, lol {}", l, l)));
            assert_parse_err(&wrap(&format!("return {} \"hi, lol", l)));
            assert_parse_err(&wrap(&format!("return {}\"hi, lol", l)));
        }
    }
    

    #[test]
    fn return_multiple_values() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals { 
            let stmts = parse_body(&format!("return {}, {}, {}, {}", l, l, l, l));
         
            assert_eq!(stmts.len(), 1);
            if let Stmt::Return(exprs) = &stmts[0] {
                assert_eq!(exprs.len(), 4);
            } else {
                panic!("Expected Return");
            }
        }
    }

    #[test]
    fn return_multiple_value_str_has_commas() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals { 
            let stmts = parse_body(&format!("return \"hi, lol\", {}, \"hey, ha, ha, ha!\"", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Return(exprs) = &stmts[0] {
                assert_eq!(exprs.len(), 3);

                assert_ne!(exprs[1], exprs[0]);
                assert_ne!(exprs[1], exprs[2]);

                if let Expr::StringLiteral { value, .. } = &exprs[0] {
                    assert_eq!(value, "hi, lol");
                } else { panic!("Expcted stringLiteral"); }

                if let Expr::StringLiteral { value, .. } = &exprs[2] {
                    assert_eq!(value, "hey, ha, ha, ha!");
                } else { panic!("Expcted stringLiteral"); }
            } else { panic!("Expected Return"); }
        }
    }

    #[test]
    fn return_multiple_values_invalid_split_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in literals { 
            if l.starts_with('-') {
                continue
            }

            assert_parse_err(&wrap(&format!("return {} {} {}", l, l, l)));
            assert_parse_err(&wrap(&format!("return {} {}", l, l)));
            assert_parse_err(&wrap(&format!("return {} {} {} {}", l, l, l, l)));
            
            assert_parse_err(&wrap(&format!("return {} {}, {} {}, {} {}", l, l, l, l, l, l)));
            assert_parse_err(&wrap(&format!("return {} {}, {}, {}", l, l, l, l)));
            assert_parse_err(&wrap(&format!("return {}, {} {}, {}", l, l, l, l)));
            assert_parse_err(&wrap(&format!("return {}, {}, {} {}", l, l, l, l)));
            assert_parse_err(&wrap(&format!("return {} {}, {}", l, l, l)));
            assert_parse_err(&wrap(&format!("return {}, {} {}", l, l, l)));
            assert_parse_err(&wrap(&format!("return {} {}, {} {}", l, l, l, l)));
        }
    }




    #[test]
    fn return_multiple_values_ints() {
        let stmts = parse_body("return 1, 2, 300, 69640");

        assert_eq!(stmts.len(), 1);
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 4);

            if let Expr::IntLiteral { value, .. } = &exprs[0] {
                assert!(matches!(value, IntLiteralValue::Int8(1)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[1] {
                assert!(matches!(value, IntLiteralValue::Int8(2)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[2] {
                assert!(matches!(value, IntLiteralValue::Int16(300)));
            } else { panic!("Expcted IntLiteral"); }


            if let Expr::IntLiteral { value, .. } = &exprs[3] {
                assert!(matches!(value, IntLiteralValue::Int32(69640)));
            } else { panic!("Expcted IntLiteral"); }


        } else {
            panic!("Expected Return");
        }
    }

    #[test]
    fn return_without_value_errors() {
        assert_parse_err(&wrap("return"));
    }

    #[test]
    fn return_variable() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = {}\nreturn x", t, l));

                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }    

                if let Stmt::Return(exprs) = &stmts[1] {
                    assert_eq!(exprs.len(), 1);
                    assert!(matches!(exprs[0], Expr::Var { .. }));
                } else {
                    panic!("Expected Return");
                }
            }
        }
    }



}
