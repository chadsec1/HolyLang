use super::*;

use crate::consts;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_ARTH,
    BIN_OP_KIND_ARTH_SYMBOLS, BIN_OP_KIND_COMP_SYMBOLS,
    BIN_OP_KIND_SYMBOLS,
};
 

// Test helper functions

/// Wrap a statement in a minimal `func main() { … }` so `parse()` can accept it.
fn wrap(body: &str) -> String {
    format!("func main() {{\n{}\n}}", body)
}

/// Parse a single-function source and return the body statements.
fn parse_body(body: &str) -> Vec<Stmt> {
    let src = wrap(body);
    let ast = parse(&src).expect("parse failed");
    assert_eq!(ast.functions.len(), 1);
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


// all literals and variable names.
fn get_all_literals_edge_cases() -> [String; 38] {
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
        "arr[i]".to_string(), "arr[:i]".to_string(), "arr[i:]".to_string(), "arr[e:h]".to_string()
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

    // empty / comment-only / outside-function errors

    #[test]
    fn parse_empty_source() {
        let ast = parse("").unwrap();
        assert!(ast.functions.is_empty());
    }

    #[test]
    fn parse_comments_and_blanks_only() {
        let src = "# comment\n\n# another\n";
        let ast = parse(src).unwrap();
        assert!(ast.functions.is_empty());
    }

    // Should be fine
    // it's up to the semantics phase to detect this illegal statement
    //
    #[test]
    fn parse_statement_outside_function_passes() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                assert!(parse(&format!("own x {} = {}", t, l)).is_ok());
            }
        }
    }

    // Functions

    #[test]
    fn parse_function_with_missing_opening_parenthesis_errors() {
        let result = parse("func main) {\n}\n");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid function header (no '(')"));
    }

    #[test]
    fn parse_function_with_missing_closing_parenthesis_errors() {
        let result = parse("func main( {\n}\n");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid function header (no ')')"));
    }


    #[test]
    fn parse_function_with_returns_missing_opening_parenthesis_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main) {} {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid function header (no '(')"));
        }
    }

    #[test]
    fn parse_function_with_returns_missing_closing_parenthesis_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main( {} {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid function header (no ')')"));
        }
    }

    #[test]
    fn parse_function_missing_opening_parenthesis_with_multiple_returns_errors() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                let result = parse(&format!("func main) ({}, {}) {{\n}}\n", t1, t2));

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Invalid function header: there is an extra closing parenthesis `)` in the function declaration header"));
            }
        }
    }


    #[test]
    fn parse_function_missing_closing_parenthesis_with_multiple_returns_errors() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                let result = parse(&format!("func main( ({}, {}) {{\n}}\n", t1, t2));

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Invalid parameter"));
            }
        }
    }



    #[test]
    fn parse_empty_function() {
        let ast = parse("func main() {\n}\n").unwrap();
        assert_eq!(ast.functions.len(), 1);
        let f = &ast.functions[0];
        assert_eq!(f.name, "main");
        assert!(f.params.is_empty());
        assert!(f.return_type.is_none());
        assert!(f.body.is_empty());
    }

    #[test]
    fn parse_function_with_params() {
        let ast = parse("func hello(a int32, b uint32, c usize) float64 {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.name, "hello");
        
        assert_eq!(f.return_type, Some(vec![Type::Float64]));

        assert_eq!(f.params.len(), 3);
        assert_eq!(f.params[0].name, "a");
        assert_eq!(f.params[0].type_name, Type::Int32);
        assert_eq!(f.params[1].name, "b");
        assert_eq!(f.params[1].type_name, Type::Uint32);
        assert_eq!(f.params[2].name, "c");
        assert_eq!(f.params[2].type_name, Type::Usize);
    }

    #[test]
    fn parse_function_single_return_type() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func foo() {} {{\n}}\n", t)).unwrap();
            let f = &ast.functions[0];

            assert_eq!(f.name, "foo");
            assert_eq!(f.params.len(), 0);
            assert_eq!(f.return_type, Some(vec![t.clone()]));
        }
    }

    #[test]
    fn parse_function_multi_return_type() {
        let ast = parse("func foo() (int32, bool) {\n}\n").unwrap();
        let f = &ast.functions[0];

        assert_eq!(f.name, "foo");
        assert_eq!(f.params.len(), 0);
        assert_eq!(f.return_type, Some(vec![Type::Int32, Type::Bool]));
    }

    #[test]
    fn parse_function_no_return_type() {
        let ast = parse("func noop() {\n}\n").unwrap();
        let f = &ast.functions[0];

        assert_eq!(f.name, "noop");
        assert_eq!(f.params.len(), 0);
        assert!(f.return_type.is_none());
    }

    #[test]
    fn parse_function_missing_open_paren_errors() {
        assert_parse_err("func bad {\n}\n");
    }

    #[test]
    fn parse_function_missing_brace_errors() {
        assert_parse_err("func bad()\n");
    }

    #[test]
    fn parse_function_unterminated_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("func bad() {{\n own x {} = 1\n", t));
        }
    }

    #[test]
    fn parse_function_keyword_name_errors() {
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("func {}() {{\n}}\n", kw));
        }
    }

    #[test]
    fn parse_function_space_in_name_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("func bad name() {{own x {} = 1\n}}\n", t));
        }
    }

    #[test]
    fn parse_function_inline_statements_in_braces_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("func bad() {{own x {} = 1\n}}\n", t));
            
            assert_parse_err(&format!("func bad() {{\nown x {} = 1}}\n", t));
        }
    }

    #[test]
    fn parse_multiple_functions() {
        let src = "func a() {\n}\nfunc b() {\n}\n";
        let ast = parse(src).unwrap();
        assert_eq!(ast.functions.len(), 2);
        assert_eq!(ast.functions[0].name, "a");
        assert_eq!(ast.functions[0].params.len(), 0);
        assert!(ast.functions[0].return_type.is_none());
        
        assert_eq!(ast.functions[1].name, "b");
        assert_eq!(ast.functions[1].params.len(), 0);
        assert!(ast.functions[1].return_type.is_none());
    }

    #[test]
    fn parse_function_array_return_type() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func foo() []{} {{\n}}\n", t)).unwrap();
            let f = &ast.functions[0];
            assert_eq!(f.return_type, Some(vec![Type::Array(Box::new(t.clone()))]));
        }
    }

    #[test]
    fn parse_function_nested_array_return_type() {
        for t in ALL_TYPES_NO_ARR {
            let mut s1 = String::with_capacity(200);

            for i in 1..100 {
                s1.push_str("[]");
                let ast = parse(&format!("func foo() []{}{} {{\n}}\n", s1, t)).unwrap();
                let f = &ast.functions[0];

                assert_eq!(f.return_type.clone().unwrap().len(), 1);

                let mut inner_ty = f.return_type.clone().unwrap()[0].clone();
                
                let mut arr_count = 0;

                while let Type::Array(inner) = inner_ty {
                    arr_count += 1;
                    inner_ty = *inner;
                }

                assert_eq!(arr_count - 1, i, "Array count is different from source");
                
                assert_eq!(inner_ty, t.clone());
            }
        }
    }

    // For statements
    #[test]
    fn for_statements_vars() {
        let stmts = parse_body("for i in x {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::Var { name, .. } = &f.value {
                assert_eq!(name, "x"); 
            } else { panic!("Expected Var expression") }
        } else {
            panic!("expected while statement");
        }
    }

    #[test]
    fn for_statements_literal() {
        let stmts = parse_body("for i in [12,\"hi\", true, 6.9, []] {\n\n}");
        assert_eq!(stmts.len(), 1);
        if let Stmt::For(f) = &stmts[0] {
            assert_eq!(f.holder_name, "i");
            assert_eq!(f.branch.len(), 0);

            if let Expr::ArrayLiteral { elements, .. } = &f.value {
                assert_eq!(elements.len(), 5);

                if let Expr::IntLiteral { value, .. } = &elements[0] {
                    assert!(matches!(value, IntLiteralValue::Int8(12)));
                } else { panic!("Expected IntLiteral"); }

                if let Expr::StringLiteral { value, .. } = &elements[1] {
                    assert_eq!(value, "hi");
                } else { panic!("Expected StringLiteral"); }

                if let Expr::BoolLiteral { value, .. } = &elements[2] {
                    assert_eq!(value, &true);
                } else { panic!("Expected BoolLiteral"); }

                if let Expr::Float64Literal { value, .. } = &elements[3] {
                    assert_eq!(*value, 6.9);
                } else { panic!("Expected Float64Literal"); }

                if let Expr::ArrayLiteral { elements, .. } = &elements[4] {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }


            } else {
                panic!("Expected ArrayLiteral");
            }

        } else {
            panic!("expected while statement");
        }
    }


    #[test]
    fn for_statements_2_holders_errors() {
        assert_parse_err(&wrap("for i v in x {\n\n}"));    
    }


    #[test]
    fn for_statements_2_values_errors() {
        assert_parse_err(&wrap("for i in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_2_holders_and_values_errors() {
        assert_parse_err(&wrap("for i v in x y {\n\n}"));    
    }

    #[test]
    fn for_statements_no_value_errors() {
        assert_parse_err(&wrap("for i in {\n\n}"));    
    }


    #[test]
    fn for_statements_no_holder_errors() {
        assert_parse_err(&wrap("for in x {\n\n}"));    
    }

    #[test]
    fn for_statements_2_in() {
        assert_parse_err(&wrap("for i in in x {\n\n}"));    
        assert_parse_err(&wrap("for in i in x {\n\n}"));    
        assert_parse_err(&wrap("for i in x in {\n\n}"));    
        assert_parse_err(&wrap("for in i x in {\n\n}"));    
        assert_parse_err(&wrap("for i x in {\n\n}"));    
    }

    #[test]
    fn for_statements_no_in() {
        assert_parse_err(&wrap("for i x {\n\n}"));    
    }

    #[test]
    fn for_statements_no_holder_no_value_no_in_errors() {
        assert_parse_err(&wrap("for {\n\n}"));    
    }


    // Infinite statements

    #[test]
    fn infinite_statements_invalid_construction_errors() {
        assert_parse_err(&wrap("infinite x {\n\n}"));    
        assert_parse_err(&wrap("infinite range(1, 10) {\n\n}"));    
        assert_parse_err(&wrap("infinite range() {\n\n}"));    
        assert_parse_err(&wrap("infinite range {\n\n}"));    
        assert_parse_err(&wrap("infinite infinite {\n\n}"));    
        assert_parse_err(&wrap("infinite i in x {\n\n}"));    
        assert_parse_err(&wrap("infinite in x {\n\n}"));    
        assert_parse_err(&wrap("infinite i in {\n\n}"));
        assert_parse_err(&wrap("infinite true {\n\n}"));
        assert_parse_err(&wrap("infinite false {\n\n}"));    
        assert_parse_err(&wrap("infinite 1 {\n\n}")); 
        assert_parse_err(&wrap("infinite 1.0 {\n\n}")); 
        assert_parse_err(&wrap("infinite \"\" {\n\n}"));    
        assert_parse_err(&wrap("infinite {\n\n"));    
        assert_parse_err(&wrap("infinite {}"));    
        assert_parse_err(&wrap("infinite \n\n}"));    

        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&wrap(&format!("infinite {} {{\n\n}}", kw)));    
        }
    }

    #[test]
    fn infinite_statements_valid_construction() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            let stmts = parse_body(&format!("infinite {} {{\n\n}}", spaces));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }
            spaces.push(' ');

        }
    }


    // While statements
    
    #[test]
    fn while_statements_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 1 {} 2 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }

    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 1 {} 2 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 1 {} 2 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);
                        
                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(1)));
                        } else { panic!(); }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(2)));
                        } else { panic!(); }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    

    #[test]
    fn while_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }

                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }

    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }

    #[test]
    fn while_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while 69 {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(69)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("while x {} 67 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::While(w) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &w.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(67)));
                    } else { panic!(); }
                } else { panic!("Expected BinOp"); }
                
                assert_eq!(w.branch.len(), 0);
            } else {
                panic!("expected while statement");
            }
        }
    }


    // Same test as above, but before the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_before_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 69 {} y {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} 67 {{\n\n}}", spaces, s));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }

            spaces.push(' ');
        }
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_after_expr() {
        const MAX_SPACES: usize = 1000;

        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 69 {} y {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {

                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert!(matches!(value, IntLiteralValue::Int8(69)));
                        } else { panic!(); }

                        if let Expr::Var { name, .. } = &**right {
                            assert_eq!(name, "y"); 
                        } else { panic!("Expected Var expression") }
                    
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }


            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} 67 {}{{\n\n}}", s, spaces));
                assert_eq!(stmts.len(), 1);
                if let Stmt::While(w) = &stmts[0] {
                    if let Expr::BinOp { left, right, op, .. } = &w.condition {
                        assert_eq!(op, b);

                        if let Expr::Var { name, .. } = &**left {
                            assert_eq!(name, "x"); 
                        } else { panic!("Expected Var expression") }

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert!(matches!(value, IntLiteralValue::Int8(67)));
                        } else { panic!(); }
                    } else { panic!("Expected BinOp"); }
                    
                    assert_eq!(w.branch.len(), 0);
                } else {
                    panic!("expected while statement");
                }
            }
            spaces.push(' ');
        }
    }






    #[test]
    fn while_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}", spaces)));
            spaces.push(' ');
        }
    }



    // If statements 

    #[test]
    fn if_statements_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }
 
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_vars() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} y {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} 10 {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(10)));
                    } else { panic!(); }

                } else { panic!() }
                
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }


        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 10 {} x {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(10)));
                    } else { panic!(); }
 
                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }
                
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }


    }



    #[test]
    fn if_statements_with_else() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} else {{\n\n}}", s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {

                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }
                } else { panic!() }
                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 0);
                assert!(i.else_branch.is_some());
            } else {
                panic!("expected if statement");
            }
        }
    }

    #[test]
    fn if_statements_with_elif_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} elif 5 {} 3 {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(3)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_with_elif_vars() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} y {{\n\n}} elif e {} a {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);


                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "e"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_with_elif_vars_and_literals() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 2 {} y {{\n\n}} elif 5 {} a {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "y"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);

                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::Var { name, .. } = &**right {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }



        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if x {} 6 {{\n\n}} elif a {} 9 {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {
                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "x"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(6)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);

                    if let Expr::Var { name, .. } = &**left {
                        assert_eq!(name, "a"); 
                    } else { panic!("Expected Var expression") }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(9)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                assert!(i.else_branch.is_none());
            } else {
                panic!("expected if statement");
            }
        }

    }





    #[test]
    fn if_statements_with_else_elif() {
        for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("if 1 {} 2 {{\n\n}} elif 5 {} 3 {{\n\n}} else {{\n\n}}", s, s));
            assert_eq!(stmts.len(), 1);
            if let Stmt::If(i) = &stmts[0] {
                if let Expr::BinOp { left, right, op, .. } = &i.condition {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(1)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(2)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }

                
                assert_eq!(i.if_branch.len(), 0);
                assert_eq!(i.elif_branches.len(), 1);


                let elif_cond = &i.elif_branches[0].0;
                if let Expr::BinOp { left, right, op, .. } = elif_cond {

                    assert_eq!(op, b);
                    
                    if let Expr::IntLiteral { value, .. } = **left {
                        assert!(matches!(value, IntLiteralValue::Int8(5)));
                    } else { panic!(); }

                    if let Expr::IntLiteral { value, .. } = **right {
                        assert!(matches!(value, IntLiteralValue::Int8(3)));
                    } else { panic!(); }

                } else { panic!("Expected BinOp") }



                assert!(i.else_branch.is_some());
            } else {
                panic!("expected if statement");
            }
        }
    }


    #[test]
    fn if_statements_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}", spaces)));    
            spaces.push(' ');
        }
    }

    #[test]
    fn if_statements_elif_no_condition_errors() {
        const MAX_SPACES: usize = 5000;
        
        let mut spaces = String::with_capacity(MAX_SPACES);
        for _ in 0..MAX_SPACES {
            assert_parse_err(&wrap(&format!("if 1 == 2 {{\n\n}} elif {}{{\n\n}}", spaces)));    
            spaces.push(' ');
        }

    }


    // Variable declarations

    #[test]
    fn var_decl_no_type_errors() {
        assert_parse_err(&wrap("own x = 1"));
    }

    #[test]
    fn var_decl() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 2", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
            } else {
                panic!("Expected VarDecl");
            }
        }
    }

    #[test]
    fn var_decl_no_value() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {}", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert!(v.value.is_none());
            } else {
                panic!("Expected VarDecl");
            }
        }
    }


    // Even though we do test all these types declarations, we never tested them in whole with their
    // respective literals. So it's worth double checking here again.
    #[test]
    fn var_decl_float64_type() {
        let stmts = parse_body("own y float64 = 1.0");
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "y");
            assert_eq!(v.type_name, Type::Float64);

            if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                assert_eq!(*value, 1.0);
            } else { panic!("Expected Float64Literal"); }
        } else { panic!("Expected VarDecl"); }    

    }

    #[test]
    fn var_decl_bool_type() {
        let stmts = parse_body("own x bool = true");
        assert_eq!(stmts.len(), 1);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::Bool);
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_string_type() {
        let stmts = parse_body(r#"own x string = "hello""#);
        assert_eq!(stmts.len(), 1);

        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::String);
        } else { panic!("Expected VarDecl"); }    
    }

    #[test]
    fn var_decl_array() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x []{} = [1, 2, 3]", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 3);
                } else {
                    panic!("Expected ArrayLiteral");
                }

            } else { panic!("Expected VarDecl");}
        }
    }

    #[test]
    fn var_decl_array_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x = [{}, {}, {}]", l, l, l)));
        }
    }

    #[test]
    fn var_decl_empty_array() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = []", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert!(elements.is_empty());
                } else {
                    panic!("Expected ArrayLiteral");
                }
            } else { panic!("Expected VarDecl");}
        }
    }

    #[test]
    fn var_decl_nested_array() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x []{} = [[{},{},{}], [{},{},{},{}]]", t, l, l, l, l, l, l, l));
                assert_eq!(stmts.len(), 1);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));

                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert_eq!(elements.len(), 2);
                        assert!(matches!(elements[0], Expr::ArrayLiteral { .. }));
                        assert!(matches!(elements[1], Expr::ArrayLiteral { .. }));
                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                } else { panic!("Expected VarDecl");}
            }
        }
    }

    #[test]
    fn var_decl_nested_array_empty() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = []", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }
            } else { panic!("Expected VarDecl");}
        }
    }


    #[test]
    fn var_decl_deeply_nested_array() {
        for t in ALL_TYPES_NO_ARR {
            let mut s1 = String::with_capacity(100);
            let mut s2 = String::with_capacity(100);

            for _ in 1..100 {
                s1.push_str("[");
                s2.push_str("]");
                let stmts = parse_body(&format!("own x {} = [{}{}]", t, s1, s2 ));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert_eq!(elements.len(), 1);

                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                }
            }
        }
    }


    #[test]
    fn var_decl_multi() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                for t3 in ALL_TYPES_NO_ARR {
                    let stmts = parse_body(&format!("own x {}, y {}, z {} = give_3_numbers()", t1, t2, t3));
                    assert!(matches!(stmts[0], Stmt::VarDeclMulti(_, _)));
                    if let Stmt::VarDeclMulti(vars, _) = &stmts[0] {
                        assert_eq!(vars.len(), 3);
                        assert_eq!(vars[0].name, "x");
                        assert_eq!(vars[0].type_name, t1.clone());
                        assert_eq!(vars[1].name, "y");
                        assert_eq!(vars[1].type_name, t2.clone());
                        assert_eq!(vars[2].name, "z");
                        assert_eq!(vars[2].type_name, t3.clone());
                    } else { panic!("Expected VarDeclMulti"); }
                }
            }
        }
    }

    #[test]
    fn var_decl_unknown_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        assert_parse_err(&wrap("own x badtype = 1"));
        assert_parse_err(&wrap("own x badtype"));
        assert_parse_err(&wrap("own x x = 1"));
        assert_parse_err(&wrap("own x x"));

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("own x {}", l)));
        }
    }

    #[test]
    fn var_decl_no_type_no_value_errors() {
        assert_parse_err(&wrap("own x"));
    }


    #[test]
    fn var_decl_keyword_name_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for kw in consts::RESERVED_KEYWORDS { 
            for t in ALL_TYPES_NO_ARR {
                for l in &literals_edge_cases {
                    assert_parse_err(&wrap(&format!("own {} {}", kw, t)));
                    assert_parse_err(&wrap(&format!("own {} {} = {}", kw, t, l)));
                }
            }
        }
    }

    // Variable assignment
    #[test]
    fn var_assign() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {}\nx = {}", t, l));
                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }    


                if let Stmt::VarAssign(va) = &stmts[1] {
                    assert_eq!(va.name, "x");
                } else {
                    panic!("Expected VarAssign");
                }
            }
        }
    }


    #[test]
    fn var_assign_multi() {
        let stmts = parse_body("x, y = swap()");
        if let Stmt::VarAssignMulti(ma) = &stmts[0] {
            assert_eq!(ma.names, vec!["x", "y"]);
        } else {
            panic!("Expected VarAssignMulti");
        }
    }

    // Return statements

    #[test]
    fn return_single_value() {
        let stmts = parse_body("return 42");
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 1);
        } else {
            panic!("Expected Return");
        }
    }

    #[test]
    fn return_multiple_values() {
        let stmts = parse_body("return 1, 2, 300, 69640");

        assert_eq!(stmts.len(), 1);
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 4);

            if let Expr::IntLiteral { value, .. } = &exprs[0] {
                assert!(matches!(value, IntLiteralValue::Int8(1)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[1] {
                assert!(matches!(value, IntLiteralValue::Int8(2)));
            } else { panic!("Expcted IntLiteral"); }

            if let Expr::IntLiteral { value, .. } = &exprs[2] {
                assert!(matches!(value, IntLiteralValue::Int16(300)));
            } else { panic!("Expcted IntLiteral"); }


            if let Expr::IntLiteral { value, .. } = &exprs[3] {
                assert!(matches!(value, IntLiteralValue::Int32(69640)));
            } else { panic!("Expcted IntLiteral"); }


        } else {
            panic!("Expected Return");
        }
    }

    #[test]
    fn return_without_value_errors() {
        assert_parse_err(&wrap("return"));
    }

    #[test]
    fn return_variable() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = {}\nreturn x", t, l));

                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }    

                if let Stmt::Return(exprs) = &stmts[1] {
                    assert_eq!(exprs.len(), 1);
                    assert!(matches!(exprs[0], Expr::Var { .. }));
                } else {
                    panic!("Expected Return");
                }
            }
        }
    }

    // Integer literals, correct type inferrence tests
    #[test]
    fn integer_literal_fits_int8() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1", t));
                
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int8(1)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int8_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 127 fits int8, 128 does not
            let stmts = parse_body(&format!("own a {} = 127\nown b {} = 128", t, t));
            
            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int8(127)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
            
            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(128)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int16() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 128", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(128)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int16_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 32767 fits int16, 32768 does not
            let stmts = parse_body(&format!("own a {} = 32767\nown b {} = 32768", t, t));

            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(32767)));
                } else { panic!("Expected IntLiteral"); }

            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(32768)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int32() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 32768", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(32768)));
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int32_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 2147483647 fits int32, 2147483648 does not
            let stmts = parse_body(&format!("own a {} = 2147483647\nown b {} = 2147483648", t, t));

            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(2147483647)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
            
            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(2147483648)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn integer_literal_fits_int64() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 2147483648", t));
            
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(2147483648)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int64_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 9223372036854775807 fits int64, 9223372036854775808 does not
            let stmts = parse_body(&format!("own a {} = 9223372036854775807\nown b {} = 9223372036854775808", t, t));
            
            assert_eq!(stmts.len(), 2);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(9223372036854775807)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(9223372036854775808)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int128() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 9223372036854775808", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(9223372036854775808)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        } 
    }

    #[test]
    fn integer_literal_int128_boundary() {
        // 170141183460469231731687303715884105727 fits int128,  170141183460469231731687303715884105728 does not
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own a {} = 170141183460469231731687303715884105727\nown b {} = 170141183460469231731687303715884105728", t, t));
            
            assert_eq!(stmts.len(), 2);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(170141183460469231731687303715884105727)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Uint128(170141183460469231731687303715884105728)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_uint128() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 340282366920938463463374607431768211455", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Uint128(340282366920938463463374607431768211455)));
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }



    #[test]
    fn integer_literal_negative() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = -128", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(-128), .. })))
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_overflow_u128_errors() {
        // A number larger than u128::MAX should produce a parse error
        let huge = "340282366920938463463374607431768211456"; // u128::MAX + 1
                                                              //
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = {}", t, huge)));
        }
    }

    // Float literals

    #[test]
    fn float_literal_f32() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1.0", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                    assert_eq!(*value, 1.0);
                } else { panic!("Expected Float64Literal"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn float_literal_f64_high_precision() {
        // More than 8 significant digits, then it must be f64
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1.123456789", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                    assert_eq!(*value, 1.123456789);
                } else { panic!("Expected Float64Literal"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn float_literal_multiple_dots_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = 1.2.3", t)));
        }
    }

    // Bool literals

    #[test]
    fn bool_literal_true() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = true", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::BoolLiteral { value: true, .. })));
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn bool_literal_false() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = false", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::BoolLiteral { value: false, .. })));
            } else { panic!("Expected VarDecl"); }
        }
    }

    // String literals

    #[test]
    fn string_literal_basic() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"hello\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, "hello");
                } else { panic!("Expected StringLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_escape_sequences() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"hello\\nworld\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, "hello\nworld");
                } else { panic!("Expected StringLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_with_escaped_quote() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"say \\\"hi\\\"\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, r#"say "hi""#);
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_unclosed_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = \"unclosed", t)));
        }
    }

    #[test]
    fn string_literal_containing_hash_not_comment() {
        for t in ALL_TYPES_NO_ARR {
            // '#' inside a string must not be stripped as a comment
            let stmts = parse_body(&format!("own x {} = \"hello # world\"", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, "hello # world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn string_literal_containing_curly_brackets_end() {
        for t in ALL_TYPES_NO_ARR {
            // '}' inside a string must not be treated as a function closing curly bracket.
            let stmts = parse_body(&format!("own x {} = \"hello }} world\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, "hello } world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn string_literal_containing_curly_brackets_start() {
        for t in ALL_TYPES_NO_ARR {
            // '}' inside a string must not be treated as a function closing curly bracket.
            let stmts = parse_body(&format!("own x {} = \"hello {{ world\"", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                    assert_eq!(value, "hello { world");
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    // Binary operations

    #[test]
    fn binop_arth_signed_literals_only() {
        let edge_cases_numbers = [
            i8::MIN as i128, i8::MAX as i128, 
            i16::MIN as i128, i16::MAX as i128, 
            i32::MIN as i128, i32::MAX as i128, 
            i64::MIN as i128, i64::MAX as i128, 
            i128::MIN, i128::MAX, 
        ];


        let edge_cases_types = [
            Type::Int8, Type::Int8,
            Type::Int16, Type::Int16,
            Type::Int32, Type::Int32,
            Type::Int64, Type::Int64,
            Type::Int128, Type::Int128,
        ];


        for (en1, et1) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (en2, et2) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = {} {} {}", et2, en1, s, en2));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, et2.clone());
                        if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                            assert_eq!(op, b);

                            if let Expr::IntLiteral { value, .. } = **left {
                                assert_eq!(value.get_type(), et1.clone());

                                if !value.is_signed() {
                                    panic!("We are in a signed testing function, but value is unsigned: {:?}", **left);
                                }

                                assert_eq!(value.as_i128(), *en1);

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }


                            if let Expr::IntLiteral { value, .. } = **right {
                                assert_eq!(value.get_type(), et2.clone());

                                if !value.is_signed() {
                                    panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                                }

                                assert_eq!(value.as_i128(), *en2);

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }


                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }



    #[test]
    fn binop_arth_unsigned_literals_only() {
        let edge_cases_numbers = [
            u8::MIN as u128, u8::MAX as u128, 
            u16::MIN as u128, u16::MAX as u128, 
            u32::MIN as u128, u32::MAX as u128, 
            u64::MIN as u128, u64::MAX as u128, 
            u128::MIN, u128::MAX, 

            usize::MIN as u128, usize::MAX as u128
        ];


        // Because we default to signed integers literals, unless we go out of range, then we switch to
        // unsigned literals types.
        // so those expected types are correct.
        // Int8 because unsigned::MIN is always 0, which can fit into int8
        //
        // I hope this test is not too much voodo for the reader, but it is what it is. It's good,
        // it's correct, it works, and it catches most parser edge cases.
        let edge_cases_types = [
            Type::Int8, Type::Int16,
            Type::Int8, Type::Int32,
            Type::Int8, Type::Int64,
            Type::Int8, Type::Int128,
            Type::Int8, Type::Uint128,
            
            Type::Int8, Type::Int128,
        ];

        for (en1, et1) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (en2, et2) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = {} {} {}", et2, en1, s, en2));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, et2.clone());
                        if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                            assert_eq!(op, b);

                            if let Expr::IntLiteral { value, .. } = **left {
                                assert_eq!(value.get_type(), et1.clone());

                                if value.is_signed() {
                                    assert!(value.as_i128() >= 0);
                                    assert_eq!(value.as_i128() as u128, *en1);
                                } else {
                                    assert_eq!(value.as_u128(), *en1);
                                } 

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }


                            if let Expr::IntLiteral { value, .. } = **right {
                                assert_eq!(value.get_type(), et2.clone());


                                if value.is_signed() {
                                    assert!(value.as_i128() >= 0);
                                    assert_eq!(value.as_i128() as u128, *en2);
                                } else {
                                    assert_eq!(value.as_u128(), *en2);
                                } 

                            } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }


                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }





    #[test]
    fn binop_arth_vars_only() {
        for t in ALL_TYPES_NO_ARR {
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} b", t, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));
                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    // Signed integer literals
    #[test]
    fn binop_arth_vars_and_signed_integer_literals_mixed() {
        let edge_cases_numbers = [
            i8::MIN as i128, i8::MAX as i128, 
            i16::MIN as i128, i16::MAX as i128, 
            i32::MIN as i128, i32::MAX as i128, 
            i64::MIN as i128, i64::MAX as i128, 
            i128::MIN, i128::MAX, 
        ];


        let edge_cases_types = [
            Type::Int8, Type::Int8,
            Type::Int16, Type::Int16,
            Type::Int32, Type::Int32,
            Type::Int64, Type::Int64,
            Type::Int128, Type::Int128,
        ];


        for (en, et) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} {}", et, s, en));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));
                        if let Expr::IntLiteral { value, .. } = **right {
                            assert_eq!(value.get_type(), et.clone());

                            if !value.is_signed() {
                                panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                            }

                            assert_eq!(value.as_i128(), *en);

                        } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }


            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = {} {} a", et, en, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert_eq!(value.get_type(), et.clone());
                            if !value.is_signed() {
                                panic!("We are in a signed testing function, but value is unsigned: {:?}", **right);
                            }

                            assert_eq!(value.as_i128(), *en);
                        } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }

                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    // Unsigned integer literals
    #[test]
    fn binop_arth_vars_and_unsigned_integer_literals_mixed() {
        let edge_cases_numbers = [
            u8::MIN as u128, u8::MAX as u128, 
            u16::MIN as u128, u16::MAX as u128, 
            u32::MIN as u128, u32::MAX as u128, 
            u64::MIN as u128, u64::MAX as u128, 
            u128::MIN, u128::MAX, 

            usize::MIN as u128, usize::MAX as u128
        ];


        // Because we default to signed integers literals, unless we go out of range, then we switch to
        // unsigned literals types.
        // so those expected types are correct.
        // Int8 because unsigned::MIN is always 0, which can fit into int8
        //
        // I hope this test is not too much voodo for the reader, but it is what it is. It's good,
        // it's correct, it works, and it catches most parser edge cases.
        let edge_cases_types = [
            Type::Int8, Type::Int16,
            Type::Int8, Type::Int32,
            Type::Int8, Type::Int64,
            Type::Int8, Type::Int128,
            Type::Int8, Type::Uint128,
            
            Type::Int8, Type::Int128,
        ];


        for (en, et) in edge_cases_numbers.iter().zip(edge_cases_types.iter()) {    
            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = a {} {}", et, s, en));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));

                        if let Expr::IntLiteral { value, .. } = **right {
                            assert_eq!(value.get_type(), et.clone());

                            if value.is_signed() {
                                assert!(value.as_i128() >= 0);
                                assert_eq!(value.as_i128() as u128, *en);
                            } else {
                                assert_eq!(value.as_u128(), *en);
                            }   
                            
                        } else { panic!("Expected IntLiteral, instead got: {:?}", **right) }

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }


            for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("own x {} = {} {} a", et, en, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, et.clone());
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        if let Expr::IntLiteral { value, .. } = **left {
                            assert_eq!(value.get_type(), et.clone());

                            if value.is_signed() {
                                assert!(value.as_i128() >= 0);
                                assert_eq!(value.as_i128() as u128, *en);
                            } else {
                                assert_eq!(value.as_u128(), *en);
                            }   

                        } else { panic!("Expected IntLiteral, instead got: {:?}", **left) }

                        assert!(matches!(**right, Expr::Var { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }
    }


    #[test]
    fn binop_missing_right_operand_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for b in BIN_OP_KIND_SYMBOLS {
                    assert_parse_err(&wrap(&format!("own x {} = {} {}", t, l, b)));
                }
            }
        }
    }

    #[test]
    fn binop_missing_left_operand_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for b in BIN_OP_KIND_SYMBOLS {
                    // So it's not unary, like -1, which is correct and wouldn't error.
                    if b == "-" {
                        continue
                    }
                    assert_parse_err(&wrap(&format!("own x {} = {} {}", t, b, l)));
                }
            }
        }
    }

    #[test]
    fn binop_nested_via_parens() {
        // e.g. own x int32 = (1 + 1) + 4
        // .. etc
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
                    let stmts = parse_body(&format!("own x {} = ({} {} {}) {} {}", t, l, s, l, s, l));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        assert_eq!(v.type_name, t.clone());
                        if let Some(Expr::BinOp { op, left, .. }) = &v.value {
                            assert_eq!(op, b);
                            assert!(matches!(**left, Expr::BinOp { .. }));
                        } else {
                            panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                        }
                    } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
                }
            }
        }
    }

    // Unary negate

    #[test]
    fn int_literals_doesnt_produce_unary_negate() {
        let literals_ints_edge_cases = get_all_ints_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_ints_edge_cases {
                // Because if l is -, it would become a float
                if *l == u128::MAX.to_string() {
                    continue
                }
                let stmts = parse_body(&format!("own x {} = -{}", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());

                    // a negative value would produce unary though because - earlier makes it do
                    // so. since theres no expressions before it. (--1 is -1 negated, etc)
                    if l.starts_with("-") {
                        if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                            assert_eq!(*op, UnaryOpKind::Negate);
                            if let Expr::IntLiteral { value, .. } = &**expr {
                                if value.is_signed() {
                                    let val_l = value.as_i128();
                                    assert_eq!(&val_l.to_string(), l);
                                } else {
                                    let val_l = value.as_u128();
                                    assert_eq!(&val_l.to_string(), l);
                                }   

                            } else { panic!("Expected IntLiteral"); }
                        } else { panic!("Expected Unary negate"); }
                    
                    } else {
                        if let Expr::IntLiteral { value, .. } = v.value.clone().unwrap() {
                            if value.is_signed() {
                                let val_l = value.as_i128();
                                if val_l == 0 {
                                    assert_eq!(&val_l.to_string(), l);
                                } else {
                                    assert_eq!(val_l.to_string(), format!("-{}", l));
                                }
                            } else {
                                let val_l = value.as_u128();
                                assert_eq!(&val_l.to_string(), l);
                            }
                        } else { panic!("Expected IntLiteral"); }
                    }


                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn unary_negate_variable() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = -y", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                    assert_eq!(*op, UnaryOpKind::Negate);
                    assert!(matches!(**expr, Expr::Var { .. }));
                } else { panic!("Expected Unary negate"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn unary_negate_array_access() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = -y[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                        assert_eq!(*op, UnaryOpKind::Negate);
                        assert!(matches!(**expr, Expr::ArraySingleAccess { .. }));
                    } else { panic!("Expected Unary negate"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }



    #[test]
    fn unary_negate_dangling_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = -", t)));
        }
    }

    // Function calls

    #[test]
    fn call_no_args() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = noop()", t));

            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::Call { name, args, .. }) = &v.value {
                    assert_eq!(name, "noop");
                    assert!(args.is_empty());
                } else { panic!("Expected Call"); }
            }       
        }
    }

    #[test]
    fn call_with_args_literals_only() {
        let literals_edge_cases = get_all_literals_edge_cases();
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = add({}, \"Hi!1\\\"\")", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());

                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "add");
                        assert_eq!(args.len(), 2);
                        assert!(matches!(args[1], Expr::StringLiteral { .. }));
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_with_args_vars_only() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = add(a, b)", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                   
                if let Some(Expr::Call { name, args, .. }) = &v.value {
                    assert_eq!(name, "add");
                    assert_eq!(args.len(), 2);
                    assert!(matches!(args[0], Expr::Var { .. }));
                    assert!(matches!(args[1], Expr::Var { .. }));
                } else { panic!("Expected Call"); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn call_with_args_mixed() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = add(a, {})", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "add");
                        assert_eq!(args.len(), 2);
                        assert!(matches!(args[0], Expr::Var { .. }));
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_nested_args() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own x {} = outer(inner({}, {}), {})", t, l, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                    if let Some(Expr::Call { name, args, .. }) = &v.value {
                        assert_eq!(name, "outer");
                        assert_eq!(args.len(), 2);
                    
                        if let Expr::Call { name, args: args2, .. } = &args[0] {
                            assert_eq!(name, "inner");
                            assert_eq!(args2.len(), 2);
                        } else { panic!("Expected Call"); }
                    } else { panic!("Expected Call"); }
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn call_as_statement() {
        let stmts = parse_body("do_thing()");
        assert_eq!(stmts.len(), 1);
        
        if let Stmt::Expr(e) = &stmts[0] {
            if let Expr::Call { name, args, .. } = e {
                assert_eq!(name, "do_thing");
                assert_eq!(args.len(), 0);

            } else { panic!("Expected Call"); }
        
        } else { panic!("Expected Expression"); }
    }

    // Built-in: copy()

    #[test]
    fn copy_call() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                let stmts = parse_body(&format!("own z {} = copy({})", t, l));
                assert_eq!(stmts.len(), 1);
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "z");
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::CopyCall { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn copy_wrong_arg_count_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                assert_parse_err(&wrap(&format!("own z {} = copy({}, {})", t, l, l)));
                assert_parse_err(&wrap(&format!("own z {} = copy()", t)));
            }
        }
    }

    // Built-in: format()

    #[test]
    fn format_call_binop_expr() {
        let literals_ints_edge_cases = get_all_ints_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_ints_edge_cases { 
                let stmts = parse_body(&format!("own s {} = format(\"Your age is {{{} + {}}}\")", t, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "s");
                    assert_eq!(v.type_name, t.clone());

                    assert!(matches!(v.value, Some(Expr::FormatCall { .. })));
                } else { panic!("Expected VarDecl"); }

            }
        }
    }

    #[test]
    fn format_call_variable() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases { 
                let stmts = parse_body(&format!("own x {} = {}\n own s {} = format(\"Hello, {{x}}!\")", t, l, t));

                assert_eq!(stmts.len(), 2);

                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());
                } else { panic!("Expected VarDecl"); }


                if let Stmt::VarDecl(v) = &stmts[1] {
                    assert_eq!(v.name, "s");
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::FormatCall { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn format_invalid_args_errors() {
        let literals_edge_cases = get_all_literals_edge_cases();
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                assert_parse_err(&wrap(&format!("own s {} = format({})", t, l)));
            }
            assert_parse_err(&wrap(&format!("own s {} = format({}[])", t, t)));
            assert_parse_err(&wrap(&format!("own s {} = format({{}})", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"{{}}\")", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"Hi {{}}\")", t)));
            assert_parse_err(&wrap(&format!("own s {} = format(\"Hi\")", t)));
        }
    }

    // Array access — single element

    #[test]
    fn array_single_access() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::ArraySingleAccess { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn array_single_access_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[0]"));
    }


    #[test]
    fn array_access_variable_index() {
        let literals_edge_cases = get_all_literals_edge_cases();
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::ArraySingleAccess { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }


    #[test]
    fn array_access_variable_index_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[i]"));
    }


    // Array access (slicing)

    #[test]
    fn array_slice_both_bounds() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}:{}]", t, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                        assert!(start.is_some());
                        assert!(end.is_some());
                    } else { panic!("Expected ArrayMultipleAccess"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_both_bounds_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[1:3]"));
    }

    #[test]
    fn array_slice_open_start() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[:{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                        assert!(start.is_none());
                        assert!(end.is_some());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_open_end() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}:]", t, l));
        
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                        assert!(start.is_some());
                        assert!(end.is_none());
                    } else { panic!(); }
                
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn array_access_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own v {} = arr[:]", t)));
            assert_parse_err(&wrap(&format!("own v {} = arr[]", t)));
        }
    }

    // Inline comment stripping

    #[test]
    fn inline_comment_stripped() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                // Statement followed by inline comment should still parse cleanly
                let stmts = parse_body(&format!("own x {} = {} # this is {} a comment", t, l, t));
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], Stmt::VarDecl(_)));
            }
        }
    }

    #[test]
    fn hash_inside_string_not_comment() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = \"val # not comment\"", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                if let Expr::StringLiteral { value, .. } = v.value.clone().unwrap() {
                    assert_eq!(value, "val # not comment");
                } else { panic!("Expected Var Expression"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    // Span tracking

    #[test]
    fn span_line_number_is_correct() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let src = format!("func main() {{\n\n\nown x {} = {}\n}}", t, l);
                let ast = parse(&src).unwrap();
                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    // Line 4 in the source (1-indexed)
                    assert_eq!(v.span.line, 4);

                    assert_eq!(v.name, "x");
                    assert_eq!(v.type_name, t.clone());

                }
            }    
        }
    }


    #[test]
    fn int_literal_get_type() {
        assert_eq!(IntLiteralValue::Int8(1).get_type(), Type::Int8);
        assert_eq!(IntLiteralValue::Int32(1).get_type(), Type::Int32);
        assert_eq!(IntLiteralValue::Int64(1).get_type(), Type::Int64);
        assert_eq!(IntLiteralValue::Int128(1).get_type(), Type::Int128);
        assert_eq!(IntLiteralValue::Byte(1).get_type(), Type::Byte);
        assert_eq!(IntLiteralValue::Uint16(1).get_type(), Type::Uint16);
        assert_eq!(IntLiteralValue::Uint32(1).get_type(), Type::Uint32);
        assert_eq!(IntLiteralValue::Uint64(1).get_type(), Type::Uint64);
        assert_eq!(IntLiteralValue::Uint128(1).get_type(), Type::Uint128);
        assert_eq!(IntLiteralValue::Usize(1).get_type(), Type::Usize);
    }

    // Testing IntLiteralValue helpers
    //
    #[test]
    fn int_literal_as_i128() {
        assert_eq!(IntLiteralValue::Int8(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int8(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int16(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int16(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int32(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int32(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int64(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int64(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int128(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int128(100).as_i128(), 100i128);
    }

    #[test]
    fn int_literal_as_u128() {
        assert_eq!(IntLiteralValue::Usize(usize::MAX).as_u128(), usize::MAX as u128);
        assert_eq!(IntLiteralValue::Byte(255).as_u128(), 255u128);
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).as_u128(), u16::MAX as u128);
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).as_u128(), u32::MAX as u128);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).as_u128(), u64::MAX as u128);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).as_u128(), u128::MAX);
    }

    // Signed literals casted as u128 should trigger a safety panic
    #[test]
    #[should_panic]
    fn int_literal_int8_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int8(i8::MIN).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int8_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int8(i8::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int16_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int16(i16::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int16_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int16(i16::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int32_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int32(i32::MIN).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int32_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int32(i32::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int64_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int64(i64::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int64_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int64(i64::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int128_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int128(i128::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int128_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int128(i128::MAX).as_u128();
    }


    // Unsigned literals casted as i128 should trigger a safety panic
    #[test]
    #[should_panic]
    fn int_literal_byte_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Byte(u8::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_byte_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Byte(u8::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint16_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint16(u16::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_uint16_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint16(u16::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint32_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint32(u32::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint32_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint32(u32::MAX).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_uint64_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint64(u64::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint64_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint64(u64::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint128_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint128(u128::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint128_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint128(u128::MAX).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_usize_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Usize(usize::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_usize_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Usize(usize::MAX).as_i128();
    }



    #[test]
    fn type_display() {
        assert_eq!(Type::Int8.to_string(), "int8");
        assert_eq!(Type::Int16.to_string(), "int16");
        assert_eq!(Type::Int32.to_string(), "int32");
        assert_eq!(Type::Int64.to_string(), "int64");
        assert_eq!(Type::Int128.to_string(), "int128");

        assert_eq!(Type::Byte.to_string(), "byte");
        assert_eq!(Type::Uint16.to_string(), "uint16");
        assert_eq!(Type::Uint32.to_string(), "uint32");
        assert_eq!(Type::Uint64.to_string(), "uint64");
        assert_eq!(Type::Uint128.to_string(), "uint128");
        
        assert_eq!(Type::Usize.to_string(), "usize");

        assert_eq!(Type::Float64.to_string(), "float64");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::String.to_string(), "string");


        assert_eq!(Type::Array(Box::new(Type::Int8)).to_string(), "[]int8");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int8)))).to_string(), "[][]int8");

        assert_eq!(Type::Array(Box::new(Type::Int16)).to_string(), "[]int16");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int16)))).to_string(), "[][]int16");

        assert_eq!(Type::Array(Box::new(Type::Int32)).to_string(), "[]int32");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int32)))).to_string(), "[][]int32");

        assert_eq!(Type::Array(Box::new(Type::Int64)).to_string(), "[]int64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int64)))).to_string(), "[][]int64");

        assert_eq!(Type::Array(Box::new(Type::Int128)).to_string(), "[]int128");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int128)))).to_string(), "[][]int128");




        assert_eq!(Type::Array(Box::new(Type::Byte)).to_string(), "[]byte");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Byte)))).to_string(), "[][]byte");

        assert_eq!(Type::Array(Box::new(Type::Uint16)).to_string(), "[]uint16");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint16)))).to_string(), "[][]uint16");

        assert_eq!(Type::Array(Box::new(Type::Uint32)).to_string(), "[]uint32");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint32)))).to_string(), "[][]uint32");

        assert_eq!(Type::Array(Box::new(Type::Uint64)).to_string(), "[]uint64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint64)))).to_string(), "[][]uint64");

        assert_eq!(Type::Array(Box::new(Type::Uint128)).to_string(), "[]uint128");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint128)))).to_string(), "[][]uint128");

        assert_eq!(Type::Array(Box::new(Type::Usize)).to_string(), "[]usize");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Usize)))).to_string(), "[][]usize");


        assert_eq!(Type::Array(Box::new(Type::Float64)).to_string(), "[]float64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Float64)))).to_string(), "[][]float64");


        assert_eq!(Type::Array(Box::new(Type::String)).to_string(), "[]string");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::String)))).to_string(), "[][]string");

        assert_eq!(Type::Array(Box::new(Type::Bool)).to_string(), "[]bool");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Bool)))).to_string(), "[][]bool");


    }

    // Variable shadowing (allowed by the spec)

    // Not allowed in semantics phase, but,  this is **syntaxally** correct
    #[test]
    fn variable_shadowing_allowed() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1\nown x {} = 2", t, t));
            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::IntLiteral { .. })));
            } else { panic!("Expected VarDecl"); }


            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.type_name, t.clone());
                assert!(matches!(v.value, Some(Expr::IntLiteral { .. })));
            } else { panic!("Expected VarDecl"); }
        }
    }

    // Empty expression / edge-case errors

    #[test]
    fn array_literal_edge_cases_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = 1, 2, 3", t)));
            assert_parse_err(&wrap(&format!("own x {} = [int32[1, 2, 3]]", t)));
            assert_parse_err(&wrap(&format!("own x {} = int32[[1, 2, 3]]", t)));
        }
    }

    #[test]
    fn empty_expression_in_call_arg_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                // Ensure we don't silently accept malformed call
                assert_parse_err(&wrap(&format!("own x {} = foo(,)", t)));
                assert_parse_err(&wrap(&format!("own x {} = foo({},)", t, l)));
                assert_parse_err(&wrap(&format!("own x {} = foo(,{})", t, l)));
                assert_parse_err(&wrap(&format!("own x {} = foo(,{},)", t, l)));
                assert_parse_err(&wrap(&format!("own x {} = foo({},{},)", t, l, l)));
            }
        }
    }
}



