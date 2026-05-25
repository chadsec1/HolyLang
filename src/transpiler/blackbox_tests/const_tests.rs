use super::*;

#[cfg(test)]
mod const_in_globals_tests {
    use super::*;

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

            let rcode = transpile(ast).replace("\n", "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("const x: {} = {};", t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));
                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));

                let globals = vec![
                    const_define_globally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
            
                let ast = &AST { functions: vec![], globals};

                let rcode = transpile(ast).replace("\n", "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("const x: {} = {};", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }
}

#[cfg(test)]
mod const_in_void_func_tests {
    use super::*;

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

            let rcode = transpile(ast).replace("\n", "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", t_str, l_str));
        }
    }

    #[test]
    fn fixed_arrays() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0usize..200usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i));

                let l_arr = array_lit(vec![l.clone(); i], Some(fixed_arr_ty.clone()));
                let body = vec![
                    const_define_locally("x", fixed_arr_ty.clone(), l_arr.clone())
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let rcode = transpile(ast).replace("\n", "");

                let fixed_arr_ty_str = holy_type_to_rust_type_str(&fixed_arr_ty);
                let l_arr_str = holy_expr_to_rust_expr(&l_arr);

                assert_eq!(rcode, format!("fn foo() {{ const x: {} = {};}}", fixed_arr_ty_str,  l_arr_str));
            }
        }
    }
}
