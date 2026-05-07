use super::*;

#[cfg(test)]
mod if_stmt_tests {
    use super::*;

    // Test if statements with only literals, with no else, no elif, and no string/bool literals
    #[test]
    fn test_if_statements_ints_floats_literals_same_type() {
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
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
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



    // Test if statements with only variables with same type, with no else, no elif, and no string/bool variables
    #[test]
    fn test_if_statements_ints_floats_vars_same_type() {
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
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
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
                    var_decl("x", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
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
                    var_decl("y", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
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
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
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
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
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
                    var_decl("x", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
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
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
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
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
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
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
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
                    var_decl("x", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
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
                    var_decl("y", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],

                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
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
