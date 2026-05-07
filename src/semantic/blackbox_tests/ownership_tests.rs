use super::*;

#[cfg(test)]
mod ownership_tests {
    use super::*;

    // move semantics tests
    //


    #[test]
    fn test_vardecl_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),
                var_decl("z", t.clone(), Some(var_expr("x")))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }



    #[test]
    fn test_vardecl_with_literal_use_after_move_errors() {
        // own a t = 5
        // own b t = a   (moves `a`)
        // own c t = a   (this must error because `a` already moved)

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", t.clone(), Some(var_expr("a"))),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn test_vardecl_without_literal_use_after_move_errors() {
        // own a = 5
        // own b = a   (moves `a`)
        // own c = a   (this must error because `a` already moved)

        
        for t in ALL_TYPES_NO_ARR {
            let body = vec![
                var_decl("a", t.clone(), None),
                var_decl("b", t.clone(), Some(var_expr("a"))),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn test_copy_call_allows_reuse() {
        // own a T = Some Literal
        // own b T = copy(a)  (copies, does not move)
        // own c T = a        (valid, because no moves happened)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_a = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", t.clone(), Some(copy_a)),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_pass_variable_to_call_marks_it_moved() {
        // bar takes one t.
        // own a t = Some Literal
        // bar(a)       (moves a)
        // own b t = a  (error)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let bar = void_func("bar", vec![param("p", t.clone())], vec![]);
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                Stmt::Expr(call_expr("bar", vec![var_expr("a")])),
                var_decl("b", t.clone(), Some(var_expr("a"))),
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![bar, caller], globals: vec![]};
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
            
        }
    }


    // Tests the rule:
    // You cannot move an upstream variable multiple times inside a loop.
    //

    #[test]
    fn test_vardecl_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }


    #[test]
    fn test_vardecl_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_vardecl_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_varassign_to_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),

                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `x`"));
        }
    }

    #[test]
    fn test_varassign_assign_to_self_doesnt_move() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), None),
                var_assign("x", var_expr("x")),
                var_assign("x", l.clone()),

            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_dynamic_array_access_on_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("a")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        // move a to x
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(var_expr("a"))), 
                        var_decl("y", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on moved variable `a`"));
                }
            }       
        }
    }

    #[test]
    fn test_fix_array_access_on_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("a")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("a", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), Some(arr_lit.clone())),
                        // move a to x
                        var_decl("x", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), Some(var_expr("a"))),
                        var_decl("y", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on moved variable `a`"));
                }
            }       
        }
    }





    #[test]
    fn test_varassign_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),
                var_decl("z", t.clone(), None),
                var_assign("z", var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }


    // Tests the rule:
    // You cannot move an upstream variable multiple times inside a loop.
    //

    #[test]
    fn test_varassign_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_varassign_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }


    #[test]
    fn test_varassign_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_multi_assign_use_of_moved_vars_errors() {
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

                var_decl("c", t1.clone(), Some(var_expr("a"))),

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
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `a`"));
        }


        // Same as above, but this time we move "b" instead of "a"
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

                var_decl("c", t2.clone(), Some(var_expr("b"))),

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
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `b`"));
        }


        // Same as above, but this time we move both "a" and "b"
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

                var_decl("c", t1.clone(), Some(var_expr("a"))),
                var_decl("d", t2.clone(), Some(var_expr("b"))),

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
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `b`"));
        }
    }


    #[test]
    fn test_array_valid_multiple_access_both_ends_on_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArraySlicing {
                        array: Box::new(var_expr("arr")),
                        start: Some(Box::new(usize_lit(1))),
                        end: Some(Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        // move arr to x
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(var_expr("arr"))), 
                        var_decl("y", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on moved variable `arr`"));
                }       
            }
        }
    }




}

