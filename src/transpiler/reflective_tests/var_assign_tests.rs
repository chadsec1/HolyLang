use super::*;

#[cfg(test)]
mod var_assign_in_void_func_tests {
    use super::*;

    #[test]
    fn assign_all_bin_op() {
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
                    var_assign("x", bin.clone())
                ];
                let func = void_func("foo", vec![], body);
                let ast = &ast_one(func);

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ x = {};}}", bin_str));
            }
        }
    }

    #[test]
    fn assign_literals_and_dynamic_arrays() {
        let literals = get_all_literals();
        
        for l in literals {
            let body = vec![
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ x = {};}}", l_str));
        }
    }

}
