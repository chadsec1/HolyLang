use super::*;

#[cfg(test)]
mod var_decl_tests {
    use super::*;

    // TODO: Add new test that hyper focuses on integers coericon, instead of this weak tests
    #[test]
    fn test_vardecl_literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            // a literal of type T with infer type should be still correctly as T in the checked
            // ast, unless the literal is illegal
            //
            let body = vec![var_decl("x", t.clone(), Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, Some(l.clone()));
            } else {
                panic!("expected VarDecl");
            }
        }
    }

    #[test]
    fn test_vardecl_variable_name_taken_by_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let main = void_func("main", vec![], vec![
                var_decl("foo", t.clone(), Some(call_expr("foo", vec![]))),
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("`foo` is already taken by a function, pick a different name for your variable."));
        }
    }

    #[test]
    fn test_vardecl_uses_non_declared_var_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("y")))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared binding `y`"));
        }
    }


    // This test is duplicated but not really.. other tests dont test it all way through.
    #[test]
    fn test_var_decl_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for t in ALL_INT_TYPES_NO_ARR {
            for l in &literals_no_ints {
                let body = vec![var_decl("x", t.clone(), Some(l.clone()))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }


    // Tests the rule: 
    // You cannot overshadow variables declared in an upstream scope
    //

    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_for_loop_holder_errors() {
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
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Cannot use variable name `x` in for loop statement as it is already declared"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_var_in_for_loop_errors() {
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
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }

    #[test]
    fn test_vardecl_overshadowing_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_var_in_if_main_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        var_decl("x", t.clone(), Some(l.clone()))
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
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }


    #[test]
    fn test_vardecl_overshadowing_var_in_if_else_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),

                    ],
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        var_decl("x", t.clone(), Some(l.clone()))
                    ]),
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }


    #[test]
    fn test_vardecl_overshadowing_var_in_if_elif_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    elif_branches: vec![
                        (bool_lit(false), vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ])
                    ],
                    else_branch: None,
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already declared"));
        }
    }

    // This tests  integers / floats only, against Bool / String
    #[test]
    fn test_vardecl_type_mismatch_int_bool_errors() {

        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in &literals_ints_floats {
            // Variables declared with explicit type of bool, but given an non-bool literal is a type mismatch
            let body = vec![var_decl("x", Type::Bool, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());    
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }


        for l in literals_ints_floats {
            // Variables declared with explicit type of string, but given an non-string literal is a type mismatch
            let body = vec![var_decl("x", Type::String, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }

    }

    // default values assigning tests
    //
    #[test]
    fn test_default_int8_zero() {
        // `own x int8` value should default to an Int literal with type Int8 and value of 0
        let body = vec![var_decl("x", Type::Int8, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int16_zero() {
        // `own x int16` value should default to an Int literal with type Int16 and value of 0
        let body = vec![var_decl("x", Type::Int16, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int16(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_int32_zero() {
        // `own x int32` value should default to an Int literal with type Int32 and value of 0
        let body = vec![var_decl("x", Type::Int32, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int32(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int64_zero() {
        // `own x int64` value should default to an Int literal with type Int64 and value of 0
        let body = vec![var_decl("x", Type::Int64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int64(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int128_zero() {
        // `own x int128` value should default to an Int literal with type Int128 and value of 0
        let body = vec![var_decl("x", Type::Int128, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int128(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


//
    
    #[test]
    fn test_default_byte_zero() {
        // `own x byte` value should default to an Int literal with type Byte and value of 0
        let body = vec![var_decl("x", Type::Byte, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Byte(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_uint16_zero() {
        // `own x uint16` value should default to an Int literal with type Uint16 and value of 0
        let body = vec![var_decl("x", Type::Uint16, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint16(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_uint32_zero() {
        // `own x uint32` value should default to an Int literal with type Uint32 and value of 0
        let body = vec![var_decl("x", Type::Uint32, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint32(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_uint64_zero() {
        // `own x uint64` value should default to an Int literal with type Uint64 and value of 0
        let body = vec![var_decl("x", Type::Uint64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint64(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_uint128_zero() {
        // `own x uint128` value should default to an Int literal with type Uint128 and value of 0
        let body = vec![var_decl("x", Type::Uint128, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint128(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_usize_zero() {
        // `own x usize` value should default to an Int literal with type Usize and value of 0
        let body = vec![var_decl("x", Type::Usize, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Usize(0), .. })));
        
        } else { panic!("expected VarDecl") }    
    }







    #[test]
    fn test_default_bool_false() {
        // `own x bool` value should default to a Bool literal with value of false
        let body = vec![var_decl("flag", Type::Bool, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::BoolLiteral { value: false, .. })));
        } else { panic!("Expected VarDecl statement") }
    }


    #[test]
    fn test_default_string_empty() {
        // `own x bool` value should default to a Bool literal with value of false
        let body = vec![var_decl("str", Type::String, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(&v.value, Some(Expr::StringLiteral { value, .. }) if value == ""));
        } else { panic!("Expected VarDecl statement") }
    }




    #[test]
    fn test_default_float64_zero() {
        // `own x float64` value should default to a Float literal with value of 0.0
        let body = vec![var_decl("f", Type::Float64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::Float64Literal { value: 0.0f64, .. })));
        } else { panic!("Expected VarDecl statement") }
    }

    #[test]
    fn test_default_dynamic_array_is_empty() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("arr", Type::Array(Box::new(t.clone())), None)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert!(elements.is_empty());
                } else {
                    panic!("expected empty ArrayLiteral");
                }
            } else { panic!("Expected VarDecl statement") }
        }
    }


    #[test]
    fn test_default_fixed_array_errors() {
        for t in ALL_TYPES_NO_ARR {
            for i in 0..=100 {
                let body = vec![
                    var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), None)
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Default values are not allowed for fixed-size arrays"));
            }
        }
    }

    #[test]
    fn test_default_nested_array_is_empty() {
        for t in ALL_TYPES_NO_ARR {
            for i in 1..=200 {
                let mut nested_ty = Type::Array(Box::new(t.clone()));

                for _ in 0..=i {
                    nested_ty = Type::Array(Box::new(nested_ty));
                }

                let body = vec![var_decl("nested_array", nested_ty.clone(), None)];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();
                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.type_name, nested_ty);
                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert!(elements.is_empty());
                    } else {
                        panic!("expected empty ArrayLiteral");
                    }
                }
            }
                
        }
    }

    #[test]
    fn test_use_of_undeclared_variable_other_errors() {
        // Try referencing non-existent variable "y"
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("y")))]; // y not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }

    #[test]
    fn test_use_of_undeclared_variable_ourself_errors() {
        // Try referencing non-existent variable "x" aka ourselves.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("x")))]; // x not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }



}

