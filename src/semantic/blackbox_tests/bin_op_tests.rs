use super::*;

#[cfg(test)]
mod bin_op_tests {
    use super::*;

    #[test]
    fn test_string_binop_arth_errors() {
        for b in ALL_BIN_OP_KIND_ARTH {
            // Strings may not be ever wrapped in ANY BinOpKind (except *some* comparison operators like == and !=), we use format() instead.
            let bin = Expr::BinOp {
                left: Box::new(str_lit("hello")),
                op: b,
                right: Box::new(str_lit("world")),
                span: span(),
            };
            let body = vec![var_decl("s", Type::String, Some(bin))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());

            let assert_cond = result.unwrap_err().to_string();
            let assert_cond = assert_cond.contains("Expected numeric types in binary arithmetic operation") |
                                assert_cond.contains("You cannot perform bitwise operations on non-integer types");

            assert!(assert_cond);
        }
    }

    // (includes strings)
    #[test]
    fn test_all_literals_binop_comp_eq_passes() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Bool, Some(bin.clone()))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.type_name, Type::Bool);
                    assert_eq!(v.value, Some(bin));
                } else { panic!("expected VarDecl");}
            }
        }
    }



    // Same as above test, but its mixed types
    #[test]
    fn test_all_literals_binop_comp_eq_errors() {
        let literals = get_all_literals_no_arr_few_ints();
        let literals_scattered = get_all_literals_no_arr_few_ints_scattered();


        for ((l1, t), l2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
        {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin = Expr::BinOp {
                    left: Box::new(l1.clone()),
                    op: b,
                    right: Box::new(l2.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
            }
        }
    }


    #[test]
    fn test_all_literals_binop_comp_arth_errors() {
        let literals_str_bool = [str_lit("hi"), bool_lit(false)];
        let literals_types = [Type::String, Type::Bool];

        for (l, t) in literals_str_bool.iter().zip(literals_types.iter()) {
            for b in ALL_BIN_OP_KIND_COMP_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("You cannot perform arithmetic comparison on non-numeric types"));
            }
        }
    }


    #[test]
    fn test_all_literals_binop_comp_arth_mixed_errors() {
        let literals = get_all_literals_no_arr_few_ints();
        let literals_scattered = get_all_literals_no_arr_few_ints_scattered();


        for ((l1, t), l2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
        {
            for b in ALL_BIN_OP_KIND_COMP_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l1.clone()),
                    op: b,
                    right: Box::new(l2.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Type mismatch in binary comparison operation"));
            }
        }
    }

    #[test]
    fn test_all_literals_binop_comp_eq_for_binop_logical_passes() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin_bool = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                for bl in ALL_BIN_OP_KIND_LOGIC {
                    let bin = Expr::BinOp {
                        left: Box::new(bin_bool.clone()),
                        op: bl,
                        right: Box::new(bin_bool.clone()),
                        span: span(),
                    };


                    let body = vec![var_decl("s", Type::Bool, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);

                    assert!(result.is_ok());
                }
            }
        }
    }


    #[test]
    fn test_non_boolean_binop_logical_passes() {
        let bools = [bool_lit(false), bool_lit(true)];

        for bv in &bools {
            for b in ALL_BIN_OP_KIND_LOGIC {
                let bin = Expr::BinOp {
                    left: Box::new(bv.clone()),
                    op: b,
                    right: Box::new(bv.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Bool, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());
            }
        }
    }


    #[test]
    fn test_binop_copy_var_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND {
                let copy_var = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };

                let bin = Expr::BinOp {
                    left: Box::new(copy_var),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![
                    var_decl("a", t.clone(), Some(l.clone())),
                    var_decl("s", t.clone(), Some(bin))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Copying is not needed for variables in binary operations"));
            }
        }
    }



    // Non boolean left or right with logical "AND" or "OR" should error
    #[test]
    fn test_non_boolean_binop_logical_errors() {
        let literals = get_all_literals_no_arr_bool();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_BOOL.iter()) {
            for b in ALL_BIN_OP_KIND_LOGIC {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
            }
        }

        let bool_values = [false, true];
        // Same test, but left is a bool (false and true)
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_BOOL.iter()) {
            for b in ALL_BIN_OP_KIND_LOGIC {
                for bv in bool_values {
                    let bin = Expr::BinOp {
                        left: Box::new(bool_lit(bv)),
                        op: b.clone(),
                        right: Box::new(l.clone()),
                        span: span(),
                    };
                    let body = vec![var_decl("s", t.clone(), Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
                }
            }
        }


        // Same test, but right is a bool (false and true)
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_BOOL.iter()) {
            for b in ALL_BIN_OP_KIND_LOGIC {
                for bv in bool_values {
                    let bin = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b.clone(),
                        right: Box::new(bool_lit(bv)),
                        span: span(),
                    };
                    let body = vec![var_decl("s", t.clone(), Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
                }
            }
        }
    }


    #[test]
    fn test_integers_binop_arth_passes() {
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());
            }
        }
    }


    #[test]
    fn test_floating_binop_real_arth_passes() {
        let float64_lits = [
            float64_lit(1.0),
            float64_lit(1e12)
        ];

        let t = Type::Float64;

        for l in float64_lits {
            for b in ALL_BIN_OP_KIND_REAL_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b.clone(),
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }
    }


    #[test]
    fn test_non_int_binop_bit_arth_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints(); 

        for (l, t) in literals_no_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_BIT_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
            }
        }
    }

    
    // binary operation type mismatch 

    #[test]
    fn test_binop_int_non_int_mixed_types_errors() {
        // int literals are not allowed to mix with literals of non-int type
        //
        
        let int_literals = get_all_literals_no_arr_str_bool_float();

        let non_int_literals = get_all_literals_no_arr_no_ints();

        for int in &int_literals {
            for non_int in &non_int_literals {
                for b in ALL_BIN_OP_KIND_ARTH {
                    for t in ALL_TYPES_NO_ARR {
                        let bin = Expr::BinOp {
                            left: Box::new(int.clone()),
                            right: Box::new(non_int.clone()),
                            op: b.clone(),
                            span: span(),
                        };
                        let body = vec![var_decl("x", t.clone(), Some(bin))];
                        let func = void_func("foo", vec![], body);
                        let mut ast = ast_one(func);
                        let result = check_semantics(&mut ast);
                        assert!(result.is_err());
                        let assert_condition = result.unwrap_err().to_string();
                        let assert_condition = assert_condition.contains("Type mismatch in binary") 
                                               || assert_condition.contains("You cannot perform arithmetic");

                        assert!(assert_condition);
                    }
                }
            }
        }
        
        // Same as above, but this switches non_int to left, and int to right
        for int in &int_literals {
            for non_int in &non_int_literals {
                for b in ALL_BIN_OP_KIND_ARTH {
                    for t in ALL_TYPES_NO_ARR {
                        let bin = Expr::BinOp {
                            left: Box::new(non_int.clone()),
                            right: Box::new(int.clone()),
                            op: b.clone(),
                            span: span(),
                        };
                        let body = vec![var_decl("x", t.clone(), Some(bin))];
                        let func = void_func("foo", vec![], body);
                        let mut ast = ast_one(func);
                        let result = check_semantics(&mut ast);
                        assert!(result.is_err());
                        let assert_condition = result.unwrap_err().to_string();
                        let assert_condition = assert_condition.contains("Type mismatch in binary") 
                                               || assert_condition.contains("You cannot perform arithmetic");

                        assert!(assert_condition);
                    }
                }
            }
        }
    }


    // Mixing int32, int16, float64, etc should always return an error.
    //
    #[test]
    fn test_binop_arth_mixed_types_errors() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        let literals_ints_floats_scat = get_all_literals_no_arr_str_bool_scattered();

        for (((l1, t1), l2), t2) in literals_ints_floats.iter()
            .zip(ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING.iter())
            .zip(literals_ints_floats_scat.iter())
            .zip(ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING_SCATTERED)
        {

            for b in ALL_BIN_OP_KIND_ARTH {
                // We declare variables here, because had we used literals, it would get inferred
                // in the binary operation expression
                //
                let bin = Expr::BinOp {
                    left: Box::new(var_expr("x")),
                    right: Box::new(var_expr("y")),
                    op: b,
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), Some(l1.clone())),
                    var_decl("y", t2.clone(), Some(l2.clone())),
                    var_decl("z", t1.clone(), Some(bin))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());

                let assert_condition = result.unwrap_err().to_string();
                let assert_condition = assert_condition.contains("Type mismatch in binary arithmetic operation") |
                                        assert_condition.contains("Type mismatch in binary bitwise operation");

                assert!(assert_condition);
            }
        }


        // Same, but z is t2, instead of t1
        for (((l1, t1), l2), t2) in literals_ints_floats.iter()
            .zip(ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING.iter())
            .zip(literals_ints_floats_scat.iter())
            .zip(ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING_SCATTERED)
        {

            for b in ALL_BIN_OP_KIND_ARTH {
                // We declare variables here, because had we used literals, it would get inferred
                // in the binary operation expression
                //
                let bin = Expr::BinOp {
                    left: Box::new(var_expr("x")),
                    right: Box::new(var_expr("y")),
                    op: b,
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), Some(l1.clone())),
                    var_decl("y", t2.clone(), Some(l2.clone())),
                    var_decl("z", t2.clone(), Some(bin))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());

                let assert_condition = result.unwrap_err().to_string();
                let assert_condition = assert_condition.contains("Type mismatch in binary arithmetic operation") |
                                        assert_condition.contains("Type mismatch in binary bitwise operation");

                assert!(assert_condition);
            }
        }
    }


}
