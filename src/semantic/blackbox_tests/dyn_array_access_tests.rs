use super::*;

#[cfg(test)]
mod dyn_arrays_access_tests {
    use super::*;

    // array invalid access patterns errors checks
    #[test]
    fn test_dynamic_array_out_of_bounds_single_access_errors() {
        // own arr t[] = [l, l, l]
        // own x t = arr[i]  (out of bounds)
        // i starts from 3 up to 10k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..=10000 {
                let arr_lit = array_lit(vec![l.clone(), l.clone(), l.clone()], Some(t.clone()));

                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("arr")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit),
                    var_decl("x", t.clone(), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out-of-bounds"));
            }
            
        }
    }

    #[test]
    fn test_dynamic_array_access_not_usize_var_errors() {
        let literals = get_all_literals_no_arr_no_usize();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            let arr_lit = array_lit(vec![l.clone(), l.clone(), l.clone()], Some(t.clone()));

            let access = Expr::ArrayAccess {
                array: Box::new(var_expr("arr")),
                index: Box::new(var_expr("e")),
                span: span(),
            };

            let body = vec![
                var_decl("e", t.clone(), l.clone()),
                var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit),
                var_decl("x", t.clone(), access),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected array index to be of type"));
        }
    }



    #[test]
    fn test_dynamic_array_valid_access_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                        var_decl("x", t.clone(), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }
            }       
        }
    }




}
