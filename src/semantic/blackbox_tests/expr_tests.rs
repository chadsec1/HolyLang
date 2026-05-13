use super::*;

#[cfg(test)]
mod expr_tests {
    use super::*;

    // Try referencing non-existent variable "x"
    #[test]
    fn test_expr_use_of_undeclared_variable_errors() {
        let body = vec![Stmt::Expr(var_expr("x"))]; // x not declared
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("undeclared binding"));
    }

    // Try referencing moved variable "x"
    #[test]
    fn test_expr_use_of_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone()),
                var_decl("y", t.clone(), var_expr("x")),
                Stmt::Expr(var_expr("x"))
            ]; // x not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved variable"));
        }
    }


}
