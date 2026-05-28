use super::*;

#[cfg(test)]
mod lock_in_void_func_tests {
    use super::*;

    #[test]
    fn lock_literals_expr_panics() {
        let literals = get_all_literals();

        for l in literals {
            for i in 1..100 {
                let body = vec![
                    Stmt::Lock(vec![l.clone(); i]),
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
    fn lock_binop_expr_panics() {
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
                    Stmt::Lock(vec![bin]),
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

}
