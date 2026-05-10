use super::*;

#[cfg(test)]
mod for_stmt_tests {
    use super::*; 
    
    #[test]
    fn for_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("for i in {} {{\n\n{}}}", l, l)));    
            assert_parse_err(&wrap(&format!("for range({}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("for range(, {}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("for range({}, ) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("for range({}, {}) {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("for range({}, {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("for range{}, {} {{\n\n}}", l, l)));    


            assert_parse_err(&wrap(&format!("for {} {} in {} {{\n\n}}", l, l, l)));    
            assert_parse_err(&wrap(&format!("for {} in {} {} {{\n\n}}", l, l, l)));    
            assert_parse_err(&wrap(&format!("for in {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("for {} in {{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("for{} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{} for {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}for {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", l, l)));    
        }

        assert_parse_err(&wrap("for {\n\n"));    
        assert_parse_err(&wrap("for {}"));    
        assert_parse_err(&wrap("for \n\n}"));    
        assert_parse_err(&wrap("for {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("for {} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("for {} {{\n\n}}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("for{} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{} for {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}for {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", kw, kw)));    
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", kw, kw)));    

            assert_parse_err(&wrap(&format!("for{} {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} for {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}for {{\n\n}}", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", kw.to_uppercase(), kw)));    
            
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", kw.to_uppercase(), kw)));    

        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("for {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("for{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{} for {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}for {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", t, t)));    
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", t, t)));    


            assert_parse_err(&wrap(&format!("for {} {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("for{} {{\n\n}}", t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} for {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}for {{\n\n}}", t.to_string().to_uppercase() )));

            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{}for{} {{\n\n}}", t, t.to_string().to_uppercase()))); 

            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} for {} {{\n\n}}", t.to_string().to_uppercase(), t)));

        }
    }

    #[test]
    fn for_statements_trailing_exprs_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n", l, l)));

            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n", l, l)));
            assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n", l, l)));
        }
    }

    #[test]
    fn for_statements_trailing_kw_errors() {
        let literals = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for l in &literals {
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n", l, kw)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n", l, kw)));




                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {} {} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{} {}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{}{{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{}{{\n\n", l, kw, l)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{}\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{}\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{}\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{}\n\n", l, kw, l)));
            }
        }
    }

    #[test]
    fn for_statements_trailing_types_errors() {
        let literals = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {} {{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}{{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}{{\n\n", l, t)));    

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n}}", l, t)));

                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}} {}\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("for i in {}{{\n\n}}{}\n\n", l, t)));
            }
        }
    }







    #[test]
    fn for_statements() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let stmts = parse_body(&format!("for i in {} {{\n\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }

    #[test]
    fn for_statements_vars() {
        let stmts = parse_body("for i in x {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::Var { name, .. } = &f.value {
                assert_eq!(name, "x"); 
            } else { panic!("Expected Var expression") }
        } else {
            panic!("expected while statement");
        }
    }

    #[test]
    fn for_statements_literal() {
        let stmts = parse_body("for i in [12,\"hi\", true, 6.9, []] {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::ArrayLiteral { elements, .. } = &f.value {
                assert_eq!(elements.len(), 5);

                if let Expr::IntLiteral { value, .. } = &elements[0] {
                    assert!(matches!(value, IntLiteralValue::Int8(12)));
                } else { panic!("Expected IntLiteral"); }

                if let Expr::StringLiteral { value, .. } = &elements[1] {
                    assert_eq!(value, "hi");
                } else { panic!("Expected StringLiteral"); }

                if let Expr::BoolLiteral { value, .. } = &elements[2] {
                    assert_eq!(value, &true);
                } else { panic!("Expected BoolLiteral"); }

                if let Expr::Float64Literal { value, .. } = &elements[3] {
                    assert_eq!(*value, 6.9);
                } else { panic!("Expected Float64Literal"); }

                if let Expr::ArrayLiteral { elements, .. } = &elements[4] {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }


            } else {
                panic!("Expected ArrayLiteral");
            }

        } else {
            panic!("expected while statement");
        }
    }


    #[test]
    fn for_statements_2_holders_errors() {
        assert_parse_err(&wrap("for i v in x {\n\n}"));    
    }


    #[test]
    fn for_statements_2_values_errors() {
        assert_parse_err(&wrap("for i in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_2_holders_and_values_errors() {
        assert_parse_err(&wrap("for i v in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_no_value_errors() {
        assert_parse_err(&wrap("for i in {\n\n}"));    
    }


    #[test]
    fn for_statements_no_holder_errors() {
        assert_parse_err(&wrap("for in x {\n\n}"));    
    }

    #[test]
    fn for_statements_2_in() {
        assert_parse_err(&wrap("for i in in x {\n\n}"));    
        assert_parse_err(&wrap("for in i in x {\n\n}"));    
        assert_parse_err(&wrap("for i in x in {\n\n}"));    
        assert_parse_err(&wrap("for in i x in {\n\n}"));    
        assert_parse_err(&wrap("for i x in {\n\n}"));    
    }

    #[test]
    fn for_statements_no_in() {
        assert_parse_err(&wrap("for i x {\n\n}"));    
    }

    #[test]
    fn for_statements_no_holder_no_value_no_in_errors() {
        assert_parse_err(&wrap("for {\n\n}"));    
    }
}
