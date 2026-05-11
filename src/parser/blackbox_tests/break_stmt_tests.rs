use super::*;

#[cfg(test)]
mod break_stmt_tests {
    use super::*; 

    #[test]
    fn break_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("break range(,)"));    
        assert_parse_err(&wrap("break range()"));    
        assert_parse_err(&wrap("break {} {{\n\n}}"));    

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("break {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("break {}", l)));    
            assert_parse_err(&wrap(&format!("break {} in {}", l, l)));    
            assert_parse_err(&wrap(&format!("break range({})", l)));    
            assert_parse_err(&wrap(&format!("break range(, {})", l)));    
            assert_parse_err(&wrap(&format!("break range({}, )", l)));    
            assert_parse_err(&wrap(&format!("break range({}, {})", l, l)));    
            assert_parse_err(&wrap(&format!("break range({}, {}", l, l)));    
            assert_parse_err(&wrap(&format!("break range, {}", l)));    


            assert_parse_err(&wrap(&format!("break {} {} in {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("break {} in {} {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("break in {}", l)));    
            assert_parse_err(&wrap(&format!("break {} in ", l)));

            assert_parse_err(&wrap(&format!("{} break", l)));    
            assert_parse_err(&wrap(&format!("{} break {}", l, l)));    
        }

        assert_parse_err(&wrap("break {\n\n"));    
        assert_parse_err(&wrap("break {{\n\n"));    
        assert_parse_err(&wrap("break \n\n}"));    
        assert_parse_err(&wrap("break \n\n}}"));    
        assert_parse_err(&wrap("break {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("break {}", kw)));    
            assert_parse_err(&wrap(&format!("break {} {{\n\n}}", kw)));    

            assert_parse_err(&wrap(&format!("break {}", kw)));
            assert_parse_err(&wrap(&format!("break {}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("{} break", kw)));    
            assert_parse_err(&wrap(&format!("{} break {}", kw, kw)));    

            assert_parse_err(&wrap(&format!("{} break", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} break {}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} break {}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} break {}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("break {}", t)));    
            assert_parse_err(&wrap(&format!("{} break", t)));    
            assert_parse_err(&wrap(&format!("{} break {}", t, t)));    


            assert_parse_err(&wrap(&format!("break {}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} break", t.to_string().to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} break {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} break {}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} break {}", t.to_string().to_uppercase(), t)));
        }
    }
}
