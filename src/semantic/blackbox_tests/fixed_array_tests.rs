use super::*;

#[cfg(test)]
mod fixed_array_tests {
    use super::*;

    #[test]
    fn with_literal_size_element_type_mismatch_errors() {
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
                
                let arr_lit = array_lit(elements.clone(), Some(t1.clone()));

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("x")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("x", Type::FixedArray(Box::new(t1.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                        var_decl("y", t1.clone(), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());

                    let assert_cond = result.unwrap_err().to_string();
                    let assert_cond = assert_cond.contains("Array element type mismatch:") | 
                                        assert_cond.contains("Type mismatch assigning to `x`") |
                                        assert_cond.contains("Array literal is of");

                    assert!(assert_cond);
                }
            }       
        }


        // Same as above, but this time we test with a variable. All literals.
        let literals = get_all_literals_no_arr();
        for (((l1, t1), l2), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(literals.iter())
            .zip(ALL_TYPES_NO_ARR)
        {
            for i in 0..=100 {
                let mut elements = vec![l1.clone(); i];

                elements.push(var_expr("e"));
                
                let arr_lit = array_lit(elements.clone(), Some(t1.clone()));

                for i2 in 0..i+1 {
                    let access = Expr::ArrayAccess {
                        array: Box::new(var_expr("x")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", t2.clone(), l2.clone()),
                        var_decl("x", Type::FixedArray(Box::new(t1.clone()), FixedArraySize::Literal(i)), arr_lit.clone()),
                        var_decl("y", t1.clone(), access),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());

                    let assert_cond = result.unwrap_err().to_string();
                    let assert_cond = assert_cond.contains("Array element type mismatch:") | 
                                        assert_cond.contains("Type mismatch assigning to `x`") |
                                        assert_cond.contains("Array literal is of");

                    assert!(assert_cond);
                }
            }       
        }
    }

    // TODO: Add more fixed sized array tests
}
