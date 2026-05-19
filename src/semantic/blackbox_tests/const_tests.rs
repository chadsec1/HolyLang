use super::*;

#[cfg(test)]
mod const_tests {
    use super::*;

    #[test]
    fn const_literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ const_define_locally("x", t.clone(), l.clone()) ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }
   
    #[test]
    fn const_has_copy_call_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_lit = Expr::CopyCall { expr: Box::new(var_expr("x")), span: span() };

            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                const_define_locally("y", t.clone(), copy_lit)
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("expression cannot be evaluated at compile-time"));
        }
    }

    #[test]
    fn const_has_format_call_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![var_expr("x")],
                span: span(),
            };


            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                const_define_locally("y", Type::String, fmt)
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("expression cannot be evaluated at compile-time"));
        }
    }

    #[test]
    fn const_unary_negate_on_signed_consts() {
        let signed_literals = get_all_signed_literals_no_arr();

        for (l, t) in signed_literals.iter().zip(ALL_SIGNED_TYPES_NO_ARR.iter()) {
            let unary = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(var_expr("x")),
                span: span(),
            };

            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                const_define_locally("y", t.clone(), unary.clone()) 
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
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::Const(c) = &ast.functions[0].body[1] {
                assert_eq!(c.name, "y");
                assert_eq!(c.type_name, t.clone());
                assert!(!matches!(c.value, Expr::UnaryOp { .. }))
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }

    #[test]
    fn const_unary_negate_on_signed_non_const_errors() {
        let signed_literals = get_all_signed_literals_no_arr();

        for (l, t) in signed_literals.iter().zip(ALL_SIGNED_TYPES_NO_ARR.iter()) {
            let unary = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(var_expr("x")),
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                const_define_locally("y", t.clone(), unary.clone()) 
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot use variable `x` in a constant value expression"));
        }
    }
    
    #[test]
    fn const_unary_bitwise_not_on_int_consts() {
        let int_literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in int_literals.iter().zip(ALL_INT_TYPES_NO_ARR.iter()) {
            let unary = Expr::UnaryOp {
                op: UnaryOpKind::BitwiseNot,
                expr: Box::new(var_expr("x")),
                span: span(),
            };

            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                const_define_locally("y", t.clone(), unary.clone()) 
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
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::Const(c) = &ast.functions[0].body[1] {
                assert_eq!(c.name, "y");
                assert_eq!(c.type_name, t.clone());
                assert!(!matches!(c.value, Expr::UnaryOp { .. }))
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }


    #[test]
    fn const_unary_logical_not_on_binop_consts() {
        let boolean_conditions = get_many_boolean_conditions_no_dyn_arr();

        for bl in boolean_conditions {
            let unary = Expr::UnaryOp {
                op: UnaryOpKind::Not,
                expr: Box::new(var_expr("x")),
                span: span(),
            };

            let body = vec![
                const_define_locally("x", Type::Bool, bl.clone()),
                const_define_locally("y", Type::Bool, unary) 
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 2);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, Type::Bool);
                assert!(matches!(c.value, Expr::BoolLiteral { .. }))
            } else { panic!("expected Const, got {:?}", ast); }

            if let Stmt::Const(c) = &ast.functions[0].body[1] {
                assert_eq!(c.name, "y");
                assert_eq!(c.type_name, Type::Bool);
                assert!(!matches!(c.value, Expr::UnaryOp { .. }))
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }
    


    #[test]
    fn const_all_arth_binop_on_literals() {
        let numeric_literals = get_all_literals_no_arr_str_bool();

        for (l, t) in numeric_literals.iter().zip(ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING.iter())
        {
            for b in ALL_BIN_OP_KIND_ARTH {
                if ALL_BIN_OP_KIND_BIT_ARTH.contains(&b) && !t.is_integer_type() {
                    continue
                }

                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    right: Box::new(l.clone()),
                    op: b.clone(),
                    span: span(),
                };

                let body = vec![
                    const_define_locally("x", t.clone(), bin),
                ];

                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();
                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.functions[0].body.len(), 1);
                assert_eq!(ast.globals.len(), 0);

                if let Stmt::Const(c) = &ast.functions[0].body[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                    if t.is_integer_type() {
                        assert!(matches!(c.value, Expr::IntLiteral { .. }))
                    } else {
                        assert!(matches!(c.value, Expr::Float64Literal { .. }))
                    }
                } else { panic!("expected Const, got {:?}", ast); }
            }
        }
    }





    #[test]
    fn const_as_arg_to_fun() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),

                Stmt::Expr(call_expr("foo", vec![ var_expr("x") ]))
            ];
            let main = void_func("main", vec![], body);
            let foo = void_func("foo", vec![param("h", t.clone())], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            check_semantics(&mut ast).unwrap();
            assert_eq!(ast.functions.len(), 2);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }




    #[test]
    fn type_mismatch_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for t in ALL_INT_TYPES_NO_ARR {
            for l in &literals_no_ints {
                let body = vec![const_define_locally("x", t.clone(), l.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }

    #[test]
    fn fixed_array_with_literal_size() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };

                let body = vec![const_define_locally("x", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit.clone())];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                check_semantics(&mut ast).unwrap();
                
                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.globals.len(), 0);
            }
        }
    }

    #[test]
    fn array_access_on_fixed_array_with_literal_size() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 1..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };
                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("arr")),
                    index: Box::new(usize_lit(i - 1)),
                    span: span(),
                };

                let body = vec![
                    const_define_locally("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                    const_define_locally("x", t.clone(), access)
                ];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                check_semantics(&mut ast).unwrap();
                
                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.globals.len(), 0);
            }
        }
    }

    #[test]
    fn array_access_on_non_const_fixed_array_with_literal_size_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 1..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };
                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("arr")),
                    index: Box::new(usize_lit(i - 1)),
                    span: span(),
                };

                let body = vec![
                    var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                    const_define_locally("x", t.clone(), access)
                ];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("You cannot use variable `arr` in a constant value expression"));
            }
        }
    }

    #[test]
    fn array_access_using_non_const_index_on_fixed_array_with_literal_size_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 1..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };
                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("arr")),
                    index: Box::new(var_expr("h")),
                    span: span(),
                };

                let body = vec![
                    var_decl("h", Type::Usize, usize_lit(i - 1)),
                    const_define_locally("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                    const_define_locally("x", t.clone(), access)
                ];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);
                
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("You cannot use variable `h` in a constant value expression"));
            }
        }
    }

    #[test]
    fn array_out_of_bounds_access_on_fixed_array_with_literal_size_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 1..=100 {
                let elements = vec![l.clone(); i];

                for i2 in 1..=50 {
                    let arr_lit = Expr::ArrayLiteral { elements: elements.clone(), span: span() };
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(usize_lit(i + i2)),
                        span: span(),
                    };

                    let body = vec![
                        const_define_locally("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                        const_define_locally("x", t.clone(), access)
                    ];
                 
                    let func = void_func("foo", vec![], body);

                    let result = std::panic::catch_unwind(|| { 
                        let mut ast = ast_one(func.clone());
                        check_semantics(&mut ast).unwrap();
                    });

                    assert!(result.is_err(), "Expected panic for: {:?}", func);
                }
            }
        }
    }


    
    #[test]
    fn dynamic_arrays_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };

                let body = vec![const_define_locally("x", Type::Array(Box::new(t.clone())), arr_lit.clone())];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                
                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Dynamic arrays cannot be evaluated at compile time"));
            }
        }
    }

    #[test]
    fn dynamic_array_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        // We use no_ints here because if we included int literals, they would get inferred to
        // correct type if they fit, and since functions return 1 for all ints, they would always
        // fit.
        for ((l1, t1), l2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(literals_no_ints.iter())
        {
            for i in 0..=100 {
                let mut elements = vec![l1.clone(); i];

                elements.push(l2.clone());
                
                let arr_lit = Expr::ArrayLiteral { elements: elements.clone(), span: span(), };

                let body = vec![const_define_locally("x", Type::Array(Box::new(t1.clone())), arr_lit.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());

                let assert_cond = result.unwrap_err().to_string();
                let assert_cond = assert_cond.contains("Array element type mismatch:") | 
                                    assert_cond.contains("Type mismatch assigning to `x`");

                assert!(assert_cond);
            }       
        }
    }


    #[test]
    fn test_define_const_name_taken_by_func_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let main = void_func("main", vec![], vec![
                const_define_locally("foo", t.clone(), l.clone())
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("`foo` is already taken by a function."));
        }
    }


    #[test]
    fn test_define_const_name_taken_by_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let main = void_func("main", vec![], vec![
                var_decl("foo", t.clone(), l.clone()),
                const_define_locally("foo", t.clone(), l.clone())
            ]);

            let mut ast = AST { functions: vec![main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("it is already declared"));
        }
    }

    #[test]
    fn test_define_const_assign_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()),
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("You cannot assign to constant"));
        }
    }
}


#[cfg(test)]
mod const_integer_overflow_tests {
    use super::*;

    #[test]
    fn const_binop_int8_addition_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(125)),
            right: Box::new(int8_lit(3)),
            op: BinOpKind::Add,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int8`"));
    }

    #[test]
    fn const_binop_int8_subtract_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(-125)),
            right: Box::new(int8_lit(10)),
            op: BinOpKind::Subtract,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int8`"));
    }

    #[test]
    fn const_binop_int8_multiplication_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(125)),
            right: Box::new(int8_lit(2)),
            op: BinOpKind::Multiply,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int8`"));
    }

    #[test]
    fn const_binop_int8_divide_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(i8::MIN)),
            right: Box::new(int8_lit(-1)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int8`"));
    }

    #[test]
    fn const_binop_int8_divide_by_zero_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(1)),
            right: Box::new(int8_lit(0)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int8_bitshift_left_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(1)),
            right: Box::new(int8_lit(-1)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int8_bitshift_left_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(1)),
            right: Box::new(int8_lit(i8::BITS as i8)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot exceed "));
    }

    #[test]
    fn const_binop_int8_bitshift_right_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(1)),
            right: Box::new(int8_lit(-1)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int8_bitshift_right_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int8_lit(1)),
            right: Box::new(int8_lit(i8::BITS as i8)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int8, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot exceed "));
    }


    // int16
    //
    #[test]
    fn const_binop_int16_addition_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(i16::MAX - 2)),
            right: Box::new(int16_lit(3)),
            op: BinOpKind::Add,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int16`"));
    }

    #[test]
    fn const_binop_int16_subtract_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(i16::MIN + 2)),
            right: Box::new(int16_lit(10)),
            op: BinOpKind::Subtract,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int16`"));
    }

    #[test]
    fn const_binop_int16_multiplication_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(i16::MAX - 2)),
            right: Box::new(int16_lit(2)),
            op: BinOpKind::Multiply,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int16`"));
    }

    #[test]
    fn const_binop_int16_divide_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(i16::MIN)),
            right: Box::new(int16_lit(-1)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int16`"));
    }

    #[test]
    fn const_binop_int16_divide_by_zero_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(1)),
            right: Box::new(int16_lit(0)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int16_bitshift_left_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(1)),
            right: Box::new(int16_lit(-1)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int16_bitshift_left_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(1)),
            right: Box::new(int16_lit(i16::BITS as i16)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot exceed "));
    }

    #[test]
    fn const_binop_int16_bitshift_right_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(1)),
            right: Box::new(int16_lit(-1)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int16_bitshift_right_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int16_lit(1)),
            right: Box::new(int16_lit(i16::BITS as i16)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int16, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot exceed "));
    }

    // int32
    //
    #[test]
    fn const_binop_int32_addition_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(i32::MAX - 2)),
            right: Box::new(int32_lit(3)),
            op: BinOpKind::Add,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int32`"));
    }

    #[test]
    fn const_binop_int32_subtract_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(i32::MIN + 2)),
            right: Box::new(int32_lit(10)),
            op: BinOpKind::Subtract,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int32`"));
    }

    #[test]
    fn const_binop_int32_multiplication_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(i32::MAX - 2)),
            right: Box::new(int32_lit(2)),
            op: BinOpKind::Multiply,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int32`"));
    }

    #[test]
    fn const_binop_int32_divide_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(i32::MIN)),
            right: Box::new(int32_lit(-1)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int32`"));
    }

    #[test]
    fn const_binop_int32_divide_by_zero_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(int32_lit(0)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int32_bitshift_left_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(int32_lit(-1)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int32_bitshift_left_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(int32_lit(i32::BITS as i32)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot exceed "));
    }

    #[test]
    fn const_binop_int32_bitshift_right_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(int32_lit(-1)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int32_bitshift_right_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(int32_lit(i32::BITS as i32)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int32, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot exceed "));
    }



    // int64
    //
    #[test]
    fn const_binop_int64_addition_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(i64::MAX - 2)),
            right: Box::new(int64_lit(3)),
            op: BinOpKind::Add,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int64`"));
    }

    #[test]
    fn const_binop_int64_subtract_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(i64::MIN + 2)),
            right: Box::new(int64_lit(10)),
            op: BinOpKind::Subtract,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int64`"));
    }

    #[test]
    fn const_binop_int64_multiplication_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(i64::MAX - 2)),
            right: Box::new(int64_lit(2)),
            op: BinOpKind::Multiply,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int64`"));
    }

    #[test]
    fn const_binop_int64_divide_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(i64::MIN)),
            right: Box::new(int64_lit(-1)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range for type `int64`"));
    }

    #[test]
    fn const_binop_int64_divide_by_zero_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(1)),
            right: Box::new(int64_lit(0)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int64_bitshift_left_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(1)),
            right: Box::new(int64_lit(-1)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int64_bitshift_left_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(1)),
            right: Box::new(int64_lit(i64::BITS as i64)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot exceed "));
    }

    #[test]
    fn const_binop_int64_bitshift_right_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(1)),
            right: Box::new(int64_lit(-1)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int64_bitshift_right_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int64_lit(1)),
            right: Box::new(int64_lit(i64::BITS as i64)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int64, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot exceed "));
    }




    // int128
    //
    #[test]
    fn const_binop_int128_addition_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(i128::MAX - 2)),
            right: Box::new(int128_lit(3)),
            op: BinOpKind::Add,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic addition result would cause an integer overflow."));
    }

    #[test]
    fn const_binop_int128_subtract_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(i128::MIN + 2)),
            right: Box::new(int128_lit(10)),
            op: BinOpKind::Subtract,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic subtraction result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int128_multiplication_overflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(i128::MAX - 2)),
            right: Box::new(int128_lit(2)),
            op: BinOpKind::Multiply,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic multiplication result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int128_divide_underflow_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(i128::MIN)),
            right: Box::new(int128_lit(-1)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int128_divide_by_zero_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(1)),
            right: Box::new(int128_lit(0)),
            op: BinOpKind::Divide,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant arithemtic division result would cause an integer overflow"));
    }

    #[test]
    fn const_binop_int128_bitshift_left_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(1)),
            right: Box::new(int128_lit(-1)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int128_bitshift_left_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(1)),
            right: Box::new(int128_lit(i128::BITS as i128)),
            op: BinOpKind::BitwiseShiftLeft,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the left's right-side value cannot exceed "));
    }

    #[test]
    fn const_binop_int128_bitshift_right_negative_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(1)),
            right: Box::new(int128_lit(-1)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot be negative"));
    }

    #[test]
    fn const_binop_int128_bitshift_right_exceeds_bit_width_errors() {
        let bin = Expr::BinOp {
            left: Box::new(int128_lit(1)),
            right: Box::new(int128_lit(i128::BITS as i128)),
            op: BinOpKind::BitwiseShiftRight,
            span: span(),
        };

        let body = vec![ const_define_locally("x", Type::Int128, bin) ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Constant bitwise shift to the right's right-side value cannot exceed "));
    }







}
