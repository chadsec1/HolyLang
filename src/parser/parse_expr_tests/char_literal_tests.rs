use super::*;

#[cfg(test)]
mod char_literal_tests {
    use super::*;

    #[test]
    fn test_char() {
        let chars: Vec<char> = (0u32..=0x10FFFF)
                .filter_map(char::from_u32)
                .filter(|&c| c != '\\' && c != '\'')
                .collect();

        for c in chars {
            match parse(&format!("'{}'", c)).unwrap() {
                Expr::CharLiteral { value, .. } => assert_eq!(value, c),
                other => panic!("expected StringLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn empty_errors() {
        assert_parse_err(&"''");
    }

    #[test]
    fn valid_escapes() {
        let valid_escapes = ['n', 'r', 't', '\\', '\'', '0'];

        for v in valid_escapes {
            match parse(&format!("'\\{}'", v)).unwrap() {
                Expr::CharLiteral { .. } => {},
                other => panic!("expected CharLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn char_missing_quotes_errors() {
        assert_parse_err("'a");
        assert_parse_err("a'");
    }

    #[test]
    fn invalid_char_2_binop_errors() {
        assert_parse_err(&"e 'a'");
        assert_parse_err(&"'a' e");
    }
}
