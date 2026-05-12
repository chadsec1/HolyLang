use super::*;

#[cfg(test)]
mod array_slicing_tests {
    use super::*;

    #[test]
    fn test_array_slice_both_bounds() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            match parse(&format!("arr[{}:{}]", l, l)).unwrap() {
                Expr::ArraySlicing { array, range, .. } => {
                    assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                    assert!(matches!(range, ArraySliceRange::FromTo(_, _)))
                }
                other => panic!("expected ArraySlicing, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_array_slice_both_bounds_invalid_exprs_errors() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            if l.starts_with('-') {
                continue
            }

            assert_parse_err(&format!("arr[{} {}:{}]", l, l, l));
            assert_parse_err(&format!("arr[{}:{} {}]", l, l, l));
            assert_parse_err(&format!("arr[{} {}:{} {}]", l, l, l, l));
        }
    }




    #[test]
    fn test_array_slice_more_than_2_bounds() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            assert_parse_err(&format!("arr[{}:{}:{}]", l, l, l));
        }
    }



    #[test]
    fn test_array_slice_start_only() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            match parse(&format!("arr[{}:]", l)).unwrap() {
                Expr::ArraySlicing { array, range, .. } => {
                    assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                    assert!(matches!(range, ArraySliceRange::From(_)))
                }
                other => panic!("expected ArraySlicing, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_array_slice_start_only_invalid_expr_errors() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            if l.starts_with('-') {
                continue
            }

            assert_parse_err(&format!("arr[{} {}:]", l, l));
        }
    }



    #[test]
    fn test_array_slice_end_only() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            match parse(&format!("arr[:{}]", l)).unwrap() {
                Expr::ArraySlicing { array, range, .. } => {
                    assert!(matches!(*array, Expr::Var { name, .. } if name == "arr"));
                    assert!(matches!(range, ArraySliceRange::To(_)))
                }
                other => panic!("expected ArraySlicing, got {:?}", other),
            }
        }
    }

    #[test]
    fn test_array_slice_end_only_invalid_expr_errors() {
        let literals = get_all_literals_edge_cases();
        
        for l in literals { 
            if l.starts_with('-') {
                continue
            }

            assert_parse_err(&format!("arr[:{} {}]", l, l));
        }
    }



    #[test]
    fn test_array_slice_both_empty_errors() {
        assert_parse_err("arr[:]");
    }

}
