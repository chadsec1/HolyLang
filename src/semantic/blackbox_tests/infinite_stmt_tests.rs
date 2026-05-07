use super::*;

#[cfg(test)]
mod infinite_stmt_tests {
    use super::*;

    #[test]
    fn test_infinite_statements_pass() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![ 
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    // Ensure infinite loops empty branches not allowed
    #[test]
    fn test_infinite_statements_empty_branch_errors() {
        let body = vec![ 
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);

        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
    }


}
