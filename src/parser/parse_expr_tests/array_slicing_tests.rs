use super::*;

#[cfg(test)]
mod array_slicing_tests {
    use super::*;

    #[test]
    fn test_array_slice_both_bounds() {
        match parse("arr[1:5]").unwrap() {
            Expr::ArraySlicing { start, end, .. } => {
                assert!(start.is_some());
                assert!(end.is_some());
            }
            other => panic!("expected ArraySlicing, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_start_only() {
        match parse("arr[1:]").unwrap() {
            Expr::ArraySlicing { start, end, .. } => {
                assert!(start.is_some());
                assert!(end.is_none());
            }
            other => panic!("expected ArraySlicing, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_end_only() {
        match parse("arr[:5]").unwrap() {
            Expr::ArraySlicing { start, end, .. } => {
                assert!(start.is_none());
                assert!(end.is_some());
            }
            other => panic!("expected ArraySlicing, got {:?}", other),
        }
    }

    #[test]
    fn test_array_slice_both_empty_errors() {
        assert_parse_err("arr[:]");
    }

}
