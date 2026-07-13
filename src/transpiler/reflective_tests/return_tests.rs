use super::*;

#[cfg(test)]
mod return_tests {
    use super::*; 

    #[test]
    fn decl_inited() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            let pair_body = vec![return_stmt(vec![l.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let body = vec![
                var_decl("a", t.clone(), call_expr("pair", vec![]), true)
            ];
            let main = void_func("main", vec![], body);

            let ast = &AST{ functions: vec![pair, main], globals: vec![] };

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            assert_eq!(
                rcode, 
                format!(
                    "fn pair() -> {} {{ return {}}}fn main() {{ let a: {} = pair();}}",
                    t_str, l_str, t_str 
                )
            );
        }
    }

    #[test]
    fn decl_uninited() {
        let literals = get_all_literals();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_WITH_DYN_ARR.iter()) {
            let t_str = gold_type_to_rust_type_str(&t);
            let l_str = gold_expr_to_rust_expr(&l);

            let pair_body = vec![return_stmt(vec![l.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let body = vec![
                var_decl("a", t.clone(), call_expr("pair", vec![]), false)
            ];
            let main = void_func("main", vec![], body);

            let ast = &AST{ functions: vec![pair, main], globals: vec![] };

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            assert_eq!(
                rcode, 
                format!(
                    "fn pair() -> {} {{ return {}}}fn main() {{ let mut a: {} = pair();}}",
                    t_str, l_str, t_str 
                )
            );
        }
    }

    #[test]
    fn decl_inited_binop() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let bl_str = gold_expr_to_rust_expr(&bl);

            let pair_body = vec![return_stmt(vec![bl.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let body = vec![
                var_decl("a", t.clone(), call_expr("pair", vec![]), true)
            ];
            let main = void_func("main", vec![], body);

            let ast = &AST{ functions: vec![pair, main], globals: vec![] };

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            assert_eq!(
                rcode, 
                format!(
                    "fn pair() -> {} {{ return {}}}fn main() {{ let a: {} = pair();}}",
                    t_str, bl_str, t_str 
                )
            );
        }
    }

    #[test]
    fn decl_uninited_binop() {
        let boolean_conds = get_many_boolean_conditions();
        
        let t = Type::Bool;
        let t_str = gold_type_to_rust_type_str(&t);
        
        for bl in boolean_conds {
            let bl_str = gold_expr_to_rust_expr(&bl);

            let pair_body = vec![return_stmt(vec![bl.clone()])];
            let pair = returning_func("pair", vec![], vec![t.clone()], pair_body);

            let body = vec![
                var_decl("a", t.clone(), call_expr("pair", vec![]), false)
            ];
            let main = void_func("main", vec![], body);

            let ast = &AST{ functions: vec![pair, main], globals: vec![] };

            let internals = import_internals();
            let rcode = transpile(ast);
            assert!(rcode.starts_with(&internals));
            let rcode = rcode[internals.len()..].replace('\n', "");

            assert_eq!(
                rcode, 
                format!(
                    "fn pair() -> {} {{ return {}}}fn main() {{ let mut a: {} = pair();}}",
                    t_str, bl_str, t_str 
                )
            );
        }
    }
}
