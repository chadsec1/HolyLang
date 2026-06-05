use super::*;

fn assert_float64(s: &str, expected: f64) {
    match parse(s).unwrap_or_else(|e| panic!("expected Ok for {:?}\nerror: {:?}", s, e)) {
        Expr::Float64Literal { value, .. } => {
            assert_eq!(value , expected);
        }
        other => panic!("expected Float64Literal for {:?}, got {:?}", s, other),
    }
}



#[cfg(test)]
mod float64_literals_tests {
    use super::*;

    #[test]
    fn test_float64_finite() {
        assert_float64(&f64::MIN.to_string() , f64::MIN);
        assert_float64(&f64::MAX.to_string(), f64::MAX);
    }

    #[test]
    fn test_float64_non_finite_errors() {
        assert_parse_err("1.7976931348623157e+309");
        assert_parse_err(&f64::INFINITY.to_string());
        assert_parse_err(&f64::NEG_INFINITY.to_string());
        assert_parse_err(&f64::NAN.to_string());
    }

    #[test]
    fn test_float64_basic() {
        assert_float64("1.0", 1.0);
    }

    #[test]
    fn test_float64_zero() {
        assert_float64("0.0", 0.0);
    }

    #[test]
    fn test_float64_pi_precision() {
        assert_float64("3.141592653589793", 3.141592653589793);
    }

    #[test]
    fn test_float64_precision() {
        assert_float64("1.123456789", 1.123456789);
        assert_float64("1e-12", 1e-12);
        assert_float64("1e-27", 1e-27);
    }

    #[test]
    fn test_float64_multiple_dots_errors() {
        assert_parse_err("1.2.3");
    }

    #[test]
    fn test_float64_multiple_chars_errors() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();
        
        for l in letters {
            assert_parse_err(&format!("1{}{}-12", l, l)); 
        }
    }

    #[test]
    fn test_float64_invalid_chars_errors() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .filter(|&c| c != 'e' && c != 'E')
            .collect();
        
        for l in letters {
            assert_parse_err(&format!("1{}-12", l)); 
        }
    }

    #[test]
    fn test_float64_bare_dot_errors() {
        assert_parse_err(".");
    }

}
