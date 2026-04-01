use super::*;

use crate::parser::{
    ForStmt, IfStmt, WhileStmt, InfiniteStmt, BreakStmt
};

use crate::semantic::branch_analysis::{
    block_always_terminates,
    dead_code_analysis
};

// Test Helpers

fn span() -> Span {
    Span { line: 1, column: 1 }
}

fn int8_lit(n: i8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int8(n), span: span() }
}

fn int16_lit(n: i16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int16(n), span: span() }
}

fn int32_lit(n: i32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int32(n), span: span() }
}

fn int64_lit(n: i64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int64(n), span: span() }
}

fn int128_lit(n: i128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int128(n), span: span() }
}



fn byte_lit(b: u8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Byte(b), span: span() }
}

fn uint16_lit(n: u16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint16(n), span: span() }
}

fn uint32_lit(n: u32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint32(n), span: span() }
}

fn uint64_lit(n: u64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint64(n), span: span() }
}

fn uint128_lit(n: u128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint128(n), span: span() }
}


fn usize_lit(n: usize) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Usize(n), span: span() }
}


fn float32_lit(f: f32) -> Expr {
    Expr::FloatLiteral { value: FloatLiteralValue::Float32(f), span: span() }
}


fn float64_lit(f: f64) -> Expr {
    Expr::FloatLiteral { value: FloatLiteralValue::Float64(f), span: span() }
}


fn bool_lit(b: bool) -> Expr {
    Expr::BoolLiteral { value: b, span: span() }
}

fn str_lit(s: &str) -> Expr {
    Expr::StringLiteral { value: s.to_string(), span: span() }
}

fn make_return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}

fn make_break_stmt() -> Stmt {
    Stmt::Break(BreakStmt { span: span() })
}

fn var_expr(name: &str) -> Expr {
    Expr::Var { name: name.to_string(), span: span() }
}

fn get_all_literals_no_arr() -> [Expr; 15] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}


fn get_all_literals_with_var_no_arr() -> [Expr; 16] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi"),
        var_expr("a")
    ];

    return literals;
}


#[cfg(test)]
mod test_block_always_terminates {
    use super::*;
    

    #[test]
    fn empty_infinite_statement_branch() {
        let stmts: Vec<Stmt> = vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            })
        ];

        let result: bool = block_always_terminates(&stmts, false);
        // Branch does not terminate
        assert_eq!(result, false);
    }


    #[test]
    fn infinite_statement_branch_not_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }



    #[test]
    fn infinite_statement_branch_return_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch terminates
                assert_eq!(result, true);
            }
        }
    }


    // TODO: Make it 100 nested infinite statements.

    #[test]
    fn empty_infinite_statement_nested_branch_not_terminates() {
        let stmts: Vec<Stmt> = vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![],
                        span: span(),
                    })
                ],
                span: span(),
            })
        ];

        let result: bool = block_always_terminates(&stmts, false);
        // Branch does not terminate
        assert_eq!(result, false);
    }

    #[test]
    fn infinite_statement_nested_branch_not_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Infinite(InfiniteStmt{
                                branch: dummy_branch,
                                span: span(),
                            })
                        ],
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }

    #[test]
    fn infinite_statement_nested_branch_return_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Infinite(InfiniteStmt{
                                branch: dummy_branch,
                                span: span(),
                            })
                        ],
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does terminate
                assert_eq!(result, true);
            }
        }
    }   


    #[test]
    fn infinite_statement_nested_branch_break_not_terminates() {
        for i in 0..=1000 {
            let dummy_branch = vec![make_break_stmt(); i + 1];

            let stmts: Vec<Stmt> = vec![
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        Stmt::Infinite(InfiniteStmt{
                            branch: dummy_branch,
                            span: span(),
                        })
                    ],
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }    





    #[test]
    fn empty_while_statement_branch() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::While(WhileStmt{
                    condition: l,
                    branch: vec![],
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }


    // Even if the while loop branch is not empty
    // it should never terminate, because the while statement may or may not execute at all.
    #[test]
    fn while_statement_branch_not_empty_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::While(WhileStmt{
                        condition: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // Even if the while loop branch returns
    // it should never terminate, because the while statement may or may not execute at all.
    #[test]
    fn while_statement_branch_returns_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);

            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::While(WhileStmt{
                        condition: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // Even if the while loop branch breaks
    // it should never terminate, because the while statement may or may not execute at all.
    #[test]
    fn while_statement_branch_break_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            for i in 0..=1000 {
                let dummy_branch = vec![make_break_stmt(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::While(WhileStmt{
                        condition: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    #[test]
    fn empty_for_statement_branch_never_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: l,
                    branch: vec![],
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }


    // Even if the for loop branch is not empty
    // it should never terminate, because the for statement may or may not execute at all.
    #[test]
    fn for_statement_branch_not_empty_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // Even if the for loop branch returns
    // it should never terminate, because the for statement may or may not execute at all.
    #[test]
    fn for_statement_branch_returns_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);

            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // Even if the for loop branch breaks
    // it should never terminate, because the for statement may or may not execute at all.
    #[test]
    fn for_statement_branch_break_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            for i in 0..=1000 {
                let dummy_branch = vec![make_break_stmt(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: lv.clone(),
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }






    #[test]
    fn empty_if_statement_branch() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }


    // Even if the if statement main branch is not empty
    // it should never terminate, because there is no else branch, 
    // meaning it may or may not execute at all.
    #[test]
    fn if_statement_branch_never_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {

            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: dummy_branch,
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }






    // Same as the empty_if_statement_branch test, except else branch now contains Some() but is empty inside.
    #[test]
    fn empty_if_statement_branch_and_empty_else_branch() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: Some(vec![]),
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branches do not terminate
            assert_eq!(result, false);
        }
    }


    // Main if branch terminates via `return` statement, but it has no else branch 
    // therefore the block does not always terminate.
    #[test]
    fn if_statement_main_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_return_stmt(vec![l])
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }



   
    // else branch terminates via `return` statement, but main branch does not 
    // terminate, therefore the block does not always terminate.
    #[test]
    fn if_statement_else_branch_return_not_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: dummy_branch, 
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            make_return_stmt(vec![lv.clone()])
                        ]),
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }



    // elif branch terminates via `return` statement, but main branch and else does not 
    // terminate, therefore the block does not always terminate.
    #[test]
    fn if_statement_elif_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![], 
                    elif_branches: vec![(l.clone(), vec![
                        make_return_stmt(vec![l])
                    ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }

    // main and elif branch terminates via `return` statement, but else branch does not
    // exist, therefore the block does not always terminate.
    #[test]
    fn if_statement_main_and_elif_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l1 in &literals {
            for l2 in &literals {
                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: l1.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![l1.clone()])
                        ], 
                        elif_branches: vec![(l1.clone(), vec![
                            make_return_stmt(vec![l2.clone()])
                        ])],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // main and elif branch terminates via `return` statement, but else branch is empty
    // therefore the block does not always terminate.
    #[test]
    fn if_statement_main_and_elif_branch_return_else_empty_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l1 in &literals {
            for l2 in &literals {
                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: l1.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![l1.clone()])
                        ], 
                        elif_branches: vec![(l1.clone(), vec![
                            make_return_stmt(vec![l2.clone()])
                        ])],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }






    // main and else branches terminates via `return` statement, but elif branches are not empty,
    // and do not terminate, therefore the block does not always terminate.
    #[test]
    fn if_statement_main_branch_and_else_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();


        for l1 in &literals {
            for l2 in &literals {
                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: l1.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![l2.clone()])
                        ], 

                        elif_branches: vec![(l1.clone(), vec![])],
                        else_branch: Some(vec![
                            make_return_stmt(vec![l2.clone()])
                        ]),
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }



    // if statement main branch terminates via `return` statement, and so does the else branch,
    // and there are no elif branches, therefore the block always terminates.
    #[test]
    fn if_statement_main_and_else_branch_return_terminates() {
        let literals = get_all_literals_no_arr();

        for l1 in &literals {
            for l2 in &literals {
                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: l1.clone(),
                        if_branch: vec![
                            make_return_stmt(vec![l1.clone()])
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            make_return_stmt(vec![l2.clone()])
                        ]),
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does terminate
                assert_eq!(result, true);
            }
        }
    }


    // if statement main branch terminates via `return` statement, and so does the else branch,
    // and there are `i` of elif branches that also terminates, 
    // therefore the block always terminates.
    #[test]
    fn if_statement_main_and_else_and_elif_branches_return_terminates() {
        let literals = get_all_literals_no_arr();

        for l1 in &literals {
            for l2 in &literals {
                let mut elif_branches = vec![];
                for _ in 1..100 { 
                    elif_branches.push((l1.clone(), vec![
                        make_return_stmt(vec![l2.clone()])
                    ]));

                    let stmts: Vec<Stmt> = vec![
                        Stmt::If(IfStmt{
                            condition: l1.clone(),
                            if_branch: vec![
                                make_return_stmt(vec![l1.clone()])
                            ],
                            elif_branches: elif_branches.clone(),
                            else_branch: Some(vec![
                                make_return_stmt(vec![l2.clone()])
                            ]),
                            span: span(),
                        })
                    ];

                    let result: bool = block_always_terminates(&stmts, false);
                    // Branch does terminate
                    assert_eq!(result, true);
                }
            }
        }
    }




    // Same as the above tests, but these variants terminates via a `break` statement instead.
    //
    #[test]
    fn if_statement_main_branch_break_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_break_stmt()
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }
    
    // else branch terminates via `break` statement, but main branch does not 
    // terminate, therefore the block does not always terminate.
    #[test]
    fn if_statement_else_branch_break_not_terminates() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: lv.clone(),
                        if_branch: dummy_branch, 
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            make_break_stmt()
                        ]),
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does not terminate
                assert_eq!(result, false);
            }
        }
    }


    // if statement main branch terminates via `break` statement, and so does the else branch,
    // and there are no elif branches, therefore the block always terminates.
    #[test]
    fn if_statement_main_and_else_branch_break_terminates() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_break_stmt()
                    ],
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        make_break_stmt()
                    ]),
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does terminate
            assert_eq!(result, true);
        }
    }




    // elif branch terminates via `break` statement, but main branch and else does not 
    // terminate, therefore the block does not always terminate.
    #[test]
    fn if_statement_elif_branch_break_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![], 
                    elif_branches: vec![(l.clone(), vec![
                        make_break_stmt()
                    ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }

    // main and elif branch terminates via `breakk` statement, but else branch does not
    // exist, therefore the block does not always terminate.
    #[test]
    fn if_statement_main_and_elif_branch_break_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_break_stmt()
                    ], 
                    elif_branches: vec![(l.clone(), vec![
                        make_break_stmt()
                    ])],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts, false);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }





    // if statement main branch terminates via `break` statement, and so does the else branch,
    // and there are `i` of elif branches that also terminates, 
    // therefore the block always terminates.
    #[test]
    fn if_statement_main_and_else_and_elif_branches_break_terminates() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let mut elif_branches = vec![];
            for _ in 1..100 { 
                elif_branches.push((l.clone(), vec![
                    make_break_stmt()
                ]));

                let stmts: Vec<Stmt> = vec![
                    Stmt::If(IfStmt{
                        condition: l.clone(),
                        if_branch: vec![
                            make_break_stmt()
                        ],
                        elif_branches: elif_branches.clone(),
                        else_branch: Some(vec![
                            make_break_stmt()
                        ]),
                        span: span(),
                    })
                ];

                let result: bool = block_always_terminates(&stmts, false);
                // Branch does terminate
                assert_eq!(result, true);
            }
        }
    }



}

#[cfg(test)]
mod test_dead_code_analysis {
    use super::*;
    

    #[test]
    fn empty_infinite_statement_branch() {
        let stmts: Vec<Stmt> = vec![
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            })
        ];

        let result = dead_code_analysis(&stmts);
        // Block has dead code (because of empty branch).
        assert!(result.is_err());
                
        assert!(result.unwrap_err().to_string().starts_with("Semantic error: Infinite loop branch has no statements. Empty branches are not allowed"));
    }


    #[test]
    fn infinite_statement_branch_not_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts);
                // Block has no dead code.
                assert!(result.is_ok());
            }
        }
    }



    #[test]
    fn infinite_statement_branch_return_multiple_times_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = make_return_stmt(vec![lv.clone()]);
            for i in 1..=1000 {
                let dummy_branch = vec![stmt.clone(); i + 1];

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts);
                // Block has dead code because it returns more than once.
                assert!(result.is_err());
            }
        }
    }


    #[test]
    fn infinite_statement_branch_stmts_after_return_dead() {
        let literals_with_var = get_all_literals_with_var_no_arr();

        for lv in literals_with_var {
            let stmt = Stmt::Expr(lv.clone());
            for i in 0..=1000 {
                let mut dummy_branch = vec![stmt.clone(); i + 1];
            
                // Insert return statement at `i`
                let rstmt = make_return_stmt(vec![lv.clone()]);
                dummy_branch.insert(i, rstmt);

                let stmts: Vec<Stmt> = vec![
                    Stmt::Infinite(InfiniteStmt{
                        branch: dummy_branch,
                        span: span(),
                    })
                ];

                let result = dead_code_analysis(&stmts);
                // Block has dead code because it contains statements after the certain return.
                assert!(result.is_err());
            }
        }
    }


}
