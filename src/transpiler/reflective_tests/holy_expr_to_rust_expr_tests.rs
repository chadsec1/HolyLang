/// This is manual verification tests to ensure that
/// `holy_expr_to_rust_expr_tests` results are correct, by manually checking expected
/// results
///
use super::*;


#[cfg(test)]
mod holy_expr_to_rust_prog_func_call_exprs_tests {
    use super::*;

    #[test]
    fn no_args() {
        let expr = Expr::Call {
            name: "foo".to_string(), 
            args: vec![], 
            span: span() 
        };
        let expr_str = holy_expr_to_rust_expr(&expr);

        assert_eq!(expr_str, "foo()")
    }

    #[test]
    fn all_literals_arg() {
        let literals = get_all_literals();

        for arg1 in &literals {
            let arg1_str = holy_expr_to_rust_expr(&arg1);

            for arg2 in &literals {
                let arg2_str = holy_expr_to_rust_expr(&arg2);

                for arg3 in &literals {
                    let arg3_str = holy_expr_to_rust_expr(&arg3);

                    let call_expr = Expr::Call {
                        name: "foo".to_string(), 
                        args: vec![arg1.clone(), arg2.clone(), arg3.clone()], 
                        span: span() 
                    };
                    let call_expr_str = holy_expr_to_rust_expr(&call_expr);

                    assert_eq!(call_expr_str, format!("foo({},{},{})", arg1_str, arg2_str, arg3_str))
                }
            }
        }
    }
}


#[cfg(test)]
mod holy_expr_to_rust_internal_func_call_exprs_tests {
    use super::*;

    #[test]
    fn range_call_all_literals() {
        let literals = get_all_literals();

        for start_expr in &literals {
            let start_expr_str = holy_expr_to_rust_expr(&start_expr);

            for end_expr in &literals {
                let end_expr_str = holy_expr_to_rust_expr(&end_expr);

                let range_call_expr = Expr::RangeCall {
                    start: Box::new(start_expr.clone()),
                    end: Box::new(end_expr.clone()),
                    span: span() 
                };
                let range_call_expr_str = holy_expr_to_rust_expr(&range_call_expr);

                assert_eq!(range_call_expr_str, format!("{}..{}", start_expr_str, end_expr_str))
            }
        }
    }

    #[test]
    fn copy_call_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let copy_call_expr = Expr::CopyCall {
                expr: Box::new(expr),
                span: span() 
            };
            let copy_call_expr_str = holy_expr_to_rust_expr(&copy_call_expr);

            assert_eq!(copy_call_expr_str, format!("{}.clone()", expr_str))
        }
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn format_call_template_no_template_no_exprs_panics() {
        let format_call_expr = Expr::FormatCall {
            template: "".to_string(),
            expressions: vec![],
            span: span() 
        };
        holy_expr_to_rust_expr(&format_call_expr);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn format_call_template_no_placeholder_no_exprs_panics() {
        let format_call_expr = Expr::FormatCall {
            template: "hi".to_string(),
            expressions: vec![],
            span: span() 
        };
        holy_expr_to_rust_expr(&format_call_expr);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn format_call_template_with_placeholder_no_exprs_panics() {
        let format_call_expr = Expr::FormatCall {
            template: "hi {}".to_string(),
            expressions: vec![],
            span: span() 
        };
        holy_expr_to_rust_expr(&format_call_expr);
    }

    #[test]
    fn format_call_no_template_with_exprs_panics() {
        let literals = get_all_literals();

        for expr in literals {
            for i in 1..1000 {
                let format_call_expr = Expr::FormatCall {
                    template: "".to_string(),
                    expressions: vec![expr.clone(); i],
                    span: span() 
                };

                let result = std::panic::catch_unwind(|| { 
                            holy_expr_to_rust_expr(&format_call_expr);
                        });

                assert!(result.is_err(), "Expected panic for: {:?}", format_call_expr);
            }
        }
    }
}

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

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Equal,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} == {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_not_equal_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::NotEqual,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} != {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_greater_equal_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::GreaterEqual,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} >= {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_less_equal_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::LessEqual,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} <= {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_greater_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Greater,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} > {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_less_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Less,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} < {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_and_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::And,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} && {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_or_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Or,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} || {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_bitwise_shift_left_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::BitwiseShiftLeft,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_shl({right_expr_str}.try_into().unwrap_or_else(|_| panic!(\"bitwise shift left count `{{}}` does not fit in u32\", {right_expr_str}))).unwrap_or_else(|| panic!(\"bitwise shift left overflow\"))"))
            }
        }
    }

    #[test]
    fn binop_bitwise_shift_right_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::BitwiseShiftRight,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_shr({right_expr_str}.try_into().unwrap_or_else(|_| panic!(\"bitwise shift right count `{{}}` does not fit in u32\", {right_expr_str}))).unwrap_or_else(|| panic!(\"bitwise shift right overflow\"))"));
            }
        }
    }

    #[test]
    fn binop_bitwise_and_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::BitwiseAnd,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} & {})", left_expr_str, right_expr_str))
            }
        }
    }

    #[test]
    fn binop_bitwise_or_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::BitwiseOr,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("({} | {})", left_expr_str, right_expr_str))
            }
        }
    }



    #[test]
    fn binop_add_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Add,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_add({right_expr_str}).unwrap_or_else(|| panic!(\"arithmetic addition overflow\"))"));
            }
        }
    }

    #[test]
    fn binop_subtract_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Subtract,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_sub({right_expr_str}).unwrap_or_else(|| panic!(\"arithmetic subtraction overflow\"))"));
            }
        }
    }

    #[test]
    fn binop_multiply_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    right: Box::new(right_expr.clone()),
                    left: Box::new(left_expr.clone()),
                    op: BinOpKind::Multiply,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_mul({right_expr_str}).unwrap_or_else(|| panic!(\"arithmetic multiplication overflow\"))"));
            }
        }
    }


    #[test]
    fn binop_divide_all_literals() {
        let literals = get_all_literals();

        for right_expr in &literals {
            let right_expr_str = holy_expr_to_rust_expr(&right_expr);

            for left_expr in &literals {
                let left_expr_str = holy_expr_to_rust_expr(&left_expr);

                let bin_expr = Expr::BinOp {
                    left: Box::new(left_expr.clone()),
                    right: Box::new(right_expr.clone()),
                    op: BinOpKind::Divide,
                    span: span()
                };
                let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);

                assert_eq!(bin_expr_str, format!("{left_expr_str}.checked_div({right_expr_str}).unwrap_or_else(|| panic!(\"arithmetic division overflow\"))"));
            }
        }
    }
} 


#[cfg(test)]
mod holy_expr_to_rust_literals_in_unaryop_non_arr_exprs_tests {
    use super::*;

    #[test]
    fn unary_negate_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let unary_expr = Expr::UnaryOp {
                expr: Box::new(expr),
                op: UnaryOpKind::Negate,
                span: span()
            };
            let unary_expr_str = holy_expr_to_rust_expr(&unary_expr);

            assert_eq!(unary_expr_str, format!("{expr_str}.checked_neg().unwrap_or_else(|| panic!(\"unary negate integer overflow\"))"))
        }
    }

    #[test]
    fn unary_logic_not_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let unary_expr = Expr::UnaryOp {
                expr: Box::new(expr),
                op: UnaryOpKind::Not,
                span: span()
            };
            let unary_expr_str = holy_expr_to_rust_expr(&unary_expr);

            assert_eq!(unary_expr_str, format!("!{expr_str}"))
        }
    }

    #[test]
    fn unary_bitwise_not_all_literals() {
        let literals = get_all_literals();

        for expr in literals {
            let expr_str = holy_expr_to_rust_expr(&expr);

            let unary_expr = Expr::UnaryOp {
                expr: Box::new(expr),
                op: UnaryOpKind::BitwiseNot,
                span: span()
            };
            let unary_expr_str = holy_expr_to_rust_expr(&unary_expr);

            assert_eq!(unary_expr_str, format!("!{expr_str}"))
        }
    }
}


#[cfg(test)]
mod holy_expr_to_rust_literals_dyn_array_literals_exprs_tests {
    use super::*;

    #[test]
    fn arr_of_all_literals() {
        let literals = get_all_literals();

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let type_name = Type::Array(Box::new(t.clone()));

            for expr1 in &literals {
                let expr1_str = holy_expr_to_rust_expr(&expr1);

                for expr2 in &literals {
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    for expr3 in &literals {
                        let expr3_str = holy_expr_to_rust_expr(&expr3);

                        let elements = vec![expr1.clone(), expr2.clone(), expr3.clone()];
                        let arr_expr = Expr::ArrayLiteral {
                            elements,
                            type_name: Some(type_name.clone()),
                            span: span()
                        };
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);

                        assert_eq!(arr_expr_str, format!("vec![{},{},{}]", expr1_str, expr2_str, expr3_str))
                    }
                }
            }
        }
    }


    #[test]
    fn arr_of_all_literals_none_type_name_panics() {
        let literals = get_all_literals();

        for expr1 in &literals {
            for expr2 in &literals {
                for expr3 in &literals {
                    let elements = vec![expr1.clone(), expr2.clone(), expr3.clone()];
                    let arr_expr = Expr::ArrayLiteral {
                        elements,
                        type_name: None,
                        span: span()
                    };

                    let result = std::panic::catch_unwind(|| { 
                            holy_expr_to_rust_expr(&arr_expr)
                        });

                    assert!(result.is_err(), "Expected panic for: {:?}", arr_expr);
                }
            }
        }
    }


    #[test]
    fn arr_of_all_literals_type_name_non_arr_type_panics() {
        let literals = get_all_literals();

        for t in ALL_TYPES_NO_ARR {
            for expr1 in &literals {
                for expr2 in &literals {
                    for expr3 in &literals {
                        let elements = vec![expr1.clone(), expr2.clone(), expr3.clone()];
                        let arr_expr = Expr::ArrayLiteral {
                            elements,
                            type_name: Some(t.clone()),
                            span: span()
                        };

                        let result = std::panic::catch_unwind(|| { 
                                holy_expr_to_rust_expr(&arr_expr)
                            });

                        assert!(result.is_err(), "Expected panic for: {:?}", arr_expr);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod holy_expr_to_rust_literals_fixed_array_literals_exprs_tests {
    use super::*;

    #[test]
    fn arr_with_literal_size_of_all_literals() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let type_name = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for expr1 in &literals {
                let expr1_str = holy_expr_to_rust_expr(&expr1);

                for expr2 in &literals {
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    for expr3 in &literals {
                        let expr3_str = holy_expr_to_rust_expr(&expr3);

                        let elements = vec![expr1.clone(), expr2.clone(), expr3.clone()];
                        let arr_expr = Expr::ArrayLiteral {
                            elements,
                            type_name: Some(type_name.clone()),
                            span: span()
                        };
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);

                        assert_eq!(arr_expr_str, format!("[{},{},{}]", expr1_str, expr2_str, expr3_str))
                    }
                }
            }
        }
    }

    #[test]
    fn arr_with_const_size_of_all_literals() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let type_name = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr1 in &literals {
                let expr1_str = holy_expr_to_rust_expr(&expr1);

                for expr2 in &literals {
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    for expr3 in &literals {
                        let expr3_str = holy_expr_to_rust_expr(&expr3);

                        let elements = vec![expr1.clone(), expr2.clone(), expr3.clone()];
                        let arr_expr = Expr::ArrayLiteral {
                            elements,
                            type_name: Some(type_name.clone()),
                            span: span()
                        };
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);

                        assert_eq!(arr_expr_str, format!("[{},{},{}]", expr1_str, expr2_str, expr3_str))
                    }
                }
            }
        }
    }

}


#[cfg(test)]
mod holy_expr_to_rust_literals_array_access_exprs_tests {
    use super::*;

    #[test]
    fn fixed_arr_with_literal_size_access_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_access_expr = Expr::ArrayAccess {
                    array: Box::new(arr_expr.clone()),
                    index: Box::new(expr.clone()),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_literal_size_access_with_all_binop_on_arr_expr() {
        let literals = get_all_literals();

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));
            for b in ALL_BIN_OP_KIND {
                for left_expr in &literals {
                    for right_expr in &literals {
                        let arr_expr = Expr::ArrayLiteral {
                            elements: vec![],
                            type_name: Some(arr_t.clone()),
                            span: span()
                        };

                        let bin_expr = Expr::BinOp {
                            left: Box::new(left_expr.clone()),
                            right: Box::new(right_expr.clone()),
                            op: b.clone(),
                            span: span()
                        };

                        let arr_access_expr = Expr::ArrayAccess {
                            array: Box::new(arr_expr.clone()),
                            index: Box::new(bin_expr.clone()),
                            span: span()
                        };

                        let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                        let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                        assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, bin_expr_str))
                    }
                }
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_access_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_access_expr = Expr::ArrayAccess {
                    array: Box::new(arr_expr.clone()),
                    index: Box::new(expr.clone()),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_access_with_all_binop_on_arr_expr() {
        let literals = get_all_literals();

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));
            for b in ALL_BIN_OP_KIND {
                for left_expr in &literals {
                    for right_expr in &literals {
                        let arr_expr = Expr::ArrayLiteral {
                            elements: vec![],
                            type_name: Some(arr_t.clone()),
                            span: span()
                        };

                        let bin_expr = Expr::BinOp {
                            left: Box::new(left_expr.clone()),
                            right: Box::new(right_expr.clone()),
                            op: b.clone(),
                            span: span()
                        };

                        let arr_access_expr = Expr::ArrayAccess {
                            array: Box::new(arr_expr.clone()),
                            index: Box::new(bin_expr.clone()),
                            span: span()
                        };

                        let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                        let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                        assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, bin_expr_str))
                    }
                }
            }
        }
    }

    #[test]
    fn dyn_arr_access_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_access_expr = Expr::ArrayAccess {
                    array: Box::new(arr_expr.clone()),
                    index: Box::new(expr.clone()),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn dyn_arr_access_with_all_binop_on_arr_expr() {
        let literals = get_all_literals();

        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for b in ALL_BIN_OP_KIND {
                for left_expr in &literals {
                    for right_expr in &literals {
                        let arr_expr = Expr::ArrayLiteral {
                            elements: vec![],
                            type_name: Some(arr_t.clone()),
                            span: span()
                        };

                        let bin_expr = Expr::BinOp {
                            left: Box::new(left_expr.clone()),
                            right: Box::new(right_expr.clone()),
                            op: b.clone(),
                            span: span()
                        };

                        let arr_access_expr = Expr::ArrayAccess {
                            array: Box::new(arr_expr.clone()),
                            index: Box::new(bin_expr.clone()),
                            span: span()
                        };

                        let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                        let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                        let arr_access_expr_str = holy_expr_to_rust_expr(&arr_access_expr);

                        assert_eq!(arr_access_expr_str, format!("{}[{}]", arr_expr_str, bin_expr_str))
                    }
                }
            }
        }
    }
}



#[cfg(test)]
mod holy_expr_to_rust_literals_fixed_array_slicing_exprs_tests {
    use super::*;

    #[test]
    fn fixed_arr_with_literal_size_slicing_from_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::From(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[{}..].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_literal_size_slicing_to_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::To(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[..{}].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_literal_size_slicing_fromto_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for expr1 in &literals {
                for expr2 in &literals {
                    let arr_expr = Expr::ArrayLiteral {
                        elements: vec![],
                        type_name: Some(arr_t.clone()),
                        span: span()
                    };

                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(arr_expr.clone()),
                        range: ArraySliceRange::FromTo(Box::new(expr1.clone()), Box::new(expr2.clone())),
                        span: span()
                    };

                    let expr1_str = holy_expr_to_rust_expr(&expr1);
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("{}[{}..{}].to_vec()", arr_expr_str, expr1_str, expr2_str))
                }
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_slicing_from_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::From(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[{}..].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_slicing_to_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::To(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[..{}].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_slicing_fromto_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr1 in &literals {
                for expr2 in &literals {
                    let arr_expr = Expr::ArrayLiteral {
                        elements: vec![],
                        type_name: Some(arr_t.clone()),
                        span: span()
                    };

                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(arr_expr.clone()),
                        range: ArraySliceRange::FromTo(Box::new(expr1.clone()), Box::new(expr2.clone())),
                        span: span()
                    };

                    let expr1_str = holy_expr_to_rust_expr(&expr1);
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("{}[{}..{}].to_vec()", arr_expr_str, expr1_str, expr2_str))
                }
            }
        }
    }

    #[test]
    fn arr_slicing_from_with_all_literal_on_var_expr() {
        let literals = get_all_literals();

        for expr in literals {
            let arr_slicing_expr = Expr::ArraySlicing {
                array: Box::new(var_expr("x")),
                range: ArraySliceRange::From(Box::new(expr.clone())),
                span: span()
            };

            let expr_str = holy_expr_to_rust_expr(&expr);
            let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

            assert_eq!(arr_slicing_expr_str, format!("x[{}..].to_vec()", expr_str))
        }
    }

    #[test]
    fn arr_slicing_to_with_all_literal_on_var_expr() {
        let literals = get_all_literals();

        for expr in literals {
            let arr_slicing_expr = Expr::ArraySlicing {
                array: Box::new(var_expr("x")),
                range: ArraySliceRange::To(Box::new(expr.clone())),
                span: span()
            };

            let expr_str = holy_expr_to_rust_expr(&expr);
            let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

            assert_eq!(arr_slicing_expr_str, format!("x[..{}].to_vec()", expr_str))
        }
    }

    #[test]
    fn arr_slicing_fromto_with_all_literal_on_var_expr() {
        let literals = get_all_literals();

        for expr1 in &literals {
            for expr2 in &literals {
                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(var_expr("x")),
                    range: ArraySliceRange::FromTo(Box::new(expr1.clone()), Box::new(expr2.clone())),
                    span: span()
                };

                let expr1_str = holy_expr_to_rust_expr(&expr1);
                let expr2_str = holy_expr_to_rust_expr(&expr2);

                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("x[{}..{}].to_vec()", expr1_str, expr2_str))
            }
        }
    }

    #[test]
    fn arr_slicing_from_with_all_binop_on_var_expr() {
        let literals = get_all_literals();

        for b in ALL_BIN_OP_KIND {
            for left_expr in &literals {
                for right_expr in &literals {
                    let bin_expr = Expr::BinOp {
                        left: Box::new(left_expr.clone()),
                        right: Box::new(right_expr.clone()),
                        op: b.clone(),
                        span: span()
                    };
                
                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(var_expr("x")),
                        range: ArraySliceRange::From(Box::new(bin_expr.clone())),
                        span: span()
                    };

                    let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("x[{}..].to_vec()", bin_expr_str))
                }
            }
        }
    }


    #[test]
    fn arr_slicing_to_with_all_binop_on_var_expr() {
        let literals = get_all_literals();

        for b in ALL_BIN_OP_KIND {
            for left_expr in &literals {
                for right_expr in &literals {
                    let bin_expr = Expr::BinOp {
                        left: Box::new(left_expr.clone()),
                        right: Box::new(right_expr.clone()),
                        op: b.clone(),
                        span: span()
                    };
                
                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(var_expr("x")),
                        range: ArraySliceRange::To(Box::new(bin_expr.clone())),
                        span: span()
                    };

                    let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("x[..{}].to_vec()", bin_expr_str))
                }
            }
        }
    }

    #[test]
    fn arr_slicing_fromto_with_all_binop_on_var_expr() {
        let literals = get_all_literals();

        for b in ALL_BIN_OP_KIND {
            for left_expr in &literals {
                for right_expr in &literals {
                    let bin_expr = Expr::BinOp {
                        left: Box::new(left_expr.clone()),
                        right: Box::new(right_expr.clone()),
                        op: b.clone(),
                        span: span()
                    };
                
                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(var_expr("x")),
                        range: ArraySliceRange::FromTo(Box::new(bin_expr.clone()), Box::new(left_expr.clone())),
                        span: span()
                    };

                    let bin_expr_str = holy_expr_to_rust_expr(&bin_expr);
                    let left_expr_str = holy_expr_to_rust_expr(&left_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("x[{}..{}].to_vec()", bin_expr_str, left_expr_str))
                }
            }
        }
    }
}


#[cfg(test)]
mod holy_expr_to_rust_literals_dyn_array_slicing_exprs_tests {
    use super::*;

    #[test]
    fn dyn_arr_slicing_from_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::From(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[{}..].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn dyn_arr_slicing_to_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::To(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[..{}].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn dyn_arr_slicing_fromto_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for expr1 in &literals {
                for expr2 in &literals {
                    let arr_expr = Expr::ArrayLiteral {
                        elements: vec![],
                        type_name: Some(arr_t.clone()),
                        span: span()
                    };

                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(arr_expr.clone()),
                        range: ArraySliceRange::FromTo(Box::new(expr1.clone()), Box::new(expr2.clone())),
                        span: span()
                    };

                    let expr1_str = holy_expr_to_rust_expr(&expr1);
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("{}[{}..{}].to_vec()", arr_expr_str, expr1_str, expr2_str))
                }
            }
        }
    }

    #[test]
    fn dyn_arr_with_const_size_slicing_from_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::Array(Box::new(t.clone()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::From(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[{}..].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_slicing_to_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr in &literals {
                let arr_expr = Expr::ArrayLiteral {
                    elements: vec![],
                    type_name: Some(arr_t.clone()),
                    span: span()
                };

                let arr_slicing_expr = Expr::ArraySlicing {
                    array: Box::new(arr_expr.clone()),
                    range: ArraySliceRange::To(Box::new(expr.clone())),
                    span: span()
                };

                let expr_str = holy_expr_to_rust_expr(&expr);
                let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                assert_eq!(arr_slicing_expr_str, format!("{}[..{}].to_vec()", arr_expr_str, expr_str))
            }
        }
    }

    #[test]
    fn fixed_arr_with_const_size_slicing_fromto_with_all_literal_on_arr_expr() {
        let literals = get_all_literals();
        for t in ALL_TYPES_WITH_DYN_ARR.iter() {
            let arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Const("x".to_string()));

            for expr1 in &literals {
                for expr2 in &literals {
                    let arr_expr = Expr::ArrayLiteral {
                        elements: vec![],
                        type_name: Some(arr_t.clone()),
                        span: span()
                    };

                    let arr_slicing_expr = Expr::ArraySlicing {
                        array: Box::new(arr_expr.clone()),
                        range: ArraySliceRange::FromTo(Box::new(expr1.clone()), Box::new(expr2.clone())),
                        span: span()
                    };

                    let expr1_str = holy_expr_to_rust_expr(&expr1);
                    let expr2_str = holy_expr_to_rust_expr(&expr2);

                    let arr_expr_str = holy_expr_to_rust_expr(&arr_expr);
                    let arr_slicing_expr_str = holy_expr_to_rust_expr(&arr_slicing_expr);

                    assert_eq!(arr_slicing_expr_str, format!("{}[{}..{}].to_vec()", arr_expr_str, expr1_str, expr2_str))
                }
            }
        }
    }
}

