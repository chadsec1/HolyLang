use super::*;

#[cfg(test)]
mod multi_return_tests {
    use super::*;

    // return statement with multiple values (aka multi-return)
    // with multi-assignments
    #[test]
    fn test_multi_return_assign_correct() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { 
        //  own a t1
        //  own b t2
        //  a, b = pair() 
        //  }

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            check_semantics(&mut ast).unwrap();

            if let Stmt::VarDecl(v) = &ast.functions[1].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t1.clone());
                assert_ne!(v.value, None);
            } else { panic!("Expected VarDecl") }


            if let Stmt::VarDecl(v) = &ast.functions[1].body[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t2.clone());
                assert_ne!(v.value, None);
            } else { panic!("Expected VarDecl") }

            if let Stmt::VarAssignMulti(ma) = &ast.functions[1].body[2] {
                assert_eq!(ma.names.len(), 2, "Expected 2 variable names");
                assert_eq!(ma.names[0], "a");
                assert_eq!(ma.names[1], "b");

                if let Expr::Call { name, .. } = &ma.value {
                    assert_eq!(name, "pair");
                } else { panic!("Expected Call expression, instead got {:?}", ma.value) }

            } else { panic!("Expected VarAssignMulti") }
        }
    }


    #[test]
    fn test_multi_return_assign_type_mismatch_errors() {
        let literals = get_all_literals_no_arr();


        // a is mismatch, b is correct
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t2.clone(), None),
                var_decl("b", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));

        }

        // now b is mismatched while a is correct

        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![]  };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));

        }



        // Both mismatched
    
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t2.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![]  };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));

        }
    }



    #[test]
    fn test_multi_assign_func_not_return_errors() {
        for (t1, t2) in ALL_TYPES_NO_ARR.iter().zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair = void_func("pair", vec![], vec![]);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Function `pair` has no return type declared but is used in an expression"));

        }
    }


    #[test]
    fn test_multi_assign_undeclared_vars_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `b`"));
        }


        // Same as above, but `a` is undeclared instead of `b`
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("b", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `a`"));
        }


        // Same as above, but both are undeclared
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] , globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `b`"));
        }
    }


    #[test]
    fn test_multi_assign_multi_return_not_func_call_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();


        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::Lock(vec![var_expr("a")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: l1.clone(),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Multi-assignment requires only a single function call on the right-hand side"));
        }
    }




    #[test]
    fn test_multi_assign_return_count_mismatch_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));

        }



        // Same test b ut this time extra variable
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),
                var_decl("c", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }


    // return statement with multiple values (aka multi-return)
    // with multi-declaration
    #[test]
    fn test_multi_return_decl_correct() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { own a, b = pair() }

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            check_semantics(&mut ast).unwrap();

            if let Stmt::VarDeclMulti(vs, ce) = &ast.functions[1].body[0] {
                assert_eq!(vs.len(), 2, "Expected 2 variable declarations");
                assert_eq!(vs[0].type_name, t1.clone());
                assert_eq!(vs[1].type_name, t2.clone());
            
                if let Expr::Call { name, .. } = ce {
                    assert_eq!(name, "pair");
                } else { panic!("Expected Call expression, instead got {:?}", ce) }

            } else { panic!("Expected VarDecl") }
        }
    }


    #[test]
    fn test_multi_return_assign_one_variable_locked_errors() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { 
        // own a t1
        // lock a
        // a, b = pair() }

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),
                Stmt::Lock(vec![var_expr("a")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
        }



        // Same test, but this time `b` is locked instead
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),
                Stmt::Lock(vec![var_expr("b")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }
    }


    // Same test, but both `a` and `b` are locked
    #[test]
    fn test_multi_return_assign_two_variables_locked_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();
        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t1.clone(), None),
                Stmt::Lock(vec![var_expr("a"), var_expr("b")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
            // assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }
    }

    // Cuz u can't overshadow variables declared in upstream scopes
    #[test]
    fn test_multi_return_decl_one_already_declared_variable_errors() {
        // This tests against function arguments considering they are upstream
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();
        
        // `a` is a `main` argument, aka it is already declared
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![
                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![param("a", t1.clone())], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is already declared, overshadowing is not allowed."));
        }


        // Same but this time for `b`
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![
                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![param("b", t2.clone())], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `b` is already declared, overshadowing is not allowed."));
        }
    }

    #[test]
    fn test_multi_return_decl_two_already_declared_variable_errors() {
        // This tests against function arguments considering they are upstream
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();
        
        // `a` is a `main` argument, aka it is already declared
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![
                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![param("a", t1.clone()), param("b", t2.clone())], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is already declared, overshadowing is not allowed."));
            // assert!(result.unwrap_err().to_string().contains("Variable `b` is already declared, overshadowing is not allowed."));
        }
    }

    #[test]
    fn test_multi_return_decl_not_func_call_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![
                Stmt::VarDeclMulti(vars, l1.clone())
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Multi-declarement requires only a single function call on the right-hand side"));
        }
    }


    #[test]
    fn test_multi_return_decl_typemismatch_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));
        }


        // Same test but the mismatch is in "a" instead of "b"
        for ((l1, t1), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t2.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
        }


        // Same test but the mismatch is in both "a" and "b"
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t2.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
        }
    }


    #[test]
    fn test_multidec_return_count_mismatch_errors() {
        // pair returns 2 values, but programmer only binds 1 variable

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main], globals: vec![] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }

}
