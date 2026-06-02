/// This is manual verification tests to ensure that
/// `holy_expr_to_rust_expr_tests` results are correct, by manually checking expected
/// results
///
use super::*;


#[cfg(test)]
mod holy_expr_to_rust_literals_non_arr_exprs_tests {
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

    #[test]
    fn var() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l in letters {
            let expr = Expr::Var { name: l.to_string(), span: span() };
            let expr_str = holy_expr_to_rust_expr(&expr);

            assert_eq!(expr_str, l.to_string())
        }
    }
}

#[cfg(test)]
mod holy_expr_to_rust_literals_in_binop_non_arr_exprs_tests {
    use super::*;

    #[test]
    fn binop_equal_string_all_letters() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l1 in &letters {
            let expr1 = Expr::StringLiteral { value: l1.to_string(), span: span() };
            let expr1_str = holy_expr_to_rust_expr(&expr1);

            assert_eq!(expr1_str, format!("\"{}\".to_string()", l1));

            for l2 in &letters {
                let expr2 = Expr::StringLiteral { value: l2.to_string(), span: span() };
                let expr2_str = holy_expr_to_rust_expr(&expr2);
            
                assert_eq!(expr2_str, format!("\"{}\".to_string()", l2));

                let bin_expr = Expr::BinOp {
                    left: Box::new(expr1.clone()),
                    right: Box::new(expr2),
                    op: BinOpKind::Equal,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} == {})", expr1_str, expr2_str))
            }
        }
    }

    #[test]
    fn binop_not_equal_string_all_letters() {
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

        for l1 in &letters {
            let expr1 = Expr::StringLiteral { value: l1.to_string(), span: span() };
            let expr1_str = holy_expr_to_rust_expr(&expr1);

            assert_eq!(expr1_str, format!("\"{}\".to_string()", l1));

            for l2 in &letters {
                let expr2 = Expr::StringLiteral { value: l2.to_string(), span: span() };
                let expr2_str = holy_expr_to_rust_expr(&expr2);
            
                assert_eq!(expr2_str, format!("\"{}\".to_string()", l2));

                let bin_expr = Expr::BinOp {
                    left: Box::new(expr1.clone()),
                    right: Box::new(expr2),
                    op: BinOpKind::NotEqual,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} != {})", expr1_str, expr2_str))
            }
        }
    }

    #[test]
    fn binop_equal_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Equal,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} == {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_not_equal_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::NotEqual,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} != {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_greater_equal_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::GreaterEqual,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} >= {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_less_equal_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::LessEqual,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} <= {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_greater_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Greater,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} > {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_less_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Less,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} < {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_and_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::And,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} && {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_or_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Or,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} || {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_bitwise_and_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::BitwiseAnd,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} & {})", expr_str, expr_str))
        }
    }

    #[test]
    fn binop_bitwise_or_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::BitwiseOr,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("({} | {})", expr_str, expr_str))
        }
    }



    #[test]
    fn binop_add_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Add,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("{expr_str}.checked_add({expr_str}).unwrap_or_else(|| panic!(\"arithmetic addition overflow\"))"));
        }
    }

    #[test]
    fn binop_subtract_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Subtract,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("{expr_str}.checked_sub({expr_str}).unwrap_or_else(|| panic!(\"arithmetic subtraction overflow\"))"));
        }
    }

    #[test]
    fn binop_multiply_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Multiply,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("{expr_str}.checked_mul({expr_str}).unwrap_or_else(|| panic!(\"arithmetic multiplication overflow\"))"));
        }
    }


    #[test]
    fn binop_divide_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let bin_expr = Expr::BinOp {
                left: Box::new(expr.clone()),
                right: Box::new(expr),
                op: BinOpKind::Divide,
                span: span()
            };
            let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

            assert_eq!(bin_expr_str, format!("{expr_str}.checked_div({expr_str}).unwrap_or_else(|| panic!(\"arithmetic division overflow\"))"));
        }
    }
} 
