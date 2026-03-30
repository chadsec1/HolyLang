use super::*;

use crate::consts;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR_NO_INFER,
    ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_ARTH,
    BIN_OP_KIND_ARTH_SYMBOLS, BIN_OP_KIND_COMP_SYMBOLS
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

    #[test]
    fn parse_statement_outside_function_errors() {
        assert_parse_err("own x = 1");
    }

    // Function declarations

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
        let ast = parse("func hello(a int32, b uint32, c usize) float32 {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.name, "hello");
        
        assert_eq!(f.return_type, Some(vec![Type::Float32]));

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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
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
        assert_parse_err("func bad() {\n own x = 1\n");
    }

    #[test]
    fn parse_function_keyword_name_errors() {
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("func {}() {{\n}}\n", kw));
        }
    }

    #[test]
    fn parse_function_space_in_name_errors() {
        assert_parse_err("func bad name() {own x = 1\n}\n");
    }

    #[test]
    fn parse_function_inline_statements_in_braces_errors() {
        assert_parse_err("func bad() {own x = 1\n}\n");
        
        assert_parse_err("func bad() {\nown x = 1}\n");
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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let ast = parse(&format!("func foo() {}[] {{\n}}\n", t)).unwrap();
            let f = &ast.functions[0];
            assert_eq!(f.return_type, Some(vec![Type::Array(Box::new(t.clone()))]));
        }
    }

    #[test]
    fn parse_function_nested_array_return_type() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            for i in 1..100 {
                let ast = parse(&format!("func foo() {}[]{} {{\n}}\n", t, "[]".repeat(i))).unwrap();
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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("for i in {}[12,\"hi\", true, 6.9, {}[]] {{\n\n}}", t, t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::For(f) = &stmts[0] {
                assert_eq!(f.holder_name, "i");
                assert_eq!(f.branch.len(), 0);

                if let Expr::ArrayLiteral { array_ty, elements, .. } = &f.value {
                    assert_eq!(*array_ty, t.clone());
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

                    if let Expr::FloatLiteral { value, .. } = &elements[3] {
                        assert!(matches!(value, FloatLiteralValue::Float32(6.9)));
                    } else { panic!("Expected FloatLiteral"); }

                    if let Expr::ArrayLiteral { array_ty, elements, .. } = &elements[4] {
                        assert_eq!(*array_ty, t.clone());
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
        for i in 0..10000 {
            let stmts = parse_body(&format!("infinite {} {{\n\n}}", " ".repeat(i)));
            assert_eq!(stmts.len(), 1);
            if let Stmt::Infinite(inf) = &stmts[0] {
                assert_eq!(inf.branch.len(), 0);

            } else {
                panic!("Expected infinite statement");
            }

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
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 1 {} 2 {{\n\n}}", " ".repeat(i), s));
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
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_literals_spaces_after_expr() {
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 1 {} 2 {}{{\n\n}}", s, " ".repeat(i)));
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
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} x {} y {{\n\n}}", " ".repeat(i), s));
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
    }

    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_spaces_after_expr() {
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while x {} y {}{{\n\n}}", s, " ".repeat(i)));
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
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while{} 69 {} y {{\n\n}}", " ".repeat(i), s));
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
                let stmts = parse_body(&format!("while{} x {} 67 {{\n\n}}", " ".repeat(i), s));
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
    }



    // Same test as above, but after the expression, there is an `i` of spaces.
    #[test]
    fn while_statements_vars_and_literals_spaces_after_expr() {
        for i in 0..5000 {
            for (b, s) in ALL_BIN_OP_KIND_COMP.iter().zip(BIN_OP_KIND_COMP_SYMBOLS.iter()) {
                let stmts = parse_body(&format!("while 69 {} y {}{{\n\n}}", s, " ".repeat(i)));
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
                let stmts = parse_body(&format!("while x {} 67 {}{{\n\n}}", s, " ".repeat(i)));
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
    }






    #[test]
    fn while_statements_no_condition_errors() {
        for i in 0..10000 {
            assert_parse_err(&wrap(&format!("while {}{{\n\n}}", " ".repeat(i))));    
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
        for i in 0..10000 {
            assert_parse_err(&wrap(&format!("if {}{{\n\n}}", " ".repeat(i))));    
        }
    }

    #[test]
    fn if_statements_elif_no_condition_errors() {
        for i in 0..10000 {
            assert_parse_err(&wrap(&format!("if 1 == 2 {{\n\n}} elif {}{{\n\n}}", " ".repeat(i))));    
        }

    }


    // Variable declarations

    #[test]
    fn var_decl_inferred_int() {
        let stmts = parse_body("own x = 1");
        assert_eq!(stmts.len(), 1);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::Infer);
            assert!(v.value.is_some());
        } else {
            panic!("Expected VarDecl");
        }
    }

    #[test]
    fn var_decl_explicit_type() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
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
    fn var_decl_float_types() {
        parse_body("own x float32 = 1.0");
        parse_body("own x float64 = 1.0");
    }

    #[test]
    fn var_decl_bool_type() {
        let stmts = parse_body("own x bool = true");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.type_name, Type::Bool);
        } else {
            panic!();
        }
    }

    #[test]
    fn var_decl_string_type() {
        let stmts = parse_body(r#"own x string = "hello""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.type_name, Type::String);
        } else {
            panic!();
        }
    }

    #[test]
    fn var_decl_array_explicit_type() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x {}[] = {}[1, 2, 3]", t, t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
            } else {
                panic!("Expected VarDecl");
            }
        }
    }

    #[test]
    fn var_decl_array_inferred() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x = {}[1, 2, 3]", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.type_name, Type::Infer);
                if let Some(Expr::ArrayLiteral { array_ty, elements, .. }) = &v.value {
                    assert_eq!(*array_ty, t.clone());
                    assert_eq!(elements.len(), 3);
                } else {
                    panic!("Expected ArrayLiteral");
                }
            } else {
                panic!("Expected VarDecl");
            }
        }
    }

    #[test]
    fn var_decl_empty_array() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x = {}[]", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert!(elements.is_empty());
                } else {
                    panic!("Expected ArrayLiteral");
                }
            }
        }
    }

    #[test]
    fn var_decl_nested_array() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x = {}[][{}[1,2], {}[3,4]]", t, t, t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 2);
                    assert!(matches!(elements[0], Expr::ArrayLiteral { .. }));
                } else {
                    panic!("Expected ArrayLiteral");
                }
            }
        }
    }

    #[test]
    fn var_decl_nested_array_empty() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x = {}[][]", t));
            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                    assert_eq!(elements.len(), 0);
                } else {
                    panic!("Expected ArrayLiteral");
                }
            }
        }
    }


    #[test]
    fn var_decl_deeply_nested_array() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            for i in 1..100 {
                let stmts = parse_body(&format!("own x = {}[][]{}", t, "[]".repeat(i) ));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                        assert_eq!(elements.len(), 0);
                    } else {
                        panic!("Expected ArrayLiteral");
                    }
                }
            }
        }
    }


    #[test]
    fn var_decl_multi() {
        let stmts = parse_body("own x, y, z = give_3_numbers()");
        assert!(matches!(stmts[0], Stmt::VarDeclMulti(_, _)));
        if let Stmt::VarDeclMulti(vars, _) = &stmts[0] {
            assert_eq!(vars.len(), 3);
            assert_eq!(vars[0].name, "x");
            assert_eq!(vars[1].name, "y");
            assert_eq!(vars[2].name, "z");
        }
    }

    #[test]
    fn var_decl_unknown_type_errors() {
        assert_parse_err(&wrap("own x badtype = 1"));
        assert_parse_err(&wrap("own x badtype"));
        assert_parse_err(&wrap("own x x = 1"));
        assert_parse_err(&wrap("own x x"));
    }

    #[test]
    fn var_decl_no_type_no_value_errors() {
        assert_parse_err(&wrap("own x"));
    }


    #[test]
    fn var_decl_keyword_name_errors() {

        for kw in consts::RESERVED_KEYWORDS { 
            for t in ALL_TYPES_NO_ARR_NO_INFER {
                assert_parse_err(&wrap(&format!("own {} = 1", kw)));
                assert_parse_err(&wrap(&format!("own {} {}", kw, t)));
            }
        }
    }

    // Variable assignment
    #[test]
    fn var_assign() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let stmts = parse_body(&format!("own x {}\nx = 5", t));
            assert_eq!(stmts.len(), 2);
            if let Stmt::VarAssign(va) = &stmts[1] {
                assert_eq!(va.name, "x");
            } else {
                panic!("Expected VarAssign");
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
        let stmts = parse_body("return 1, 2, 3");
        if let Stmt::Return(exprs) = &stmts[0] {
            assert_eq!(exprs.len(), 3);
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
        let stmts = parse_body("own x = 1\nreturn x");
        if let Stmt::Return(exprs) = &stmts[1] {
            assert_eq!(exprs.len(), 1);
            assert!(matches!(exprs[0], Expr::Var { .. }));
        } else {
            panic!("Expected Return");
        }
    }

    // Integer literals, correct type inferrence tests
    #[test]
    fn integer_literal_fits_int8() {
        let stmts = parse_body("own x = 1");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int8(1)));
            } else { panic!(); }
        }
    }

    #[test]
    fn integer_literal_int8_boundary() {
        // 127 fits int8, 128 does not
        let stmts = parse_body("own a = 127\nown b = 128");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int8(127)));
            }
        }
        if let Stmt::VarDecl(v) = &stmts[1] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(!matches!(value, IntLiteralValue::Int8(_)));
            }
        }
    }


    #[test]
    fn integer_literal_fits_int16() {
        let stmts = parse_body("own x = 128");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int16(128)));
            } else { panic!(); }
        }
    }

    #[test]
    fn integer_literal_int16_boundary() {
        // 32767 fits int16, 32768 does not
        let stmts = parse_body("own a = 32767\nown b = 32768");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int16(32767)));
            }
        }
        if let Stmt::VarDecl(v) = &stmts[1] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(!matches!(value, IntLiteralValue::Int16(_)));
            }
        }
    }

    #[test]
    fn integer_literal_fits_int32() {
        let stmts = parse_body("own x = 32768");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int32(32768)));
            } else { panic!(); }
        }
    }

    #[test]
    fn integer_literal_int32_boundary() {
        // 2147483647 fits int32, 2147483648 does not
        let stmts = parse_body("own a = 2147483647\nown b = 2147483648");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int32(2147483647)));
            }
        }
        if let Stmt::VarDecl(v) = &stmts[1] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(!matches!(value, IntLiteralValue::Int32(_)));
            }
        }
    }


    #[test]
    fn integer_literal_fits_int64() {
        let stmts = parse_body("own x = 2147483648");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int64(2147483648)));
            } else { panic!(); }
        }
    }

    #[test]
    fn integer_literal_int64_boundary() {
        // 9223372036854775807 fits int64, 9223372036854775808 does not
        let stmts = parse_body("own a = 9223372036854775807\nown b = 9223372036854775808");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int64(9223372036854775807)));
            }
        }
        if let Stmt::VarDecl(v) = &stmts[1] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(!matches!(value, IntLiteralValue::Int64(_)));
            }
        }
    }

    #[test]
    fn integer_literal_fits_int128() {
        let stmts = parse_body("own x = 9223372036854775808");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int128(9223372036854775808)));
            } else { panic!(); }
        }
    }

    #[test]
    fn integer_literal_int128_boundary() {
        // 170141183460469231731687303715884105727 fits int128,  170141183460469231731687303715884105728 does not
        let stmts = parse_body("own a = 170141183460469231731687303715884105727\nown b = 170141183460469231731687303715884105728");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Int128(170141183460469231731687303715884105727)));
            }
        }
        if let Stmt::VarDecl(v) = &stmts[1] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(!matches!(value, IntLiteralValue::Int128(_)));
            }
        }
    }

    #[test]
    fn integer_literal_fits_uint128() {
        let stmts = parse_body("own x = 170141183460469231731687303715884105728");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                assert!(matches!(value, IntLiteralValue::Uint128(170141183460469231731687303715884105728)));
            } else { panic!(); }
        }
    }



    #[test]
    fn integer_literal_negative() {
        let stmts = parse_body("own x = -128");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(-128), .. })));
        }
    }

    #[test]
    fn integer_overflow_u128_errors() {
        // A number larger than u128::MAX should produce a parse error
        let huge = "340282366920938463463374607431768211456"; // u128::MAX + 1
        assert_parse_err(&wrap(&format!("own x = {}", huge)));
    }

    // Float literals

    #[test]
    fn float_literal_f32() {
        let stmts = parse_body("own x = 1.0");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::FloatLiteral { value, .. }) = &v.value {
                assert!(matches!(value, FloatLiteralValue::Float32(_)));
            } else { panic!("Expected FloatLiteral"); }
        }
    }

    #[test]
    fn float_literal_f64_high_precision() {
        // More than 8 significant digits, then it must be f64
        let stmts = parse_body("own x = 1.123456789");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::FloatLiteral { value, .. }) = &v.value {
                assert!(matches!(value, FloatLiteralValue::Float64(_)));
            } else { panic!("Expected FloatLiteral"); }
        }
    }

    #[test]
    fn float_literal_multiple_dots_errors() {
        assert_parse_err(&wrap("own x = 1.2.3"));
    }

    // Bool literals

    #[test]
    fn bool_literal_true() {
        let stmts = parse_body("own x = true");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::BoolLiteral { value: true, .. })));
        }
    }

    #[test]
    fn bool_literal_false() {
        let stmts = parse_body("own x = false");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::BoolLiteral { value: false, .. })));
        }
    }

    // String literals

    #[test]
    fn string_literal_basic() {
        let stmts = parse_body(r#"own x = "hello""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, "hello");
            } else { panic!("Expected StringLiteral"); }
        }
    }

    #[test]
    fn string_literal_escape_sequences() {
        let stmts = parse_body(r#"own x = "hello\nworld""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, "hello\nworld");
            } else { panic!(); }
        }
    }

    #[test]
    fn string_literal_with_escaped_quote() {
        let stmts = parse_body(r#"own x = "say \"hi\"""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, r#"say "hi""#);
            } else { panic!(); }
        }
    }

    #[test]
    fn string_literal_unclosed_errors() {
        assert_parse_err(&wrap(r#"own x = "unclosed"#));
    }

    #[test]
    fn string_literal_containing_hash_not_comment() {
        // '#' inside a string must not be stripped as a comment
        let stmts = parse_body(r#"own x = "hello # world""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, "hello # world");
            } else { panic!(); }
        }
    }


    #[test]
    fn string_literal_containing_curly_brackets_end() {
        // '}' inside a string must not be treated as a function closing curly bracket.
        let stmts = parse_body(r#"own x = "hello } world""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, "hello } world");
            } else { panic!(); }
        }
    }

    #[test]
    fn string_literal_containing_curly_brackets_start() {
        // '{' inside a string must not be treated as a function start curly bracket.
        let stmts = parse_body(r#"own x = "hello { world""#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::StringLiteral { value, .. }) = &v.value {
                assert_eq!(value, "hello { world");
            } else { panic!(); }
        }
    }


    // Binary operations

    #[test]
    fn binop_arth_literals_only() {
        for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
            for i1 in 0..260 {
                for i2 in 0..260 {
                    let stmts = parse_body(&format!("own x = {} {} {}", i1, s, i2));
                    if let Stmt::VarDecl(v) = &stmts[0] {
                        if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                            assert_eq!(op, b);

                            assert!(matches!(**left, Expr::IntLiteral { .. }));
                            assert!(matches!(**right, Expr::IntLiteral { .. }));

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
        for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("own x = a {} b", s));
            if let Stmt::VarDecl(v) = &stmts[0] {
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

    #[test]
    fn binop_arth_vars_and_literals_mixed() {
        for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
            for i in 0..100000 {
                let stmts = parse_body(&format!("own x = a {} {}", s, i));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::Var { .. }));
                        assert!(matches!(**right, Expr::IntLiteral { .. }));

                    } else {
                        panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                    }
                } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
            }
        }


        for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
            for i in 0..100000 {
                let stmts = parse_body(&format!("own x = {} {} a", i, s));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::BinOp { left, right, op, .. }) = &v.value {
                        assert_eq!(op, b);

                        assert!(matches!(**left, Expr::IntLiteral { .. }));
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
        assert_parse_err(&wrap("own x = 1 +"));
    }

    #[test]
    fn binop_missing_left_operand_errors() {
        assert_parse_err(&wrap("own x = + 2"));
    }

    #[test]
    fn binop_nested_via_parens() {
        for (b, s) in ALL_BIN_OP_KIND_ARTH.iter().zip(BIN_OP_KIND_ARTH_SYMBOLS.iter()) {
            let stmts = parse_body(&format!("own x = (1 + 2) {} 3", s));
            if let Stmt::VarDecl(v) = &stmts[0] {
                if let Some(Expr::BinOp { op, left, .. }) = &v.value {
                    assert_eq!(op, b);
                    assert!(matches!(**left, Expr::BinOp { .. }));
                } else {
                    panic!("Expected {:?}, instead we got {:?}", b, &v.value);
                }
            } else { panic!("Expected VarDecl, instead we got {:?}", &stmts[0]) }
        }
    }

    // Unary negate

    #[test]
    fn literals_doesnt_produce_unary_negate() {
        let stmts = parse_body("own x = -42");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(-42), .. })));
        } else { panic!("Expected VarDecl"); }
    }

    #[test]
    fn unary_negate_variable() {
        let stmts = parse_body("own x = -y");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                assert_eq!(*op, UnaryOpKind::Negate);
                assert!(matches!(**expr, Expr::Var { .. }));
            } else { panic!("Expected Unary negate"); }
        } else { panic!("Expected VarDecl"); }
    }

    #[test]
    fn unary_negate_dangling_errors() {
        assert_parse_err(&wrap("own x = -"));
    }

    // Function calls

    #[test]
    fn call_no_args() {
        let stmts = parse_body("own x = noop()");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "noop");
                assert!(args.is_empty());
            } else { panic!("Expected Call"); }
    }
    }

    #[test]
    fn call_with_args_literals_only() {
        let stmts = parse_body("own x = add(1, \"Hi!1\\\"\")");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(args[0], Expr::IntLiteral { .. }));
                assert!(matches!(args[1], Expr::StringLiteral { .. }));
            } else { panic!("Expected Call"); }
        }
    }

    #[test]
    fn call_with_args_vars_only() {
        let stmts = parse_body("own x = add(a, b)");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(args[0], Expr::Var { .. }));
                assert!(matches!(args[1], Expr::Var { .. }));
            } else { panic!("Expected Call"); }
        }
    }


    #[test]
    fn call_with_args_mixed() {
        let stmts = parse_body("own x = add(a, 69)");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
                assert!(matches!(args[0], Expr::Var { .. }));
                assert!(matches!(args[1], Expr::IntLiteral { .. }));
            } else { panic!("Expected Call"); }
        }
    }


    #[test]
    fn call_nested_args() {
        let stmts = parse_body("own x = outer(inner(1, 2), 3)");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "outer");
                assert_eq!(args.len(), 2);
            
                assert!(matches!(args[1], Expr::IntLiteral { .. }));
                if let Expr::Call { name, args, .. } = &args[0] {
                    assert_eq!(name, "inner");
                    assert!(matches!(args[0], Expr::IntLiteral { .. }));
                    assert!(matches!(args[1], Expr::IntLiteral { .. }));
                } else { panic!("Expected Call"); }
            } else { panic!("Expected Call"); }
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
        let stmts = parse_body("own z = copy(y)");
        assert_eq!(stmts.len(), 1);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::CopyCall { .. })));
        } else { panic!("Expected VarDecl"); }
    }

    #[test]
    fn copy_wrong_arg_count_errors() {
        assert_parse_err(&wrap("own z = copy(a, b)"));
        assert_parse_err(&wrap("own z = copy()"));
    }

    // Built-in: format()

    #[test]
    fn format_call_binop_expr() {
        let stmts = parse_body(r#"own s = format("Your age is {17 + 1}")"#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::FormatCall { .. })));
        } else { panic!("Expected VarDecl"); }

    }


    #[test]
    fn format_call_variable() {
        let stmts = parse_body("own x = \"World\"\n own s = format(\"Hello, {x}!\")");
        if let Stmt::VarDecl(v) = &stmts[1] {
            assert!(matches!(v.value, Some(Expr::FormatCall { .. })));
        } else { panic!("Expected VarDecl"); }

    }

    #[test]
    fn format_invalid_args_errors() {
        assert_parse_err(&wrap("own s = format()"));
        assert_parse_err(&wrap("own s = format(a, b)"));
        assert_parse_err(&wrap("own s = format(1)"));
        assert_parse_err(&wrap("own s = format(true)"));
        assert_parse_err(&wrap("own s = format(int32[1,2,3])"));
        assert_parse_err(&wrap("own s = format(format(\"Hi\"))"));
        
        assert_parse_err(&wrap("own s = format(\"{}\")"));
        assert_parse_err(&wrap("own s = format(\"Hi {}\")"));
        assert_parse_err(&wrap("own s = format(\"Hi\")"));
    }

    // Array access — single element

    #[test]
    fn array_single_access() {
        let stmts = parse_body("own v = arr[0]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::ArraySingleAccess { .. })));
        } else { panic!("Expected VarDecl"); }
    }

    #[test]
    fn array_access_variable_index() {
        let stmts = parse_body("own v = arr[i]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArraySingleAccess { index, .. }) = &v.value {
                assert!(matches!(**index, Expr::Var { .. }));
            } else { panic!(); }
        }
    }

    // Array access — slice

    #[test]
    fn array_slice_both_bounds() {
        let stmts = parse_body("own v = arr[1:3]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                assert!(start.is_some());
                assert!(end.is_some());
            } else { panic!("Expected ArrayMultipleAccess"); }
        }
    }

    #[test]
    fn array_slice_open_start() {
        let stmts = parse_body("own v = arr[:5]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                assert!(start.is_none());
                assert!(end.is_some());
            } else { panic!(); }
        }
    }

    #[test]
    fn array_slice_open_end() {
        let stmts = parse_body("own v = arr[2:]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArrayMultipleAccess { start, end, .. }) = &v.value {
                assert!(start.is_some());
                assert!(end.is_none());
            } else { panic!(); }
        
        } else { panic!("Expected VarDecl"); }
    }

    #[test]
    fn array_access_errors() {
        assert_parse_err(&wrap("own v = arr[:]"));
        assert_parse_err(&wrap("own v = arr[]"));
    }

    // Inline comment stripping

    #[test]
    fn inline_comment_stripped() {
        // Statement followed by inline comment should still parse cleanly
        let stmts = parse_body("own x = 1 # this is a comment");
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Stmt::VarDecl(_)));
    }

    #[test]
    fn hash_inside_string_not_comment() {
        let stmts = parse_body(r#"own x = "val # not comment""#);
        assert_eq!(stmts.len(), 1);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            if let Expr::StringLiteral { value, .. } = v.value.clone().unwrap() {
                assert_eq!(value, "val # not comment");
            } else { panic!("Expected Var Expression"); }
        } else { panic!("Expected VarDecl"); }
    }

    // Span tracking

    #[test]
    fn span_line_number_is_correct() {
        let src = "func main() {\n\n\nown x = 1\n}\n";
        let ast = parse(src).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            // Line 4 in the source (1-indexed)
            assert_eq!(v.span.line, 4);
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
    fn int_literal_int8_as_u128_unsafe_panics_on_signed() {
        for i in i8::MIN..=i8::MAX {
            IntLiteralValue::Int8(i).as_u128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_int16_as_u128_unsafe_panics_on_signed() {
        for i in i16::MIN..=i16::MAX {
            IntLiteralValue::Int16(i).as_u128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_int32_as_u128_unsafe_panics_on_signed() {
        for i in i32::MIN..=i32::MAX {
            IntLiteralValue::Int32(i).as_u128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_int64_as_u128_unsafe_panics_on_signed() {
        for i in i64::MIN..=i64::MAX {
            IntLiteralValue::Int64(i).as_u128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_int128_as_u128_unsafe_panics_on_signed() {
        for i in i128::MIN..=i128::MAX {
            IntLiteralValue::Int128(i).as_u128();
        }
    }


    // Unsigned literals casted as i128 should trigger a safety panic
    #[test]
    #[should_panic]
    fn int_literal_byte_as_i128_panics_on_unsigned() {
        for i in u8::MIN..=u8::MAX {
            IntLiteralValue::Byte(i).as_i128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_uint16_as_i128_panics_on_unsigned() {
        for i in u16::MIN..=u16::MAX {
            IntLiteralValue::Uint16(i).as_i128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_uint32_as_i128_panics_on_unsigned() {
        for i in u32::MIN..=u32::MAX {
            IntLiteralValue::Uint32(i).as_i128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_uint64_as_i128_panics_on_unsigned() {
        for i in u64::MIN..=u64::MAX {
            IntLiteralValue::Uint64(i).as_i128();
        }
    }

    #[test]
    #[should_panic]
    fn int_literal_uint128_as_i128_panics_on_unsigned() {
        for i in u128::MIN..=u128::MAX {
            IntLiteralValue::Uint128(i).as_i128();
        }
    }


    #[test]
    #[should_panic]
    fn int_literal_usize_as_i128_panics_on_unsigned() {
        for i in usize::MIN..=usize::MAX {
            IntLiteralValue::Usize(i).as_i128();
        }
    }



    // Testing FloatLiteralValue helpers
    // 
    #[test]
    fn float_literal_get_type() {
        assert_eq!(FloatLiteralValue::Float32(1.0).get_type(), Type::Float32);
        assert_eq!(FloatLiteralValue::Float64(1.0).get_type(), Type::Float64);
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

        assert_eq!(Type::Float32.to_string(), "float32");
        assert_eq!(Type::Float64.to_string(), "float64");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::String.to_string(), "string");
        assert_eq!(Type::Array(Box::new(Type::Int32)).to_string(), "int32[]");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int32)))).to_string(), "int32[][]");
        assert_eq!(Type::Infer.to_string(), "infer");
    }

    // Variable shadowing (allowed by the spec)

    #[test]
    fn variable_shadowing_allowed() {
        // Declaring the same name twice should produce two VarDecl nodes without error
        // as long as its in the same scope
        let stmts = parse_body("own x = 1\nown x = 2");
        assert_eq!(stmts.len(), 2);
        assert!(matches!(stmts[0], Stmt::VarDecl(_)));
        assert!(matches!(stmts[1], Stmt::VarDecl(_)));
    }

    // Empty expression / edge-case errors

    #[test]
    fn untyped_array_literal_edge_cases_errors() {
        // '[' without a type prefix is not allowed
        assert_parse_err(&wrap("own x = [1, 2, 3]"));
        assert_parse_err(&wrap("own x = [[1, 2, 3]]"));
        assert_parse_err(&wrap("own x = 1, 2, 3"));
        
        assert_parse_err(&wrap("own x = [int32[1, 2, 3]]"));
        assert_parse_err(&wrap("own x = int32[[1, 2, 3]]"));
    }

    #[test]
    fn empty_expression_in_call_arg_not_crash() {
        // Ensure we don't silently accept malformed call
        assert_parse_err(&wrap("own x = foo(,)"));
    }
}

