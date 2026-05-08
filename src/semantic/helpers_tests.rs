use super::*;


use crate::ast::{
    IntLiteralValue
};

use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_TYPES_NO_INTS_NO_ARR,
    ALL_INT_TYPES_NO_ARR

};

use crate::semantic::helpers::{
    assign_default_value_for_type,
    get_bigger_type_of_two_integers,
};


#[cfg(test)]
mod helpers_tests {
    use super::*;

    fn dummy_span() -> Span {
        Span { line: 1, column: 1 }
    }

    // assign_default_value_for_type
    //
    #[test]
    fn default_value_signed_integers_are_zero() {
        let cases: &[(Type, IntLiteralValue)] = &[
            (Type::Int8,   IntLiteralValue::Int8(0)),
            (Type::Int16,  IntLiteralValue::Int16(0)),
            (Type::Int32,  IntLiteralValue::Int32(0)),
            (Type::Int64,  IntLiteralValue::Int64(0)),
            (Type::Int128, IntLiteralValue::Int128(0)),
        ];
        for (ty, expected_lit) in cases {
            let mut expr: Option<Expr> = None;
            assign_default_value_for_type(&mut expr, ty, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::IntLiteral { value, .. } => {
                    assert_eq!(value, *expected_lit,
                        "Default for {:?} should be {:?}", ty, expected_lit);
                }
                other => panic!("Expected IntLiteral for {:?}, got {:?}", ty, other),
            }
        }
    }

    #[test]
    fn default_value_unsigned_integers_are_zero() {
        let cases: &[(Type, IntLiteralValue)] = &[
            (Type::Byte,    IntLiteralValue::Byte(0)),
            (Type::Uint16,  IntLiteralValue::Uint16(0)),
            (Type::Uint32,  IntLiteralValue::Uint32(0)),
            (Type::Uint64,  IntLiteralValue::Uint64(0)),
            (Type::Uint128, IntLiteralValue::Uint128(0)),
            (Type::Usize,   IntLiteralValue::Usize(0)),
        ];
        for (ty, expected_lit) in cases {
            let mut expr: Option<Expr> = None;
            assign_default_value_for_type(&mut expr, ty, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::IntLiteral { value, .. } => {
                    assert_eq!(value, *expected_lit,
                        "Default for {:?} should be {:?}", ty, expected_lit);
                }
                other => panic!("Expected IntLiteral for {:?}, got {:?}", ty, other),
            }
        }
    }

    #[test]
    fn default_value_float64_is_zero() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::Float64, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::Float64Literal { value, .. } => {
                assert_eq!(value, 0.0f64);
            }
            other => panic!("Expected Float64 literal, got {:?}", other),
        }
    }

    #[test]
    fn default_value_bool_is_false() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::Bool, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::BoolLiteral { value: false, .. } => {}
            other => panic!("Expected BoolLiteral false, got {:?}", other),
        }
    }

    #[test]
    fn default_value_string_is_empty() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::String, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::StringLiteral { value, .. } => {
                assert_eq!(value, "", "Default string must be empty");
            }
            other => panic!("Expected StringLiteral, got {:?}", other),
        }
    }

    #[test]
    fn default_value_array_is_empty_with_correct_inner_type() {
        for t in ALL_TYPES_NO_ARR {
            let mut expr: Option<Expr> = None;
            let arr_t = Type::Array(Box::new(t.clone()));
            assign_default_value_for_type(&mut expr, &arr_t, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert!(elements.is_empty(), "Default array must be empty");
                }
                other => panic!("Expected ArrayLiteral, got {:?}", other),
            }
            
        }
    }

    // Nested array: inner type must also be preserved correctly
    #[test]
    fn default_value_nested_array_preserves_inner_type() {
        for t in ALL_TYPES_NO_ARR {
            let mut expr: Option<Expr> = None;
            let arr_t = Type::Array(Box::new(Type::Array(Box::new(t.clone()))));
            assign_default_value_for_type(&mut expr, &arr_t, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::ArrayLiteral { elements, .. } => {
                    assert!(elements.is_empty());
                }
                other => panic!("Expected nested ArrayLiteral, got {:?}", other),
            }
        }
    }

    // Calling on an existing Some(...) value, should trigger a panic guard
    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn default_value_overwrites_existing_some() {
        let mut expr: Option<Expr> = Some(Expr::IntLiteral {
            value: IntLiteralValue::Int32(999),
            span: dummy_span(),
        });
        assign_default_value_for_type(&mut expr, &Type::Int32, dummy_span()).unwrap();
    }


    // get_bigger_type_of_two_integers

    #[test]
    fn bigger_int_larger_signed_wins() {
        assert_eq!(get_bigger_type_of_two_integers(Type::Int64,  Type::Int32),  Type::Int64);
        assert_eq!(get_bigger_type_of_two_integers(Type::Int32,  Type::Int64),  Type::Int64);
        assert_eq!(get_bigger_type_of_two_integers(Type::Int128, Type::Int64),  Type::Int128);
        assert_eq!(get_bigger_type_of_two_integers(Type::Int16,  Type::Int8),   Type::Int16);
    }

    #[test]
    fn bigger_int_larger_unsigned_wins() {
        assert_eq!(get_bigger_type_of_two_integers(Type::Uint64,  Type::Uint32), Type::Uint64);
        assert_eq!(get_bigger_type_of_two_integers(Type::Uint32,  Type::Uint64), Type::Uint64);
        assert_eq!(get_bigger_type_of_two_integers(Type::Uint128, Type::Uint64), Type::Uint128);
        assert_eq!(get_bigger_type_of_two_integers(Type::Byte,    Type::Uint16), Type::Uint16);
    }

    // Same type — result must be that type (not garbage)
    #[test]
    fn bigger_int_same_type_returns_that_type() {
        assert_eq!(get_bigger_type_of_two_integers(Type::Int32,   Type::Int32),   Type::Int32);
        assert_eq!(get_bigger_type_of_two_integers(Type::Uint64,  Type::Uint64),  Type::Uint64);
        assert_eq!(get_bigger_type_of_two_integers(Type::Usize,   Type::Usize),   Type::Usize);
    }

    // Usize scores 8, same as Uint64. When scores tie, t_2 wins (falls through).
    // This documents current behavior so a future change to the scoring will be caught.
    #[test]
    fn bigger_int_usize_vs_uint64_tie_behavior_is_documented() {
        // Both score 8. When t_1 == t_2 score, the function returns t_2.
        let result = get_bigger_type_of_two_integers(Type::Usize, Type::Uint64);
        assert_eq!(result, Type::Uint64,
            "Tie behavior: when Usize (score=8) vs Uint64 (score=8), t_2 (Uint64) should win due to fall-through");

        let result2 = get_bigger_type_of_two_integers(Type::Uint64, Type::Usize);
        assert_eq!(result2, Type::Usize,
            "Tie behavior: when Uint64 (score=8) vs Usize (score=8), t_2 (Usize) should win due to fall-through");
    }

    // FOOTGUN: this function does NOT reject signed+unsigned mixing.
    // It will happily compare Int32 vs Uint64 and return one of them.
    // This test documents that gap so it's a conscious decision, not an oversight.
    #[test]
    fn bigger_int_does_not_reject_signed_unsigned_mix_footgun() {
        // Int32 scores 5, Uint64 scores 8 — so Uint64 wins.
        // No panic, no error. The caller must ensure they only pass same-signedness.
        let result = get_bigger_type_of_two_integers(Type::Int32, Type::Uint64);
        assert_eq!(result, Type::Uint64,
            "Documents that get_bigger_type_of_two_integers does NOT guard against signed/unsigned mixing. Callers are responsible.");
    }

    #[test]
    fn bigger_int_panics_on_non_integer_left() {
        for t1 in ALL_TYPES_NO_INTS_NO_ARR {
            for t2 in ALL_INT_TYPES_NO_ARR {
                let result = std::panic::catch_unwind(|| { 
                    get_bigger_type_of_two_integers(t1.clone(), t2.clone());
                });

                assert!(result.is_err(), "Expected panic for: {:?} {:?}", t1, t2);
            }
        }
    }

    #[test]
    fn bigger_int_panics_on_non_integer_right() {
        for t1 in ALL_TYPES_NO_INTS_NO_ARR {
            for t2 in ALL_INT_TYPES_NO_ARR {
                let result = std::panic::catch_unwind(|| { 
                    get_bigger_type_of_two_integers(t2.clone(), t1.clone());
                });

                assert!(result.is_err(), "Expected panic for: {:?} {:?}", t1, t2);
            }
        }
    }

    #[test]
    fn bigger_int_panics_on_both_non_integer() {
        for t in ALL_TYPES_NO_INTS_NO_ARR {
            let result = std::panic::catch_unwind(|| { 
                get_bigger_type_of_two_integers(t.clone(), t.clone());
            });

            assert!(result.is_err(), "Expected panic for: {:?}", t);
        }
    }
}
