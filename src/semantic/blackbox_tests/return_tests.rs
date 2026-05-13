use super::*;

#[cfg(test)]
mod return_tests {
    use super::*;

    // Code after return is not allowed
    //
    #[test]
    fn test_code_after_return_errors() {
        // returning func: return then another return.
        //
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                return_stmt(vec![l.clone()]),
                var_decl("x", t.clone(), l.clone()),
            ];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Dead code detected"));
        }
    }

    // Missing return statement in a function with return types
    //
    //
    #[test]
    fn test_missing_return_in_returning_function_errors() {
        // Function declares return type but body has no return statement.

        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![var_decl("x", t.clone(), l.clone())];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            assert!(err.starts_with("Semantic error: Function `foo` declares return type(s)"));
            assert!(err.contains("but statement branch body does not end with a return statement"));
        }
    }

    #[test]
    fn test_return_in_void_function_errors() {
        let literals = get_all_literals_no_arr();
        
        for l in literals {
            // Void function that tries to return a value.
            let body = vec![return_stmt(vec![l.clone()])];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("no declared return type"));
        }
    }


    #[test]
    fn test_non_returning_func_in_expr_errors() {
        for t in ALL_TYPES_NO_ARR {
            let callee = void_func("bar", vec![], vec![]);
            let body = vec![
                var_decl("x", t.clone(), call_expr("bar", vec![]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![] };

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("has no return type declared but is used in an expression"));
        }
    }


    #[test]
    fn test_type_mismatch_return_errors() {
        // Function returns Int32 but body returns Bool.
        let body = vec![return_stmt(vec![bool_lit(true)])];
        let func = returning_func("foo", vec![], vec![Type::Int32], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Return type mismatch"));
    }

    #[test]
    fn test_return_count_mismatch_errors() {
        // Declares two return types but returns one value.

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![return_stmt(vec![l.clone()])];
            let func = returning_func("foo", vec![], vec![t.clone(), t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }



}

