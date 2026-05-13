use super::*;

#[cfg(test)]
mod locking_unlocking_tests {
    use super::*;

    // locking / unlocking variables
    //

    #[test]
    fn test_assign_to_locked_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("locked"));
        }
    }

    #[test]
    fn test_assign_locked_variable_same_literal_errors() {
        let literals = get_all_literals_no_arr();
       
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                var_assign("x", l.clone()),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is locked"));
        }
    }

    // Same test as above, but re-declartion use a different literal
    #[test]
    fn test_assign_locked_variable_different_literal_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();


        for ((l1, t), l2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
        {
            let body = vec![
                var_decl("x", t.clone(), l1.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                var_assign("x", l2.clone()),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());

            let assert_cond = result.unwrap_err().to_string();
            let assert_cond = assert_cond.contains("is locked") |
                                assert_cond.contains("Type mismatch assigning to");

            assert!(assert_cond);
        }
    }


    #[test]
    fn test_unlock_non_var_expr_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Unlock(vec![l.clone()]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected variable name, instead got"));
        }
    }



    #[test]
    fn test_unlock_allows_assignment_same_literal() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_assign("x", l.clone()),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    // Same test as above, but re-declartion use a different litral of a different type
    #[test]
    fn test_unlock_allows_assigment_but_different_literal_type_errors() {
        let literals = get_all_literals_no_arr_few_ints();
        let literals_scattered = get_all_literals_no_arr_few_ints_scattered();

        for ((l1, t), l2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
        {
            let body = vec![
                var_decl("x", t.clone(), l1.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_assign("x", l2.clone()),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }
    }


    #[test]
    fn test_unlock_allows_reassign() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_unlock_upstream_variable_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            Stmt::Unlock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_unlock_upstream_variable_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Unlock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_unlock_upstream_variable_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                span: span(),
            };


            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit),

                Stmt::For(ForStmt{
                    holder_name: "i".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        Stmt::Unlock(vec![var_expr("x")]),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
        }
    }







    #[test]
    fn test_lock_unlock_lock_unlock_variable() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_lock_non_var_expr_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![l.clone()]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected variable name, instead got"));
        }
    }



    #[test]
    fn test_lock_repeated_var_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x"), var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Lock arguments have duplicated variable"));
        }
    }


    #[test]
    fn test_double_lock_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Lock(vec![var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("already locked"));
        }
    }

    #[test]
    fn test_unlock_unlocked_variable_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Unlock(vec![var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already unlocked"));
        }
    }

    // overshadowing is not allowed at all in holylang
    #[test]
    fn test_shadowing_variable_lockek_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Lock(vec![var_expr("x")]),
                var_decl("x", t.clone(), l.clone()),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }



    #[test]
    fn test_lock_upstream_variable_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            Stmt::Lock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_lock_upstream_variable_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Lock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_lock_upstream_variable_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                span: span(),
            };


            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit),

                Stmt::For(ForStmt{
                    holder_name: "i".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        Stmt::Lock(vec![var_expr("x")]),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }
 
    #[test]
    fn test_multi_assign_locked_vars_errors() {
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
                var_decl("a", t1.clone(), l1.clone()),
                var_decl("b", t2.clone(), l2.clone()),

                Stmt::Lock(vec![var_expr("a")]),

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
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
        }


        // Same as above, but this locks "b" instead of "a"
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), l1.clone()),
                var_decl("b", t2.clone(), l2.clone()),

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

        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), l1.clone()),
                var_decl("b", t2.clone(), l2.clone()),

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




}
