/// These are tests for functions themselves (like naming, parameters, etc),
/// and not necessarily function calls. Those are tested in "function_call_tests" file
///
use super::*;

#[cfg(test)]
mod function_tests {
    use super::*;

    #[test]
    fn function_name_taken_by_function_name_errors() {
        let f1 = void_func("foo", vec![], vec![]);
        let f2 = void_func("foo", vec![], vec![]);
        let mut ast = AST { functions: vec![f1, f2], globals: vec![] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate function"));
    }


    #[test]
    fn function_name_taken_by_global_const_errors() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let c = const_define_globally("foo", t.clone(), l.clone());
            let f = void_func("foo", vec![], vec![]);
            let mut ast = AST { functions: vec![f], globals: vec![c] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
        }

        // Same test, but t is now a fixed array type
        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                if !matches!(l, Expr::ArrayLiteral{ .. }) { // skip non array t's
                    continue
                }

                let c = const_define_globally("foo", Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1)), l.clone());
                let f = void_func("foo", vec![], vec![]);
                let mut ast = AST { functions: vec![f], globals: vec![c] };
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
            }
        }
        
        // Same test, but t is now a dynamic array type
        for t in ALL_TYPES_NO_ARR {
            for l in &literals {
                if !matches!(l, Expr::ArrayLiteral{ .. }) { // skip non array t's
                    continue
                }

                let c = const_define_globally("foo", Type::Array(Box::new(t.clone())), l.clone());
                let f = void_func("foo", vec![], vec![]);
                let mut ast = AST { functions: vec![f], globals: vec![c] };
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("is already taken by a function"));
            }
        }
    }

    // function parameters
    //
    #[test]
    fn params_are_in_scope_basic() {
        // Checks if function parameters are in scope, without testing for inner scopes.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![return_stmt(vec![var_expr("n")])];
            let func = returning_func("foo", vec![param("n", t.clone())], vec![t.clone()], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn param_name_taken_by_global_const_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let c = const_define_globally("bar", t.clone(), l.clone());

            for t2 in ALL_TYPES_NO_ARR {
                let f = void_func("foo", vec![param("bar", t2.clone())], vec![]);
                let mut ast = AST { functions: vec![f], globals: vec![c.clone()] };
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("because it is already declared globally"));
            }
        }
    }

    // Empty branches are not allowed, that includes function branch
    //
    #[test]
    fn test_empty_function_errors() {
        let mut ast = AST {
            functions: vec![
                Function {
                    name: "foo".to_string(),
                    params: vec![],
                    return_type: None,
                    body: vec![],
                    span: span(),
                }
            ],
            globals: vec![]
        };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty functions are not allowed"));
    }


}
