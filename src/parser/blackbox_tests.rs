use super::*;

use crate::consts;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_BIN_OP_KIND,
    ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_ARTH,
    BIN_OP_KIND_ARTH_SYMBOLS, BIN_OP_KIND_COMP_SYMBOLS,
    BIN_OP_KIND_SYMBOLS,
};


mod const_decl_tests;
mod var_decl_tests;
mod var_assign_tests;

mod int_literal_tests;
mod float64_literal_tests;
mod bool_literal_tests;
mod string_literal_tests;

mod array_literal_tests;
mod array_access_tests;
mod array_slicing_tests;


mod bin_op_tests;
mod unary_op_tests;


mod function_tests;
mod function_call_tests;

mod copy_call_tests;
mod format_call_tests;

mod infinite_stmt_tests;
mod while_stmt_tests;
mod for_stmt_tests;
mod if_stmt_tests;
mod return_stmt_tests;

mod comment_tests;
 

// Tests helper functions

/// Wraps a statement in a minimal `func main() { ... }` 
fn wrap(body: &str) -> String {
    format!("func main() {{\n{}\n}}", body)
}

/// Parse a single-function source and return the body statements.
fn parse_body(body: &str) -> Vec<Stmt> {
    let src = wrap(body);
    let ast = parse(&src).expect("parse failed");

    // Basic global asserts.
    // NOTE: Make sure to update these asserts if you update the wrap function.
    assert_eq!(ast.functions.len(), 1);
    assert_eq!(ast.functions[0].params.len(), 0);
    assert_eq!(ast.functions[0].name, "main");
    assert!(ast.functions[0].return_type.is_none());
    assert!(ast.functions[0].body.len() >= 1);

    ast.functions[0].body.clone()
}

/// Assert that parsing fails (returns an Err).
fn assert_parse_err(src: &str) {
    assert!(
        parse(src).is_err(),
        "Expected parse error for: {:?}",
        src
    );
}

// all literals (ints, floats, array, strings literals of ints, floats, strings, other arrays, and variable names, and array access and slicing.
fn get_all_literals_edge_cases() -> [String; 42] {
    return [
        i8::MIN.to_string(), i8::MAX.to_string(),
        i16::MIN.to_string(), i16::MAX.to_string(),
        i32::MIN.to_string(), i32::MAX.to_string(),
        i64::MIN.to_string(), i64::MAX.to_string(),
        i128::MIN.to_string(), i128::MAX.to_string(),
        
        u8::MIN.to_string(), u8::MAX.to_string(),
        u16::MIN.to_string(), u16::MAX.to_string(),
        u32::MIN.to_string(), u32::MAX.to_string(),
        u64::MIN.to_string(), u64::MAX.to_string(),
        u128::MIN.to_string(), u128::MAX.to_string(),
        usize::MIN.to_string(), usize::MAX.to_string(),
        
        format!("{}.0", f64::MIN.to_string()), format!("{}.0", f64::MAX.to_string()), 

        "false".to_string(), "true".to_string(),
        "\"\"".to_string(), "\"h\"".to_string(), "\"hi\"".to_string(),
        "i".to_string(), "arr".to_string(), "x".to_string(), "y".to_string(), "xyz".to_string(),
        "arr[i]".to_string(), "arr[:i]".to_string(), "arr[i:]".to_string(), "arr[e:h]".to_string(),

        "idk()".to_string(), "idk(1)".to_string(), "idk(false, \"hi\")".to_string(), "idk(lol())".to_string()
    ]
}


// all integer literals
fn get_all_ints_literals_edge_cases() -> [String; 22] {
    return [
        i8::MIN.to_string(), i8::MAX.to_string(),
        i16::MIN.to_string(), i16::MAX.to_string(),
        i32::MIN.to_string(), i32::MAX.to_string(),
        i64::MIN.to_string(), i64::MAX.to_string(),
        i128::MIN.to_string(), i128::MAX.to_string(),
        
        u8::MIN.to_string(), u8::MAX.to_string(),
        u16::MIN.to_string(), u16::MAX.to_string(),
        u32::MIN.to_string(), u32::MAX.to_string(),
        u64::MIN.to_string(), u64::MAX.to_string(),
        u128::MIN.to_string(), u128::MAX.to_string(),
        usize::MIN.to_string(), usize::MAX.to_string(),
    ]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_source() {
        let ast = parse("").unwrap();
        assert!(ast.functions.is_empty());
        assert!(ast.globals.is_empty());
    }

    #[test]
    fn parse_comments_and_blanks_only() {
        let ascii_printable: Vec<char> = (32u8..=126).map(|b| b as char).collect();
         
        for l in ascii_printable {
            let ast = parse(&format!("# {}comment {}\n#{} Another comment {}", l, l, l, l)).unwrap();
            assert!(ast.functions.is_empty());
            assert!(ast.globals.is_empty());
        }
    }

    // Should be fine
    // it's up to the semantics phase to detect this illegal statement
    //
    #[test]
    fn parse_statement_outside_function_passes() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        
        for lit in &literals_edge_cases {
            for l in &letters {
                let ast = parse(&format!("{}", l)).unwrap();
                
                assert!(ast.functions.is_empty());
                assert_eq!(ast.globals.len(), 1);
                
                for t in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("own {} {} = {}", l, t, lit)).unwrap();

                    assert!(ast.functions.is_empty());
                    assert_eq!(ast.globals.len(), 1);
                
                    if let Stmt::VarDecl(v) = &ast.globals[0] {
                        assert_eq!(v.name, l.to_string());
                        assert_eq!(v.type_name, t.clone());
                        assert!(v.value.is_some());

                    } else { panic!("Expected VarDecl, instead got: {:?}", ast.globals[0]); }

                }
            }
        }
    }

    // Span tracking

    #[test]
    fn span_is_correct() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let src = format!("func main() {{\n\n\nown x {} = {}\n}}", t, l);
                let ast = parse(&src).unwrap();
                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.globals.len(), 0);
                
                assert_eq!(ast.functions[0].body.len(), 1);

                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());

                    // Line 4 in the source (1-indexed)
                    assert_eq!(v.span.line, 4);

                    // Column 0 (span column tracking still not implemented yet for error messages)
                    assert_eq!(v.span.column, 0);
                } else { panic!("Expected VarDecl, instead we got {:?}", &ast) }
            }    
        }
    }



    // Empty expression / Invalid syntax / edge-case errors

    #[test]
    fn empty_expression_in_call_arg_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
        
        for lit in &literals_edge_cases {
            // Ensure we don't silently accept malformed call
            assert_parse_err(&wrap(&format!("foo(,)")));
            assert_parse_err(&wrap(&format!("foo({},)", lit)));
            assert_parse_err(&wrap(&format!("foo(,{})", lit)));
            assert_parse_err(&wrap(&format!("foo(,{},)", lit)));
            assert_parse_err(&wrap(&format!("foo({},{},)", lit, lit)));


            for t in ALL_TYPES_NO_ARR {
                for l in &letters {
                    // Ensure we don't silently accept malformed call
                    assert_parse_err(&wrap(&format!("own {} {} = foo(,)", l, t)));
                    assert_parse_err(&wrap(&format!("own {} {} = foo({},)", l, t, lit)));
                    assert_parse_err(&wrap(&format!("own {} {} = foo(,{})", l, t, lit)));
                    assert_parse_err(&wrap(&format!("own {} {} = foo(,{},)", l, t, lit)));
                    assert_parse_err(&wrap(&format!("own {} {} = foo({},{},)", l, t, lit, lit)));
                }
            }
        }
    }
}



