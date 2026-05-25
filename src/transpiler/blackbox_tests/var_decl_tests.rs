use super::*;

#[cfg(test)]
mod var_decl_tests {
    use super::*;

    #[test]
    fn literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), l.clone(), true)
            ];
            let func = void_func("foo", vec![], body);
            let ast = &ast_one(func);

            let rcode = transpile(ast).replace("\n", "");

            let t_str = holy_type_to_rust_type_str(&t);
            let l_str = holy_expr_to_rust_expr(&l);

            assert_eq!(rcode, format!("fn foo() {{ let mut x: {} = {};}}", t_str, l_str));
        }
    }
}
