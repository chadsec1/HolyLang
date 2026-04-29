use super::*;

#[cfg(test)]
mod format_call_tests {
    use super::*;

    #[test]
    fn test_format_call_basic() {
        match parse(r#"format("Hello {name}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "Hello {}");
                assert_eq!(expressions.len(), 1);
                assert!(matches!(&expressions[0], Expr::Var { name, .. } if name == "name"));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_multiple_expressions() {
        match parse(r#"format("{a} and {b}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{} and {}");
                assert_eq!(expressions.len(), 2);
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_escaped_braces() {
        match parse(r#"format("{{literal}} {x}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{{literal}} {}");
                assert_eq!(expressions.len(), 1);
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_expression_in_placeholder() {
        match parse(r#"format("{a + b}")"#).unwrap() {
            Expr::FormatCall { expressions, .. } => {
                assert!(matches!(&expressions[0], Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_no_placeholders_errors() {
        assert_parse_err(r#"format("no placeholders here")"#);
    }

    #[test]
    fn test_format_call_not_string_arg_errors() {
        assert_parse_err("format(x)");
    }

    #[test]
    fn test_format_call_no_args_errors() {
        assert_parse_err("format()");
    }

    #[test]
    fn test_format_call_too_many_args_errors() {
        assert_parse_err(r#"format("hello", "world")"#);
    }

    #[test]
    fn test_format_call_empty_placeholder_errors() {
        // {} is not allowed — must have an expression inside
        assert_parse_err(r#"format("{}")"#);
    }

    #[test]
    fn test_format_call_unclosed_brace_errors() {
        assert_parse_err(r#"format("hello {name")"#);
    }

    #[test]
    fn test_format_triple_brace_escaped_plus_placeholder() {
        // "{{{x}}}" should parse to template="{{{}}}", expressions=["x"]
        match parse(r#"format("{{{x}}}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{{{}}}");
                assert_eq!(expressions.len(), 1);
                assert!(matches!(&expressions[0], Expr::Var { name, .. } if name == "x"));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_only_escaped_braces_no_placeholder_errors() {
        // "{{x}}" is purely escaped, with no actual placeholder
        assert_parse_err(r#"format("{{x}}")"#);
    }
}
