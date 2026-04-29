use super::*;

#[cfg(test)]
mod infinite_stmt_tests {
    use super::*;


    #[test]
    fn infinite_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        assert_parse_err(&wrap("infinite x {\n\n}"));    
        assert_parse_err(&wrap("infinite range(1, 10) {\n\n}"));    
        assert_parse_err(&wrap("infinite range() {\n\n}"));    
        assert_parse_err(&wrap("infinite range {\n\n}"));    
        assert_parse_err(&wrap("infinite infinite {\n\n}"));    
        assert_parse_err(&wrap("infinite i in x {\n\n}"));    
        assert_parse_err(&wrap("infinite in x {\n\n}"));    
        assert_parse_err(&wrap("infinite i in {\n\n}"));
        assert_parse_err(&wrap("infinite true {\n\n}"));
        assert_parse_err(&wrap("infinite false {\n\n}"));    
        assert_parse_err(&wrap("infinite 1 {\n\n}")); 
        assert_parse_err(&wrap("infinite 1.0 {\n\n}")); 
        assert_parse_err(&wrap("infinite \"\" {\n\n}"));    
        assert_parse_err(&wrap("infinite {\n\n"));    
        assert_parse_err(&wrap("infinite {}"));    
        assert_parse_err(&wrap("infinite \n\n}"));    

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", kw)));    
        }

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", l)));    
        }
    }

    #[test]
    fn infinite_statements_valid_construction() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            let stmts = parse_body(&format!("infinite {} {{\n\n}}", spaces));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }
}
