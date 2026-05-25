use super::*;

#[cfg(test)]
mod while_stmt_tests {
    use super::*;

    // Ensure while loops empty branches are not allowed
    #[test]
    fn test_while_statements_empty_branch_errors() {
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
                        condition: condition,
                        branch: vec![],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }


   
    // Tests while statements without booleans, or binop, or anything that could be evaluated to
    // bool, is an error.
    #[test]
    fn test_while_statements_no_bool_eval_expr_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            // Skip bools
            if *t == Type::Bool {
                continue
            }
            let body = vec![ 
                Stmt::While(WhileStmt{
                    condition: l.clone(),
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl(true, "z", t.clone(), l.clone()),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("While statement require an expression to be evaulatable to type `bool`"));
        }
    }





    // Test while statements with only literals, no strings/bools
    #[test]
    fn test_while_statements_ints_floats_literals_same_type() {
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
                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test while statements with only variables, no strings/bools
    #[test]
    fn test_while_statements_ints_floats_vars_same_type() {
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
                    var_decl(true, "x", t.clone(), l.clone()),
                    var_decl(true, "y", t.clone(), l.clone()),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test while statements with literals and variables mixed  (left & right side), no strings/bools
    #[test]
    fn test_while_statements_ints_floats_vars_literals_same_type() {
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
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
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
                    var_decl(true, "y", t.clone(), l.clone()),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl(true, "z", t.clone(), l.clone()),
                        ],
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
