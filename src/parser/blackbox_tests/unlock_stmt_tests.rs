use super::*;

#[cfg(test)]
mod unlock_stmt_in_function_tests {
    use super::*; 

    #[test]
    fn unlock_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("unlock range(,)"));    
        assert_parse_err(&wrap("unlock range()"));    
        assert_parse_err(&wrap("unlock {} {{\n\n}}"));

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("unlock {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("unlock {} in {}", l, l)));    
            assert_parse_err(&wrap(&format!("unlock range({})", l)));    
            assert_parse_err(&wrap(&format!("unlock range(, {})", l)));    
            assert_parse_err(&wrap(&format!("unlock range({}, )", l)));    
            assert_parse_err(&wrap(&format!("unlock range({}, {})", l, l)));    
            assert_parse_err(&wrap(&format!("unlock range({}, {}", l, l)));    
            assert_parse_err(&wrap(&format!("unlock range, {}", l)));    


            assert_parse_err(&wrap(&format!("unlock {} {} in {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("unlock {} in {} {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("unlock in {}", l)));    
            assert_parse_err(&wrap(&format!("unlock {} in ", l)));

            assert_parse_err(&wrap(&format!("{} unlock", l)));    
            assert_parse_err(&wrap(&format!("{} unlock {}", l, l)));    
        }

        assert_parse_err(&wrap("unlock {\n\n"));    
        assert_parse_err(&wrap("unlock {{\n\n"));    
        assert_parse_err(&wrap("unlock \n\n}"));    
        assert_parse_err(&wrap("unlock \n\n}}"));    
        assert_parse_err(&wrap("unlock {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "true" || *kw == "false" {
                continue
            }

            assert_parse_err(&wrap(&format!("unlock {}", kw)));    
            assert_parse_err(&wrap(&format!("unlock {} {{\n\n}}", kw)));    

            assert_parse_err(&wrap(&format!("unlock {}", kw)));
            assert_parse_err(&wrap(&format!("unlock {}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("{} unlock", kw)));    
            assert_parse_err(&wrap(&format!("{} unlock {}", kw, kw)));    

            assert_parse_err(&wrap(&format!("{} unlock", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} unlock {}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} unlock {}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} unlock {}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("unlock {}", t)));    
            assert_parse_err(&wrap(&format!("{} unlock", t)));    
            assert_parse_err(&wrap(&format!("{} unlock {}", t, t)));    


            assert_parse_err(&wrap(&format!("unlock {}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} unlock", t.to_string().to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} unlock {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} unlock {}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} unlock {}", t.to_string().to_uppercase(), t)));
        }
    }


    #[test]
    fn unlock_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let stmts = parse_body(&format!("unlock {}", l));
            assert_eq!(stmts.len(), 1);
        
            if let Stmt::Unlock(expr_vec) = &stmts[0] {
                assert_eq!(expr_vec.len(), 1);
            } else {panic!("Expected unlock statement"); }

        }
    }

    #[test]
    fn unlock_stmt_unterminated_string_errors() {
        let literals = get_all_literals_edge_cases();

        assert_parse_err(&wrap("unlock \"hi"));    
        assert_parse_err(&wrap("unlock \"hi, lol"));    

        for l in literals { 
            assert_parse_err(&wrap(&format!("unlock {}, \"hi", l)));    
            assert_parse_err(&wrap(&format!("unlock \"hi, {}", l)));    
        }
    }

    #[test]
    fn unlock_stmt_multiple() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let stmts = parse_body(&format!("unlock {}, {}, {}", l, l, l));
            assert_eq!(stmts.len(), 1);
        
            if let Stmt::Unlock(expr_vec) = &stmts[0] {
                assert_eq!(expr_vec.len(), 3);
            } else {panic!("Expected unlock statement"); }

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

#[cfg(test)]
mod unlock_stmt_in_global_tests { 
    use super::*; 

    #[test]
    fn unlock_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err("unlock range(,)");    
        assert_parse_err("unlock range()");    
        assert_parse_err("unlock {} {{\n\n}}");

        for l in &literals_edge_cases {
            assert_parse_err(&format!("unlock {} {{\n\n}}", l));    
            assert_parse_err(&format!("unlock {} in {}", l, l));    
            assert_parse_err(&format!("unlock range({})", l));    
            assert_parse_err(&format!("unlock range(, {})", l));    
            assert_parse_err(&format!("unlock range({}, )", l));    
            assert_parse_err(&format!("unlock range({}, {})", l, l));    
            assert_parse_err(&format!("unlock range({}, {}", l, l));    
            assert_parse_err(&format!("unlock range, {}", l));

            assert_parse_err(&format!("unlock {} {} in {}", l, l, l));    
            assert_parse_err(&format!("unlock {} in {} {}", l, l, l));    
            assert_parse_err(&format!("unlock in {}", l));
            assert_parse_err(&format!("unlock {} in ", l));

            assert_parse_err(&format!("{} unlock", l));    
            assert_parse_err(&format!("{} unlock {}", l, l));    
        }

        assert_parse_err("unlock {\n\n");    
        assert_parse_err("unlock {{\n\n");    
        assert_parse_err("unlock \n\n}");    
        assert_parse_err("unlock \n\n}}");    
        assert_parse_err("unlock {{\n\n}}");    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "true" || *kw == "false" {
                continue
            }

            assert_parse_err(&format!("unlock {}", kw));    
            assert_parse_err(&format!("unlock {} {{\n\n}}", kw));    

            assert_parse_err(&format!("unlock {}", kw));
            assert_parse_err(&format!("unlock {}", kw.to_uppercase()));

            assert_parse_err(&format!("{} unlock", kw));    
            assert_parse_err(&format!("{} unlock {}", kw, kw));    

            assert_parse_err(&format!("{} unlock", kw.to_uppercase()));    

            assert_parse_err(&format!("{} unlock {}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} unlock {}", kw, kw.to_uppercase()));    
            assert_parse_err(&format!("{} unlock {}", kw.to_uppercase(), kw));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("unlock {}", t));    
            assert_parse_err(&format!("{} unlock", t));    
            assert_parse_err(&format!("{} unlock {}", t, t));

            assert_parse_err(&format!("unlock {}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{} unlock", t.to_string().to_uppercase()));    

            assert_parse_err(&format!("{} unlock {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() ));
            assert_parse_err(&format!("{} unlock {}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} unlock {}", t.to_string().to_uppercase(), t));
        }
    }


    #[test]
    fn unlock_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let ast = parse(&format!("unlock {}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);
        
            if let Stmt::Unlock(expr_vec) = &ast.globals[0] {
                assert_eq!(expr_vec.len(), 1);
            } else {panic!("Expected unlock statement"); }

        }
    }

    #[test]
    fn unlock_stmt_unterminated_string_errors() {
        let literals = get_all_literals_edge_cases();

        assert_parse_err(&wrap("unlock \"hi"));    
        assert_parse_err(&wrap("unlock \"hi, lol"));    

        for l in literals { 
            assert_parse_err(&wrap(&format!("unlock {}, \"hi", l)));    
            assert_parse_err(&wrap(&format!("unlock \"hi, {}", l)));    
        }
    }

    #[test]
    fn unlock_stmt_multiple() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let ast = parse(&format!("unlock {}, {}, {}", l, l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);
        
            if let Stmt::Unlock(expr_vec) = &ast.globals[0] {
                assert_eq!(expr_vec.len(), 3);
            } else {panic!("Expected unlock statement"); }

        }
    }

}
