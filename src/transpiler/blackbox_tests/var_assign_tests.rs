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

                let rcode = transpile(ast).replace("\n", "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(rcode, format!("fn foo() {{ x = {};}}", bin_str));
            }
        }
    }


}
