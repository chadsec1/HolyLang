use super::*;

#[cfg(test)]
mod array_literal_tests {
    use super::*;

    #[test]
    fn array_literal_edge_cases_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = 1, 2, 3", t)));
            assert_parse_err(&wrap(&format!("own x {} = [int32[1, 2, 3]]", t)));
            assert_parse_err(&wrap(&format!("own x {} = int32[[1, 2, 3]]", t)));
        }
    }
}
