use super::*;

#[cfg(test)]
mod var_decl_in_void_func_tests {
    use super::*;

    #[test]
    fn inited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = holy_type_to_rust_type_str(&t);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![var_decl("x", t.clone(), bin.clone(), true)];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", t_str, bin_str));
            }
        }
    }
    
    #[test]
    fn uninited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = holy_type_to_rust_type_str(&t);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![var_decl("x", t.clone(), bin.clone(), false)];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", t_str, bin_str));
            }
        }
    }

    #[test]
    fn inited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = holy_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t.clone(), bl.clone(), true)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = holy_expr_to_rust_expr(&bl);

            assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", t_str, bl_str));
        }
    }

    #[test]
    fn uninited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = holy_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t.clone(), bl.clone(), false)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = holy_expr_to_rust_expr(&bl);

            assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", t_str, bl_str));
        }
    }

    #[test]
    fn inited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone(), true)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", t_str, l_str));
        }
    }

    #[test]
    fn uninited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone(), false)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", t_str, l_str));
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }
}

#[cfg(test)]
mod var_decl_in_void_func_with_params_tests {
    use super::*;

    #[test]
    fn inited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), true)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t1.clone()), param("b", t2.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", t1_str, t2_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn uninited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), false)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = void_func("foo", vec![param("a", t1.clone()), param("b", t2.clone())], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", t1_str, t2_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn inited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), true)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", t2_str, t2_str, t1_str, bl_str));
            }
        }
    }


    #[test]
    fn uninited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), false)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, bl_str));
            }
        }
    }

    #[test]
    fn inited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), true)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", t2_str, t2_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn uninited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), false)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = void_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                
                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                
                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

}


// Same tests as above, except its now in a returning single type function.
//
#[cfg(test)]
mod var_decl_in_single_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn inited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), true)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn uninited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), false)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn inited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), true)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, bl_str));
            }
        }
    }


    #[test]
    fn uninited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), false)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, bl_str));
            }
        }
    }



    #[test]
    fn inited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), true)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn uninited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), false)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone()], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");
                
                assert_eq!(
                    rcode, 
                    format!("fn foo(a: {}, b: {}) -> {} {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str)
                );
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");
                
                assert_eq!(
                    rcode, 
                    format!("fn foo(a: {}, b: {}) -> {} {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str)
                );
            }
        }
    }



}




// Same tests as above, except its now in a multi returning function.
//
#[cfg(test)]
mod var_decl_in_multi_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn inited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), true)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn uninited_var_all_bin_op() {
        let literals = get_all_literals();

        for (l, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    var_decl("x", t1.clone(), bin.clone(), false)
                ];

                for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                    let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..].replace('\n', "");

                    let t2_str = holy_type_to_rust_type_str(&t2);
                    let bin_str = holy_expr_to_rust_expr(&bin);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, bin_str));
                }
            }
        }
    }

    #[test]
    fn inited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), true)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, bl_str));
            }
        }
    }


    #[test]
    fn uninited_var_bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t1 = Type::Bool;
        let t1_str = holy_type_to_rust_type_str(&t1);
        
        for bl in boolean_conds {
            let body = vec![
                var_decl("x", t1.clone(), bl.clone(), false)
            ];

            let bl_str = holy_expr_to_rust_expr(&bl);

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, bl_str));
            }
        }
    }

    #[test]
    fn inited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), true)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn uninited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let l1_str = holy_expr_to_rust_expr(&l1);
            let t1_str = holy_type_to_rust_type_str(&t1);

            let body = vec![
                var_decl("x", t1.clone(), l1.clone(), false)
            ];

            for t2 in ALL_TYPES_WITH_DYN_ARR.iter() {
                let func = returning_func("foo", vec![param("a", t2.clone()), param("b", t2.clone())], vec![t1.clone(); 3], body.clone());
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, t1_str, t1_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone(); 3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone(); 3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");
                
                assert_eq!(
                    rcode, 
                    format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str)
                );
            }
        }
    }


    #[test]
    fn inited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone(); 3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone(); 3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");
                
                assert_eq!(
                    rcode, 
                    format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str)
                );
            }
        }
    }

}



