use super::*;

#[cfg(test)]
mod var_assign_tests {
    use super::*;

    #[test]
    fn test_varassign_type_mismatch_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;
        
        for l in literals_ints {
            for t in ALL_TYPES_NO_INTS_NO_ARR {
                let body = vec![
                    var_decl("x", t.clone(), None),
                    var_assign("x", l.clone())
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }



    #[test]
    fn test_varassign_uses_non_declared_var_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![
                var_decl("x", t.clone(), None), 
                var_assign("x", var_expr("y"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared binding `y`"));
        }
    }


    #[test]
    fn test_assignment_of_undeclared_variable_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let body = vec![var_assign("x", l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared variable"));
        }
    }

    #[test]
    fn test_assignment_of_undeclared_variable_other_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![
                var_decl("x", t.clone(), Some(var_expr("y"))),
            ]; 
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }



}
