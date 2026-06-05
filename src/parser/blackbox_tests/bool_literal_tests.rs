use super::*;

#[cfg(test)]
mod bool_literals_tests {
    use super::*;

    #[test]
    fn bool_literal_true() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = true", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Expr::BoolLiteral { value: true, .. }));
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn bool_literal_false() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = false", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Expr::BoolLiteral { value: false, .. }));
            } else { panic!("Expected VarDecl"); }
        }
    }



} 
