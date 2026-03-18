use super::*;
 
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
 
    // validate_identifier_name
 
    #[test]
    fn identifier_valid() {
        assert!(helpers::validate_identifier_name("foo", 1).is_ok());
        assert!(helpers::validate_identifier_name("_foo", 1).is_ok());
        assert!(helpers::validate_identifier_name("foo_bar", 1).is_ok());
        assert!(helpers::validate_identifier_name("FOO", 1).is_ok());
        assert!(helpers::validate_identifier_name("x123", 1).is_ok());
        assert!(helpers::validate_identifier_name("1xd", 1).is_err());
    }
 
    #[test]
    fn identifier_empty() {
        assert!(helpers::validate_identifier_name("", 1).is_err());
    }
 
    #[test]
    fn identifier_starts_with_digit() {
        assert!(helpers::validate_identifier_name("1foo", 1).is_err());
        assert!(helpers::validate_identifier_name("9", 1).is_err());
    }
 
    #[test]
    fn identifier_invalid_chars() {
        assert!(helpers::validate_identifier_name("foo-bar", 1).is_err());
        assert!(helpers::validate_identifier_name("foo.bar", 1).is_err());
        assert!(helpers::validate_identifier_name("foo bar", 1).is_err());
        assert!(helpers::validate_identifier_name("foo@bar", 1).is_err());
    }
 
    #[test]
    fn identifier_reserved_keywords() {
        for kw in &["func", "own", "return", "for", "forever", "if", "else",
                    "true", "false", "int8", "int32", "float64", "bool", "string",
                    "copy", "format"] {
            assert!(
                helpers::validate_identifier_name(kw, 1).is_err(),
                "Expected error for keyword `{}`", kw
            );
        }
    }
 
    #[test]
    fn identifier_keyword_case_insensitive() {
        // Keywords are matched case-insensitively
        assert!(helpers::validate_identifier_name("FUNC", 1).is_err());
        assert!(helpers::validate_identifier_name("OWN", 1).is_err());
        assert!(helpers::validate_identifier_name("Return", 1).is_err());
    }
 
    // split_comma_top_level
 
    #[test]
    fn split_single_item() {
        let result = helpers::split_comma_top_level("a").unwrap();
        assert_eq!(result, vec!["a"]);
    }
 
    #[test]
    fn split_multiple_items() {
        let result = helpers::split_comma_top_level("a, b, c").unwrap();
        assert_eq!(result, vec!["a", "b", "c"]);
    }
 
    #[test]
    fn split_nested_parens_not_split() {
        let result = helpers::split_comma_top_level("foo(a, b), c").unwrap();
        assert_eq!(result, vec!["foo(a, b)", "c"]);
    }
 
    #[test]
    fn split_nested_brackets_not_split() {
        let result = helpers::split_comma_top_level("int32[1, 2], int32[3, 4]").unwrap();
        assert_eq!(result, vec!["int32[1, 2]", "int32[3, 4]"]);
    }
 
    #[test]
    fn split_string_containing_comma() {
        let result = helpers::split_comma_top_level(r#""hello, world", b"#).unwrap();
        assert_eq!(result, vec![r#""hello, world""#, "b"]);
    }
 
    #[test]
    fn split_unclosed_string_errors() {
        assert!(helpers::split_comma_top_level(r#""unclosed, x"#).is_err());
    }
 
    // Top-level parse: empty / comment-only / outside-function errors
 
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
        let ast = parse("func add(a int32, b int32) int32 {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.name, "add");
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.params[0].name, "a");
        assert_eq!(f.params[0].type_name, Type::Int32);
        assert_eq!(f.params[1].name, "b");
        assert_eq!(f.params[1].type_name, Type::Int32);
    }
 
    #[test]
    fn parse_function_single_return_type() {
        let ast = parse("func foo() int64 {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.return_type, Some(vec![Type::Int64]));
    }
 
    #[test]
    fn parse_function_multi_return_type() {
        let ast = parse("func foo() (int32, bool) {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.return_type, Some(vec![Type::Int32, Type::Bool]));
    }
 
    #[test]
    fn parse_function_no_return_type() {
        let ast = parse("func noop() {\n}\n").unwrap();
        let f = &ast.functions[0];
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
        assert_parse_err("func own() {\n}\n");
    }
 
    #[test]
    fn parse_multiple_functions() {
        let src = "func a() {\n}\nfunc b() {\n}\n";
        let ast = parse(src).unwrap();
        assert_eq!(ast.functions.len(), 2);
        assert_eq!(ast.functions[0].name, "a");
        assert_eq!(ast.functions[1].name, "b");
    }
 
    #[test]
    fn parse_function_array_return_type() {
        let ast = parse("func foo() int32[] {\n}\n").unwrap();
        let f = &ast.functions[0];
        assert_eq!(f.return_type, Some(vec![Type::Array(Box::new(Type::Int32))]));
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
        let stmts = parse_body("own x int32 = 2");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.type_name, Type::Int32);
        } else {
            panic!("Expected VarDecl");
        }
    }
 
    #[test]
    fn var_decl_no_value() {
        let stmts = parse_body("own x int32");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.name, "x");
            assert_eq!(v.type_name, Type::Int32);
            assert!(v.value.is_none());
        } else {
            panic!("Expected VarDecl");
        }
    }
 
    #[test]
    fn var_decl_all_integer_types() {
        for ty in &["int8", "int16", "int32", "int64", "int128",
                    "byte", "uint16", "uint32", "uint64", "uint128", "usize"] {
            let src = format!("own x {} = 0", ty);
            parse_body(&src); // should not panic
        }
    }
 
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
        let stmts = parse_body("own x int32[] = int32[1, 2, 3]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.type_name, Type::Array(Box::new(Type::Int32)));
        } else {
            panic!("Expected VarDecl");
        }
    }
 
    #[test]
    fn var_decl_array_inferred() {
        let stmts = parse_body("own x = int32[1, 2, 3]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert_eq!(v.type_name, Type::Infer);
            if let Some(Expr::ArrayLiteral { array_ty, elements, .. }) = &v.value {
                assert_eq!(*array_ty, Type::Int32);
                assert_eq!(elements.len(), 3);
            } else {
                panic!("Expected ArrayLiteral");
            }
        } else {
            panic!("Expected VarDecl");
        }
    }
 
    #[test]
    fn var_decl_empty_array() {
        // own x = int32[] — empty typed array literal
        let stmts = parse_body("own x = int32[]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                println!("Wtf ? {:?}", elements);
                assert!(elements.is_empty());
            } else {
                panic!("Expected ArrayLiteral");
            }
        }
    }
 
    #[test]
    fn var_decl_nested_array() {
        // own x = int32[][int32[1,2], int32[3,4]]
        let stmts = parse_body("own x = int32[][int32[1,2], int32[3,4]]");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                assert_eq!(elements.len(), 2);
                assert!(matches!(elements[0], Expr::ArrayLiteral { .. }));
            } else {
                panic!("Expected ArrayLiteral");
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
    }
 
    #[test]
    fn var_decl_keyword_name_errors() {
        assert_parse_err(&wrap("own if = 1"));
        assert_parse_err(&wrap("own return = 1"));
    }
 
    // Variable assignment
 
    #[test]
    fn var_assign() {
        let stmts = parse_body("own x int32\nx = 5");
        assert_eq!(stmts.len(), 2);
        if let Stmt::VarAssign(va) = &stmts[1] {
            assert_eq!(va.name, "x");
        } else {
            panic!("Expected VarAssign");
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
 
    // Integer literals — correct type selection
 
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
    fn integer_literal_negative_via_unary() {
        let stmts = parse_body("own x = -1");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::UnaryOp { op: UnaryOpKind::Negate, .. })));
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
        // More than 8 significant digits → must be f64
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
 
    // Binary operations
 
    #[test]
    fn binop_add() {
        let stmts = parse_body("own x = 1 + 2");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { op, .. }) = &v.value {
                assert!(matches!(op, BinOpKind::Add));
            } else { panic!("Expected BinOp"); }
        }
    }
 
    #[test]
    fn binop_subtract() {
        let stmts = parse_body("own x = 10 - 3");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { op, .. }) = &v.value {
                assert!(matches!(op, BinOpKind::Subtract));
            } else { panic!(); }
        }
    }
 
    #[test]
    fn binop_multiply() {
        let stmts = parse_body("own x = 4 * 5");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { op, .. }) = &v.value {
                assert!(matches!(op, BinOpKind::Multiply));
            } else { panic!(); }
        }
    }
 
    #[test]
    fn binop_divide() {
        let stmts = parse_body("own x = 8 / 2");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { op, .. }) = &v.value {
                assert!(matches!(op, BinOpKind::Divide));
            } else { panic!(); }
        }
    }
 
    #[test]
    fn binop_left_and_right_operands() {
        let stmts = parse_body("own x = a + b");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { left, right, .. }) = &v.value {
                assert!(matches!(**left, Expr::Var { .. }));
                assert!(matches!(**right, Expr::Var { .. }));
            }
        }
    }
 
    #[test]
    fn binop_missing_right_operand_errors() {
        assert_parse_err(&wrap("own x = 1 +"));
    }
 
    #[test]
    fn binop_missing_left_operand_errors() {
        // bare "+ 2" as an expression — left side is empty
        assert_parse_err(&wrap("own x = + 2"));
    }
 
    #[test]
    fn binop_nested_via_parens() {
        // Parenthesised grouping: (1 + 2) * 3
        let stmts = parse_body("own x = (1 + 2) * 3");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::BinOp { op, left, .. }) = &v.value {
                assert!(matches!(op, BinOpKind::Multiply));
                assert!(matches!(**left, Expr::BinOp { .. }));
            }
        }
    }
 
    // Unary negate
 
    #[test]
    fn unary_negate_literal() {
        let stmts = parse_body("own x = -42");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                assert_eq!(*op, UnaryOpKind::Negate);
                assert!(matches!(**expr, Expr::IntLiteral { .. }));
            } else { panic!("Expected UnaryOp"); }
        }
    }
 
    #[test]
    fn unary_negate_variable() {
        let stmts = parse_body("own x = -y");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::UnaryOp { op, expr, .. }) = &v.value {
                assert_eq!(*op, UnaryOpKind::Negate);
                assert!(matches!(**expr, Expr::Var { .. }));
            }
        }
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
    fn call_with_args() {
        let stmts = parse_body("own x = add(1, 2)");
        if let Stmt::VarDecl(v) = &stmts[0] {
            if let Some(Expr::Call { name, args, .. }) = &v.value {
                assert_eq!(name, "add");
                assert_eq!(args.len(), 2);
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
                assert!(matches!(args[0], Expr::Call { .. }));
            }
        }
    }
 
    #[test]
    fn call_as_statement() {
        let stmts = parse_body("do_thing()");
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Stmt::Expr(Expr::Call { .. })));
    }
 
    // Built-in: copy()
 
    #[test]
    fn copy_call() {
        let stmts = parse_body("own z = copy(y)");
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::CopyCall { .. })));
        }
    }
 
    #[test]
    fn copy_wrong_arg_count_errors() {
        assert_parse_err(&wrap("own z = copy(a, b)"));
        assert_parse_err(&wrap("own z = copy()"));
    }
 
    // Built-in: format()
 
    #[test]
    fn format_call() {
        let stmts = parse_body(r#"own s = format(x)"#);
        if let Stmt::VarDecl(v) = &stmts[0] {
            assert!(matches!(v.value, Some(Expr::FormatCall { .. })));
        }
    }
 
    #[test]
    fn format_wrong_arg_count_errors() {
        assert_parse_err(&wrap("own s = format()"));
        assert_parse_err(&wrap("own s = format(a, b)"));
    }
 
    // Array access — single element
 
    #[test]
    fn array_single_access() {
        let stmts = parse_body("own v = arr[0]");
        if let Stmt::VarDecl(v) = &stmts[0] {
    assert!(matches!(v.value, Some(Expr::ArraySingleAccess { .. })));
        }
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
        }
    }
 
    // Inline comment stripping
 
    #[test]
    fn inline_comment_stripped() {
        // Statement followed by inline comment should still parse cleanly
        let stmts = parse_body("own x = 1 # this is x");
        assert_eq!(stmts.len(), 1);
        assert!(matches!(stmts[0], Stmt::VarDecl(_)));
    }
 
    #[test]
    fn hash_inside_string_not_comment() {
        let stmts = parse_body(r#"own x = "val # not comment""#);
        assert_eq!(stmts.len(), 1);
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
 
    // IntLiteralValue helpers
 
    #[test]
    fn int_literal_get_type() {
        assert_eq!(IntLiteralValue::Int8(1).get_type(), Type::Int8);
        assert_eq!(IntLiteralValue::Int32(1).get_type(), Type::Int32);
        assert_eq!(IntLiteralValue::Uint64(1).get_type(), Type::Uint64);
        assert_eq!(IntLiteralValue::Usize(1).get_type(), Type::Usize);
    }
 
    #[test]
    fn int_literal_as_i128() {
        assert_eq!(IntLiteralValue::Int8(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int32(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int128(i128::MAX).as_i128(), i128::MAX);
    }
 
    #[test]
    fn int_literal_as_u128_unsafe_unsigned() {
        assert_eq!(IntLiteralValue::Byte(255).as_u128_UNSAFE(), 255u128);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).as_u128_UNSAFE(), u64::MAX as u128);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).as_u128_UNSAFE(), u128::MAX);
    }
 
    #[test]
    #[should_panic]
    fn int_literal_as_u128_unsafe_panics_on_negative_signed() {
        IntLiteralValue::Int32(-5).as_u128_UNSAFE();
    }
 
    // FloatLiteralValue helpers
    #[test]
    fn float_literal_get_type() {
        assert_eq!(FloatLiteralValue::Float32(1.0).get_type(), Type::Float32);
        assert_eq!(FloatLiteralValue::Float64(1.0).get_type(), Type::Float64);
    }

    #[test]
    fn type_display() {
        assert_eq!(Type::Int32.to_string(), "int32");
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
        let stmts = parse_body("own x = 1\nown x = 2");
        assert_eq!(stmts.len(), 2);
        assert!(matches!(stmts[0], Stmt::VarDecl(_)));
        assert!(matches!(stmts[1], Stmt::VarDecl(_)));
    }
 
    // Empty expression / edge-case errors
 
    #[test]
    fn untyped_bare_bracket_literal_errors() {
        // '[' without a type prefix is not allowed
        assert_parse_err(&wrap("own x = [1, 2, 3]"));
    }
 
    #[test]
    fn empty_expression_in_call_arg_not_crash() {
        // Ensure we don't silently accept malformed call
        assert_parse_err(&wrap("own x = foo(,)"));
    }

