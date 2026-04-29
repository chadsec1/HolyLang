use super::*;

#[cfg(test)]
mod string_literals_tests {
    use super::*;

    #[test]
    fn test_string_simple() {
        match parse(r#""hello""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "hello"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_empty() {
        match parse(r#""""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, ""),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_with_spaces() {
        match parse(r#""hello world""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "hello world"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_with_escaped_quote() {
        match parse(r#""say \"hi\"""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, r#"say "hi""#),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_unclosed_errors() {
        assert_parse_err(r#""hello"#);
    }

    #[test]
    fn test_string_with_escape_sequences() {
        match parse(r#""line1\nline2""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "line1\nline2"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_that_looks_like_int() {
        match parse(r#""42""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "42"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_string_that_looks_like_bool() {
        match parse(r#""true""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "true"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }


}
