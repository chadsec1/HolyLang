use super::*;
use crate::parser::parse_expr::parse_expr;
use crate::tests_consts::{
    ALL_BIN_OP_KIND, BIN_OP_KIND_SYMBOLS, ALL_TYPES_NO_ARR_NO_INFER
};

#[cfg(test)]
mod parse_expr_tests {
    use super::*;


    fn get_all_literals_as_str_no_arr() -> [&'static str; 15] {
        let literals = [
            "1",
            "1",
            "1",
            "1",
            "1",

            "1",
            "1",
            "1",
            "1",
            "1",

            "1",

            "1.0",
            "1.0",
            "false",
            "\"Hi\""
        ];

        return literals;
    }

    fn span() -> Span {
        Span { line: 1, column: 1 }
    }

    fn parse(s: &str) -> Result<Expr, HolyError> {
        parse_expr(s, span())
    }

    // HELPERS

    fn assert_parse_err(s: &str) {
        assert!(
            parse(s).is_err(),
            "Expected parse error for input {:?}, but got Ok",
            s
        );
    }

    fn assert_int_literal(s: &str, expected: IntLiteralValue) {
        match parse(s).expect(&format!("expected Ok for {:?}", s)) {
            Expr::IntLiteral { value, .. } => assert_eq!(value, expected, "input: {:?}", s),
            other => panic!("expected IntLiteral for {:?}, got {:?}", s, other),
        }
    }

    fn assert_float32(s: &str, expected: f32) {
        match parse(s).expect(&format!("expected Ok for {:?}", s)) {
            Expr::FloatLiteral { value: FloatLiteralValue::Float32(v), .. } => {
                assert!((v - expected).abs() < f32::EPSILON * 10.0,
                    "f32 mismatch for {:?}: got {}, expected {}", s, v, expected);
            }
            other => panic!("expected Float32 for {:?}, got {:?}", s, other),
        }
    }

    fn assert_float64(s: &str, expected: f64) {
        match parse(s).expect(&format!("expected Ok for {:?}", s)) {
            Expr::FloatLiteral { value: FloatLiteralValue::Float64(v), .. } => {
                assert!((v - expected).abs() < f64::EPSILON * 10.0,
                    "f64 mismatch for {:?}: got {}, expected {}", s, v, expected);
            }
            other => panic!("expected Float64 for {:?}, got {:?}", s, other),
        }
    }

    #[test]
    fn test_empty_string_errors() {
        assert_parse_err("");
    }

    #[test]
    fn test_whitespace_only_errors() {
        for i in 1..10000 {
            assert_parse_err(&" ".repeat(i));
            assert_parse_err(&"\t".repeat(i));
            assert_parse_err(&"\n".repeat(i));
            assert_parse_err(&"\r".repeat(i));
            assert_parse_err(&" \n\r".repeat(i));
            assert_parse_err(&" \n\r\t".repeat(i));
        }
    }

    // Bool literals

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

        for i in 1..10000 {
            assert!(matches!(parse(&format!("{}true", " ".repeat(i))), Ok(Expr::BoolLiteral { value: true, .. })));
            assert!(matches!(parse(&format!("true{}", " ".repeat(i))), Ok(Expr::BoolLiteral { value: true, .. })));
            assert!(matches!(parse(&format!("{}true{}", " ".repeat(i), " ".repeat(i))), Ok(Expr::BoolLiteral { value: true, .. })));

            assert!(matches!(parse(&format!("{}false", " ".repeat(i))), Ok(Expr::BoolLiteral { value: false, .. })));
            assert!(matches!(parse(&format!("false{}", " ".repeat(i))), Ok(Expr::BoolLiteral { value: false, .. })));
            assert!(matches!(parse(&format!("{}false{}", " ".repeat(i), " ".repeat(i))), Ok(Expr::BoolLiteral { value: false, .. })));
        }
    }

    // Integer literals

    #[test]
    fn test_int_zero() {
        // 0 fits in i8
        assert_int_literal("0", IntLiteralValue::Int8(0));
    }

    #[test]
    fn test_int_i8_boundary() {
        assert_int_literal(&i8::MIN.to_string(), IntLiteralValue::Int8(i8::MIN));
        assert_int_literal(&i8::MAX.to_string(), IntLiteralValue::Int8(i8::MAX));
    }

    #[test]
    fn test_int_i8_overflow_promotes_to_i16() {
        assert_int_literal("128", IntLiteralValue::Int16(128));
    }

    #[test]
    fn test_int_i16_max() {
        assert_int_literal("32767", IntLiteralValue::Int16(32767));
    }

    #[test]
    fn test_int_i16_overflow_promotes_to_i32() {
        assert_int_literal("32768", IntLiteralValue::Int32(32768));
    }

    #[test]
    fn test_int_i32_max() {
        assert_int_literal("2147483647", IntLiteralValue::Int32(2147483647));
    }

    #[test]
    fn test_int_i32_overflow_promotes_to_i64() {
        assert_int_literal("2147483648", IntLiteralValue::Int64(2147483648));
    }

    #[test]
    fn test_int_i64_max() {
        assert_int_literal("9223372036854775807", IntLiteralValue::Int64(9223372036854775807));
    }

    #[test]
    fn test_int_i64_overflow_promotes_to_i128() {
        assert_int_literal("9223372036854775808", IntLiteralValue::Int128(9223372036854775808));
    }

    #[test]
    fn test_int_i128_max() {
        assert_int_literal(
            "170141183460469231731687303715884105727",
            IntLiteralValue::Int128(170141183460469231731687303715884105727_i128),
        );
    }

    #[test]
    fn test_int_u128_max() {
        // Beyond i128 max but fits u128
        assert_int_literal(
            "340282366920938463463374607431768211455",
            IntLiteralValue::Uint128(u128::MAX),
        );
    }

    #[test]
    fn test_int_overflow_u128_errors() {
        assert_parse_err("340282366920938463463374607431768211456");
    }

    #[test]
    fn test_int_with_whitespace() {
        assert_int_literal("  42  ", IntLiteralValue::Int8(42));
    }

    // Unary negate

    #[test]
    fn test_unary_negate_int() {
        // integer literals negated shouldn't produce unary negate, but instead just the literal
        // its self.
        match parse("-5").unwrap() {
            Expr::IntLiteral { value, .. } => {
                assert!(matches!(value, IntLiteralValue::Int8(-5)));
            }
            other => panic!("expected IntLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_float() {
        // float literals negated shouldn't produce unary negate, but instead just the literal
        // its self.
        match parse("-3.14").unwrap() {
            Expr::FloatLiteral { value, .. } => {
                assert!(matches!(value, FloatLiteralValue::Float32(-3.14)));
            }
            other => panic!("expected FloatLiteral, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_variable() {
        match parse("-foo").unwrap() {
            Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                assert!(matches!(*expr, Expr::Var { name, .. } if name == "foo"));
            }
            other => panic!("expected UnaryOp, got {:?}", other),
        }
    }

    #[test]
    fn test_unary_negate_alone_errors() {
        assert_parse_err("-");
    }

    #[test]
    fn test_unary_negate_whitespace_only_after_errors() {
        assert_parse_err("-   ");
    }

    // Float literals

    #[test]
    fn test_float_basic() {
        assert_float32("1.0", 1.0f32);
    }

    #[test]
    fn test_float_zero() {
        assert_float32("0.0", 0.0f32);
    }

    #[test]
    fn test_float_pi_f32_precision() {
        // 3.14159 fits in 7 significant digits, so it would be f32
        assert_float32("3.14159", 3.14159f32);
    }

    #[test]
    fn test_float_promotes_to_f64_when_precision_requires() {
        // 8+ significant digits, so that's a  f64
        assert_float64("1.123456789", 1.123456789f64);
    }

    #[test]
    fn test_float_multiple_dots_errors() {
        assert_parse_err("1.2.3");
    }

    #[test]
    fn test_float_invalid_chars_errors() {
        assert_parse_err("1.2e5"); // exponent notation not supported
    }

    #[test]
    fn test_float_bare_dot_errors() {
        assert_parse_err(".");
    }


    // String literals
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


    // Variable names
    //
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

    // Parentheses grouping

    #[test]
    fn test_parens_simple() {
        assert_int_literal("(5)", IntLiteralValue::Int8(5));
    }

    #[test]
    fn test_parens_nested() {
        assert_int_literal("((5))", IntLiteralValue::Int8(5));
    }

    #[test]
    fn test_parens_wrapping_binop() {
        match parse("(1 + 2)").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, .. } => {}
            other => panic!("expected BinOp Add, got {:?}", other),
        }
    }

    #[test]
    fn test_parens_partial_wrap_not_treated_as_group() {
        // (1 + 2) * 3 — outer parens don't wrap the whole expression
        match parse("(1 + 2) * 3").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, .. } => {}
            other => panic!("expected BinOp Multiply at top level, got {:?}", other),
        }
    }

    // Binary operations

    #[test]
    fn test_binop_add() {
        match parse("1 + 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, .. } => {}
            other => panic!("expected Add, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_subtract() {
        match parse("5 - 3").unwrap() {
            Expr::BinOp { op: BinOpKind::Subtract, .. } => {}
            other => panic!("expected Subtract, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_multiply() {
        match parse("4 * 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, .. } => {}
            other => panic!("expected Multiply, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_divide() {
        match parse("10 / 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Divide, .. } => {}
            other => panic!("expected Divide, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_equal() {
        match parse("x == y").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, .. } => {}
            other => panic!("expected Equal, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_not_equal() {
        match parse("x != y").unwrap() {
            Expr::BinOp { op: BinOpKind::NotEqual, .. } => {}
            other => panic!("expected NotEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_greater() {
        match parse("x > y").unwrap() {
            Expr::BinOp { op: BinOpKind::Greater, .. } => {}
            other => panic!("expected Greater, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_greater_equal() {
        match parse("x >= y").unwrap() {
            Expr::BinOp { op: BinOpKind::GreaterEqual, .. } => {}
            other => panic!("expected GreaterEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_less() {
        match parse("x < y").unwrap() {
            Expr::BinOp { op: BinOpKind::Less, .. } => {}
            other => panic!("expected Less, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_less_equal() {
        match parse("x <= y").unwrap() {
            Expr::BinOp { op: BinOpKind::LessEqual, .. } => {}
            other => panic!("expected LessEqual, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_missing_left_errors() {
        assert_parse_err("+ 2");
        assert_parse_err("* y");
    }

    #[test]
    fn test_binop_missing_right_errors() {
        assert_parse_err("1 +");
        assert_parse_err("x *");
    }

    #[test]
    fn test_single_equals_not_binop_errors() {
        assert_parse_err("x = y");
    }

    #[test]
    fn test_single_bang_not_binop_errors() {
        assert_parse_err("x ! y");
    }

    #[test]
    fn test_binop_left_associative_add_subtract() {
        // "1 + 2 + 3" — top-level op split gives left = "1 + 2", right = "3"
        // or depending on find_top_level_op_any semantics, at least it parses
        assert!(parse("1 + 2 + 3").is_ok());
    }

    #[test]
    fn test_binop_vars() {
        match parse("a + b").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, right, .. } => {
                assert!(matches!(*left, Expr::Var { name, .. } if name == "a"));
                assert!(matches!(*right, Expr::Var { name, .. } if name == "b"));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_nested() {
        // The top-level split should give us a BinOp at the top
        assert!(matches!(parse("a + b * c"), Ok(Expr::BinOp { .. })));
    }

    #[test]
    fn test_binop_with_parens_changes_grouping() {
        // (a + b) * c — top-level op is *
        match parse("(a + b) * c").unwrap() {
            Expr::BinOp { op: BinOpKind::Multiply, left, right, .. } => {
                assert!(matches!(*left, Expr::BinOp { op: BinOpKind::Add, .. }));
                assert!(matches!(*right, Expr::Var { name, .. } if name == "c"));
            }
            other => panic!("expected top-level Multiply, got {:?}", other),
        }
    }

    // Function calls

    #[test]
    fn test_function_call_no_args() {
        match parse("foo()").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "foo");
                assert!(args.is_empty());
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_one_arg() {
        match parse("foo(1)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "foo");
                assert_eq!(args.len(), 1);
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_multiple_args() {
        match parse("add(1, 2, 3)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 3);
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_expression_args() {
        match parse("add(x + 1, y)").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(&args[0], Expr::BinOp { op: BinOpKind::Add, .. }));
                assert!(matches!(&args[1], Expr::Var { .. }));
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    #[test]
    fn test_function_call_nested() {
        match parse("outer(inner(1))").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "outer");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::Call { name, .. } if name == "inner"));
            }
            other => panic!("expected Call, got {:?}", other),
        }
    }

    // Copy call

    #[test]
    fn test_copy_call_valid() {
        match parse("copy(x)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::Var { name, .. } if name == "x"));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    #[test]
    fn test_copy_call_no_args_errors() {
        assert_parse_err("copy()");
    }

    #[test]
    fn test_copy_call_too_many_args_errors() {
        assert_parse_err("copy(x, y)");
    }

    #[test]
    fn test_copy_call_expression_arg() {
        match parse("copy(a + b)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    // Format call

    #[test]
    fn test_format_call_basic() {
        match parse(r#"format("Hello {name}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "Hello {}");
                assert_eq!(expressions.len(), 1);
                assert!(matches!(&expressions[0], Expr::Var { name, .. } if name == "name"));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_multiple_expressions() {
        match parse(r#"format("{a} and {b}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{} and {}");
                assert_eq!(expressions.len(), 2);
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_escaped_braces() {
        match parse(r#"format("{{literal}} {x}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{{literal}} {}");
                assert_eq!(expressions.len(), 1);
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_expression_in_placeholder() {
        match parse(r#"format("{a + b}")"#).unwrap() {
            Expr::FormatCall { expressions, .. } => {
                assert!(matches!(&expressions[0], Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_call_no_placeholders_errors() {
        assert_parse_err(r#"format("no placeholders here")"#);
    }

    #[test]
    fn test_format_call_not_string_arg_errors() {
        assert_parse_err("format(x)");
    }

    #[test]
    fn test_format_call_no_args_errors() {
        assert_parse_err("format()");
    }

    #[test]
    fn test_format_call_too_many_args_errors() {
        assert_parse_err(r#"format("hello", "world")"#);
    }

    #[test]
    fn test_format_call_empty_placeholder_errors() {
        // {} is not allowed — must have an expression inside
        assert_parse_err(r#"format("{}")"#);
    }

    #[test]
    fn test_format_call_unclosed_brace_errors() {
        assert_parse_err(r#"format("hello {name")"#);
    }

    // Array literals

    #[test]
    fn test_bare_array_literal_without_type_errors() {
        assert_parse_err("[1, 2, 3]");
    }

    #[test]
    fn test_typed_array_literals() {
        let literals = get_all_literals_as_str_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            match parse(&format!("{}[{}, {}, {}]", t, l, l, l)).unwrap() {
                Expr::ArrayLiteral { elements, array_ty, .. } => {
                    assert_eq!(array_ty, t.clone());
                    assert_eq!(elements.len(), 3);
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_typed_array_literal_empty() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            match parse(&format!("{}[]", t)).unwrap() {
                Expr::ArrayLiteral { elements, array_ty, .. } => {
                    assert_eq!(array_ty, t.clone());
                    assert!(elements.is_empty());
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_typed_array_literal_nested() {
        // array of arrays
        //
        let literals = get_all_literals_as_str_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            match parse(&format!("{}[][{}[{},{}], {}[{},{}]]", t, t, l, l, t, l, l)).unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert_eq!(elements.len(), 2);
                    for elem in &elements {
                        assert!(matches!(elem, Expr::ArrayLiteral { .. }));
                    }
                }
                other => panic!("expected nested ArrayLiteral, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_typed_array_with_expressions() {
        let literals = get_all_literals_as_str_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            match parse(&format!("{}[a + {}, b * {}]", t, l, l)).unwrap() {
                Expr::ArrayLiteral { elements, array_ty, .. } => {
                    assert_eq!(elements.len(), 2);
                    assert_eq!(array_ty, t.clone());
                    assert!(matches!(&elements[0], Expr::BinOp { op: BinOpKind::Add, .. }));
                    assert!(matches!(&elements[1], Expr::BinOp { op: BinOpKind::Multiply, .. }));
                }
                other => panic!("expected ArrayLiteral, got {:?}", other),
            }
        }
    }

    // Array access, aka single access

    #[test]
    fn test_array_single_access() {
        match parse("arr[0]").unwrap() {
            Expr::ArraySingleAccess { array, index, .. } => {
                assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                assert!(matches!(*index, Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. }));
            }
            other => panic!("expected ArraySingleAccess, got {:?}", other),
        }
    }

    #[test]
    fn test_array_single_access_expression_index() {
        match parse("arr[i + 1]").unwrap() {
            Expr::ArraySingleAccess { index, .. } => {
                assert!(matches!(*index, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected ArraySingleAccess, got {:?}", other),
        }
    }

    // Array access, array slicing / multiple access.

    #[test]
    fn test_array_slice_both_bounds() {
        match parse("arr[1:5]").unwrap() {
            Expr::ArrayMultipleAccess { start, end, .. } => {
                assert!(start.is_some());
                assert!(end.is_some());
            }
            other => panic!("expected ArrayMultipleAccess, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_start_only() {
        match parse("arr[1:]").unwrap() {
            Expr::ArrayMultipleAccess { start, end, .. } => {
                assert!(start.is_some());
                assert!(end.is_none());
            }
            other => panic!("expected ArrayMultipleAccess, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_end_only() {
        match parse("arr[:5]").unwrap() {
            Expr::ArrayMultipleAccess { start, end, .. } => {
                assert!(start.is_none());
                assert!(end.is_some());
            }
            other => panic!("expected ArrayMultipleAccess, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_both_empty_errors() {
        assert_parse_err("arr[:]");
    }

    // Complex / compund expressions
    //
    #[test]
    fn test_binop_with_function_call() {
        match parse("foo(1) + 2").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, .. } => {
                assert!(matches!(*left, Expr::Call { name, .. } if name == "foo"));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn test_negate_function_result() {
        match parse("-foo(1)").unwrap() {
            Expr::UnaryOp { op: UnaryOpKind::Negate, expr, .. } => {
                assert!(matches!(*expr, Expr::Call { name, .. } if name == "foo"));
            }
            other => panic!("expected UnaryOp, got {:?}", other),
        }
    }

    #[test]
    fn test_comparison_between_calls() {
        match parse("foo(1) == bar(2)").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, left, right, .. } => {
                assert!(matches!(*left, Expr::Call { name, .. } if name == "foo"));
                assert!(matches!(*right, Expr::Call { name, .. } if name == "bar"));
            }
            other => panic!("expected BinOp Equal, got {:?}", other),
        }
    }

    #[test]
    fn test_array_access_in_binop() {
        match parse("arr[0] + 1").unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, .. } => {
                assert!(matches!(*left, Expr::ArraySingleAccess { .. }));
            }
            other => panic!("expected BinOp with ArraySingleAccess on left, got {:?}", other),
        }
    }

    #[test]
    fn test_binop_of_string_and_var() {
        // This will parse structurally even if semantics would reject it
        match parse(r#""hello" + name"#).unwrap() {
            Expr::BinOp { op: BinOpKind::Add, left, right, .. } => {
                assert!(matches!(*left, Expr::StringLiteral { .. }));
                assert!(matches!(*right, Expr::Var { .. }));
            }
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn test_deeply_nested_parens() {
        // (((x))) should resolve to a Var
        match parse("(((x)))").unwrap() {
            Expr::Var { name, .. } => assert_eq!(name, "x"),
            other => panic!("expected Var, got {:?}", other),
        }
    }


    // Many spacing variants should parse identically
    // i.e. 1+2,  1 + 2, 1+ 2, 1 +2, 2   * 1, etc.
    #[test]
    fn test_whitespace_around_and_within_operators_literals() {
        // just a helper so i don't have spam code with it over and over again.
        fn checker(variant: &str, b: BinOpKind) {
            match parse(variant).unwrap() {
                Expr::BinOp { op, left, right, .. } => {
                    assert_eq!(op, b.clone());
                    assert!(matches!(*left, Expr::IntLiteral { value: IntLiteralValue::Int8(1), .. }));
                    assert!(matches!(*right, Expr::IntLiteral { value: IntLiteralValue::Int8(2), .. }));
                }
                other => panic!("expected BinOp, got {:?}", other),
            }
        }

        for i in 0..1000 {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let variant = &format!("{}1{}2", " ".repeat(i), s);
                checker(variant, b.clone());
               
                let variant = &format!("1{}2{}", s, " ".repeat(i));
                checker(variant, b.clone());

                let variant = &format!("1{}{}2", s, " ".repeat(i));
                checker(variant, b.clone());

                let variant = &format!("1{}{}2", " ".repeat(i), s);
                checker(variant, b.clone());

            }
        }
    }


    // Same as above test, except its for variables
    #[test]
    fn test_whitespace_around_and_within_operators_vars() {
        // just a helper so i don't have spam code with it over and over again.
        fn checker(variant: &str, b: BinOpKind) {
            match parse(variant).unwrap() {
                Expr::BinOp { op, left, right, .. } => {
                    assert_eq!(op, b.clone());
                    assert!(matches!(*left, Expr::Var { name, .. } if name == "x" ));
                    assert!(matches!(*right, Expr::Var { name, ..} if name == "y" ));

                }
                other => panic!("expected BinOp, got {:?}", other),
            }
        }

        for i in 0..1000 {
            for (b, s) in ALL_BIN_OP_KIND.iter().zip(BIN_OP_KIND_SYMBOLS.iter()) {
                let variant = &format!("{}x{}y", " ".repeat(i), s);
                checker(variant, b.clone());

                let variant = &format!("x{}y{}", s, " ".repeat(i));
                checker(variant, b.clone());

                let variant = &format!("x{}{}y", s, " ".repeat(i));
                checker(variant, b.clone());

                let variant = &format!("x{}{}y", " ".repeat(i), s);
                checker(variant, b.clone());


            }
        }
    }


    // Edge cases testing

    #[test]
    fn test_format_triple_brace_escaped_plus_placeholder() {
        // "{{{x}}}" should parse to template="{{{}}}", expressions=["x"]
        match parse(r#"format("{{{x}}}")"#).unwrap() {
            Expr::FormatCall { template, expressions, .. } => {
                assert_eq!(template, "{{{}}}");
                assert_eq!(expressions.len(), 1);
                assert!(matches!(&expressions[0], Expr::Var { name, .. } if name == "x"));
            }
            other => panic!("expected FormatCall, got {:?}", other),
        }
    }

    #[test]
    fn test_format_only_escaped_braces_no_placeholder_errors() {
        // "{{x}}" is purely escaped, with no actual placeholder
        assert_parse_err(r#"format("{{x}}")"#);
    }

    #[test]
    fn test_copy_of_array_single_access() {
        match parse("copy(arr[0])").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::ArraySingleAccess { .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    #[test]
    fn test_copy_of_array_multiple_access() {
        match parse("copy(arr[0:2])").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::ArrayMultipleAccess { .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }


    #[test]
    fn test_function_call_with_array_literal_arg() {
        match parse("foo(int32[1, 2])").unwrap() {
            Expr::Call { name, args, .. } => {
                assert_eq!(name, "foo");
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::ArrayLiteral { array_ty: Type::Int32, .. }));
            }
            other => panic!("expected Call, got {:?}", other),
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

    #[test]
    fn test_binop_between_bool_literals() {
        match parse("true == false").unwrap() {
            Expr::BinOp { op: BinOpKind::Equal, left, right, .. } => {
                assert!(matches!(*left, Expr::BoolLiteral { value: true, .. }));
                assert!(matches!(*right, Expr::BoolLiteral { value: false, .. }));
            }
            other => panic!("expected BinOp Equal, got {:?}", other),
        }
    }

    #[test]
    fn test_copy_call_with_binop() {
        match parse("copy(x + 1)").unwrap() {
            Expr::CopyCall { expr, .. } => {
                assert!(matches!(*expr, Expr::BinOp { op: BinOpKind::Add, .. }));
            }
            other => panic!("expected CopyCall, got {:?}", other),
        }
    }

    fn test_int_256_does_not_fit_byte_promotes_to_int16() {
        assert_int_literal("256", IntLiteralValue::Int16(256));
    }
}
