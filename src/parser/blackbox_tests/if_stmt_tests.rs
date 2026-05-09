use super::*;

#[cfg(test)]
mod if_stmt_tests {
    use super::*; 
 
    #[test]
    fn if_statements_invalid_construction_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("if range({}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("if range(, {}) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("if range({}, ) {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("if range({}, {}) {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("if range({}, {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("if range{}, {} {{\n\n}}", l, l)));    


            assert_parse_err(&wrap(&format!("if {} in {} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("if in {} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("if {} in {{\n\n}}", l)));

            assert_parse_err(&wrap(&format!("if{} {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{} if {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}if {{\n\n}}", l)));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", l, l)));    
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", l, l)));    
        }

        assert_parse_err(&wrap("if {\n\n"));    
        assert_parse_err(&wrap("if {}"));    
        assert_parse_err(&wrap("if \n\n}"));    
        assert_parse_err(&wrap("if {{\n\n}}"));    

        
        for kw in consts::RESERVED_KEYWORDS { 
            if kw == &"true" || kw == &"false" {
                continue 
            }
            assert_parse_err(&wrap(&format!("if {} {{\n\n}}", kw)));
            assert_parse_err(&wrap(&format!("if {} {{\n\n}}", kw.to_uppercase())));
        }

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("if{} {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{} if {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}if {{\n\n}}", kw)));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", kw, kw)));    
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", kw, kw)));    

            assert_parse_err(&wrap(&format!("if{} {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} if {{\n\n}}", kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}if {{\n\n}}", kw.to_uppercase())));    

            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", kw.to_uppercase(), kw)));    
            
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", kw.to_uppercase(), kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", kw, kw.to_uppercase())));    
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", kw.to_uppercase(), kw)));    

        }

        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("if {} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("if{} {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{} if {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}if {{\n\n}}", t)));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", t, t)));    
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", t, t)));    


            assert_parse_err(&wrap(&format!("if {} {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("if{} {{\n\n}}", t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} if {{\n\n}}", t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}if {{\n\n}}", t.to_string().to_uppercase() )));

            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase())));    
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{}if{} {{\n\n}}", t, t.to_string().to_uppercase()))); 

            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", t.to_string().to_uppercase(), t.to_string().to_uppercase() )));
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", t, t.to_string().to_uppercase())));
            assert_parse_err(&wrap(&format!("{} if {} {{\n\n}}", t.to_string().to_uppercase(), t)));

        }
    }

    #[test]
    fn if_statements_trailing_exprs_errors() {
        let literals = get_all_literals_edge_cases(); 

        for l in &literals {
            assert_parse_err(&wrap(&format!("if {} {{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {} {{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {} {{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {} {{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n", l, l)));

            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n}}", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n}}", l, l)));

            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n", l, l)));
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n", l, l)));
        }
    }

    #[test]
    fn if_statements_trailing_kw_errors() {
        let literals = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            if *kw == "else" || *kw == "elif" {
                continue
            }

            for l in &literals {
                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n", l, kw)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n}}", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n}}", l, kw)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n", l, kw)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n", l, kw)));




                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {} {} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{} {}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{} {{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{}{{\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{}{{\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{} {{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{}{{\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{}{{\n\n", l, kw, l)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{} \n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{}\n\n}}", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{}\n\n}}", l, kw, l)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{} \n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{}\n\n", l, kw, l)));
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{}\n\n", l, kw, l)));
            }
        }
    }

    #[test]
    fn if_statements_trailing_types_errors() {
        let literals = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {} {{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {} {{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n}}", l, t)));    

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} {{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}{{\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}{{\n\n", l, t)));    

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n}}", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n}}", l, t)));

                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{} \n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}} {}\n\n", l, t)));    
                assert_parse_err(&wrap(&format!("if {}{{\n\n}}{}\n\n", l, t)));
            }
        }
    }




    #[test]
    fn if_statements() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\n\n}}", l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }

    #[test]
    fn if_statements_else_branch() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\n\n}} else {{\n\n}}", l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_some());
                    assert_eq!(i.else_branch.clone().unwrap().len(), 0);
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }


    #[test]
    fn if_statements_elif_branch() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\n\n}} elif {} {} {} {{\n\n}}", l, s, l, l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 1);
                    assert!(i.else_branch.is_none());
                    
                    assert_eq!(i.elif_branches[0].1.len(), 0);
                    
                    if let Expr::BinOp { left: elif_left, right: elif_right, op: elif_op, .. } = &i.elif_branches[0].0 {
                        assert_eq!(elif_op, b);
                        assert_eq!(elif_left, elif_right);
                    } else { panic!() }
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }


    #[test]
    fn if_statements_elif_else_branches() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\n\n}} elif {} {} {} {{\n\n}} else {{\n\n}}", l, s, l, l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 0);
                    assert_eq!(i.elif_branches.len(), 1);
                    assert!(i.else_branch.is_some());
                    assert_eq!(i.else_branch.clone().unwrap().len(), 0);
                    
                    assert_eq!(i.elif_branches[0].1.len(), 0);
                    
                    if let Expr::BinOp { left: elif_left, right: elif_right, op: elif_op, .. } = &i.elif_branches[0].0 {
                        assert_eq!(elif_op, b);
                        assert_eq!(elif_left, elif_right);
                    } else { panic!() }
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }


    #[test]
    fn if_statements_nested() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\nif {} {} {} {{\n\n}}\n}}", l, s, l, l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 1);
                    if let Stmt::If(i) = &stmts[0] {
                        if let Expr::BinOp { left, right, op, .. } = &i.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!() }
                    } else { panic!("expected if statement"); }

                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }

    #[test]
    fn if_statements_with_else_branch_nested() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\nif {} {} {} {{\n\n}}\n}} else {{\n\n}}", l, s, l, l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 1);
                    if let Stmt::If(i) = &stmts[0] {
                        if let Expr::BinOp { left, right, op, .. } = &i.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!() }
                    } else { panic!("expected if statement"); }

                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_some());
                    assert_eq!(i.else_branch.clone().unwrap().len(), 0);
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }


    #[test]
    fn if_statements_with_elif_branch_nested() {
        let literals = get_all_literals_edge_cases();

        for l in &literals { 
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("if {} {} {} {{\nif {} {} {} {{\n\n}}\n}} elif {} {} {} {{\n\n}}", l, s, l, l, s, l, l, s, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::If(i) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &i.condition {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!() }
                    
                    assert_eq!(i.if_branch.len(), 1);
                    if let Stmt::If(i) = &stmts[0] {
                        if let Expr::BinOp { left, right, op, .. } = &i.condition {
                            assert_eq!(op, b);
                            assert_eq!(left, right);
                        } else { panic!() }
                    } else { panic!("expected if statement"); }

                    assert_eq!(i.elif_branches.len(), 1);
                    assert!(i.else_branch.is_none());

                    let elif_cond = &i.elif_branches[0].0;
                    if let Expr::BinOp { left, right, op, .. } = elif_cond {
                        assert_eq!(op, b);
                        assert_eq!(left, right);
                    } else { panic!("Expected BinOp") }
                } else {
                    panic!("expected if statement");
                }
            }
        }
    }






    #[test]
    fn if_statements_int_literal() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }
 
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} 10 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(10)));
                    } else { panic!(); }

                } else { panic!() }
                
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 10 {} x {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(10)));
                    } else { panic!(); }
 
                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }


    }



    #[test]
    fn if_statements_with_else() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} else {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
            } else {
                panic!("expected if statement");
            }
        }
    }

    #[test]
    fn if_statements_with_elif_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} elif 5 {} 3 {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(3)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_with_elif_vars() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} y {{\n\n}} elif e {} a {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);


                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "e"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_with_elif_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 2 {} y {{\n\n}} elif 5 {} a {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }



        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} 6 {{\n\n}} elif a {} 9 {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(6)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(9)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }

    }





    #[test]
    fn if_statements_with_else_elif() {
        for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} elif 5 {} 3 {{\n\n}} else {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(3)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }



                assert!(i.else_branch.is_some());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}", spaces)));    
            spaces.push(' ');
        }
    }

    #[test]
    fn if_statements_elif_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("if 1 == 2 {{\n\n}} elif {}{{\n\n}}", spaces)));    
            spaces.push(' ');
        }

    }
}
