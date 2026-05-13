use super::*;

#[cfg(test)]
mod fixed_array_slicing_tests {
    use super::*;

    #[test]
    fn test_fixed_array_slicing_out_of_bounds_errors() {
        // own arr t[3] = [l, l, l]
        // own x t = arr[0:i]  (out of bounds)
        // i starts from 3 up to 10k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..=10000 {
                let arr_lit = Expr::ArrayLiteral {
                    elements: vec![l.clone(), l.clone(), l.clone()],
                    span: span(),
                };

                let access = Expr::ArraySlicing {
                    array: Box::new(var_expr("arr")),
                    range: ArraySliceRange::FromTo(Box::new(usize_lit(0)), Box::new(usize_lit(i))),
                    span: span(),
                };
                let body = vec![
                    var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(3)), arr_lit.clone()),
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
    fn test_fixed_array_slicing_both_ends() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArraySlicing {
                        array: Box::new(var_expr("arr")),
                        range: ArraySliceRange::FromTo(Box::new(usize_lit(1)), Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                        var_decl("x", Type::Array(Box::new(t.clone())), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }       
            }
        }
    }


    // Same as above test, but this makes start and end variables instead of literals
    #[test]
    fn test_fixed_array_slicing_both_ends_vars_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArraySlicing {
                        array: Box::new(var_expr("arr")),
                        range: ArraySliceRange::FromTo(Box::new(var_expr("e")), Box::new(var_expr("h"))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", Type::Usize, usize_lit(1)),
                        var_decl("h", Type::Usize, usize_lit(i2+1)),
                        var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                        var_decl("x", Type::Array(Box::new(t.clone())), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }       
            }
        }
    }


    // Same as above test, but this makes start usize var, but end is not usize
    // and vice versa.
    #[test]
    fn test_fixed_array_slicing_both_ends_vars_start_not_usize_errors() {
        let literals_no_usize = get_all_literals_no_arr_no_usize();
        
        for (l, t) in literals_no_usize.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArraySlicing {
                        array: Box::new(var_expr("arr")),
                        range: ArraySliceRange::FromTo(Box::new(var_expr("e")), Box::new(var_expr("h"))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", t.clone(), l.clone()),
                        var_decl("h", Type::Usize, usize_lit(i2+1)),
                        var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                        var_decl("x", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected start index to be of type `usize` for array"));

                }       
            }
        }

        // Same as above, but a little weaker because we can't do i2+1 for l.. its just always 1.

        for (l, t) in literals_no_usize.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                let access = Expr::ArraySlicing {
                    array: Box::new(var_expr("arr")),
                    range: ArraySliceRange::FromTo(Box::new(var_expr("e")), Box::new(var_expr("h"))),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", Type::Usize, usize_lit(1)),
                    var_decl("h", t.clone(), l.clone()),
                    var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                    var_decl("x", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected end index to be of type `usize` for array"));
            }
        }
    }


}
