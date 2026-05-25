use super::*;

#[cfg(test)]
mod happy_path_tests {
    use super::*;

    #[test]
    fn test_full_valid_program_integers() {
        // This program tests all integers / floating points and all arthemtic binary operations
        // it also tests variable declaration, function declaration, and function calling 
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;

        for b in ALL_BIN_OP_KIND_ARTH {
            let add_body = vec![return_stmt(vec![
                Expr::BinOp {
                    left: Box::new(var_expr("a")),
                    op: b,
                    right: Box::new(var_expr("b")),
                    span: span(),
                }
            ])];
            

            for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
                let add = returning_func(
                    "add",
                    vec![param("a", t.clone()), param("b", t.clone())],
                    vec![t.clone()],
                    add_body.clone(),
                );

                let main_body = vec![
                    var_decl(true, "r", t.clone(), call_expr("add", vec![l.clone(), l.clone()])),
                ];
                let main = void_func("main", vec![], main_body);

                let mut ast = AST { functions: vec![add, main], globals: vec![] };
                
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }

}
