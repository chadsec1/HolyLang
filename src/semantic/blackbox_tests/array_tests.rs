/// This file doesn't actually test arrays, instead, it tests how other expressions  interact with other
/// expressions, if programmer specifices a expression to be an array implicitly
/// i.e. array accessing on a literal, or a non array type. etc.
///
use super::*;

#[cfg(test)]
mod array_tests {
    use super::*;

    // i.e. "hi"[0] is an error. You can only access variables, of type array, not literals.
    #[test]
    fn test_array_access_on_literals_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=1000 {
                let access = Expr::ArrayAccess {
                    array: Box::new(l.clone()),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", t.clone(), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected variable of any `array` type"));

            }       
        }
    }

    // Array access on undeclared variable
    #[test]
    fn test_array_access_on_undeclared_var_errors() {
        for t in ALL_TYPES_NO_ARR {
            for i in 0..=1000 {
                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("e")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", t.clone(), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on undeclared variable `e`"));
            }       
        }
    }

    // Array access on non-array variable
    #[test]
    fn test_array_access_on_non_array_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=1000 {
                let access = Expr::ArrayAccess {
                    array: Box::new(var_expr("e")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", t.clone(), l.clone()),
                    var_decl("x", t.clone(), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on non-array variable `e`"));
            }       
        }
    }

    // Array access on non-array variable
    #[test]
    fn test_array_multiple_access_on_non_array_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..1000 {
                let access = Expr::ArraySlicing {
                    array: Box::new(var_expr("e")),
                    range: ArraySliceRange::FromTo(Box::new(usize_lit(1)), Box::new(usize_lit(i+1))),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", t.clone(), l.clone()),
                    var_decl("x", t.clone(), access),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on non-array variable `e`"));
            }       
        }
    }



     

}
