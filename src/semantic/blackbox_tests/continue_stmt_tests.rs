use super::*;

#[cfg(test)]
mod continue_stmt_tests {
    use super::*;
  
    #[test]
    fn test_continue_statement_no_loop_errors() {
        let body = vec![ 
            Stmt::Continue(ContinueStmt{
                span: span()
            })
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
    }

    #[test]
    fn test_continue_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in literals_ints_floats {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt{
                                span: span()
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());

                if let Stmt::While(ws) = &ast.functions[0].body[0] {
                    assert_eq!(ws.condition, condition);
                    assert_eq!(ws.branch.len(), 1);

                    assert!( matches!(ws.branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected While loop statement") }

            }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition.clone(),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition.clone(),
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::While(ws) = &ast.functions[0].body[0] {
                    assert_eq!(ws.condition, condition.clone());
                    assert_eq!(ws.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &ws.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected While loop statement") }
            }
        }
    }

    #[test]
    fn test_continue_statement_outside_while_statements_errors() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),

                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
            }
        }
        // Same test, but the `continue` is after the while loop

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),

                        ],
                        span: span(),
                    }),
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),


                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
            }
        }
    }

    #[test]
    fn test_continue_statement_in_infinite_statements() {
        let body = vec![ 
            Stmt::Infinite(InfiniteStmt{
                branch: vec![
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),
                ],
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_ok());
        if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
            assert_eq!(infs.branch.len(), 1);
        
            assert!( matches!(infs.branch[0], Stmt::Continue(_)), "Expected continue statement");

        } else { panic!("Expected Infinite loop statement") }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_infinite_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition,
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
                    assert_eq!(infs.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &infs.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected Infinite loop statement") }
            }
        }
    }



    #[test]
    fn test_continue_statement_outside_infinite_statements_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), l.clone()),

                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), l.clone()),

                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }
    }



    #[test]
    fn test_continue_statement_in_for_statement_with_arr() {
        for t in ALL_TYPES_NO_ARR {
            let arr_lit = array_lit(vec![], Some(Type::Array(Box::new(t.clone()))));

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::Continue(ContinueStmt{
                                span: span()
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 2);
            assert_eq!(ast.globals.len(), 0);


            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, arr_lit);

            } else { panic!("Expected VarDecl statement") }


            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Continue(_)), "Expected continue statement");

            } else { panic!("Expected For loop statement") }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_for_statements_with_arr() {
        for t in ALL_TYPES_NO_ARR {
            let arr_lit = array_lit(vec![], Some(Type::Array(Box::new(t.clone()))));

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: bool_lit(false),
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, arr_lit);

            } else { panic!("Expected VarDecl statement") }

            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }






    #[test]
    fn test_continue_statement_outside_for_statements_with_arr_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }
    }


    // Same as above for statement tests, but this time with RangeCall
    //


    
    #[test]
    fn test_continue_statement_in_for_statement_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::Continue(ContinueStmt{
                            span: span()
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok());

            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }

                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Continue(_)), "Expected continue statement");
            } else { panic!("Expected For statement") }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_for_statements_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: bool_lit(false),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt{
                                    span: span()
                                }),
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());


            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }

    #[test]
    fn test_continue_statement_outside_for_statements_with_range_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop

        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![  
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }
    }
}
