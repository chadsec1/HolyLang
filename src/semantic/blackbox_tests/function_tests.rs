/// These are tests for functions themselves (like naming, parameters, etc),
/// and not necessarily function calls. Those are tested in "function_call_tests" file
///
use super::*;

#[cfg(test)]
mod function_tests {
    use super::*;

    // duplicate functions are not allowed
    #[test]
    fn test_duplicate_function_name_errors() {
        let f1 = void_func("foo", vec![], vec![]);
        let f2 = void_func("foo", vec![], vec![]);
        let mut ast = AST { functions: vec![f1, f2], globals: vec![] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate function"));
    }



    // function parameters
    //
    #[test]
    fn test_params_are_in_scope_basic() {
        // Checks if function parameters are in scope, without testing for inner scopes.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![return_stmt(vec![var_expr("n")])];
            let func = returning_func("foo", vec![param("n", t.clone())], vec![t.clone()], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    // Empty branches are not allowed, that includes functions
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
        assert!(result.unwrap_err().to_string().contains("has no statements, empty functions are not allowed!"));
    }


}
