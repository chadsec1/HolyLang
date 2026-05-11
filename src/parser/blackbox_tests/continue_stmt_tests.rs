use super::*;

#[cfg(test)]
mod continue_stmt_in_function_tests {
    use super::*; 

    #[test]
    fn continue_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("continue range(,)"));    
        assert_parse_err(&wrap("continue range()"));    
        assert_parse_err(&wrap("continue {} {{\n\n}}"));    

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("continue {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("continue {}", l)));    
            assert_parse_err(&wrap(&format!("continue {} in {}", l, l)));    
            assert_parse_err(&wrap(&format!("continue range({})", l)));    
            assert_parse_err(&wrap(&format!("continue range(, {})", l)));    
            assert_parse_err(&wrap(&format!("continue range({}, )", l)));    
            assert_parse_err(&wrap(&format!("continue range({}, {})", l, l)));    
            assert_parse_err(&wrap(&format!("continue range({}, {}", l, l)));    
            assert_parse_err(&wrap(&format!("continue range, {}", l)));    


            assert_parse_err(&wrap(&format!("continue {} {} in {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("continue {} in {} {}", l, l, l)));    
            assert_parse_err(&wrap(&format!("continue in {}", l)));    
            assert_parse_err(&wrap(&format!("continue {} in ", l)));

            assert_parse_err(&wrap(&format!("{} continue", l)));    
            assert_parse_err(&wrap(&format!("{} continue {}", l, l)));    
        }

        assert_parse_err(&wrap("continue {\n\n"));    
        assert_parse_err(&wrap("continue {{\n\n"));    
        assert_parse_err(&wrap("continue \n\n}"));    
        assert_parse_err(&wrap("continue \n\n}}"));    
        assert_parse_err(&wrap("continue {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("continue {}", kw)));    
            assert_parse_err(&wrap(&format!("continue {} {{\n\n}}", kw)));    

            assert_parse_err(&wrap(&format!("continue {}", kw)));
            assert_parse_err(&wrap(&format!("continue {}", kw.to_uppercase())));

            assert_parse_err(&wrap(&format!("{} continue", kw)));    
            assert_parse_err(&wrap(&format!("{} continue {}", kw, kw)));    

            assert_parse_err(&wrap(&format!("{} continue", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} continue {}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} continue {}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} continue {}", kw.to_uppercase(), kw)));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("continue {}", t)));    
            assert_parse_err(&wrap(&format!("{} continue", t)));    
            assert_parse_err(&wrap(&format!("{} continue {}", t, t)));    


            assert_parse_err(&wrap(&format!("continue {}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} continue", t.to_string().to_uppercase())));    

            assert_parse_err(&wrap(&format!("{} continue {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} continue {}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} continue {}", t.to_string().to_uppercase(), t)));
        }
    }

    #[test]
    fn continue_stmt() {
        let stmts = parse_body("continue");
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Stmt::Continue(_)));
    }

    #[test]
    fn continue_stmt_in_var_decl_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own {} {} = continue", l, t)));    
            }
        }
    }


    #[test]
    fn continue_stmt_in_infinite_stmt() {
        let stmts = parse_body("infinite {\ncontinue\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::Infinite(inf) = &stmts[0] {
            assert_eq!(inf.branch.len(), 1);
            assert!(matches!(inf.branch[0], Stmt::Continue(_)));
        } else {panic!("Expected infinite statement"); }
    }


    #[test]
    fn continue_stmt_in_while_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("while {} {{\ncontinue\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {
                assert_eq!(w.branch.len(), 1);
                assert!(matches!(w.branch[0], Stmt::Continue(_)));
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_for_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let stmts = parse_body(&format!("for i in {} {{\ncontinue\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Continue(_)));
                assert!(!matches!(f.value, Expr::RangeCall{ .. }));

            } else { panic!("expected for statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_for_stmt_range() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let stmts = parse_body(&format!("for i in range({}, {}){{\ncontinue\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Continue(_)));
                assert!(matches!(f.value, Expr::RangeCall{ .. }));
            } else { panic!("expected for statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\ncontinue\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} else {{\ncontinue\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_with_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\ncontinue\n}} else {{\ncontinue\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} elif {} {{\ncontinue\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\ncontinue\n}} elif {} {{\ncontinue\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} elif {} {{\ncontinue\n}} else {{\ncontinue\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_with_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\ncontinue\n}} elif {} {{\ncontinue\n}} else {{\ncontinue\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }
}


//
//
//
//
//
//
// Now globals
//


#[cfg(test)]
mod continue_stmt_in_global_tests {
    use super::*; 

    #[test]
    fn continue_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err("continue range(,)");    
        assert_parse_err("continue range()");    
        assert_parse_err("continue {} {{\n\n}}");    

        for l in &literals_edge_cases {
            assert_parse_err(&format!("continue {} {{\n\n}}", l));    
            assert_parse_err(&format!("continue {}", l));
            assert_parse_err(&format!("continue {} in {}", l, l));
            assert_parse_err(&format!("continue range({})", l));
            assert_parse_err(&format!("continue range(, {})", l));
            assert_parse_err(&format!("continue range({}, )", l));
            assert_parse_err(&format!("continue range({}, {})", l, l));
            assert_parse_err(&format!("continue range({}, {}", l, l));
            assert_parse_err(&format!("continue range, {}", l));


            assert_parse_err(&format!("continue {} {} in {}", l, l, l));    
            assert_parse_err(&format!("continue {} in {} {}", l, l, l));    
            assert_parse_err(&format!("continue in {}", l));
            assert_parse_err(&format!("continue {} in ", l));

            assert_parse_err(&format!("{} continue", l));
            assert_parse_err(&format!("{} continue {}", l, l));
        }

        assert_parse_err("continue {\n\n");    
        assert_parse_err("continue {{\n\n");    
        assert_parse_err("continue \n\n}");    
        assert_parse_err("continue \n\n}}");    
        assert_parse_err("continue {{\n\n}}");    

        
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("continue {}", kw));    
            assert_parse_err(&format!("continue {} {{\n\n}}", kw));    

            assert_parse_err(&format!("continue {}", kw));
            assert_parse_err(&format!("continue {}", kw.to_uppercase()));

            assert_parse_err(&format!("{} continue", kw));    
            assert_parse_err(&format!("{} continue {}", kw, kw));    

            assert_parse_err(&format!("{} continue", kw.to_uppercase()));    

            assert_parse_err(&format!("{} continue {}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} continue {}", kw, kw.to_uppercase()));
            assert_parse_err(&format!("{} continue {}", kw.to_uppercase(), kw));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("continue {}", t));    
            assert_parse_err(&format!("{} continue", t));    
            assert_parse_err(&format!("{} continue {}", t, t));    


            assert_parse_err(&format!("continue {}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{} continue", t.to_string().to_uppercase()));    

            assert_parse_err(&format!("{} continue {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() ));
            assert_parse_err(&format!("{} continue {}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} continue {}", t.to_string().to_uppercase(), t));
        }
    }

    #[test]
    fn continue_stmt() {
        let ast = parse("continue").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        assert!(matches!(ast.globals[0], Stmt::Continue(_)));
    }

    #[test]
    fn continue_stmt_in_var_decl_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("own {} {} = continue", l, t));    
            }
        }
    }


    #[test]
    fn continue_stmt_in_infinite_stmt() {
        let ast = parse("infinite {\ncontinue\n}").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        if let Stmt::Infinite(inf) = &ast.globals[0] {
            assert_eq!(inf.branch.len(), 1);
            assert!(matches!(inf.branch[0], Stmt::Continue(_)));
        } else {panic!("Expected infinite statement"); }
    }


    #[test]
    fn continue_stmt_in_while_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("while {} {{\ncontinue\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                assert_eq!(w.branch.len(), 1);
                assert!(matches!(w.branch[0], Stmt::Continue(_)));
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_for_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let ast = parse(&format!("for i in {} {{\ncontinue\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::For(f) = &ast.globals[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Continue(_)));
                assert!(!matches!(f.value, Expr::RangeCall{ .. }));

            } else { panic!("expected for statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_for_stmt_range() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let ast = parse(&format!("for i in range({}, {}){{\ncontinue\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::For(f) = &ast.globals[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Continue(_)));
                assert!(matches!(f.value, Expr::RangeCall{ .. }));
            } else { panic!("expected for statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\ncontinue\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} else {{\ncontinue\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_with_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\ncontinue\n}} else {{\ncontinue\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} elif {} {{\ncontinue\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\ncontinue\n}} elif {} {{\ncontinue\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn continue_stmt_in_if_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} elif {} {{\ncontinue\n}} else {{\ncontinue\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn continue_stmt_in_if_with_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\ncontinue\n}} elif {} {{\ncontinue\n}} else {{\ncontinue\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Continue(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Continue(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Continue(_)));
            } else {panic!("Expected if statement"); }
        }
    }
}


