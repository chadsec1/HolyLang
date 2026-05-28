use super::*;

#[cfg(test)]
mod multi_return_tests {
    use super::*; 

    #[test]
    fn multi_decl() {
        let literals = get_all_literals();
        
        for (l1, t1) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t1_str = holy_type_to_rust_type_str(&t1);
            let l1_str = holy_expr_to_rust_expr(&l1);

            for (l2, t2) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
                let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
                let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

                let vars = vec![
                    MultiVariableDeclaration { name: "a".to_string(), type_name: t1.clone(), span: span() },
                    MultiVariableDeclaration { name: "b".to_string(), type_name: t2.clone(), span: span() },
                ];
                let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
                let main = void_func("main", vec![], body);

                let ast = &AST{ functions: vec![pair, main], globals: vec![] };

                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let t2_str = holy_type_to_rust_type_str(&t2);
                let l2_str = holy_expr_to_rust_expr(&l2);

                assert_eq!(
                    rcode, 
                    format!(
                        "fn pair() -> ({}, {}) {{ return ({}, {})}}fn main() {{ let (a, b): ({}, {}) = pair();}}",
                        t1_str, t2_str, l1_str, l2_str, t1_str, t2_str, 
                    )
                );
            }
        }
    }

    #[test]
    fn multi_decl_bincop_condition() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = holy_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let bl_str = holy_expr_to_rust_expr(&bl);

            let pair_body = vec![return_stmt(vec![bl.clone();3])];
            let pair = returning_func("pair", vec![], vec![t.clone(); 3], pair_body);

            let vars = vec![
                MultiVariableDeclaration { name: "a".to_string(), type_name: t.clone(), span: span() },
                MultiVariableDeclaration { name: "b".to_string(), type_name: t.clone(), span: span() },
                MultiVariableDeclaration { name: "c".to_string(), type_name: t.clone(), span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let ast = &AST{ functions: vec![pair, main], globals: vec![] };

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");


            assert_eq!(
                rcode, 
                format!(
                    "fn pair() -> ({}, {}, {}) {{ return ({}, {}, {})}}fn main() {{ let (a, b, c): ({}, {}, {}) = pair();}}",
                    t_str, t_str, t_str, bl_str, bl_str, bl_str, t_str, t_str, t_str
                )
            );
        }
    }

    #[test]
    fn multi_decl_all_bin_op() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = holy_type_to_rust_type_str(&t);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let pair_body = vec![return_stmt(vec![bin.clone();3])];
                let pair = returning_func("pair", vec![], vec![t.clone(); 3], pair_body);

                let vars = vec![
                    MultiVariableDeclaration { name: "a".to_string(), type_name: t.clone(), span: span() },
                    MultiVariableDeclaration { name: "b".to_string(), type_name: t.clone(), span: span() },
                    MultiVariableDeclaration { name: "c".to_string(), type_name: t.clone(), span: span() },
                ];
                let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
                let main = void_func("main", vec![], body);

                let ast = &AST{ functions: vec![pair, main], globals: vec![] };
            
                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(
                    rcode, 
                    format!(
                        "fn pair() -> ({}, {}, {}) {{ return ({}, {}, {})}}fn main() {{ let (a, b, c): ({}, {}, {}) = pair();}}",
                        t_str, t_str, t_str, bin_str, bin_str, bin_str, t_str, t_str, t_str
                    )
                );
            }
        }
    }

    #[test]
    fn multi_assignment_all_bin_op() {
        let literals = get_all_literals();

        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = holy_type_to_rust_type_str(&t);
            
            for b in ALL_BIN_OP_KIND {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let pair_body = vec![return_stmt(vec![bin.clone();3])];
                let pair = returning_func("pair", vec![], vec![t.clone(); 3], pair_body);

                let body = vec![
                    Stmt::VarAssignMulti(MultiAssignment{
                        names: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                        value: call_expr("pair", vec![]),
                        span: span()
                    })
                ];
                let main = void_func("main", vec![], body);

                let ast = &AST{ functions: vec![pair, main], globals: vec![] };
            
                let internals = import_internals();
                let rcode = transpile(ast);
                assert!(rcode.starts_with(&internals));
                let rcode = rcode[internals.len()..].replace('\n', "");

                let bin_str = holy_expr_to_rust_expr(&bin);

                assert_eq!(
                    rcode, 
                    format!(
                        "fn pair() -> ({}, {}, {}) {{ return ({}, {}, {})}}fn main() {{ (a, b, c) = pair();}}",
                        t_str, t_str, t_str, bin_str, bin_str, bin_str
                    )
                );
            }
        }
    }

}
