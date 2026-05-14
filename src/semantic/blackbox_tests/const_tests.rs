use super::*;

#[cfg(test)]
mod const_tests {
    use super::*;

    #[test]
    fn const_literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ const_define_locally("x", t.clone(), l.clone()) ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            assert_eq!(ast.functions.len(), 1);
            assert_eq!(ast.globals.len(), 0);

            if let Stmt::Const(c) = &ast.functions[0].body[0] {
                assert_eq!(c.name, "x");
                assert_eq!(c.type_name, t.clone());
                assert_eq!(c.value, l.clone());
            } else { panic!("expected Const, got {:?}", ast); }
        }
    }


    #[test]
    fn type_mismatch_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for t in ALL_INT_TYPES_NO_ARR {
            for l in &literals_no_ints {
                let body = vec![const_define_locally("x", t.clone(), l.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.globals.len(), 0);


                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }

    
    #[test]
    fn dynamic_arrays_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral { elements: elements, span: span() };

                let body = vec![const_define_locally("x", Type::Array(Box::new(t.clone())), arr_lit.clone())];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert_eq!(ast.functions.len(), 1);
                assert_eq!(ast.globals.len(), 0);



                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Dynamic arrays cannot be evaluated at compile time"));
            }
        }
    }

    #[test]
    fn dynamic_array_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        // We use no_ints here because if we included int literals, they would get inferred to
        // correct type if they fit, and since functions return 1 for all ints, they would always
        // fit.
        for ((l1, t1), l2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(literals_no_ints.iter())
        {
            for i in 0..=100 {
                let mut elements = vec![l1.clone(); i];

                elements.push(l2.clone());
                
                let arr_lit = Expr::ArrayLiteral { elements: elements.clone(), span: span(), };

                let body = vec![const_define_locally("x", Type::Array(Box::new(t1.clone())), arr_lit.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());

                let assert_cond = result.unwrap_err().to_string();
                let assert_cond = assert_cond.contains("Array element type mismatch:") | 
                                    assert_cond.contains("Type mismatch assigning to `x`");

                assert!(assert_cond);
            }       
        }
    }


    #[test]
    fn test_define_const_name_taken_by_func_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let main = void_func("main", vec![], vec![
                const_define_locally("foo", t.clone(), l.clone())
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("`foo` is already taken by a function."));
        }
    }


    #[test]
    fn test_define_const_name_taken_by_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let main = void_func("main", vec![], vec![
                var_decl("foo", t.clone(), l.clone()),
                const_define_locally("foo", t.clone(), l.clone())
            ]);

            let mut ast = AST { functions: vec![main], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("it is already declared"));
        }
    }

    #[test]
    fn test_define_const_assign_errors() {
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


}
