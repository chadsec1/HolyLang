use super::*;

#[cfg(test)]
mod string_literals_tests {
    use super::*;

    #[test]
    fn string_literal_basic() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"hello\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "hello");
                } else { panic!("Expected StringLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_escape_sequences() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"hello\\nworld\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "hello\nworld");
                } else { panic!("Expected StringLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_with_escaped_quote() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"say \\\"hi\\\"\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, r#"say "hi""#);
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_unclosed_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = \"unclosed", t)));
        }
    }

    #[test]
    fn string_literal_containing_hash_not_comment() {
        for t in ALL_TYPES_NO_ARR {
            // '#' inside a string must not be stripped as a comment
            let stmts = parse_body(&format!("own x {} = \"hello # world\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "hello # world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn string_literal_containing_curly_brackets_end() {
        for t in ALL_TYPES_NO_ARR {
            // '}' inside a string must not be treated as a function closing curly bracket.
            let stmts = parse_body(&format!("own x {} = \"hello }} world\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "hello } world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_containing_curly_brackets_start() {
        for t in ALL_TYPES_NO_ARR {
            // '}' inside a string must not be treated as a function closing curly bracket.
            let stmts = parse_body(&format!("own x {} = \"hello {{ world\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Expr::StringLiteral { value, .. } = &v.value {
                    assert_eq!(value, "hello { world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }
}
