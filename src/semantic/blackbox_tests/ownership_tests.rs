/// This file tests ownership rules
/// over variables, constants, etc. in different contextes
/// to catch violation of semantics rules, and legal cases
///
///
use super::*;

#[cfg(test)]
mod ownership_tests {
    use super::*;

    #[test]
    fn function_call_arg_moves_var() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Expr(call_expr("bar", vec![var_expr("x")]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 2);
            assert_eq!(ast.functions[0].body.len(), 1);
            assert_eq!(ast.functions[1].body.len(), 2);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::VarDecl(v) = &ast.functions[1].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }
        }
    }


    #[test]
    fn function_call_arg_does_not_move_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                Stmt::Expr(call_expr("bar", vec![var_expr("x")])),
                Stmt::Expr(call_expr("bar", vec![var_expr("x")]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 2);
            assert_eq!(ast.functions[0].body.len(), 1);
            assert_eq!(ast.functions[1].body.len(), 3);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::Const(c) = &ast.functions[1].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected constant, got {:?}", ast); }
        }
    }



    #[test]
    fn vardecl_uses_moved_var_to_func_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Expr(call_expr("bar", vec![var_expr("x")])),
                var_decl(true, "y", t.clone(), var_expr("x")),
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }


    #[test]
    fn vardecl_moves_var() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 2);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        }
    }


    #[test]
    fn vardecl_in_if_main_branch_does_not_move_upstream_const() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    const_define_locally("x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x")),
                            var_decl(true, "h", t.clone(), var_expr("x"))
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 2);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::Const(c) = &ast.functions[0].body[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                    assert_eq!(c.value, l.clone());
                } else { panic!("expected Constant, got {:?}", ast); }

                if let Stmt::If(i) = &ast.functions[0].body[1] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 2);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());

                    if let Stmt::VarDecl(v) = &i.if_branch[0] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", i); }

                    if let Stmt::VarDecl(v) = &i.if_branch[1] {
                        assert_eq!(v.name, "h");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", i); }

                } else { panic!("expected if statement, got {:?}", ast); }
            }
        }
    }


    #[test]
    fn vardecl_in_if_main_branch_moves_upstream_var() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 2);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }

                if let Stmt::If(i) = &ast.functions[0].body[1] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());

                    if let Stmt::VarDecl(v) = &i.if_branch[0] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", i); }
                } else { panic!("expected if statement, got {:?}", ast); }
            }
        }
    }


    #[test]
    fn vardecl_in_if_main_branch_moves_upstream_var_use_of_var_inside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x")),
                            var_decl(true, "h", t.clone(), var_expr("x"))
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }

    #[test]
    fn vardecl_in_if_main_branch_moves_upstream_var_use_of_var_outside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),

                    var_decl(true, "h", t.clone(), var_expr("x"))
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }



    #[test]
    fn vardecl_in_if_else_branch_moves_upstream_var() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ]),
                        span: span(),
                    })
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 2);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }

                if let Stmt::If(i) = &ast.functions[0].body[1] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_some());
                    assert_eq!(i.else_branch.clone().unwrap().len(), 1);

                    if let Stmt::VarDecl(v) = &i.if_branch[0] {
                        assert_eq!(v.name, "z");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", i); }

                    if let Stmt::VarDecl(v) = &i.else_branch.clone().unwrap()[0] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", i); }
                } else { panic!("expected if statement, got {:?}", ast); }
            }
        }
    }


    #[test]
    fn vardecl_in_if_else_branch_moves_upstream_var_use_of_var_inside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            var_decl(true, "y", t.clone(), var_expr("x")),
                            var_decl(true, "h", t.clone(), var_expr("x"))
                        ]),
                        span: span(),
                    }),
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }

    #[test]
    fn vardecl_in_if_else_branch_moves_upstream_var_use_of_var_outside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ]),
                        span: span(),
                    }),

                    var_decl(true, "h", t.clone(), var_expr("x"))
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }




    #[test]
    fn vardecl_in_if_elif_branch_moves_upstream_var() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![ (bl.clone(), vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ]) ],
                        else_branch: None,
                        span: span(),
                    })
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 2);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }

                if let Stmt::If(i) = &ast.functions[0].body[1] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 1);
                    assert!(i.else_branch.is_none());

                    if let Stmt::VarDecl(v) = &i.if_branch[0] {
                        assert_eq!(v.name, "z");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", i); }

                    if let Stmt::VarDecl(v) = &i.elif_branches[0].1[0] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", i); }
                } else { panic!("expected if statement, got {:?}", ast); }
            }
        }
    }


    #[test]
    fn vardecl_in_if_elif_branch_moves_upstream_var_use_of_var_inside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![ (bl.clone(), vec![
                            var_decl(true, "y", t.clone(), var_expr("x")),
                            var_decl(true, "h", t.clone(), var_expr("x"))
                        ]) ],
                        else_branch: None,
                        span: span(),
                    }),
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }

    #[test]
    fn vardecl_in_if_elif_branch_moves_upstream_var_use_of_var_outside_branch_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),

                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            // dummy
                            var_decl(true, "z", t.clone(), l.clone())
                        ],
                        elif_branches: vec![ (bl.clone(), vec![
                            var_decl(true, "y", t.clone(), var_expr("x")),
                        ]) ],
                        else_branch: None,
                        span: span(),
                    }),

                    var_decl(true, "h", t.clone(), var_expr("x"))
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
            }
        }
    }







    #[test]
    fn vardecl_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), var_expr("x")),
                var_decl(true, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }


    #[test]
    fn vardecl_does_not_move_local_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), var_expr("x")),
                var_decl(true, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 0);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 3);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[2] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }
    
    #[test]
    fn vardecl_does_not_move_global_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "y", t.clone(), var_expr("x")),
                var_decl(true, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = AST { functions: vec![func], globals: vec![  const_define_globally("x", t.clone(), l.clone()) ]};
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 1);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 2);

            if let GlobalStmt::Const(c) = &ast.globals[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }





    #[test]
    fn vardecl_with_literal_use_after_move_errors() {
        // own a t = 5
        // own b t = a   (moves `a`)
        // own c t = a   (this must error because `a` already moved)

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "a", t.clone(), l.clone()),
                var_decl(true, "b", t.clone(), var_expr("a")),
                var_decl(true, "c", t.clone(), var_expr("a")),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn vardecl_without_literal_use_after_move_errors() {
        // own a = 5
        // own b = a   (moves `a`)
        // own c = a   (this must error because `a` already moved)

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) { 
            let body = vec![
                var_decl(true, "a", t.clone(), l.clone()),
                var_decl(true, "b", t.clone(), var_expr("a")),
                var_decl(true, "c", t.clone(), var_expr("a")),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn inited_vars_assign_moves_var() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), l.clone()),
                Stmt::Unlock(vec![var_expr("y")]),
                var_assign("y", var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 4);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::Unlock(vec) = &ast.functions[0].body[2] {
                assert_eq!(vec.len(), 1);
            } else { panic!("expected Unlock, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[3] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }

        }
    }

    #[test]
    fn uninited_vars_assign_moves_var() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(false, "x", t.clone(), l.clone()),
                var_decl(false, "y", t.clone(), l.clone()),
                var_assign("y", var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 3);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[2] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }

        }
    }

    #[test]
    fn inited_var_assign_does_not_move_local_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), l.clone()),
                Stmt::Unlock(vec![var_expr("y")]),
                var_assign("y", var_expr("x")),
                var_decl(true, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 0);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 5);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::Unlock(vec) = &ast.functions[0].body[2] {
                assert_eq!(vec.len(), 1);
            } else { panic!("expected Unlock, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[3] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }

        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[4] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }

    #[test]
    fn uninited_var_assign_does_not_move_local_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                var_decl(false, "y", t.clone(), l.clone()),
                var_assign("y", var_expr("x")),
                var_decl(false, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 0);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 4);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[1] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[2] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }

        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[3] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }

    #[test]
    fn inited_var_assign_does_not_move_global_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "y", t.clone(), l.clone()),
                Stmt::Unlock(vec![var_expr("y")]),
                var_assign("y", var_expr("x")),
                var_decl(true, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = AST { functions: vec![func], globals: vec![  const_define_globally("x", t.clone(), l.clone()) ]};
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 1);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 4);

            if let GlobalStmt::Const(c) = &ast.globals[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::Unlock(vec) = &ast.functions[0].body[1] {
                assert_eq!(vec.len(), 1);
            } else { panic!("expected Unlock, got {:?}", ast); }



            if let Stmt::VarAssign(va) = &ast.functions[0].body[2] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }
        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[3] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }

    #[test]
    fn uninited_var_assign_does_not_move_global_const() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(false, "y", t.clone(), l.clone()),
                var_assign("y", var_expr("x")),
                var_decl(false, "z", t.clone(), var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = AST { functions: vec![func], globals: vec![  const_define_globally("x", t.clone(), l.clone()) ]};
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 1);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 3);

            if let GlobalStmt::Const(c) = &ast.globals[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "y");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[1] {
                assert_eq!(va.name, "y");
                assert_eq!(va.value, var_expr("x"));
            } else { panic!("expected VarAssign, got {:?}", ast); }
        
            if let Stmt::VarDecl(v) = &ast.functions[0].body[2] {
                assert_eq!(v.name, "z");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, var_expr("x"));
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
        }
    }




    #[test]
    fn copy_call_allows_reuse() {
        // own a T = EXPRESSION
        // own b T = copy(a)  (copies, does not move)
        // own c T = a        (valid, because no moves happened)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_a = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };
            let body = vec![
                var_decl(true, "a", t.clone(), l.clone()),
                var_decl(true, "b", t.clone(), copy_a),
                var_decl(true, "c", t.clone(), var_expr("a")),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn pass_variable_to_func_call_marks_it_moved() {
        // bar takes one t.
        // own a t = EXPRESSION
        // bar(a)       (moves a)
        // own b t = a  (error)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let bar = void_func("bar", vec![param("p", t.clone())], vec![]);
            let body = vec![
                var_decl(true, "a", t.clone(), l.clone()),
                Stmt::Expr(call_expr("bar", vec![var_expr("a")])),
                var_decl(true, "b", t.clone(), var_expr("a")),
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![bar, caller], globals: vec![]};
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
            
        }
    }


    #[test]
    fn vardecl_moving_local_var_in_while_loop() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    Stmt::While(WhileStmt{
                            condition: bl.clone(),
                            branch: vec![
                                var_decl(true, "x", t.clone(), l.clone()),
                                var_decl(true, "y", t.clone(), var_expr("x"))
                            ],
                            span: span(),
                        }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);
                
                if let Stmt::While(w) = &ast.functions[0].body[0] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(w.condition, bl.clone());
                    }

                    assert_eq!(w.branch.len(), 2);
                    if let Stmt::VarDecl(v) = &w.branch[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                    
                    if let Stmt::VarDecl(v) = &w.branch[1] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                } else { panic!("expected While loop statement, got {:?}", ast); }
            }
        }
    }

    #[test]
    fn vardecl_moving_local_var_in_infinite_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone()),
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 1);
            assert_eq!(ast.globals.len(), 0);

            
            if let Stmt::Infinite(i) = &ast.functions[0].body[0] {
                assert_eq!(i.branch.len(), 2);
                if let Stmt::VarDecl(v) = &i.branch[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }
                
                if let Stmt::VarDecl(v) = &i.branch[1] {
                    assert_eq!(v.name, "y");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, var_expr("x"));
                } else { panic!("expected VarDecl, got {:?}", ast); }
            } else { panic!("expected Infinite loop statement, got {:?}", ast); }
        }
    }

    #[test]
    fn vardecl_moving_local_var_in_for_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(Type::Array(Box::new(t.clone()))));

            let body = vec![
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone()),
                            var_decl(true, "y", t.clone(), var_expr("x"))
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
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
                assert_eq!(v.value, arr_lit);
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
            if let Stmt::For(i) = &ast.functions[0].body[1] {
                assert_eq!(i.holder_name, "e");
                assert_eq!(i.value, var_expr("a"));

                assert_eq!(i.branch.len(), 2);
                if let Stmt::VarDecl(v) = &i.branch[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }
                
                if let Stmt::VarDecl(v) = &i.branch[1] {
                    assert_eq!(v.name, "y");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, var_expr("x"));
                } else { panic!("expected VarDecl, got {:?}", ast); }
            } else { panic!("expected For loop statement, got {:?}", ast); }
        }
    }

    #[test]
    fn vardecl_moving_local_var_in_for_range_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            for i in 0usize..=1000usize {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    Stmt::For(ForStmt{
                            holder_name: "e".to_string(),
                            value: Expr::RangeCall{ start: Box::new(usize_lit(0)), end: Box::new(usize_lit(i)), span: span()},
                            branch: vec![
                                var_decl(true, "y", t.clone(), var_expr("x"))
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
    }

    #[test]
    fn varassign_moving_local_var_in_while_loop() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    Stmt::While(WhileStmt{
                            condition: bl.clone(),
                            branch: vec![
                                var_decl(true, "x", t.clone(), l.clone()),
                                var_decl(false, "y", t.clone(), l.clone()),
                                var_assign("y", var_expr("x"))
                            ],
                            span: span(),
                        }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);
                
                if let Stmt::While(w) = &ast.functions[0].body[0] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(w.condition, bl.clone());
                    }

                    assert_eq!(w.branch.len(), 3);
                    if let Stmt::VarDecl(v) = &w.branch[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                    
                    if let Stmt::VarDecl(v) = &w.branch[1] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }


                    if let Stmt::VarAssign(va) = &w.branch[2] {
                        assert_eq!(va.name, "y");
                        assert_eq!(va.value, var_expr("x"));
                    } else { panic!("expected VarAssign, got {:?}", ast); }

                } else { panic!("expected While loop statement, got {:?}", ast); }
            }
        }
    }

    #[test]
    fn varassign_moving_local_var_in_infinite_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone()),
                            var_decl(false, "y", t.clone(), l.clone()),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 1);
            assert_eq!(ast.globals.len(), 0);

            
            if let Stmt::Infinite(i) = &ast.functions[0].body[0] {
                assert_eq!(i.branch.len(), 3);
                if let Stmt::VarDecl(v) = &i.branch[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }
                
                if let Stmt::VarDecl(v) = &i.branch[1] {
                    assert_eq!(v.name, "y");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());

                } else { panic!("expected VarDecl, got {:?}", ast); }

                if let Stmt::VarAssign(va) = &i.branch[2] {
                    assert_eq!(va.name, "y");
                    assert_eq!(va.value, var_expr("x"));
                } else { panic!("expected VarAssign, got {:?}", ast); }

            } else { panic!("expected Infinite loop statement, got {:?}", ast); }
        }
    }

    #[test]
    fn varassign_moving_local_var_in_for_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(Type::Array(Box::new(t.clone()))));

            let body = vec![
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(true, "x", t.clone(), l.clone()),
                            var_decl(false, "y", t.clone(), l.clone()),
                            var_assign("y", var_expr("x"))
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
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
                assert_eq!(v.value, arr_lit);
            } else { panic!("expected VarDecl, got {:?}", ast); }
        
            if let Stmt::For(i) = &ast.functions[0].body[1] {
                assert_eq!(i.holder_name, "e");
                assert_eq!(i.value, var_expr("a"));

                assert_eq!(i.branch.len(), 3);
                if let Stmt::VarDecl(v) = &i.branch[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }
                
                if let Stmt::VarDecl(v) = &i.branch[1] {
                    assert_eq!(v.name, "y");
                    assert_eq!(v.type_name, t.clone());
                    assert_eq!(v.value, l.clone());
                } else { panic!("expected VarDecl, got {:?}", ast); }

                if let Stmt::VarAssign(va) = &i.branch[2] {
                    assert_eq!(va.name, "y");
                    assert_eq!(va.value, var_expr("x"));
                } else { panic!("expected VarAssign, got {:?}", ast); }

            } else { panic!("expected For loop statement, got {:?}", ast); }
        }
    }

    #[test]
    fn varassign_moving_local_var_in_for_range_loop() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            for i in 0usize..=1000usize {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    Stmt::For(ForStmt{
                            holder_name: "e".to_string(),
                            value: Expr::RangeCall{ start: Box::new(usize_lit(0)), end: Box::new(usize_lit(i)), span: span()},
                            branch: vec![
                                var_decl(false, "y", t.clone(), l.clone()),
                                var_assign("y", var_expr("x")),
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
    }

    #[test]
    fn use_of_moved_upstream_var_in_one_if_stmt_branch_reflects_even_if_other_branches_dont_move_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
                    Stmt::If(IfStmt{
                        condition: bl.clone(),
                        if_branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            var_decl(true, "h", t.clone(), l.clone())
                        ]),
                        span: span(),
                    }),

                    var_decl(true, "q", t.clone(), var_expr("x"))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Use of moved variable"));
            }
        }
    }



    #[test]
    fn vardecl_moving_local_var_in_if_stmt_main_branch() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bl.clone(),
                            if_branch: vec![
                                var_decl(true, "x", t.clone(), l.clone()),
                                var_decl(true, "y", t.clone(), var_expr("x"))
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::If(i) = &ast.functions[0].body[0] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 2);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_none());

                    if let Stmt::VarDecl(v) = &i.if_branch[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                    
                    if let Stmt::VarDecl(v) = &i.if_branch[1] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                } else { panic!("expected if statement, got {:?}", ast); }
            }
        }
    }

    #[test]
    fn vardecl_moving_local_var_in_if_stmt_else_branch() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bl.clone(),
                            if_branch: vec![
                                // Dummy
                                var_decl(true, "z", t.clone(), l.clone()),
                            ],
                            elif_branches: vec![],
                            else_branch: Some(vec![
                                var_decl(true, "x", t.clone(), l.clone()),
                                var_decl(true, "y", t.clone(), var_expr("x"))
                            ]),
                            span: span(),
                        }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::If(i) = &ast.functions[0].body[0] {
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                    }
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 0);
                    assert!(i.else_branch.is_some());
                    assert_eq!(i.else_branch.clone().unwrap().len(), 2);

                    if let Stmt::VarDecl(v) = &i.else_branch.clone().unwrap()[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                    
                    if let Stmt::VarDecl(v) = &i.else_branch.clone().unwrap()[1] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                } else { panic!("expected if tatement, got {:?}", ast); }
            }
        }
    }


    #[test]
    fn vardecl_moving_local_var_in_if_stmt_elif_branch() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    Stmt::If(IfStmt{
                            condition: bl.clone(),
                            if_branch: vec![
                                // Dummy
                                var_decl(true, "z", t.clone(), l.clone()),
                            ],
                            elif_branches: vec![
                                (bl.clone(), vec![
                                    var_decl(true, "x", t.clone(), l.clone()),
                                    var_decl(true, "y", t.clone(), var_expr("x"))
                                ]),
                            ],
                            else_branch: None, 
                            span: span(),
                        }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::If(i) = &ast.functions[0].body[0] {
                    assert_eq!(i.if_branch.len(), 1);
                    assert_eq!(i.elif_branches.len(), 1);
                    assert_eq!(i.elif_branches[0].1.len(), 2);
                    if !contains_array_literal(&bl) {
                        assert_eq!(i.condition, bl.clone());
                        assert_eq!(i.elif_branches[0].0, bl.clone());
                    }
                    assert!(i.else_branch.is_none());

                    if let Stmt::VarDecl(v) = &i.elif_branches[0].1[0] {
                        assert_eq!(v.name, "x");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, l.clone());
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                    
                    if let Stmt::VarDecl(v) = &i.elif_branches[0].1[1] {
                        assert_eq!(v.name, "y");
                        assert_eq!(v.type_name, t.clone());
                        assert_eq!(v.value, var_expr("x"));
                    } else { panic!("expected VarDecl, got {:?}", ast); }
                } else { panic!("expected if tatement, got {:?}", ast); }
            }
        }
    }


    // Tests the rule:
    // You cannot move an upstream variable multiple times inside a loop.
    //

    #[test]
    fn vardecl_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        let boolean_conditions = get_many_boolean_conditions();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for bl in &boolean_conditions {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
                    Stmt::While(WhileStmt{
                            condition: bl.clone(),
                            branch: vec![
                                var_decl(true, "y", t.clone(), var_expr("x"))
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
    }


    #[test]
    fn vardecl_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
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
    fn vardecl_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(true, "y", t.clone(), var_expr("x"))
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
    fn vardecl_moving_upstream_var_in_for_range_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            for i in 0usize..=1000usize {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
                    var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    Stmt::For(ForStmt{
                            holder_name: "e".to_string(),
                            value: Expr::RangeCall{ start: Box::new(usize_lit(0)), end: Box::new(usize_lit(i)), span: span()},
                            branch: vec![
                                var_decl(true, "y", t.clone(), var_expr("x"))
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
    }

    #[test]
    fn varassign_to_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), var_expr("x")),

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
    fn initied_var_assign_to_self_doesnt_move() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                
                Stmt::Unlock(vec![var_expr("x")]),

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
    fn uninitied_var_assign_to_self_doesnt_move() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(false, "x", t.clone(), l.clone()),
                
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
    fn dynamic_array_access_on_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("a")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                        // move a to x
                        var_decl(true, "x", Type::Array(Box::new(t.clone())), var_expr("a")), 
                        var_decl(true, "y", t.clone(), access),
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
    fn fix_array_access_on_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("a")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl(true, "a", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                        // move a to x
                        var_decl(true, "x", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), var_expr("a")),
                        var_decl(true, "y", t.clone(), access),
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
    fn varassign_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "y", t.clone(), var_expr("x")),
                var_decl(true, "z", t.clone(), l.clone()),
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
    fn initied_var_assign_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(true, "y", t.clone(), l.clone()),
                            Stmt::Unlock(vec![var_expr("y")]),
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
    fn uninitied_var_assign_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl(false, "y", t.clone(), l.clone()),
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
    fn initied_var_assign_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl(true, "y", t.clone(), l.clone()),
                            Stmt::Unlock(vec![var_expr("y")]),
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
    fn uninitied_var_assign_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl(false, "y", t.clone(), l.clone()),
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
    fn inited_vars_assign_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(true, "y", t.clone(), l.clone()),
                            Stmt::Unlock(vec![var_expr("y")]),
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
    fn uninited_vars_assign_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = array_lit(vec![], Some(t.clone()));

            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()),
                var_decl(true, "a", Type::Array(Box::new(t.clone())), arr_lit),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl(false, "y", t.clone(), l.clone()),
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
    fn multi_assign_to_initied_vars_use_of_moved_vars_errors() {
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
                var_decl(true, "a", t1.clone(), l1.clone()),
                var_decl(true, "b", t2.clone(), l2.clone()),
                
                Stmt::Unlock(vec![var_expr("a"), var_expr("b")]),

                var_decl(true, "c", t1.clone(), var_expr("a")),

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
                var_decl(true, "a", t1.clone(), l1.clone()),
                var_decl(true, "b", t2.clone(), l2.clone()),
                Stmt::Unlock(vec![var_expr("a"), var_expr("b")]),

                var_decl(true, "c", t2.clone(), var_expr("b")),

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
                var_decl(true, "a", t1.clone(), l1.clone()),
                var_decl(true, "b", t2.clone(), l2.clone()),

                Stmt::Unlock(vec![var_expr("a"), var_expr("b")]),

                var_decl(true, "c", t1.clone(), var_expr("a")),
                var_decl(true, "d", t2.clone(), var_expr("b")),
                
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
    fn array_valid_multiple_access_both_ends_on_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i-1 {
                    let access = Expr::ArraySlicing {
                        array: Box::new(var_expr("arr")),
                        range: ArraySliceRange::FromTo(Box::new(usize_lit(1)), Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl(true, "arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                        // move arr to x
                        var_decl(true, "x", Type::Array(Box::new(t.clone())), var_expr("arr")), 
                        var_decl(true, "y", t.clone(), access),
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

