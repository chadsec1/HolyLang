/// Tests the built-in `copy` function
/// These tests focus on correctness, and edge cases
/// not nessacrily on ownership, as that's handled by ownership_tests file
///
use super::*;

#[cfg(test)]
mod copy_tests {
    use super::*;
        
    // copy call guards 
    //

    #[test]
    fn test_copy_of_literals_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_lit = Expr::CopyCall { expr: Box::new(l.clone()), span: span() };
            let body = vec![var_decl(true, "x", t.clone(), copy_lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Copying a literal"));
        }
    }


    #[test]
    fn test_copy_of_func_call_errors() {
        let call_expr = Expr::Call{
            name: "x".to_string(),
            args: vec![],
            span: span()
        };

        for t in ALL_TYPES_NO_ARR {
            let copy_expr = Expr::CopyCall { expr: Box::new(call_expr.clone()), span: span() };
            let body = vec![var_decl(true, "x", t.clone(), copy_expr)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Copy call expects a variable"));
        }
    }


    #[test]
    fn test_copy_of_array_access_errors() {
        for i in 0..1000 {
            for t in ALL_TYPES_NO_ARR {
                let array_expr = Expr::ArrayAccess {
                    array: Box::new(var_expr("e")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };


                let copy_expr = Expr::CopyCall { expr: Box::new(array_expr), span: span() };
                let body = vec![var_decl(true, "x", t.clone(), copy_expr)];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Copying is not needed for array access, when you access or slice an array or a string, a new copy is made. Remove the copy call and use operation directly."));
            }
        }
    }


    #[test]
    fn test_copy_of_array_multiple_access_errors() {
        for i in 0..=1000 {
            for t in ALL_TYPES_NO_ARR {
                let array_expr = Expr::ArraySlicing {
                            array: Box::new(var_expr("arr")),
                            range: ArraySliceRange::FromTo(Box::new(usize_lit(0)), Box::new(usize_lit(i))),
                            span: span(),
                        };

                let copy_expr = Expr::CopyCall { expr: Box::new(array_expr), span: span() };
                let body = vec![var_decl(true, "x", t.clone(), copy_expr)];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Copying is not needed for array access, when you access or slice an array or a string, a new copy is made. Remove the copy call and use operation directly."));
            }
        }
    }




    #[test]
    fn test_double_copy_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "a", t.clone(), l.clone()),
                var_decl(true, "b", t.clone(), Expr::CopyCall {
                        expr: Box::new(Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() }),
                        span: span(),
                    }
                ),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Double copying is not needed. Remove the extra copy call. "));
            
        }
    }


    // Because array access index variables are always copied.
    #[test]
    fn test_dynamic_array_valid_access_variable_copy_errors() {

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i+1 {
                    let copy_var = Expr::CopyCall { expr: Box::new(var_expr("e")), span: span() };
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(copy_var),
                        span: span(),
                    };

                    let body = vec![
                        var_decl(true, "e", Type::Usize, usize_lit(i2)),
                        var_decl(true, "arr", Type::Array(Box::new(t.clone())), arr_lit.clone()),
                        var_decl(true, "x", t.clone(), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("You do not need to Copy an index when you are accessing an array, it is always copied. Remove the copy call"));
                }
            }       
        }
    }


    #[test]
    fn test_fixed_array_valid_access_variable_copy_errors() {

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = array_lit(elements, Some(t.clone()));

                for i2 in 0..i+1 {
                    let copy_var = Expr::CopyCall { expr: Box::new(var_expr("e")), span: span() };
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(copy_var),
                        span: span(),
                    };

                    let body = vec![
                        var_decl(true, "e", Type::Usize, usize_lit(i2)),
                        var_decl(true, "arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), arr_lit.clone()),
                        var_decl(true, "x", t.clone(), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("You do not need to Copy an index when you are accessing an array, it is always copied. Remove the copy call"));
                }
            }       
        }
    }




}
