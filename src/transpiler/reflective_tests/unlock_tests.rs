use super::*;

#[cfg(test)]
mod unlock_in_void_func_tests {
    use super::*;

    #[test]
    fn unlock_literals_expr_panics() {
        let literals = get_all_literals();

        for l in literals {
            for i in 1..100 {
                let body = vec![
                    Stmt::Unlock(vec![l.clone(); i]),
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }

    #[test]
    fn unlock_binop_expr_panics() {
        let literals = get_all_literals();

        for l in literals {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::Unlock(vec![bin]),
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }


    #[test]
    fn unlock_single_char_var() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();

        for l in letters {
            for i in 1..100 {
                let body = vec![
                    Stmt::Unlock(vec![var_expr(&l.to_string()); i]),
                ];

                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..]
                                .replace('\n', "")
                                .replace("// Unlock statement started", "")
                                .replace("// Unlock statement ended", "");

                let expected_rcode = format!("let mut {} = {};", l, l);
                let expected_rcode = expected_rcode.repeat(i);

                assert_eq!(rcode, format!("fn foo() {{ {}}}", expected_rcode));
            }
        }
    }
}

#[cfg(test)]
mod unlock_in_void_func_with_params_tests {
    use super::*;

    #[test]
    fn unlock_literals_expr_panics() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 1..100 {
                let body = vec![
                    Stmt::Unlock(vec![l.clone(); i]),
                ];
            
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }

    #[test]
    fn unlock_binop_expr_panics() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::Unlock(vec![bin]),
                ];
                let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }


    #[test]
    fn unlock_single_char_var() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();

        for l in letters {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                for i in 1..10 {
                    let body = vec![
                        Stmt::Unlock(vec![var_expr(&l.to_string()); i]),
                    ];

                    let func = void_func("foo", vec![param("a", t.clone()), param("b", t.clone())], body);
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..]
                                    .replace('\n', "")
                                    .replace("// Unlock statement started", "")
                                    .replace("// Unlock statement ended", "");

                    let expected_rcode = format!("let mut {} = {};", l, l);
                    let expected_rcode = expected_rcode.repeat(i);

                    let t_str = holy_type_to_rust_type_str(&t);
                    
                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) {{ {}}}", t_str, t_str, expected_rcode));
                }
            }
        }
    }
}


#[cfg(test)]
mod unlock_in_returning_func_tests {
    use super::*;

    #[test]
    fn unlock_literals_expr_panics() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for i in 1..100 {
                let body = vec![
                    Stmt::Unlock(vec![l.clone(); i]),
                ];
            
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }

    #[test]
    fn unlock_binop_expr_panics() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![
                    Stmt::Unlock(vec![bin]),
                ];
                let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body);
                let ast = &ast_one(func);

                let result = std::panic::catch_unwind(|| { 
                    let _ = transpile(ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?}", ast);
            }
        }
    }


    #[test]
    fn unlock_single_char_var() {
        let letters: Vec<char> = ('a'..='z')
            .chain('A'..='Z')
            .collect();

        for l in letters {
            for t in ALL_TYPES_WITH_DYN_ARR.iter() {
                for i in 1..10 {
                    let body = vec![
                        Stmt::Unlock(vec![var_expr(&l.to_string()); i]),
                    ];

                    let func = returning_func("foo", vec![param("a", t.clone()), param("b", t.clone())], vec![t.clone()], body);
                    let ast = &ast_one(func);

                    let internals = import_internals();
                    let rcode = transpile(ast);
                    assert!(rcode.starts_with(&internals));
                    let rcode = rcode[internals.len()..]
                                    .replace('\n', "")
                                    .replace("// Unlock statement started", "")
                                    .replace("// Unlock statement ended", "");

                    let expected_rcode = format!("let mut {} = {};", l, l);
                    let expected_rcode = expected_rcode.repeat(i);
                    
                    let t_str = holy_type_to_rust_type_str(&t);

                    assert_eq!(rcode, format!("fn foo(a: {}, b: {}) -> {} {{ {}}}", t_str, t_str, t_str, expected_rcode));
                }
            }
        }
    }
}
