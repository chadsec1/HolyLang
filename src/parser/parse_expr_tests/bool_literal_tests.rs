use super::*;

#[cfg(test)]
mod bool_literals_tests {
    use super::*;

    #[test]
    fn test_bool_true() {
        assert!(matches!(parse("true"), Ok(Expr::BoolLiteral { value: true, .. })));
    }

    #[test]
    fn test_bool_false() {
        assert!(matches!(parse("false"), Ok(Expr::BoolLiteral { value: false, .. })));
    }

    #[test]
    fn test_bool_leading_trailing_whitespace() {
        const MAX_SPACES: usize = 1000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert!(matches!(parse(&format!("{}true", spaces)), Ok(Expr::BoolLiteral { value: true, .. })));
            assert!(matches!(parse(&format!("true{}", spaces)), Ok(Expr::BoolLiteral { value: true, .. })));
            assert!(matches!(parse(&format!("{}true{}", spaces, spaces)), Ok(Expr::BoolLiteral { value: true, .. })));

            assert!(matches!(parse(&format!("{}false", spaces)), Ok(Expr::BoolLiteral { value: false, .. })));
            assert!(matches!(parse(&format!("false{}", spaces)), Ok(Expr::BoolLiteral { value: false, .. })));
            assert!(matches!(parse(&format!("{}false{}", spaces, spaces)), Ok(Expr::BoolLiteral { value: false, .. })));
            spaces.push(' ');
        }
    }


}
