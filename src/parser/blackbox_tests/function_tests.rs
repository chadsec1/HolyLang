use super::*;

#[cfg(test)]
mod function_tests {
    use super::*;


    #[test]
    fn function_with_missing_opening_parenthesis_errors() {
        let result = parse("func main) {\n}\n");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid function header (no '(')"));
    }

    #[test]
    fn function_with_missing_closing_parenthesis_errors() {
        let result = parse("func main( {\n}\n");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid function header (no ')')"));
    }


    #[test]
    fn function_missing_opening_parenthesis_with_return_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main) {} {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid function header (no '(')"));
        }
    }

    #[test]
    fn function_missing_closing_parenthesi_with_returns_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main( {} {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid function header (no ')')"));
        }
    }

    #[test]
    fn function_missing_opening_parenthesis_with_multiple_returns_errors() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                let result = parse(&format!("func main) ({}, {}) {{\n}}\n", t1, t2));

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Invalid function header: there is an extra closing parenthesis `)` in the function declaration header"));
            }
        }
    }


    #[test]
    fn function_missing_closing_parenthesis_with_multiple_returns_errors() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                let result = parse(&format!("func main( ({}, {}) {{\n}}\n", t1, t2));

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Invalid parameter"));
            }
        }
    }

    #[test]
    fn function_missing_multiple_return_opening_parenthesis_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main() {}) {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Missing opening parentheses for return type"));
        }
    }


    #[test]
    fn function_missing_multiple_return_closing_parenthesis_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func main() ({} {{\n}}\n", t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Missing closing parentheses for return type"));
        }
    }


    #[test]
    fn empty_function() {
        let ast = parse("func main() {\n}\n").unwrap();
        assert_eq!(ast.functions.len(), 1);
        let f = &ast.functions[0];
        assert_eq!(f.name, "main");
        assert!(f.params.is_empty());
        assert!(f.return_type.is_none());
        assert!(f.body.is_empty());
    }

    #[test]
    fn function_with_params() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func hello(a int32, b uint32, c usize) {} {{\n}}", t)).unwrap();
            let f = &ast.functions[0];
            assert_eq!(f.name, "hello");
            
            assert_eq!(f.return_type, Some(vec![t.clone()]));

            assert_eq!(f.params.len(), 3);
            assert_eq!(f.params[0].name, "a");
            assert_eq!(f.params[0].type_name, Type::Int32);
            assert_eq!(f.params[1].name, "b");
            assert_eq!(f.params[1].type_name, Type::Uint32);
            assert_eq!(f.params[2].name, "c");
            assert_eq!(f.params[2].type_name, Type::Usize);
        }
    }

    #[test]
    fn function_with_invalid_param_name_errors() {
        for t in ALL_TYPES_NO_ARR {
            let result = parse(&format!("func hello({} {}) {{\n}}", t, t));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Binding identifier name"));
        }
    }

    #[test]
    fn function_single_return_invalid_type_errors() {
        let literals = get_all_literals_edge_cases();

        for l in &literals {
            let result = parse(&format!("func foo() {} {{\n}}", l));

            assert!(result.is_err());
            let assert_cond = result.unwrap_err().to_string();
            assert!(assert_cond.contains("Unknown type") || assert_cond.contains("Missing opening parentheses for return type"));
        }

        // Same test, but with reserved language keywords.
        for kw in consts::RESERVED_KEYWORDS {
            let mut kw_in_t = false;
            for t in ALL_TYPES_NO_ARR {
                if t.to_string() == *kw {
                    kw_in_t = true;
                    break;
                }
            }

            if kw_in_t {
                continue;
            }

            let result = parse(&format!("func foo() {} {{\n}}", kw));
            assert!(result.is_err());
            let assert_cond = result.unwrap_err().to_string();
            assert!(assert_cond.contains("Unknown type") || assert_cond.contains("Missing opening parentheses for return type"));
        }
    }



    #[test]
    fn function_single_return_type() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func foo() {} {{\n}}\n", t)).unwrap();
            let f = &ast.functions[0];

            assert_eq!(f.name, "foo");
            assert_eq!(f.params.len(), 0);
            assert_eq!(f.return_type, Some(vec![t.clone()]));
        }
    }


    #[test]
    fn function_multiple_return_one_type_invalid_type_errors() {
        let literals = get_all_literals_edge_cases();

        for l in &literals {
            let result = parse(&format!("func foo() ({}) {{\n}}", l));

            assert!(result.is_err());
            let assert_cond = result.unwrap_err().to_string();
            assert!(assert_cond.contains("Unknown type"));
        }

        // Same test, but with reserved language keywords.
        for kw in consts::RESERVED_KEYWORDS {
            let mut kw_in_t = false;
            for t in ALL_TYPES_NO_ARR {
                if t.to_string() == *kw {
                    kw_in_t = true;
                    break;
                }
            }

            if kw_in_t {
                continue;
            }

            let result = parse(&format!("func foo() ({}) {{\n}}", kw));
            assert!(result.is_err());
            let assert_cond = result.unwrap_err().to_string();
            assert!(assert_cond.contains("Unknown type"));
        }
    }

    #[test]
    fn function_multiple_return_invalid_type_errors() {
        let literals = get_all_literals_edge_cases();

        for l in &literals {
            let result = parse(&format!("func foo() ({}, {}) {{\n}}", l, l));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Unknown type"));
        }

        // Same test, but with reserved language keywords.
        for kw in consts::RESERVED_KEYWORDS {
            let mut kw_in_t = false;
            for t in ALL_TYPES_NO_ARR {
                if t.to_string() == *kw {
                    kw_in_t = true;
                    break;
                }
            }

            if kw_in_t {
                continue;
            }

            let result = parse(&format!("func foo() ({}, {}) {{\n}}", kw, kw));
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Unknown type"));
        }
    }

    #[test]
    fn function_multiple_return_invalid_stringliteral_split_errors() {
        let result = parse("func foo() (\"hi ) {\n}");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed string literal"));
    }


    #[test]
    fn function_multiple_return_empty_errors() {
        const MAX_SPACES: usize = 1000;
        let mut spaces = String::with_capacity(MAX_SPACES);

        for _ in 0..MAX_SPACES {
            let result = parse(&format!("func foo() ({}) {{\n}}", spaces));

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Missing types in the `()`"));
            spaces.push(' ');
        }
    }
    

    #[test]
    fn function_multi_return_type_one_type() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func foo() ({}) {{\n}}", t)).unwrap();
            let f = &ast.functions[0];

            assert_eq!(f.name, "foo");
            assert_eq!(f.params.len(), 0);
            assert_eq!(f.return_type, Some(vec![t.clone()]));
        }
    }


    #[test]
    fn function_multi_return_type() {
        for t1 in ALL_TYPES_NO_ARR {
            for t2 in ALL_TYPES_NO_ARR {
                for t3 in ALL_TYPES_NO_ARR {
                    let ast = parse(&format!("func foo() ({}, {}, {}) {{\n}}", t1, t2, t3)).unwrap();
                    let f = &ast.functions[0];

                    assert_eq!(f.name, "foo");
                    assert_eq!(f.params.len(), 0);
                    assert_eq!(f.return_type, Some(vec![t1.clone(), t2.clone(), t3.clone()]));
                }
            }
        }
    }

    #[test]
    fn function_no_return_type_stmt_in_bottom_brace_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {  
            assert_parse_err(&format!("func foo() {{\n{}}}", l));
        }
    }

    #[test]
    fn function_no_return_type_stmt_in_top_brace_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals {   
            assert_parse_err(&format!("func foo() {{{}\n}}", l));
        }
    }

    #[test]
    fn function_no_return_type() {
        let ast = parse("func noop() {\n}").unwrap();
        let f = &ast.functions[0];

        assert_eq!(f.name, "noop");
        assert_eq!(f.params.len(), 0);
        assert!(f.return_type.is_none());
    }

    #[test]
    fn function_missing_open_paren_errors() {
        assert_parse_err("func bad {\n}");
        assert_parse_err("func bad {\n\n}");
        assert_parse_err("func bad {\n\n}\n");
    }

    #[test]
    fn function_missing_brace_errors() {
        assert_parse_err("func bad()\n");
        assert_parse_err("func bad()\n\n");
    }

    #[test]
    fn function_unterminated_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("func bad() {{\n own x {} = 1\n", t));
        }
    }

    #[test]
    fn function_keyword_name_errors() {
        for kw in consts::RESERVED_KEYWORDS { 
            assert_parse_err(&format!("func {}() {{\n}}\n", kw));
        }
    }

    #[test]
    fn function_space_in_name_errors() {
        let literals = get_all_literals_edge_cases();

        for l in literals { 
            for t in ALL_TYPES_NO_ARR {
                assert_parse_err(&format!("func bad name() {{own x {} = {}\n}}", t, l));
            }
        }
        assert_parse_err("func bad name() {{\n}}");
    }

    #[test]
    fn function_inline_statements_in_braces_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("func bad() {{own x {} = 1\n}}\n", t));
            
            assert_parse_err(&format!("func bad() {{\nown x {} = 1}}\n", t));
        }
    }

    #[test]
    fn multiple_functions() {
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
    fn function_array_return_type() {
        for t in ALL_TYPES_NO_ARR {
            let ast = parse(&format!("func foo() []{} {{\n}}\n", t)).unwrap();
            let f = &ast.functions[0];
            assert_eq!(f.return_type, Some(vec![Type::Array(Box::new(t.clone()))]));
        }
    }

    #[test]
    fn function_nested_array_return_type() {
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

}
