use super::*;

#[cfg(test)]
mod format_tests {
    use super::*;

    // format call guards 
    #[test]
    fn test_format_call_with_literal_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![l.clone()], // plain literals not allowed
                span: span(),
            };
            let body = vec![var_decl(true, "s", Type::String, fmt)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Plain literals are not allowed in formating! Remove the format placeholders and use the literal directly!"));
        }
    } 


    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn test_format_call_without_any_template_placeholders_panics() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value".to_string(),
                expressions: vec![l.clone()], 
                span: span(),
            };
            let body = vec![var_decl(true, "s", Type::String, fmt)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let _ = check_semantics(&mut ast);
        }
    }

    #[test]
    fn test_format_call_with_variable_passes() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![var_expr("n")],
                span: span(),
            };
            let body = vec![
                var_decl(true, "n", t.clone(), l.clone()),
                var_decl(true, "s", Type::String, fmt),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }


    // Format calls copies expressions passed to it, if you attempt to copy manually, that's an
    // error.
    #[test]
    fn test_format_call_with_expressions_copied_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_n = Expr::CopyCall { expr: Box::new(var_expr("n")), span: span() };
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![copy_n],
                span: span(),
            };
            let body = vec![
                var_decl(true, "n", t.clone(), l.clone()),
                var_decl(true, "s", Type::String, fmt),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Format calls copy by default, Remove the extra copy call."));
        }
    }

    #[test]
    fn test_nested_format_call_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![var_expr("n")], 
                span: span(),
            };

            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![fmt], 
                span: span(),
            };


            let body = vec![
                var_decl(true, "n", t.clone(), l.clone()),
                var_decl(true, "s", Type::String, fmt),
            ];
            
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Nested FormatCalls are not allowed."));
            
        }
    }
}
