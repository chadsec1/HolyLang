use super::*;

#[cfg(test)]
mod lock_stmt_in_function_tests {
    use super::*; 

    #[test]
    fn invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("lock range(,)"));    
        assert_parse_err(&wrap("lock range()"));    
        assert_parse_err(&wrap("lock {} {{\n\n}}"));

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("lock {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("lock {} in {}", l, l)));    
            assert_parse_err(&wrap(&format!("lock range({})", l)));    
            assert_parse_err(&wrap(&format!("lock range(, {})", l)));    
            assert_parse_err(&wrap(&format!("lock range({}, )", l)));    
            assert_parse_err(&wrap(&format!("lock range({}, {})", l, l)));    
            assert_parse_err(&wrap(&format!("lock range({}, {}", l, l)));    
            assert_parse_err(&wrap(&format!("lock range, {}", l)));    


            assert_parse_err(&wrap(&format!("lock {} {} in {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("lock {} in {} {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("lock in {}", l)));    
            assert_parse_err(&wrap(&format!("lock {} in ", l)));

            assert_parse_err(&wrap(&format!("{} lock", l)));    
            assert_parse_err(&wrap(&format!("{} lock {}", l, l)));    
        }

        assert_parse_err(&wrap("lock {\n\n"));    
        assert_parse_err(&wrap("lock {{\n\n"));    
        assert_parse_err(&wrap("lock \n\n}"));    
        assert_parse_err(&wrap("lock \n\n}}"));    
        assert_parse_err(&wrap("lock {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "true" || *kw == "false" {
                continue
            }

            assert_parse_err(&wrap(&format!("lock {}", kw)));    
            assert_parse_err(&wrap(&format!("lock {} {{\n\n}}", kw)));    

            assert_parse_err(&wrap(&format!("lock {}", kw)));
            assert_parse_err(&wrap(&format!("lock {}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("{} lock", kw)));    
            assert_parse_err(&wrap(&format!("{} lock {}", kw, kw)));    

            assert_parse_err(&wrap(&format!("{} lock", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} lock {}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} lock {}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} lock {}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("lock {}", t)));    
            assert_parse_err(&wrap(&format!("{} lock", t)));    
            assert_parse_err(&wrap(&format!("{} lock {}", t, t)));    


            assert_parse_err(&wrap(&format!("lock {}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} lock", t.to_string().to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} lock {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} lock {}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} lock {}", t.to_string().to_uppercase(), t)));
        }
    }


    #[test]
    fn lock_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let stmts = parse_body(&format!("lock {}", l));
            assert_eq!(stmts.len(), 1);
        
            if let Stmt::Lock(expr_vec) = &stmts[0] {
                assert_eq!(expr_vec.len(), 1);
            } else {panic!("Expected lock statement"); }

        }
    }

    #[test]
    fn unterminated_string_errors() {
        let literals = get_all_literals_edge_cases();

        assert_parse_err(&wrap("lock \"hi"));    
        assert_parse_err(&wrap("lock \"hi, lol"));    

        for l in literals { 
            assert_parse_err(&wrap(&format!("lock {}, \"hi", l)));    
            assert_parse_err(&wrap(&format!("lock \"hi, {}", l)));    
        }
    }

    #[test]
    fn multiple() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            let stmts = parse_body(&format!("lock {}, {}, {}", l, l, l));
            assert_eq!(stmts.len(), 1);
        
            if let Stmt::Lock(expr_vec) = &stmts[0] {
                assert_eq!(expr_vec.len(), 3);
            } else {panic!("Expected lock statement"); }

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
mod lock_stmt_in_global_tests { 
    use super::*; 

    #[test]
    fn invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err("lock range(,)");    
        assert_parse_err("lock range()");    
        assert_parse_err("lock {} {{\n\n}}");

        for l in &literals_edge_cases {
            assert_parse_err(&format!("lock {} {{\n\n}}", l));    
            assert_parse_err(&format!("lock {} in {}", l, l));    
            assert_parse_err(&format!("lock range({})", l));    
            assert_parse_err(&format!("lock range(, {})", l));    
            assert_parse_err(&format!("lock range({}, )", l));    
            assert_parse_err(&format!("lock range({}, {})", l, l));    
            assert_parse_err(&format!("lock range({}, {}", l, l));    
            assert_parse_err(&format!("lock range, {}", l));

            assert_parse_err(&format!("lock {} {} in {}", l, l, l));    
            assert_parse_err(&format!("lock {} in {} {}", l, l, l));    
            assert_parse_err(&format!("lock in {}", l));
            assert_parse_err(&format!("lock {} in ", l));

            assert_parse_err(&format!("{} lock", l));    
            assert_parse_err(&format!("{} lock {}", l, l));    
        }

        assert_parse_err("lock {\n\n");    
        assert_parse_err("lock {{\n\n");    
        assert_parse_err("lock \n\n}");    
        assert_parse_err("lock \n\n}}");    
        assert_parse_err("lock {{\n\n}}");    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "true" || *kw == "false" {
                continue
            }

            assert_parse_err(&format!("lock {}", kw));    
            assert_parse_err(&format!("lock {} {{\n\n}}", kw));    

            assert_parse_err(&format!("lock {}", kw));
            assert_parse_err(&format!("lock {}", kw.to_uppercase()));

            assert_parse_err(&format!("{} lock", kw));    
            assert_parse_err(&format!("{} lock {}", kw, kw));    

            assert_parse_err(&format!("{} lock", kw.to_uppercase()));    

            assert_parse_err(&format!("{} lock {}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} lock {}", kw, kw.to_uppercase()));    
            assert_parse_err(&format!("{} lock {}", kw.to_uppercase(), kw));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("lock {}", t));    
            assert_parse_err(&format!("{} lock", t));    
            assert_parse_err(&format!("{} lock {}", t, t));

            assert_parse_err(&format!("lock {}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{} lock", t.to_string().to_uppercase()));    

            assert_parse_err(&format!("{} lock {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() ));
            assert_parse_err(&format!("{} lock {}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} lock {}", t.to_string().to_uppercase(), t));
        }
    }


    #[test]
    fn lock_stmt_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            assert_parse_err(&format!("lock {}", l));
        }
    }

    #[test]
    fn unterminated_string_errors() {
        let literals = get_all_literals_edge_cases();

        assert_parse_err(&wrap("lock \"hi"));    
        assert_parse_err(&wrap("lock \"hi, lol"));    

        for l in literals { 
            assert_parse_err(&wrap(&format!("lock {}, \"hi", l)));    
            assert_parse_err(&wrap(&format!("lock \"hi, {}", l)));    
        }
    }

    #[test]
    fn multiple_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            assert_parse_err(&format!("lock {}, {}, {}", l, l, l));
        }
    }

}
