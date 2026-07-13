use super::*;
use crate::parser::parse_expr::parse_expr;
use crate::ast::{
    IntLiteralValue, BinOpKind, UnaryOpKind, ArraySliceRange
};
use crate::tests_consts::{
    ALL_BIN_OP_KIND, BIN_OP_KIND_SYMBOLS
};


mod int_literal_tests;
mod float64_literal_tests;
mod bool_literal_tests;
mod string_literal_tests;
mod char_literal_tests;
mod array_literal_tests;

mod array_access_tests;
mod array_slicing_tests;

mod vars_tests;

mod unary_op_tests;
mod bin_op_tests;

mod parentheses_grouping_tests;

mod function_call_tests;
mod copy_call_tests;
mod format_call_tests;



// Test helper functions
//

// all literals (ints, floats, array, strings literals of ints, floats, strings, other arrays, and variable names, and array access and slicing).
// Some of these literals are illegal semantically, but syntaxally, should be valid.
//
fn get_all_literals_edge_cases() -> [String; 163] {
    return [
        i8::MIN.to_string(), i8::MAX.to_string(),
        i16::MIN.to_string(), i16::MAX.to_string(),
        i32::MIN.to_string(), i32::MAX.to_string(),
        i64::MIN.to_string(), i64::MAX.to_string(),
        i128::MIN.to_string(), i128::MAX.to_string(),
        
        u8::MIN.to_string(), u8::MAX.to_string(),
        u16::MAX.to_string(),
        u32::MAX.to_string(),
        u64::MAX.to_string(),
        u128::MAX.to_string(),
        usize::MAX.to_string(),
        
        "0.0".to_string(), format!("{}.0", f64::MIN.to_string()), format!("{}.0", f64::MAX.to_string()), 

        "false".to_string(), "true".to_string(),
        "\"\"".to_string(), "\"h\"".to_string(), "\"hi\"".to_string(),
        "\"hi, lmao\"".to_string(),  "\"hi ] lmao\"".to_string(), "\"hi [ lmao\"".to_string(), 
        "\"foo]\"".to_string(), "\"foo[\"".to_string(),
        "\"]foo\"".to_string(), "\"[foo\"".to_string(),
        "\"]foo]\"".to_string(), "\"[foo[\"".to_string(),
        "\"[foo]\"".to_string(),
        "\"foo\\\"]\"".to_string(), "\"foo\\\"[\"".to_string(),
        "\"\\\"]foo\"".to_string(), "\"\\\"[foo[\"".to_string(),
        "\"foo)\"".to_string(), "\"foo(\"".to_string(),
        "\")foo\"".to_string(), "\"(foo\"".to_string(),

        "\"\\n\"".to_string(), "\"\\t\"".to_string(), "\"\\r\"".to_string(),  "\"\\0\"".to_string(),
        "\"\\\\\"".to_string(), "\"\\'\"".to_string(), "\"\\\"\"".to_string(),
        
        "\"\\tfoo\\n]\"".to_string(), "\"\\tfoo\\n[\"".to_string(),
        
        "'a'".to_string(), "'F'".to_string(),
        "'😇'".to_string(), "'🥰'".to_string(),
        "'\\n'".to_string(), "'\\t'".to_string(), "'\\r'".to_string(), "'\\0'".to_string(), 
        "'\\\\'".to_string(), "'\\''".to_string(),

        "i".to_string(), "arr".to_string(), "x".to_string(), "y".to_string(), "xyz".to_string(),
        
        format!("arr[{}]", i8::MIN.to_string()), format!("arr[{}]", i8::MAX.to_string()), 
        format!("arr[:{}]", i8::MIN.to_string()), format!("arr[:{}]", i8::MAX.to_string()), 
        format!("arr[{}:]", i8::MIN.to_string()), format!("arr[{}:]", i8::MAX.to_string()), 
        format!("arr[{0}:{0}]", i8::MIN.to_string()), format!("arr[{0}:{0}]", i8::MAX.to_string()), 

        format!("arr[{}]", usize::MIN.to_string()), format!("arr[{}]", usize::MAX.to_string()), 
        format!("arr[:{}]", usize::MIN.to_string()), format!("arr[:{}]", usize::MAX.to_string()), 
        format!("arr[{}:]", usize::MIN.to_string()), format!("arr[{}:]", usize::MAX.to_string()), 
        format!("arr[{0}:{0}]", usize::MIN.to_string()), format!("arr[{0}:{0}]", usize::MAX.to_string()), 
        

        "arr[i]".to_string(), "arr[:i]".to_string(), "arr[i:]".to_string(), "arr[e:h]".to_string(),
        
        format!("idk(arr[eh[{} / j]:ah[{} + f]])", u32::MAX.to_string(), i128::MIN.to_string()), 
        "idk()".to_string(), 
        format!("idk({})", i8::MIN.to_string()), format!("idk({})", i8::MAX.to_string()), 
        format!("idk({})", i16::MIN.to_string()), format!("idk({})", i16::MAX.to_string()), 
        format!("idk({})", i32::MIN.to_string()), format!("idk({})", i32::MAX.to_string()), 
        format!("idk({})", i64::MIN.to_string()), format!("idk({})", i64::MAX.to_string()), 
        format!("idk({})", i128::MIN.to_string()), format!("idk({})", i128::MAX.to_string()), 

        format!("idk({})", u8::MIN.to_string()), format!("idk({})", u8::MAX.to_string()), 
        format!("idk({})", u16::MAX.to_string()),
        format!("idk({})", u32::MAX.to_string()), 
        format!("idk({})", u64::MAX.to_string()), 
        format!("idk({})", u128::MAX.to_string()), 
        format!("idk({})", usize::MAX.to_string()), 


        format!("idk({0}, {0}, {0})", i8::MIN.to_string()), format!("idk({0}, {0}, {0})", i8::MAX.to_string()), 
        format!("idk({0}, {0}, {0})", i16::MIN.to_string()), format!("idk({0}, {0}, {0})", i16::MAX.to_string()), 
        format!("idk({0}, {0}, {0})", i32::MIN.to_string()), format!("idk({0}, {0}, {0})", i32::MAX.to_string()), 
        format!("idk({0}, {0}, {0})", i64::MIN.to_string()), format!("idk({0}, {0}, {0})", i64::MAX.to_string()), 
        format!("idk({0}, {0}, {0})", i128::MIN.to_string()), format!("idk({0}, {0}, {0})", i128::MAX.to_string()), 

        "idk(false)".to_string(), "idk(true)".to_string(),
        format!("idk({})", f64::MIN.to_string()), format!("idk({})", u64::MAX.to_string()), 

        "idk(\"hi\")".to_string(), 
        "idk(\"\")".to_string(), "idk(\"h\")".to_string(),
        "idk(i)".to_string(), "idk(arr)".to_string(), "idk(x)".to_string(), "idk(y)".to_string(), "idk(xyz)".to_string(),
        "idk(arr[i])".to_string(), "idk(arr[:i])".to_string(), "idk(arr[i:])".to_string(), "idk(arr[e:h])".to_string(),
        
        "idk(false, \"hi\")".to_string(), "idk(lol())".to_string(), "idk(idk())".to_string(),

        "idk([1, 2, 3])".to_string(), "idk([-1, -2, -3])".to_string(), "idk([-1, 2, -3])".to_string(), "idk([1, -2, 3])".to_string(),
        "idk([1.0, 2.0, 3.0])".to_string(), "idk([-1.0, -2.0, -3.0])".to_string(), "idk([-1.0, 2.0, -3.0])".to_string(), "idk([1.0, -2.0, 3.0])".to_string(),
        "idk([1, 2.0, 3])".to_string(), "idk([-1, -2.0, -3])".to_string(), "idk([-1, 2.0, -3])".to_string(), "idk([1, -2.0, 3])".to_string(),

        "idk([1, 2.0, \"hi\", false, -0, heh()])".to_string(),
        "idk([1, 2.0, \"hi\", false, -0, heh([1, 2.0, \"hi\", false, -0, heh()])])".to_string(),


        "[1, 2, 3]".to_string(), "[-1, -2, -3]".to_string(), "[-1, 2, -3]".to_string(), "[1, -2, 3]".to_string(),
        "[1.0, 2.0, 3.0]".to_string(), "[-1.0, -2.0, -3.0]".to_string(), "[-1.0, 2.0, -3.0]".to_string(), "[1.0, -2.0, 3.0]".to_string(),
        "[1, 2.0, 3]".to_string(), "[-1, -2.0, -3]".to_string(), "[-1, 2.0, -3]".to_string(), "[1, -2.0, 3]".to_string(),

        "[1, 2.0, \"hi\", false, -0, heh()]".to_string(),
        "[1, 2.0, \"hi\", false, -0, heh([1, 2.0, \"hi\", false, -0, heh()])]".to_string(),
    ]
}

// all integer literals
fn get_all_non_string_literals_edge_cases() -> [String; 39] {
    return [
        i8::MIN.to_string(), i8::MAX.to_string(),
        i16::MIN.to_string(), i16::MAX.to_string(),
        i32::MIN.to_string(), i32::MAX.to_string(),
        i64::MIN.to_string(), i64::MAX.to_string(),
        i128::MIN.to_string(), i128::MAX.to_string(),
        
        u8::MIN.to_string(), u8::MAX.to_string(),
        u16::MAX.to_string(),
        u32::MAX.to_string(),
        u64::MAX.to_string(),
        u128::MAX.to_string(),
        usize::MAX.to_string(),
        
        
        f64::MIN.to_string(), f64::MAX.to_string(),
        "false".to_string(), "true".to_string(),
        "[]".to_string(), "()".to_string(), "{}".to_string(), 
        "[".to_string(), "]".to_string(), "][".to_string(), "]]".to_string(), "[[".to_string(), 
        "(".to_string(), ")".to_string(), ")(".to_string(), "))".to_string(), "((".to_string(), 
        "{".to_string(), "}".to_string(), "}{".to_string(), "}}".to_string(), "{{".to_string(), 
    ]
}



fn span() -> Span {
    Span { line: 1, column: 1 }
}

fn parse(s: &str) -> Result<Expr, GoldError> {
    parse_expr(s, span())
}


fn assert_parse_err(s: &str) {
    assert!(
        parse(s).is_err(),
        "Expected parse error for input {:?}, but got Ok",
        s
    );
}

fn assert_int_literal(s: &str, expected: IntLiteralValue) {
    match parse(s).unwrap_or_else(|e| panic!("expected Ok for {:?}\nerror: {:?}", s, e)) {
        Expr::IntLiteral { value, .. } => assert_eq!(value, expected, "input: {:?}", s),
        other => panic!("expected IntLiteral for {:?}, got {:?}", s, other),
    }
}

#[cfg(test)]
mod parse_expr_tests {
    use super::*;

    #[test]
    fn test_empty_expression_errors() {
        assert_parse_err("");
    }

    #[test]
    fn test_whitespace_and_newlines_errors() {
        let mut s1 = String::with_capacity(5000);
        let mut s2 = String::with_capacity(5000);
        let mut s3 = String::with_capacity(5000);
        let mut s4 = String::with_capacity(5000);
        let mut s5 = String::with_capacity(15000);
        let mut s6 = String::with_capacity(20000);

        for _ in 1..5000 {
            s1.push(' ');
            s2.push('\t');
            s3.push('\n');
            s4.push('\r');
            s5.push_str(" \n\r");
            s6.push_str(" \n\r\t");

            assert_parse_err(&s1);
            assert_parse_err(&s2);
            assert_parse_err(&s3);
            assert_parse_err(&s4);
            assert_parse_err(&s5);
            assert_parse_err(&s6);
        }
    }

}
