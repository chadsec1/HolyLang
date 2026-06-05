use super::*;

#[cfg(test)]
mod unary_op_tests {
    use super::*;

    // unary operations 
    #[test]
    fn test_negate_unsigned_errors() {
        let unsigned_literals = get_all_unsigned_literals_no_arr();

        for (ul, t) in unsigned_literals.iter().zip(ALL_UNSIGNED_TYPES_NO_ARR.iter()) {
            let neg = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(ul.clone()),
                span: span(),
            };
            let body = vec![var_decl("x", t.clone(), neg)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("negate"));
        }
    }

    #[test]
    fn test_negate_signed_passes() {
        let signed_literals = get_all_signed_literals_no_arr();

        for (sl, t) in signed_literals.iter().zip(ALL_SIGNED_TYPES_NO_ARR.iter()) {
            let neg = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(sl.clone()),
                span: span(),
            };
            let body = vec![var_decl("x", t.clone(), neg)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }




}
