use super::*;

#[cfg(test)]
mod if_stmt_tests {
    use super::*;

    // Test if statements with non-boolean literals, with no else, no elif
    #[test]
    fn if_branch_non_bool_literals_errors() {
        let literals = get_all_literals();
        let non_boolean_conds = get_non_boolean_conditions();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for nbl in &non_boolean_conds {
                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: nbl.clone(),
                        if_branch: vec![ var_decl("z", t.clone(), l.clone()) ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("evaulatable to type `bool`"));
            }
        }
    }

    // Test if statements with non-boolean literals, with no else
    #[test]
    fn if_elif_branch_non_bool_literals_errors() {
        let literals = get_all_literals();
        let non_boolean_conds = get_non_boolean_conditions();
        let boolean_conds = get_many_boolean_conditions();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for bl in &boolean_conds {
                for nbl in &non_boolean_conds {
                    let body = vec![ 
                        Stmt::If(IfStmt{
                            condition: bl.clone(),
                            if_branch: vec![ var_decl("z", t.clone(), l.clone()) ],
                            elif_branches: vec![(nbl.clone(), vec![
                                // For above reason
                                var_decl("e", t.clone(), l.clone()),
                            ])],
                            else_branch: None,
                            span: span(),
                        }),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("evaulatable to type `bool`"));
                }
            }
        }
    }


    // Test if statements with only boolean literals and boolean evaluated binops, with no else, no elif
    //
    #[test]
    fn if_branch_bool_eval_conditions() {
        let literals = get_all_literals();
        let boolean_conds = get_many_boolean_conditions();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for bl in &boolean_conds {
                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![ var_decl("z", t.clone(), l.clone()) ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();
            }
        }
    }


    // Same as above, except this time with only variables, instead of literals in the binary
    // expression.
    #[test]
    fn if_branch_vars_bool_eval_conditions() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                // So that >= > <= < doesnt get performed on non integer/floats.
                if !ALL_BIN_OP_KIND_COMP_EQ.contains(&b) {
                    match l {
                        Expr::StringLiteral { .. } | Expr::BoolLiteral { .. } | Expr::ArrayLiteral { .. } => {
                            continue
                        },
                        _ => {}
                    }
                }
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),
                    var_decl("y", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![ var_decl("z", t.clone(), l.clone()) ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with literals and variables mixed (left & right side), with no else, no elif, and no string/bool literals
    #[test]
    fn test_if_statements_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        // Variable left side, Literal right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }

        // Literal left side, Variable right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("y", t.clone(), l.clone()),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }

    //////////////////////////////////  If statements with only elif /////////////////////////

    // Test if statements with only literals, with elif. but no else, and no string/bool literals
    #[test]
    fn test_if_statements_with_elif_ints_floats_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), l.clone()),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with only variables, with elif. but no else, and no string/bool variables
    #[test]
    fn test_if_statements_with_elif_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };


                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),
                    var_decl("y", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), l.clone()),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }
    }



    // Test if statements with literals and variables mixed (left & right side), with elif. but no else, and no string/bool literals
    #[test]
    fn test_if_statements_with_elif_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), l.clone()),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }


        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("y", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), l.clone()),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }
    }




    ////////////////////////////// end /////////////////////




    //////////////////////////////////  If statements with only else /////////////////////////

    // Test if statements with only literals, with else. but no elif, and no string/bool literals
    #[test]
    fn test_if_statements_with_else_ints_floats_literals_same_type() {
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
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), l.clone())
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with only variables with same type with else. but no elif, and no string/bool variables
    #[test]
    fn test_if_statements_with_else_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),
                    var_decl("y", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), l.clone())
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with literals and variables mixed (left & right side), with else. but no elif, and no string/bool literals
    #[test]
    fn test_if_statements_with_else_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        // Variable left side, Literal right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), l.clone()),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), l.clone())
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }

        // Literal left side, Variable right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("y", t.clone(), l.clone()),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), l.clone()),
                        ],
                        elif_branches: vec![],

                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), l.clone())
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }
}
