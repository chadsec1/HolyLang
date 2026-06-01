use super::*;

#[cfg(test)]
mod continue_stmt_in_void_func_tests {
    use super::*;

    #[test]
    fn continue_stmt() {
        let body = vec![
            Stmt::Continue(ContinueStmt { span: span() })
        ];
        let func = void_func("foo", vec![], body);
        let ast = &ast_one(func);

        let internals = import_internals();
        let rcode = transpile(ast);
        assert!(rcode.starts_with(&internals));
        let rcode = rcode[internals.len()..].replace('\n', "");

        assert_eq!(rcode, "fn foo() { continue;}");
    }


    #[test]
    fn continue_stmt_in_infinite_loop() {
        let body = vec![
            Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        let func = void_func("foo", vec![], body);
        let ast = &ast_one(func);

        let internals = import_internals();
        let rcode = transpile(ast);
        assert!(rcode.starts_with(&internals));
        let rcode = rcode[internals.len()..].replace('\n', "");

        assert_eq!(rcode, "fn foo() { loop {continue;}}");
    }


    #[test]
    fn continue_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ while {} {{continue;}}}}", l_str));
        }
    }

    #[test]
    fn continue_stmt_in_while_loop_all_binops() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::While(WhileStmt{
                            condition: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ while {} {{continue;}}}}", bin_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_var() {
        let body = vec![
            Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("arr"),
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        let func = void_func("foo", vec![], body);
        let ast = &ast_one(func);

        let internals = import_internals();
        let rcode = transpile(ast);
        assert!(rcode.starts_with(&internals));
        let rcode = rcode[internals.len()..].replace('\n', "");

        assert_eq!(rcode, "fn foo() { for x in arr {continue;}}");
    }

    #[test]
    fn continue_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ for x in {} {{continue;}}}}", l_str));
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_binop() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                 
                let body = vec![
                    Stmt::For(ForStmt{
                            holder_name: "x".to_string(),
                            value: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ for x in {} {{continue;}}}}", bin_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}}}}", l_str));
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}}}}", bin_str));
            }
        }
    }


    #[test]
    fn continue_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}} else {{continue;}}}}", l_str));
        }
    }

    #[test]
    fn continue_stmt_in_if_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}} else {{continue;}}}}", bin_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ])],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", l_str, l_str));
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ])],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", bin_str, bin_str));
            }
        }
    }
}


#[cfg(test)]
mod continue_stmt_in_void_func_with_params_tests {
    use super::*;

    #[test]
    fn continue_stmt() {
        let body = vec![
            Stmt::Continue(ContinueStmt { span: span() })
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
            let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ continue;}}", t_str, t_str));
        }
    }


    #[test]
    fn continue_stmt_in_infinite_loop() {
        let body = vec![
            Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
            let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);
            
            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ loop {{continue;}}}}", t_str, t_str));
        }
    }


    #[test]
    fn continue_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ while {} {{continue;}}}}", t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_while_loop_all_binops() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::While(WhileStmt{
                            condition: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];
                
                let bin_str = holy_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ while {} {{continue;}}}}", t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_var() {
        let body = vec![
            Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("arr"),
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
            let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in arr {{continue;}}}}", t_str, t_str));
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in {} {{continue;}}}}", t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_binop() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::For(ForStmt{
                            holder_name: "x".to_string(),
                            value: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];

                let bin_str = holy_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in {} {{continue;}}}}", t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}}}}", t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];
                
                let bin_str = holy_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}}}}", t_str, t_str, bin_str));
                }
            }
        }
    }


    #[test]
    fn continue_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}} else {{continue;}}}}", t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];
                let bin_str = holy_expr_to_rust_expr(&bin);


                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}} else {{continue;}}}}", t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ])],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ])],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];
                    
                let bin_str = holy_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                
                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }
}


#[cfg(test)]
mod continue_stmt_in_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn continue_stmt() {
        let body = vec![
            Stmt::Continue(ContinueStmt { span: span() })
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ continue;}}", t_str, t_str, t_str));
        }
    }


    #[test]
    fn continue_stmt_in_infinite_loop() {
        let body = vec![
            Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);
            
            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ loop {{continue;}}}}", t_str, t_str, t_str));
        }
    }


    #[test]
    fn continue_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ while {} {{continue;}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_while_loop_all_binops() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::While(WhileStmt{
                            condition: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];
                
                let bin_str = holy_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ while {} {{continue;}}}}", t_str, t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_var() {
        let body = vec![
            Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("arr"),
                    branch: vec![
                        Stmt::Continue(ContinueStmt { span: span() })
                    ],
                    span: span(),
                }),
        ];

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in arr {{continue;}}}}", t_str, t_str, t_str));
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in {} {{continue;}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_for_loop_with_binop() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::For(ForStmt{
                            holder_name: "x".to_string(),
                            value: bin.clone(),
                            branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            span: span(),
                        }),
                ];

                let bin_str = holy_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in {} {{continue;}}}}", t_str, t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];
                
                let bin_str = holy_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}}}}", t_str, t_str, t_str, bin_str));
                }
            }
        }
    }


    #[test]
    fn continue_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}} else {{continue;}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];
                let bin_str = holy_expr_to_rust_expr(&bin);


                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}} else {{continue;}}}}", t_str, t_str, t_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ])],
                        else_branch: Some(vec![
                            Stmt::Continue(ContinueStmt { span: span() })
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = holy_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = holy_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", t_str, t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn continue_stmt_in_if_elif_else_stmt_binop_condition() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };


                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bin.clone(),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ])],
                            else_branch: Some(vec![
                                Stmt::Continue(ContinueStmt { span: span() })
                            ]),
                            span: span(),
                        }),
                ];
                    
                let bin_str = holy_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                
                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{continue;}} else if {} {{continue;}} else {{continue;}}}}", t_str, t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }
}


