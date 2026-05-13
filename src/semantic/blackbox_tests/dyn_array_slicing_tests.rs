use super::*;

#[cfg(test)]
mod dyn_arrays_slicing_tests {
    use super::*;

    #[test]
    fn test_dyn_array_slicing_out_of_bounds_errors() {
        // own arr t[] = [l, l, l]
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
    fn test_dyn_array_slicing_both_ends() {
        // This is no black magic voodooo.. not too much of it at least.. idk..
        // This is just creating an array of dynamic sizes, and testing slicing it aka multiple
        // access
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
                        range: ArraySliceRange::FromTo(Box::new(usize_lit(0)), Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
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
    fn valid_slicing_both_ends_vars() {
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
                        var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
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
    fn test_dyn_array_valid_multiple_access_both_ends_vars_start_not_usize_errors() {
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
                        var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                        var_decl("x", Type::Array(Box::new(t.clone())), access),
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
                    var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                    var_decl("x", Type::Array(Box::new(t.clone())), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected end index to be of type `usize` for array"));
            }
        }
    }



    #[test]
    fn test_dyn_array_slicing_start_only_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone()],
                span: span(),
            };
            let access = Expr::ArraySlicing {
                array: Box::new(var_expr("arr")),
                range: ArraySliceRange::From(Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit),
                var_decl("x", Type::Array(Box::new(t.clone())), access),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_dyn_array_slicing_end_only_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone()],
                span: span(),
            };
            let access = Expr::ArraySlicing {
                array: Box::new(var_expr("arr")),
                range: ArraySliceRange::To(Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit),
                var_decl("x", Type::Array(Box::new(t.clone())), access),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }



    #[test]
    fn test_dyn_array_slicing_start_greater_than_end_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone(), l.clone()],
                span: span(),
            };
            let slice = Expr::ArraySlicing {
                array: Box::new(var_expr("arr")),
                range: ArraySliceRange::FromTo(Box::new(usize_lit(3)), Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), arr_lit),
                var_decl("s", Type::Array(Box::new(t.clone())), slice),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Start index"));
        }
    }

}
