use super::*;

#[cfg(test)]
mod var_assign_tests {
    use super::*;

    #[test]
    fn inited_var_assign() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()), 
                Stmt::Unlock(vec![var_expr("x")]),
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 0);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 3);

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::Unlock(vec) = &ast.functions[0].body[1] {
                assert_eq!(vec.len(), 1);
            } else { panic!("expected Unlock, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[2] {
                assert_eq!(va.name, "x");
                assert_eq!(va.value, l.clone());
            } else { panic!("expected VarAssign, got {:?}", ast); }
        }
    }

    #[test]
    fn uninited_var_assign() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let body = vec![
                var_decl(false, "x", t.clone(), l.clone()), 
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();

            assert_eq!(ast.globals.len(), 0);
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.functions[0].body.len(), 2);

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());
                assert_eq!(v.value, l.clone());
            } else { panic!("expected VarDecl, got {:?}", ast); }

            if let Stmt::VarAssign(va) = &ast.functions[0].body[1] {
                assert_eq!(va.name, "x");
                assert_eq!(va.value, l.clone());
            } else { panic!("expected VarAssign, got {:?}", ast); }
        }
    }

    #[test]
    fn test_varassign_local_const_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define_locally("x", t.clone(), l.clone()), 
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot assign to constant"));
        }
    }

    #[test]
    fn test_varassign_global_const_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = AST { functions: vec![func] , globals: vec![ const_define_globally("x", t.clone(), l.clone()) ] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot assign to constant"));
        }
    }

    #[test]
    fn test_varassign_type_mismatch_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;
        
        for l in literals_ints {
            for t in ALL_TYPES_NO_INTS_NO_ARR {
                let body = vec![
                    var_decl(true, "x", t.clone(), l.clone()),
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
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl(true, "x", t.clone(), l.clone()), 
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
                var_decl(true, "x", t.clone(), var_expr("y")),
            ]; 
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared binding"));
        }
    }

}
