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
            let body = vec![var_decl("x", t.clone(), Some(copy_lit))];
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
            let body = vec![var_decl("x", t.clone(), Some(copy_expr))];
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
                let body = vec![var_decl("x", t.clone(), Some(copy_expr))];
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
                        start: Some(Box::new(usize_lit(0))),
                        end: Some(Box::new(usize_lit(i))),
                        span: span(),
                    };


                let copy_expr = Expr::CopyCall { expr: Box::new(array_expr), span: span() };
                let body = vec![var_decl("x", t.clone(), Some(copy_expr))];
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
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", t.clone(), Some(
                    Expr::CopyCall {
                        expr: Box::new(Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() }),
                        span: span(),
                    }
                )),
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
                
            
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i+1 {
                
                    let copy_var = Expr::CopyCall { expr: Box::new(var_expr("e")), span: span() };
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(copy_var),
                        span: span(),
                    };

                    let body = vec![
                        var_decl("e", Type::Usize, Some(usize_lit(i2))),
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", t.clone(), Some(access)),
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
                
            
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    span: span(),
                };

                for i2 in 0..i+1 {
                
                    let copy_var = Expr::CopyCall { expr: Box::new(var_expr("e")), span: span() };
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(copy_var),
                        span: span(),
                    };

                    let body = vec![
                        var_decl("e", Type::Usize, Some(usize_lit(i2))),
                        var_decl("arr", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i + 1)), Some(arr_lit.clone())),
                        var_decl("x", t.clone(), Some(access)),
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
