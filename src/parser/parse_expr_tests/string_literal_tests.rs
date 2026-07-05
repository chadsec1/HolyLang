use super::*;

#[cfg(test)]
mod string_literals_tests {
    use super::*;

    #[test]
    fn string_simple() {
        match parse(r#""hello""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "hello"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn string_empty() {
        match parse(r#""""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, ""),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn string_with_spaces() {
        match parse(r#""hello world""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, "hello world"),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn string_with_escaped_quote() {
        match parse(r#""say \"hi\"""#).unwrap() {
            Expr::StringLiteral { value, .. } => assert_eq!(value, r#"say \"hi\""#),
            other => panic!("expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn invalid_escapes() {
        let escape = "";
            
        for _ in 1..=10000 {
            let escape = format!("{}\\", escape);
            assert_parse_err(&format!("\"hi {}\"", escape));
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
    fn string_missing_quotes_errors() {
        assert_parse_err(r#""hello"#);
        assert_parse_err(r#"hello""#);
    }

    #[test]
    fn invalid_string_2_binop_errors() {
        let literals = get_all_literals_edge_cases();
        let non_string_literals = get_all_non_string_literals_edge_cases();

        for l1 in literals.clone() {
            for l2 in non_string_literals.clone() {
                if l1.starts_with('-') || l2.starts_with('-') { continue }
                assert_parse_err(&format!("{} \"{}\"", l1, l2));
                assert_parse_err(&format!("\"{}\" {}", l2, l1));
            }
        }
    }

    #[test]
    fn string_with_escape_sequences() {
        let sequences = [ "\\n", "\\t", "\\r", "\\\\", "\\\"", "\\'", "\\0" ];

        for s in sequences {
            match parse(&format!("\"line1{}line2\"", s)).unwrap() {
                Expr::StringLiteral { value, .. } => assert_eq!(value, format!("line1{}line2", s)),
                other => panic!("expected StringLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn string_edge_cases() {
        let literals = get_all_non_string_literals_edge_cases();

        for l in literals {
            match parse(&format!("\"{l}\"")).unwrap() {
                Expr::StringLiteral { value, .. } => assert_eq!(value, l),
                other => panic!("expected StringLiteral, got {:?}", other),
            }
        }
    }
}
