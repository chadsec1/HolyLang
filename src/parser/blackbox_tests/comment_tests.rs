use super::*;

#[cfg(test)]
mod comment_tests {
    use super::*;

    #[test]
    fn const_in_globals_inline_comment_stripped() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                // Statement followed by inline comment should still parse cleanly
                let ast = parse(&format!("const x {} = {} # this is {} a comment", t, l, t)).unwrap();
                assert_eq!(ast.functions.len(), 0);
                assert_eq!(ast.globals.len(), 1);

                if let GlobalStmt::Const(c) = &ast.globals[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else { panic!("Expected Const"); }
            }
        }
    }

    #[test]
    fn const_in_func_inline_comment_stripped() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                // Statement followed by inline comment should still parse cleanly
                let stmts = parse_body(&format!("const x {} = {} # this is {} a comment", t, l, t));
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], Stmt::Const(_)));
            }
        }
    }

    #[test]
    fn var_decl_inline_comment_stripped() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                // Statement followed by inline comment should still parse cleanly
                let stmts = parse_body(&format!("own x {} = {} # this is {} a comment", t, l, t));
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], Stmt::VarDecl(_)));
            }
        }
    }

    #[test]
    fn hash_inside_string_not_comment() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"val # not comment\"", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "val # not comment");
                } else { panic!("Expected Var Expression"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

}
