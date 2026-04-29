use super::*;

#[cfg(test)]
mod array_slicing_tests {
    use super::*;
    
    #[test]
    fn array_slice_both_bounds() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}:{}]", t, l, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArraySlicing { start, end, .. }) = &v.value {
                        assert!(start.is_some());
                        assert!(end.is_some());
                    } else { panic!("Expected ArraySlicing"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_both_bounds_no_type_errors() {
        assert_parse_err(&wrap("own v = arr[1:3]"));
    }

    #[test]
    fn array_slice_open_start() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[:{}]", t, l));
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArraySlicing { start, end, .. }) = &v.value {
                        assert!(start.is_none());
                        assert!(end.is_some());
                    } else { panic!("Expected VarDecl"); }
                }
            }
        }
    }

    #[test]
    fn array_slice_open_end() {
        let literals_edge_cases = get_all_literals_edge_cases();

        for t in ALL_TYPES_NO_ARR {
            for l in &literals_edge_cases {
                let stmts = parse_body(&format!("own v {} = arr[{}:]", t, l));
        
                if let Stmt::VarDecl(v) = &stmts[0] {
                    if let Some(Expr::ArraySlicing { start, end, .. }) = &v.value {
                        assert!(start.is_some());
                        assert!(end.is_none());
                    } else { panic!(); }
                
                } else { panic!("Expected VarDecl"); }
            }
        }
    }

    #[test]
    fn array_access_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own v {} = arr[:]", t)));
            assert_parse_err(&wrap(&format!("own v {} = arr[]", t)));
        }
    }

}
