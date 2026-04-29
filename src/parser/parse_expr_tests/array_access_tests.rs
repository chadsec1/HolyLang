use super::*;

#[cfg(test)]
mod array_access_tests {
    use super::*;

    #[test]
    fn test_array_single_access() {
        match parse("arr[0]").unwrap() {
            Expr::ArrayAccess { array, index, .. } => {
                assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                assert!(matches!(*index, Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. }));
            }
            other => panic!("expected ArrayAccess, got {:?}", other),
        }
    }

    #[test]
    fn test_array_single_access_expression_index() {
        match parse("arr[i + 1]").unwrap() {
            Expr::ArrayAccess { index, .. } => {
                assert!(matches!(*index, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected ArrayAccess, got {:?}", other),
        }
    }


}
