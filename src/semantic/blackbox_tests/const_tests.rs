use super::*;

#[cfg(test)]
mod const_tests {
    use super::*;

    #[test]
    fn test_define_const_literals() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![const_define("x", t.clone(), l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
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
                let body = vec![const_define("x", t.clone(), l.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

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

                let body = vec![const_define("x", Type::Array(Box::new(t.clone())), arr_lit.clone())];
             
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Dynamic arrays cannot be evaluated at compile time"));
            }
        }
    }

    #[test]
    fn test_define_const_name_taken_by_func_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let main = void_func("main", vec![], vec![
                const_define("foo", t.clone(), l.clone())
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo], globals: vec![] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("`foo` is already taken by a function, pick a different name for your variable."));
        }
    }

    #[test]
    fn test_define_const_assign_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                const_define("x", t.clone(), l.clone()),
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
