use super::*;

#[cfg(test)]
mod if_stmt_tests {
    use super::*; 
 
    // If statements 

    #[test]
    fn if_statements_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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


        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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



        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
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
