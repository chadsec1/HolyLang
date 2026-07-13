use super::*;

#[cfg(test)]
mod expr_stmt_in_void_func_tests {
    use super::*;

    #[test]
    fn expr_stmt() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::Expr(l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");
            
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ {};}}", l_str));
        }
    }


    #[test]
    fn expr_stmt_in_infinite_loop() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Expr(l.clone())
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
            
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ loop {{{};}}}}", l_str));
        }
    }

    #[test]
    fn expr_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
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

            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ while {} {{{};}}}}", l_str, l_str));
        }
    }


    #[test]
    fn expr_stmt_in_while_loop_all_binops() {
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
                                Stmt::Expr(bin.clone())
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

                let bin_str = gold_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ while {} {{{};}}}}", bin_str, bin_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_var() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("arr"),
                        branch: vec![
                            Stmt::Expr(l.clone())
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
            
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ for x in arr {{{};}}}}", l_str));
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
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

            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ for x in {} {{{};}}}}", l_str, l_str));
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_binop() {
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
                                Stmt::Expr(bin.clone())
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

                let bin_str = gold_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ for x in {} {{{};}}}}", bin_str, bin_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
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

            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}}}}", l_str, l_str));
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
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

                let bin_str = gold_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}}}}", bin_str, bin_str));
            }
        }
    }


    #[test]
    fn expr_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
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

            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}} else {{{};}}}}", l_str, l_str, l_str));
        }
    }

    #[test]
    fn expr_stmt_in_if_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
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

                let bin_str = gold_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}} else {{{};}}}}", bin_str, bin_str, bin_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Expr(l.clone())
                        ])],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
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

            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", l_str, l_str, l_str, l_str, l_str));
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Expr(bin.clone())
                            ])],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
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

                let bin_str = gold_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", bin_str, bin_str, bin_str, bin_str, bin_str));
            }
        }
    }
}


#[cfg(test)]
mod expr_stmt_in_void_func_with_params_tests {
    use super::*;

    #[test]
    fn expr_stmt() {

        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::Expr(l.clone())
            ];
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ {};}}", t_str, t_str, l_str));
            }
        }
    }


    #[test]
    fn expr_stmt_in_infinite_loop() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);
                
                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ loop {{{};}}}}", t_str, t_str, l_str));
            }
        }
    }


    #[test]
    fn expr_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ while {} {{{};}}}}", t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_while_loop_all_binops() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            span: span(),
                        }),
                ];
                
                let bin_str = gold_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ while {} {{{};}}}}", t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_var() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("arr"),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in arr {{{};}}}}", t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in {} {{{};}}}}", t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_binop() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            span: span(),
                        }),
                ];

                let bin_str = gold_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ for x in {} {{{};}}}}", t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}}}}", t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];
                
                let bin_str = gold_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}}}}", t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }


    #[test]
    fn expr_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}} else {{{};}}}}", t_str, t_str, l_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
                            ]),
                            span: span(),
                        }),
                ];

                let bin_str = gold_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}} else {{{};}}}}", t_str, t_str, bin_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Expr(l.clone())
                        ])],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", t_str, t_str, l_str, l_str, l_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Expr(bin.clone())
                            ])],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
                            ]),
                            span: span(),
                        }),
                ];
                    
                let bin_str = gold_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                
                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", t_str, t_str, bin_str, bin_str, bin_str, bin_str, bin_str));
                }
            }
        }
    }
}


#[cfg(test)]
mod expr_stmt_in_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn expr_stmt() {
        let literals = get_all_literals();

        for l in literals {     
            let body = vec![
                Stmt::Expr(l.clone())
            ];

            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ {};}}", t_str, t_str, t_str, l_str));
            }
        }
    }


    #[test]
    fn expr_stmt_in_infinite_loop() {
        let literals = get_all_literals();

        for l in literals {     
            let body = vec![
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];

            let l_str = gold_expr_to_rust_expr(&l);
            
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);
                
                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ loop {{{};}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }


    #[test]
    fn expr_stmt_in_while_loop_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::While(WhileStmt{
                        condition: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {    
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ while {} {{{};}}}}", t_str, t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_while_loop_all_binops() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            span: span(),
                        }),
                ];
                
                let bin_str = gold_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ while {} {{{};}}}}", t_str, t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_var() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("arr"),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];

            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in arr {{{};}}}}", t_str, t_str, t_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_literal() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: l.clone(),
                        branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in {} {{{};}}}}", t_str, t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_for_loop_with_binop() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            span: span(),
                        }),
                ];

                let bin_str = gold_expr_to_rust_expr(&bin);
                
                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ for x in {} {{{};}}}}", t_str, t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}}}}", t_str, t_str, t_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];
                
                let bin_str = gold_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}}}}", t_str, t_str, t_str, bin_str, bin_str));
                }
            }
        }
    }


    #[test]
    fn expr_stmt_in_if_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}} else {{{};}}}}", t_str, t_str, t_str, l_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
                            ]),
                            span: span(),
                        }),
                ];
                let bin_str = gold_expr_to_rust_expr(&bin);


                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}} else {{{};}}}}", t_str, t_str, t_str, bin_str, bin_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_literal_condition() {
        let literals = get_all_literals();

        for l in literals {
            let body = vec![
                Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            Stmt::Expr(l.clone())
                        ],
                        elif_branches: vec![(l.clone(), vec![
                            Stmt::Expr(l.clone())
                        ])],
                        else_branch: Some(vec![
                            Stmt::Expr(l.clone())
                        ]),
                        span: span(),
                    }),
            ];
            
            let l_str = gold_expr_to_rust_expr(&l);

            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t_str = gold_type_to_rust_type_str(&t);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", t_str, t_str, t_str, l_str, l_str, l_str, l_str, l_str));
            }
        }
    }

    #[test]
    fn expr_stmt_in_if_elif_else_stmt_binop_condition() {
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
                                Stmt::Expr(bin.clone())
                            ],
                            elif_branches: vec![(bin.clone(), vec![
                                Stmt::Expr(bin.clone())
                            ])],
                            else_branch: Some(vec![
                                Stmt::Expr(bin.clone())
                            ]),
                            span: span(),
                        }),
                ];
                    
                let bin_str = gold_expr_to_rust_expr(&bin);

                for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                
                    let t_str = gold_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ if {} {{{};}} else if {} {{{};}} else {{{};}}}}", t_str, t_str, t_str, bin_str, bin_str, bin_str, bin_str, bin_str));
                }
            }
        }
    }
}

