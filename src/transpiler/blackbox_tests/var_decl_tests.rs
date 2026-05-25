use super::*;

#[cfg(test)]
mod var_decl_in_void_func_tests {
    use super::*;

    #[test]
    fn inited_var_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone(), true)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let rcode = transpile(ast).replace("\n", "");

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

            let rcode = transpile(ast).replace("\n", "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", t_str, l_str));
        }
    }

    #[test]
    fn inited_var_fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), true)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let rcode = transpile(ast).replace("\n", "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ let x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    var_decl("x", fixed_arr_ty.clone(), l_arr.clone(), false)
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let rcode = transpile(ast).replace("\n", "");

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

                let rcode = transpile(ast).replace("\n", "");

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

                let rcode = transpile(ast).replace("\n", "");

                let t2_str = holy_type_to_rust_type_str(&t2);

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", t2_str, t2_str, t1_str, l1_str));
            }
        }
    }

    #[test]
    fn inited_var_fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
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

                let rcode = transpile(ast).replace("\n", "");

                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

    #[test]
    fn uninited_var_fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
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

                let rcode = transpile(ast).replace("\n", "");

                
                assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ let mut x: {} = {};}}", fixed_arr_ty_str, fixed_arr_ty_str, fixed_arr_ty_str,  l_arr_str));
            }
        }
    }

}



