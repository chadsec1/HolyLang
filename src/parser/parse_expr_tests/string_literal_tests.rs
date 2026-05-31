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
            Expr::StringLiteral { value, .. } => assert_eq!(value, r#"say \"hi\""#),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_invalid_escapes() {
        let escape = "";
            
        for _ in 1..=10000 {
            let escape = format!("{}\\", escape);
            assert_parse_err(&format!("\"hi {}\"", escape));
        }
    }

    #[test]
    fn unknown_escape_errors() {
        let valid_escapes = ['n', 'r', 't', '\\', '"', '\'', '0'];

        let invalid_escapes: Vec<char> = (0x20u8..=0x7E)
            .map(|b| b as char)
            .filter(|c| !valid_escapes.contains(c))
            .collect();

        for i in invalid_escapes {
            assert_parse_err(&format!("\"\\{}b\"", i));
            assert_parse_err(&format!("\"a\\{}b\"", i));
            assert_parse_err(&format!("\"\\a{}\"", i));
            assert_parse_err(&format!("\"\\{}\"", i));
        }
    }

    #[test]
    fn valid_escapes() {
        let valid_escapes = ['n', 'r', 't', '\\', '"', '\'', '0'];

        for v in valid_escapes {
            match parse(&format!("\"a\\{}b\"", v)).unwrap() {
                Expr::StringLiteral { value, .. } => {
                    assert!(value.starts_with("a"));
                    assert!(value.ends_with("b"));
                    assert_ne!(value, "ab");
                    assert_ne!(value, "a b");
                }
                other => panic!("expected StringLiteral, got {:?}", other),
            }
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
