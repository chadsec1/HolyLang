use super::*;

#[cfg(test)]
mod float64_literals_tests {
    use super::*;

    #[test]
    fn infinite_float64_overflow_errors() {
        // A number larger than f64::MAX should produce a parse error
        let huge = "1.7976931348623157e+309"; // this is higher than f64::MAX 
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = {}", t, huge)));
        }
    }

    #[test]
    fn float_literal_f32() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1.0", t));
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                    assert_eq!(*value, 1.0);
                } else { panic!("Expected Float64Literal"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn float_literal_f64_high_precision() {
        // More than 8 significant digits, then it must be f64
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1.123456789", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::Float64Literal { value, .. }) = &v.value {
                    assert_eq!(*value, 1.123456789);
                } else { panic!("Expected Float64Literal"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn float_literal_multiple_dots_errors() {
        for t in ALL_TYPES_NO_ARR {
            assert_parse_err(&wrap(&format!("own x {} = 1.2.3", t)));
        }
    }


    
}
