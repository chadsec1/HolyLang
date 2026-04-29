use super::*;

#[cfg(test)]
mod vars_tests {
    use super::*;

    #[test]
    fn test_var_simple() {
        match parse("foo").unwrap() {
            Expr::Var { name, .. } => assert_eq!(name, "foo"),
            other => panic!("expected Var, got {:?}", other),
        }
    }

    #[test]
    fn test_var_with_underscores() {
        match parse("my_var").unwrap() {
            Expr::Var { name, .. } => assert_eq!(name, "my_var"),
            other => panic!("expected Var, got {:?}", other),
        }
    }

    #[test]
    fn test_var_with_numbers() {
        match parse("x2").unwrap() {
            Expr::Var { name, .. } => assert_eq!(name, "x2"),
            other => panic!("expected Var, got {:?}", other),
        }
    }

    #[test]
    fn test_var_invalid_starts_with_number_errors() {
        assert_parse_err("2abc");
    }

    #[test]
    fn test_var_invalid_special_chars_errors() {
        assert_parse_err("foo@bar");
        assert_parse_err("foo#bar");
    }


}

