use super::*;

#[cfg(test)]
mod expr_tests {
    use super::*;

    // undeclared variable usage tests

    #[test]
    fn test_expr_use_of_undeclared_variable_errors() {
        // Try referencing non-existent variable "x"
        let body = vec![Stmt::Expr(var_expr("x"))]; // x not declared
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("undeclared binding"));
    }


}
