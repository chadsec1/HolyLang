use super::*;

#[cfg(test)]
mod function_call_tests {
    use super::*;

    #[test]
    fn call_no_args() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = noop()", t));

            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::Call { name, args, .. }) = &v.value {
                    assert_eq!(name, "noop");
                    assert!(args.is_empty());
                } else { panic!("Expected Call"); }
            }       
        }
    }

    #[test]
    fn call_with_args_literals_only() {
        let literals_edge_cases = get_all_literals_edge_cases();
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = add({}, \"Hi!1\\\"\")", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());

                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "add");
                        assert_eq!(args.len(), 2);
                        assert!(matches!(args[1], Expr::StringLiteral { .. }));
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_with_args_vars_only() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = add(a, b)", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                   
                if let Some(Expr::Call { name, args, .. }) = &v.value {
                    assert_eq!(name, "add");
                    assert_eq!(args.len(), 2);
                    assert!(matches!(args[0], Expr::Var { .. }));
                    assert!(matches!(args[1], Expr::Var { .. }));
                } else { panic!("Expected Call"); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn call_with_args_mixed() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = add(a, {})", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "add");
                        assert_eq!(args.len(), 2);
                        assert!(matches!(args[0], Expr::Var { .. }));
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_nested_args() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = outer(inner({}, {}), {})", t, l, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "outer");
                        assert_eq!(args.len(), 2);
                    
                        if let Expr::Call { name, args: args2, .. } = &args[0] {
                            assert_eq!(name, "inner");
                            assert_eq!(args2.len(), 2);
                        } else { panic!("Expected Call"); }
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_as_statement() {
        let stmts = parse_body("do_thing()");
        assert_eq!(stmts.len(), 1);
        
        if let Stmt::Expr(e) = &stmts[0] {
            if let Expr::Call { name, args, .. } = e {
                assert_eq!(name, "do_thing");
                assert_eq!(args.len(), 0);

            } else { panic!("Expected Call"); }
        
        } else { panic!("Expected Expression"); }
    }



}
