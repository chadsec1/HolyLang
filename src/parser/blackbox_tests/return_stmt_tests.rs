use super::*;

#[cfg(test)]
mod return_stmt_tests {
    use super::*; 

    #[test]
    fn return_single_value() {
        let stmts = parse_body("return 42");
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 1);
        } else {
            panic!("Expected Return");
        }
    }

    #[test]
    fn return_multiple_values() {
        let stmts = parse_body("return 1, 2, 300, 69640");

        assert_eq!(stmts.len(), 1);
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 4);

            if let Expr::IntLiteral { value, .. } = &exprs[0] {
                assert!(matches!(value, IntLiteralValue::Int8(1)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[1] {
                assert!(matches!(value, IntLiteralValue::Int8(2)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[2] {
                assert!(matches!(value, IntLiteralValue::Int16(300)));
            } else { panic!("Expcted IntLiteral"); }


            if let Expr::IntLiteral { value, .. } = &exprs[3] {
                assert!(matches!(value, IntLiteralValue::Int32(69640)));
            } else { panic!("Expcted IntLiteral"); }


        } else {
            panic!("Expected Return");
        }
    }

    #[test]
    fn return_without_value_errors() {
        assert_parse_err(&wrap("return"));
    }

    #[test]
    fn return_variable() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = {}\nreturn x", t, l));

                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }    

                if let Stmt::Return(exprs) = &stmts[1] {
                    assert_eq!(exprs.len(), 1);
                    assert!(matches!(exprs[0], Expr::Var { .. }));
                } else {
                    panic!("Expected Return");
                }
            }
        }
    }



}
