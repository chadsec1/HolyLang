use super::*;

#[cfg(test)]
mod const_in_globals_tests {
    use super::*;

    #[test]
    fn bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let globals = vec![
                const_define_globally("x", t.clone(), bl.clone())
            ];

            let ast = &AST { functions: vec![], globals};

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = gold_expr_to_rust_expr(&bl);

            assert_eq!(rcode, format!("const x: {} = {};", t_str, bl_str));
        }
    }

    #[test]
    fn literals_and_dynamic_arrays() {
        // Dynamic arrays are illegal for consts, but transpiler shouldnt care. its not its
        // responsiblity
        //
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let globals = vec![
                const_define_globally("x", t.clone(), l.clone())
            ];

            let ast = &AST { functions: vec![], globals};

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("const x: {} = {};", t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let globals = vec![
                    const_define_globally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
            
                let ast = &AST { functions: vec![], globals};

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("const x: {} = {};", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let globals = vec![
                    const_define_globally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
            
                let ast = &AST { functions: vec![], globals};

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("const x: {} = {};", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }
}

#[cfg(test)]
mod const_in_void_func_tests {
    use super::*;

    #[test]
    fn bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                const_define_locally("x", t.clone(), bl.clone())
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = gold_expr_to_rust_expr(&bl);

            assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", t_str, bl_str));
        }
    }


    #[test]
    fn literals_and_dynamic_arrays() {
        // Dynamic arrays are illegal for consts, but transpiler shouldnt care. its not its
        // responsiblity
        //
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }



}

#[cfg(test)]
mod const_in_void_func_with_params_tests {
    use super::*;

    #[test]
    fn bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                const_define_locally("x", t.clone(), bl.clone())
            ];
            let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = gold_expr_to_rust_expr(&bl);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ const x: {} = {};}}", t_str, t_str, t_str, bl_str));
        }
    }

    #[test]
    fn literals_and_dynamic_arrays() {
        // Dynamic arrays are illegal for consts, but transpiler shouldnt care. its not its
        // responsiblity
        //
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone())
            ];
            let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ const x: {} = {};}}", t_str, t_str, t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ const x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }

    #[test]
    fn fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = void_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ const x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }



}

#[cfg(test)]
mod const_in_single_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                const_define_locally("x", t.clone(), bl.clone())
            ];
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = gold_expr_to_rust_expr(&bl);
            
            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ const x: {} = {};}}", t_str, t_str, t_str, t_str, bl_str));
        }
    }


    #[test]
    fn literals_and_dynamic_arrays() {
        // Dynamic arrays are illegal for consts, but transpiler shouldnt care. its not its
        // responsiblity
        //
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone())
            ];
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ const x: {} = {};}}", t_str, t_str, t_str, t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ const x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }


    #[test]
    fn fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone()], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ const x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str));
            }
        }
    }
}

#[cfg(test)]
mod const_in_multi_returning_func_with_params_tests {
    use super::*;

    #[test]
    fn bincop_condition_bool() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let body = vec![
                const_define_locally("x", t.clone(), bl.clone())
            ];
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone(); 3], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let bl_str = gold_expr_to_rust_expr(&bl);
            
            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ const x: {} = {};}}", t_str, t_str, t_str, t_str, t_str, t_str, bl_str));
        }
    }



    #[test]
    fn literals_and_dynamic_arrays() {
        // Dynamic arrays are illegal for consts, but transpiler shouldnt care. its not its
        // responsiblity
        //
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone())
            ];
            let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone(); 3], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ const x: {} = {};}}", t_str, t_str, t_str, t_str, t_str, t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays_with_literal_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone();3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(
                    rcode, 
                    format!(
                        "fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ const x: {} = {};}}", 
                        fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str
                    )
                );
            }
        }
    }

    #[test]
    fn fixed_arrays_with_const_size() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("s".to_string()));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = returning_func("foo", vec![param("a", fixed_arr_ty.clone()), param("b", fixed_arr_ty.clone())], vec![fixed_arr_ty.clone();3], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let fixed_arr_ty_str = gold_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = gold_expr_to_rust_expr(&l_arr);

                assert_eq!(
                    rcode,
                    format!(
                        "fn foo(a: {}, b: {}) -> ({}, {}, {}) {{ const x: {} = {};}}", 
                        fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str, l_arr_str
                    )
                );
            }
        }
    }
}
