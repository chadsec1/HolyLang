use super::*;

#[cfg(test)]
mod const_decl_tests {
    use super::*; 
    
    #[test]
    fn const_decl_no_type_errors() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            assert_parse_err(&wrap(&format!("const x = {}", l)));
        }
    }

    #[test]
    fn const_decl_in_func() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let stmts = parse_body(&format!("const x {} = {}", t, l));
                if let Stmt::Const(c) = &stmts[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else {
                    panic!("Expected VarDecl");
                }
            }
        }
    }

    #[test]
    fn const_decl_in_globals() {
        let literals_edge_cases = get_all_literals_edge_cases(); 

        for l in &literals_edge_cases {
            for t in ALL_TYPES_NO_ARR {
                let ast = parse(&format!("const x {} = {}", t, l)).unwrap();
                if let Stmt::Const(c) = &ast.globals[0] {
                    assert_eq!(c.name, "x");
                    assert_eq!(c.type_name, t.clone());
                } else {
                    panic!("Expected VarDecl");
                }
            }
        }
    }

    #[test]
    fn const_decl_no_value_in_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("const x {}", t)));
        }
    }

    #[test]
    fn const_decl_no_value_in_globals_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&format!("const x {}", t));
        }
    }
}
