use super::*;

#[cfg(test)]
mod while_stmt_tests {
    use super::*;
    
    #[test]
    fn while_statements_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 1 {} 2 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }

    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 1 {} 2 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 1 {} 2 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    

    #[test]
    fn while_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }

    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 69 {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(69)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} 67 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(67)));
                    } else { panic!(); }
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 69 {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} 67 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 69 {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} 67 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}", spaces)));
            spaces.push(' ');
        }
    }

}
