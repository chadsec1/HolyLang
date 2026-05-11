use super::*;

#[cfg(test)]
mod break_stmt_in_function_tests {
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

    #[test]
    fn break_stmt() {
        let stmts = parse_body("break");
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Stmt::Break(_)));
    }

    #[test]
    fn break_stmt_in_var_decl_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&wrap(&format!("own {} {} = break", l, t)));    
            }
        }
    }


    #[test]
    fn break_stmt_in_infinite_stmt() {
        let stmts = parse_body("infinite {\nbreak\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::Infinite(inf) = &stmts[0] {
            assert_eq!(inf.branch.len(), 1);
            assert!(matches!(inf.branch[0], Stmt::Break(_)));
        } else {panic!("Expected infinite statement"); }
    }


    #[test]
    fn break_stmt_in_while_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("while {} {{\nbreak\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {
                assert_eq!(w.branch.len(), 1);
                assert!(matches!(w.branch[0], Stmt::Break(_)));
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn break_stmt_in_for_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let stmts = parse_body(&format!("for i in {} {{\nbreak\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Break(_)));
                assert!(!matches!(f.value, Expr::RangeCall{ .. }));

            } else { panic!("expected for statement"); }
        }
    }

    #[test]
    fn break_stmt_in_for_stmt_range() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let stmts = parse_body(&format!("for i in range({}, {}){{\nbreak\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Break(_)));
                assert!(matches!(f.value, Expr::RangeCall{ .. }));
            } else { panic!("expected for statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\nbreak\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} else {{\nbreak\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_with_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\nbreak\n}} else {{\nbreak\n}}", l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} elif {} {{\nbreak\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\nbreak\n}} elif {} {{\nbreak\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\n\n}} elif {} {{\nbreak\n}} else {{\nbreak\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_with_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let stmts = parse_body(&format!("if {} {{\nbreak\n}} elif {} {{\nbreak\n}} else {{\nbreak\n}}", l, l));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
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
mod break_stmt_in_global_tests {
    use super::*; 

    #[test]
    fn break_statements_invalid_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err("break range(,)");    
        assert_parse_err("break range()");    
        assert_parse_err("break {} {{\n\n}}");    

        for l in &literals_edge_cases {
            assert_parse_err(&format!("break {} {{\n\n}}", l));    
            assert_parse_err(&format!("break {}", l));
            assert_parse_err(&format!("break {} in {}", l, l));
            assert_parse_err(&format!("break range({})", l));
            assert_parse_err(&format!("break range(, {})", l));
            assert_parse_err(&format!("break range({}, )", l));
            assert_parse_err(&format!("break range({}, {})", l, l));
            assert_parse_err(&format!("break range({}, {}", l, l));
            assert_parse_err(&format!("break range, {}", l));


            assert_parse_err(&format!("break {} {} in {}", l, l, l));    
            assert_parse_err(&format!("break {} in {} {}", l, l, l));    
            assert_parse_err(&format!("break in {}", l));
            assert_parse_err(&format!("break {} in ", l));

            assert_parse_err(&format!("{} break", l));
            assert_parse_err(&format!("{} break {}", l, l));
        }

        assert_parse_err("break {\n\n");    
        assert_parse_err("break {{\n\n");    
        assert_parse_err("break \n\n}");    
        assert_parse_err("break \n\n}}");    
        assert_parse_err("break {{\n\n}}");    

        
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("break {}", kw));    
            assert_parse_err(&format!("break {} {{\n\n}}", kw));    

            assert_parse_err(&format!("break {}", kw));
            assert_parse_err(&format!("break {}", kw.to_uppercase()));

            assert_parse_err(&format!("{} break", kw));    
            assert_parse_err(&format!("{} break {}", kw, kw));    

            assert_parse_err(&format!("{} break", kw.to_uppercase()));    

            assert_parse_err(&format!("{} break {}", kw.to_uppercase(), kw.to_uppercase()));    
            assert_parse_err(&format!("{} break {}", kw, kw.to_uppercase()));
            assert_parse_err(&format!("{} break {}", kw.to_uppercase(), kw));    
        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("break {}", t));    
            assert_parse_err(&format!("{} break", t));    
            assert_parse_err(&format!("{} break {}", t, t));    


            assert_parse_err(&format!("break {}", t.to_string().to_uppercase()));    
            assert_parse_err(&format!("{} break", t.to_string().to_uppercase()));    

            assert_parse_err(&format!("{} break {}", t.to_string().to_uppercase(), t.to_string().to_uppercase() ));
            assert_parse_err(&format!("{} break {}", t, t.to_string().to_uppercase()));
            assert_parse_err(&format!("{} break {}", t.to_string().to_uppercase(), t));
        }
    }

    #[test]
    fn break_stmt() {
        let ast = parse("break").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        assert!(matches!(ast.globals[0], Stmt::Break(_)));
    }

    #[test]
    fn break_stmt_in_var_decl_errors() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
         
        for l in letters {
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("own {} {} = break", l, t));    
            }
        }
    }


    #[test]
    fn break_stmt_in_infinite_stmt() {
        let ast = parse("infinite {\nbreak\n}").unwrap();
        assert_eq!(ast.functions.len(), 0);
        assert_eq!(ast.globals.len(), 1);

        if let Stmt::Infinite(inf) = &ast.globals[0] {
            assert_eq!(inf.branch.len(), 1);
            assert!(matches!(inf.branch[0], Stmt::Break(_)));
        } else {panic!("Expected infinite statement"); }
    }


    #[test]
    fn break_stmt_in_while_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("while {} {{\nbreak\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::While(w) = &ast.globals[0] {
                assert_eq!(w.branch.len(), 1);
                assert!(matches!(w.branch[0], Stmt::Break(_)));
            } else {panic!("Expected infinite statement"); }
        }
    }


    #[test]
    fn break_stmt_in_for_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let ast = parse(&format!("for i in {} {{\nbreak\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::For(f) = &ast.globals[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Break(_)));
                assert!(!matches!(f.value, Expr::RangeCall{ .. }));

            } else { panic!("expected for statement"); }
        }
    }

    #[test]
    fn break_stmt_in_for_stmt_range() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            let ast = parse(&format!("for i in range({}, {}){{\nbreak\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::For(f) = &ast.globals[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 1);
                assert!(matches!(f.branch[0], Stmt::Break(_)));
                assert!(matches!(f.value, Expr::RangeCall{ .. }));
            } else { panic!("expected for statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\nbreak\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} else {{\nbreak\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_with_else_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\nbreak\n}} else {{\nbreak\n}}", l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} elif {} {{\nbreak\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\nbreak\n}} elif {} {{\nbreak\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);
                assert!(i.else_branch.is_none());

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }


    #[test]
    fn break_stmt_in_if_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\n\n}} elif {} {{\nbreak\n}} else {{\nbreak\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }

    #[test]
    fn break_stmt_in_if_with_else_with_elif_stmt() {
        let literals = get_all_literals_edge_cases();

        for l in literals {
            let ast = parse(&format!("if {} {{\nbreak\n}} elif {} {{\nbreak\n}} else {{\nbreak\n}}", l, l)).unwrap();
            assert_eq!(ast.functions.len(), 0);
            assert_eq!(ast.globals.len(), 1);

            if let Stmt::If(i) = &ast.globals[0] {
                assert_eq!(i.if_branch.len(), 1);
                assert_eq!(i.elif_branches.len(), 1);
                assert_eq!(i.elif_branches[0].1.len(), 1);

                assert!(i.else_branch.is_some());
                assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                assert!(matches!(i.if_branch[0], Stmt::Break(_)));
                assert!(matches!(i.elif_branches[0].1[0], Stmt::Break(_)));
                assert!(matches!(i.else_branch.clone().unwrap()[0], Stmt::Break(_)));
            } else {panic!("Expected if statement"); }
        }
    }
}


