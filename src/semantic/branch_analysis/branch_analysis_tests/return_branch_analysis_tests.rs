use super::*;

#[cfg(test)]
mod return_branch_analysis_hazmat_wrapper_tests {
    use super::*;

    #[test]
    #[should_panic]
    fn func_is_empty_panics() {
        let dummy_func = Function { 
            name: "foo".to_string(), params: vec![], return_type: None, body: vec![], span: span()
        };

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func);
    }

    #[test]
    fn func_never_returns() {
        let literals = get_all_literals_with_var_and_var_arr();
        
        for l in literals {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                let dummy_func = returning_func(&"foo", vec![], vec![t.clone()], vec![Stmt::Expr(l.clone())]);

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Expected function `foo` to return, but we found no return statements"));
            }
        }
    }

}

/*
    #[test]
    fn func_returns() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                make_return_stmt(vec![lv.clone()])
            ]));

            let last_stmt = dummy_func.body.last().unwrap();
            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }


    // Must trigger a guard panic that is meant to catch misuse of return_branch_analysis_hazmat_wrapper
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn func_break_without_loop_panics() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
            make_break_stmt()
        ]));

        let last_stmt = dummy_func.body.last().unwrap();
        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }


    // Empty branches should panic because return_branch_analysis_hazmat_wrapper assumes 
    // all function branches contain at least 1 statement, which
    // is what is guaranteed by dead_code_analysis.
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn infinite_statement_empty_branch_panics() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            })
        ]));

        let last_stmt = dummy_func.body.last().unwrap();

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }

    // Same as above test, but this time nested.
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn infinite_statement_nested_branch_empty_panics() {
        // NOTE: This test unlike other tests doesn't build from inside out, because return analysis
        // doesn't care too much about deeply nested infinite loops, it only cares about 2 nested
        // loops max
        // Which I think is something worth changing, but for now, this test will be fine.
        let stmts = vec![
            Stmt::Infinite(InfiniteStmt {
                branch: vec![
                    Stmt::Infinite(InfiniteStmt {
                        branch: vec![],
                        span: span()
                    })
                ],
                span: span(),
            })
        ];
    
        let dummy_func = make_dummy_func("x".to_string(), Some(stmts));

        let last_stmt = dummy_func.body.last().unwrap();

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }



    // If you try to break in an infinite loop that is last statement, it must error.
    //
    #[test]
    fn infinite_statement_break_errors() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![
                    make_break_stmt()
                ],
                span: span(),
            })
        ]));

        let last_stmt = dummy_func.body.last().unwrap();

        let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("You cannot `break` out of a infinite loop if its the last statement in a function that returns"));
    }

    // Same as above, but this is an if statement, inside infinite statement..
    #[test]
    fn if_statement_main_branch_inside_infinite_stmt_break_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![make_break_stmt()],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot `break` out of a infinite loop if its the last statement in a function that returns"));
        }
    }

    #[test]
    fn if_statement_else_branch_inside_infinite_stmt_break_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![Stmt::Expr(lv.clone())],
                            elif_branches: vec![],
                            else_branch: Some(vec![make_break_stmt()]),
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot `break` out of a infinite loop if its the last statement in a function that returns"));
        }
    }

    #[test]
    fn if_statement_elif_branch_inside_infinite_stmt_break_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![Stmt::Expr(lv.clone())],
                            elif_branches: vec![
                                (lv.clone(), vec![make_break_stmt()]),
                            ],
                            else_branch: None,
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot `break` out of a infinite loop if its the last statement in a function that returns"));
        }
    }

    #[test]
    fn if_statement_main_branch_inside_infinite_stmt() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![Stmt::Expr(lv.clone())],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_else_branch_inside_infinite_stmt() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![Stmt::Expr(lv.clone())],
                            elif_branches: vec![],
                            else_branch: Some(vec![ Stmt::Expr(lv.clone()) ]),
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn if_statement_elif_branch_inside_infinite_stmt() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: lv.clone(),
                            if_branch: vec![Stmt::Expr(lv.clone())],
                            elif_branches: vec![
                                (lv.clone(), vec![ Stmt::Expr(lv.clone()) ]),
                            ],
                            else_branch: None,
                            span: span(),
                        }),

                    ],
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
            assert!(result.is_ok());
        }
    }


    #[test]
    fn infinite_statement_inside_if_stmt_break_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::If(IfStmt{
                    condition: lv.clone(),
                    if_branch: vec![
                        Stmt::Infinite(InfiniteStmt{
                            branch: vec![
                                make_break_stmt()
                            ],
                            span: span(),
                        })
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot `break` out of a infinite loop if its the last statement in a function that returns"));
        }
    }


    // Nested infinite loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_nested_branch_break() {
        for i in 1..=500 {
            // Build from the inside out
            let mut stmts: Vec<Stmt> = vec![make_break_stmt()];
            for _ in 0..=i {
                stmts = vec![
                    Stmt::Infinite(InfiniteStmt {
                        branch: stmts,
                        span: span(),
                    })
                ];
            }
        
            let dummy_func = make_dummy_func("x".to_string(), Some(stmts));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }


    
    // Nested while loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_while_statement_nested_branch_break() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            for i in 1..=500 {
                // Build from the inside out
                let mut stmts: Vec<Stmt> = vec![make_break_stmt()];
                for _ in 0..=i {
                    stmts = vec![
                        Stmt::While(WhileStmt {
                            condition: lv.clone(),
                            branch: stmts,
                            span: span(),
                        })
                    ];
                }

                stmts = vec![
                        Stmt::Infinite(InfiniteStmt {
                            branch: stmts,
                            span: span(),
                        })
                    ];

            
                let dummy_func = make_dummy_func("x".to_string(), Some(stmts));

                let last_stmt = dummy_func.body.last().unwrap();

                let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

                assert!(result.is_ok());
            }
        }
    }


    // Nested for loops inside infinite loops breaks shouldn't be counted as breaks upstream
    #[test]
    fn infinite_statement_for_statement_nested_branch_break() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            for i in 1..=500 {
                // Build from the inside out
                let mut stmts: Vec<Stmt> = vec![make_break_stmt()];
                for _ in 0..=i {
                    stmts = vec![
                        Stmt::For(ForStmt{
                            holder_name: "y".to_string(),
                            value: lv.clone(),
                            branch: stmts,
                            span: span(),
                        })
                    ];
                }

                stmts = vec![
                    Stmt::Infinite(InfiniteStmt {
                        branch: stmts,
                        span: span(),
                    })
                ];

            
                let dummy_func = make_dummy_func("x".to_string(), Some(stmts));
                let last_stmt = dummy_func.body.last().unwrap();
                let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

                assert!(result.is_ok());
            }
        }
    }


    // While loops may or may not execute, therefore even if they return inside their body, the
    // function its self may not return, therefore return analysis should error here
    // 
    #[test]
    fn while_statement_returns_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::While(WhileStmt {
                        condition: lv.clone(),
                        branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with(
                    "Semantic error: While loops may or may not execute at all, therefore you need a return statement outside the loop scope, or consider using `infinite` loops instead."));
        }
    }



    // For loops may or may not execute, therefore even if they return inside their body, the
    // function its self may not return, therefore return analysis should error here
    // 
    #[test]
    fn for_statement_returns_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::For(ForStmt {
                        holder_name: "y".to_string(),
                        value: lv.clone(),
                        branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with(
                    "Semantic error: For loops may or may not execute at all, therefore you need a return statement outside the loop scope."));
        }
    }



    // If statement without an else branch, may or may not execute, therefore even if they return inside the main branch, the
    // function its self may not always return, therefore return analysis should error here
    // 
    #[test]
    fn if_statement_returns_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with(
                    "Semantic error: Function `x` only returns in if statement branches, which might not always execute. Add an `else` branch"));
        }
    }


    // Same as above test, but since main if branch is empty, this should always panic.
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn if_statement_empty_panics() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                Stmt::If(IfStmt{
                    condition: int32_lit(2),
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ]));

        let last_stmt = dummy_func.body.last().unwrap();

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }



    // Same as above, but this time main branch contains return, but else branch is Some, but empty. (this should panic because
    // return_branch_analysis_hazmat_wrapper assumes all function branches contain at least 1 statement, which
    // is what is guaranteed by dead_code_analysis.)
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn if_statement_returns_else_branch_empty_panics() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
            Stmt::If(IfStmt{
                condition: int32_lit(1),
                if_branch: vec![
                    make_return_stmt(vec![int32_lit(2)])
                ],
                elif_branches: vec![],
                else_branch: Some(vec![]),
                span: span(),
            })
        ]));

        let last_stmt = dummy_func.body.last().unwrap();

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }

    // Same as above, but this time main branch empty, and else branch returns. (this should panic because
    // return_branch_analysis_hazmat_wrapper assumes all function branches contain at least 1 statement, which
    // is what is guaranteed by dead_code_analysis.)
    #[should_panic]
    #[test]
    fn empty_if_statement_else_branch_returns_panics() {
        let dummy_func = make_dummy_func("x".to_string(), Some(vec![
            Stmt::If(IfStmt{
                condition: int32_lit(1),
                if_branch: vec![],
                elif_branches: vec![],
                else_branch: Some(vec![
                    make_return_stmt(vec![int32_lit(2)])
                ]),
                span: span(),
            })
        ]));

        let last_stmt = dummy_func.body.last().unwrap();

        let _ = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);
    }


    // If statement with an else branch will always have one branch execute, but main branch does not return, meaning it can't always return,
    // 
    #[test]
    fn if_statement_not_return_with_else_branch_return_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            Stmt::Expr(lv.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            make_return_stmt(vec![lv.clone()])
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("but statement branch body does not end with a return statement"));
        }
    }


    // If statement with an else branch will always have one branch execute, but else branch does not return, meaning it can't always return,
    // 
    #[test]
    fn if_statement_return_with_else_branch_not_returns_errors() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            Stmt::Expr(lv.clone())
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("but statement branch body does not end with a return statement"));
        }
    }

    

    // If statement with an else branch will always have one branch execute, and if both branches return, it can always return,
    // 
    #[test]
    fn if_statement_with_else_branch_both_return() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            make_return_stmt(vec![lv.clone()])
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }


    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    #[test]
    fn if_statement_with_else_branch_return_elif_branch_not_return_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        elif_branches: vec![(lv.clone(), vec![
                            Stmt::Expr(lv.clone())
                        ])],
                        else_branch: Some(vec![
                            make_return_stmt(vec![lv.clone()])
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("but statement branch body does not end with a return statement"));
        }
    }


    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    // 
    #[test]
    fn if_statement_with_elif_branch_return_else_branch_not_return_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![lv.clone()])
                        ],
                        elif_branches: vec![(lv.clone(), vec![
                            make_return_stmt(vec![lv.clone()])
                        ])],
                        else_branch: Some(vec![
                            Stmt::Expr(lv.clone())
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("but statement branch body does not end with a return statement"));
        }
    }



    // If statement with elif branch, and an else branch will always have one branch execute, and if even one branch not returns, 
    // it cant gurantee that it will always return.
    // 
    #[test]
    fn if_statement_not_return_with_elif_branch_and_else_branch_return_error() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: vec![
                            Stmt::Expr(lv.clone())
                        ],
                        elif_branches: vec![(lv.clone(), vec![
                            make_return_stmt(vec![lv.clone()])
                        ])],
                        else_branch: Some(vec![
                            make_return_stmt(vec![lv.clone()])
                        ]),
                        span: span(),
                    })
                ]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("but statement branch body does not end with a return statement"));
        }
    }





    // If statements within infinite loops without an else branch, returning only in one branch (main branch), 
    // even though the statement it may or may not execute, that is fine. because the if statement
    // is not the last statement of function, the infinite loop is.
    // 
    #[test]
    fn infinite_if_statement_returns() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: lv.clone(),
                                if_branch: vec![
                                    make_return_stmt(vec![lv.clone()])
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            })
                        ],
                        span: span()
                    })]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }

    #[test]
    fn infinite_if_statement_returns_with_else_branch_returns() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: lv.clone(),
                                if_branch: vec![
                                    make_return_stmt(vec![lv.clone()])
                                ],
                                elif_branches: vec![],
                                else_branch: Some(vec![
                                    make_return_stmt(vec![lv.clone()])
                                ]),
                                span: span(),
                            })
                        ],
                        span: span()
                    })]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }


    #[test]
    fn infinite_if_statement_returns_with_elif_branch_returns() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: lv.clone(),
                                if_branch: vec![
                                    make_return_stmt(vec![lv.clone()])
                                ],
                                elif_branches: vec![(lv.clone(), vec![

                                    make_return_stmt(vec![lv.clone()])
                                ])],
                                else_branch: None,
                                span: span(),
                            })
                        ],
                        span: span()
                    })]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }


    #[test]
    fn infinite_if_statement_returns_with_elif_branch_returns_else_branch_retirns() {
        let literals_with_var = get_all_literals_with_var_no_arr();
        for lv in literals_with_var {
            let dummy_func = make_dummy_func("x".to_string(), Some(vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: lv.clone(),
                                if_branch: vec![
                                    make_return_stmt(vec![lv.clone()])
                                ],
                                elif_branches: vec![(lv.clone(), vec![
                                    make_return_stmt(vec![lv.clone()])
                                ])],
                                else_branch: Some(vec![
                                    make_return_stmt(vec![lv.clone()])
                                ]),
                                span: span(),
                            })
                        ],
                        span: span()
                    })]));

            let last_stmt = dummy_func.body.last().unwrap();

            let result = return_branch_analysis_hazmat_wrapper(&dummy_func, &last_stmt, false, false);

            assert!(result.is_ok());
        }
    }
}

*/
