use super::*;

#[cfg(test)]
mod array_access_tests {
    use super::*;

    #[test]
    fn array_single_access() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::ArrayAccess { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn array_single_access_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[0]"));
    }


    #[test]
    fn array_access_variable_index() {
        let literals_edge_cases = get_all_literals_edge_cases();
        
        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    assert_eq!(v.type_name, t.clone());
                    assert!(matches!(v.value, Some(Expr::ArrayAccess { .. })));
                } else { panic!("Expected VarDecl"); }
            }
        }
    }


    #[test]
    fn array_access_variable_index_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[i]"));
    }



}
