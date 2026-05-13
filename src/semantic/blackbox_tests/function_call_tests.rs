/// This file only focuses on actual real functions
/// and not any internal "fake" function such as "copy", or "format", etc.
///
use super::*;
use crate::consts;

#[cfg(test)]
mod function_calls_tests {
    use super::*;

    #[test]
    fn unknown_function_name_is_reserved_identifer_errors() {
        for kw in consts::RESERVED_KEYWORDS {
            let body = vec![Stmt::Expr(call_expr(kw, vec![]))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("unknown function"));
        }
    }

    #[test]
    fn unknown_function_name_is_a_type_name_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![Stmt::Expr(call_expr(&t.to_string(), vec![]))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("unknown function"));
        }
    }

    #[test]
    fn test_call_unknown_function_errors() {
        let body = vec![Stmt::Expr(call_expr("nonexistent", vec![]))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown function"));
    }

    #[test]
    fn test_call_wrong_arity_errors() {
        for t in ALL_TYPES_NO_ARR {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![]))]; // 0 args instead of 1
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![]};
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("expects 1 arguments, got 0"));
        }
    }

    #[test]
    fn test_call_wrong_arg_type_errors() {
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        for ((l, t1), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(ALL_TYPES_NO_ARR)
        {
            let callee = void_func("bar", vec![param("a", t2.clone())], vec![]);

            let body = vec![
                var_decl("x", t1.clone(), l.clone()),

                Stmt::Expr(call_expr("bar", vec![var_expr("x")]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![]};
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("type mismatch"));
        }
    }

    #[test]
    fn test_call_wrong_return_arity_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = returning_func("bar", vec![], vec![t.clone(), t.clone()], vec![
                return_stmt(vec![l.clone(), l.clone()])
            ]);
            let body = vec![
                var_decl("x", t.clone(), call_expr("bar", vec![]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![] };

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Call to function `bar` returns 2 values but is used in a single-value expression"));
        }
    }


    #[test]
    fn test_call_assign_from_non_returning_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let callee = void_func("bar", vec![], vec![]);
            let body = vec![
                var_decl("x", t.clone(), call_expr("bar", vec![]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller], globals: vec![]  };

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("has no return type declared but is used in an expression"));
        }
    }

    #[test]
    fn test_correct_call_no_parameter_no_arg() {
        let callee = void_func("bar", vec![], vec![]);
        let body = vec![Stmt::Expr(call_expr("bar", vec![]))];
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
        check_semantics(&mut ast).unwrap();
    }

    #[test]
    fn test_correct_call_with_parameter_and_arg() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![l.clone()]))];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
            check_semantics(&mut ast).unwrap();
        }
    }


    #[test]
    fn test_correct_call_with_parameter_and_var_arg() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                Stmt::Expr(call_expr("bar", vec![var_expr("x")]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
            check_semantics(&mut ast).unwrap();
        }
    }


    // All signed literals whose value equal to or more than `0` can be safely converted to uint,
    // so passing integer literals directly to functions should always work
    #[test]
    fn test_correct_call_literal_inference_passes() {
        let signed_literals = get_all_signed_literals_no_arr_no_float();
        
        for (sl, t) in signed_literals.iter().zip(ALL_UNSIGNED_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![sl.clone()]))];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] , globals: vec![] };
            check_semantics(&mut ast).unwrap();
        }
    }


 

}
