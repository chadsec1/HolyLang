/// Tests for built-in "fake" function: 
/// format()

use super::*;

#[cfg(test)]
mod format_call_tests {
    use super::*;

    #[test]
    fn format_call_binop_expr() {
        let literals_ints_edge_cases = get_all_ints_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_ints_edge_cases { 
                let stmts = parse_body(&format!("own s {} = format(\"Your age is {{{} + {}}}\")", t, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "s");
                    assert_eq!(v.type_name, t.clone());
                    if v.type_name != Type::Char {
                        assert_ne!(v.type_name.get_default_value(span()).unwrap(), v.value);
                    }

                    assert!(matches!(v.value, Expr::FormatCall { .. }));
                } else { panic!("Expected VarDecl"); }

            }
        }
    }

    #[test]
    fn format_call_variable() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                let stmts = parse_body(&format!("own x {} = {}\n own s {} = format(\"Hello, {{x}}!\")", t, l, t));

                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    if v.type_name != Type::Char {
                        assert_ne!(v.type_name.get_default_value(span()).unwrap(), v.value);
                    }
                } else { panic!("Expected VarDecl"); }


                if let Stmt::VarDecl(v) = &stmts[1] {
                    assert_eq!(v.name, "s");
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Expr::FormatCall { .. }));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn format_invalid_args_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own s {} = format({})", t, l)));
            }
            assert_parse_err(&wrap(&format!("own s {} = format({}[])", t, t)));
            assert_parse_err(&wrap(&format!("own s {} = format({{}})", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"{{}}\")", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"Hi {{}}\")", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"Hi\")", t)));
        }
    }


}
