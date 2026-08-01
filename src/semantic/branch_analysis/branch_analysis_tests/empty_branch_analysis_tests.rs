use super::*;

#[cfg(test)]
mod empty_branch_analysis_tests {
    use super::*;

    #[test]
    #[should_panic(expected="Compiler bug")]
    // The reason `empty_branch_analysis_hazmat` panics when fed empty block of code directly, is
    // because it expects caller to give it an initial, non empty block of code. because if it were
    // given empty block of code, the function wouldn't be able to print error with line and column, and i dont want keep
    // passing spans all over.
    //
    fn empty_block_of_code_panics() {
        let _ = empty_branch_analysis_hazmat(&vec![]);
    }

    #[test]
    fn empty_infinite_statement() {
        for i in 1..1000 {
            let result = empty_branch_analysis_hazmat(&vec![Stmt::Infinite(InfiniteStmt{ branch: vec![], span: span() }); i]);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
        }
    }
}
