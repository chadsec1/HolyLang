/// This is manual verification tests to ensure that
/// `holy_expr_to_rust_expr_tests` results are correct, by manually checking expected
/// results
///
use super::*;


#[cfg(test)]
mod holy_expr_to_rust_expr_non_arr_literals_tests {
    use super::*;

    #[test]
    fn string() {
        let expr = Expr::StringLiteral { value: "hi".to_string(), span: span() };
        let expr_str = holy_expr_to_rust_expr(&expr);

        assert_eq!(expr_str, "\"hi\".to_string()")
    }

    #[test]
    fn string_all_letters() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            let expr = Expr::StringLiteral { value: l.to_string(), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("\"{}\".to_string()", l))
        }
    }

    #[test]
    fn bool_false() {
        let expr = Expr::BoolLiteral { value: false, span: span() };
        let expr_str = holy_expr_to_rust_expr(&expr);

        assert_eq!(expr_str, "false")
    }


    #[test]
    fn bool_true() {
        let expr = Expr::BoolLiteral { value: true, span: span() };
        let expr_str = holy_expr_to_rust_expr(&expr);

        assert_eq!(expr_str, "true")
    }

    #[test]
    fn float64() {
        let edge_cases = [
            f64::MIN, f64::MAX, 0.0f64
        ];

        for e in edge_cases {
            let expr = Expr::Float64Literal { value: e, span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}f64", e))
        }
    }

    #[test]
    fn int8() {
        let edge_cases = [
            i8::MIN, i8::MAX, 0i8
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Int8(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}i8", e))
        }
    }

    #[test]
    fn int16() {
        let edge_cases = [
            i16::MIN, i16::MAX, 0i16
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Int16(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}i16", e))
        }
    }

    #[test]
    fn int32() {
        let edge_cases = [
            i32::MIN, i32::MAX, 0i32
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Int32(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}i32", e))
        }
    }

    #[test]
    fn int64() {
        let edge_cases = [
            i64::MIN, i64::MAX, 0i64
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Int64(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}i64", e))
        }
    }

    #[test]
    fn int128() {
        let edge_cases = [
            i128::MIN, i128::MAX, 0i128
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Int128(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}i128", e))
        }
    }

    #[test]
    fn byte() {
        let edge_cases = [
            u8::MIN, u8::MAX, 0u8
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Byte(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}u8", e))
        }
    }

    #[test]
    fn uint16() {
        let edge_cases = [
            u16::MIN, u16::MAX, 0u16
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Uint16(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}u16", e))
        }
    }

    #[test]
    fn uint32() {
        let edge_cases = [
            u32::MIN, u32::MAX, 0u32
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Uint32(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}u32", e))
        }
    }

    #[test]
    fn uint64() {
        let edge_cases = [
            u64::MIN, u64::MAX, 0u64
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Uint64(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}u64", e))
        }
    }

    #[test]
    fn uint128() {
        let edge_cases = [
            u128::MIN, u128::MAX, 0u128
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Uint128(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}u128", e))
        }
    }

    #[test]
    fn usize() {
        let edge_cases = [
            usize::MIN, usize::MAX, 0usize
        ];

        for e in edge_cases {
            let expr = Expr::IntLiteral { value: IntLiteralValue::Usize(e), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, format!("{}usize", e))
        }
    }

}
