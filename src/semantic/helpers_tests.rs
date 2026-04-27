use super::*;


use crate::ast::{
    IntLiteralValue
};

use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_TYPES_NO_ARR_NO_FLOAT,
    ALL_TYPES_NO_INTS_NO_ARR,
    ALL_INT_TYPES_NO_ARR

};

use crate::semantic::helpers::{
    assign_default_value_for_type,
    get_bigger_type_of_two_integers,
};


fn get_int_literals_value() -> [IntLiteralValue; 11] {
    let literals = [
        IntLiteralValue::Int8(0),   IntLiteralValue::Int16(0),
        IntLiteralValue::Int32(0),  IntLiteralValue::Int64(0),
        IntLiteralValue::Int128(0), IntLiteralValue::Byte(0),
        IntLiteralValue::Uint16(0), IntLiteralValue::Uint32(0),
        IntLiteralValue::Uint64(0), IntLiteralValue::Uint128(0),
        IntLiteralValue::Usize(0)
    ];

    return literals;
}


#[cfg(test)]
mod type_tests {
    use super::*;

    // Type::is_integer_type
    #[test]
    fn integer_type_signed_all_variants() {
        assert!(Type::Int8.is_integer_type());
        assert!(Type::Int16.is_integer_type());
        assert!(Type::Int32.is_integer_type());
        assert!(Type::Int64.is_integer_type());
        assert!(Type::Int128.is_integer_type());
    }

    #[test]
    fn integer_type_unsigned_all_variants() {
        assert!(Type::Byte.is_integer_type());
        assert!(Type::Uint16.is_integer_type());
        assert!(Type::Uint32.is_integer_type());
        assert!(Type::Uint64.is_integer_type());
        assert!(Type::Uint128.is_integer_type());
        assert!(Type::Usize.is_integer_type());
    }

    #[test]
    fn integer_type_rejects_non_integers() {
        // Every non-integer type must return false, no leaks
        assert!(!Type::Float64.is_integer_type());
        assert!(!Type::Bool.is_integer_type());
        assert!(!Type::String.is_integer_type());
        
        for ty in ALL_TYPES_NO_ARR {
            assert!(!Type::Array(Box::new(ty.clone())).is_integer_type());
        }

    }

    // Type::is_floating_type
    #[test]
    fn floating64_type_pass() {
        assert!(Type::Float64.is_floating_type());
    }

    #[test]
    fn floating_type_rejects_non_floats() {
        for t in ALL_TYPES_NO_ARR_NO_FLOAT {
            assert!(!t.is_floating_type());
        }
    }

    // A type cannot be both integer AND float, ever.
    #[test]
    fn no_type_is_both_integer_and_float() {
        for ty in ALL_TYPES_NO_ARR {
            assert!(
                !(ty.is_integer_type() && ty.is_floating_type()),
                "Type {:?} is claiming to be both integer AND float, which is impossible", ty
            );

            let arr_ty = Type::Array(Box::new(ty.clone()));

            assert!(
                !(arr_ty.is_integer_type() && arr_ty.is_floating_type()),
                "Array of type {:?} is claiming to be both integer AND float, which is impossible", ty
            );

        }
    }

    // IntLiteralValue::get_type: every variant maps to exactly its own Type

    #[test]
    fn int_literal_get_type_all_variants() {
        assert_eq!(IntLiteralValue::Int8(0).get_type(),    Type::Int8);
        assert_eq!(IntLiteralValue::Int16(0).get_type(),   Type::Int16);
        assert_eq!(IntLiteralValue::Int32(0).get_type(),   Type::Int32);
        assert_eq!(IntLiteralValue::Int64(0).get_type(),   Type::Int64);
        assert_eq!(IntLiteralValue::Int128(0).get_type(),  Type::Int128);

        assert_eq!(IntLiteralValue::Byte(0).get_type(),    Type::Byte);
        assert_eq!(IntLiteralValue::Uint16(0).get_type(),  Type::Uint16);
        assert_eq!(IntLiteralValue::Uint32(0).get_type(),  Type::Uint32);
        assert_eq!(IntLiteralValue::Uint64(0).get_type(),  Type::Uint64);
        assert_eq!(IntLiteralValue::Uint128(0).get_type(), Type::Uint128);
        assert_eq!(IntLiteralValue::Usize(0).get_type(),   Type::Usize);
    }


    // IntLiteralValue::is_signed

    #[test]
    fn is_signed_true_for_all_signed_variants() {
        assert!(IntLiteralValue::Int8(0).is_signed());
        assert!(IntLiteralValue::Int16(0).is_signed());
        assert!(IntLiteralValue::Int32(0).is_signed());
        assert!(IntLiteralValue::Int64(0).is_signed());
        assert!(IntLiteralValue::Int128(0).is_signed());
    }

    #[test]
    fn is_signed_false_for_all_unsigned_variants() {
        assert!(!IntLiteralValue::Byte(0).is_signed());
        assert!(!IntLiteralValue::Uint16(0).is_signed());
        assert!(!IntLiteralValue::Uint32(0).is_signed());
        assert!(!IntLiteralValue::Uint64(0).is_signed());
        assert!(!IntLiteralValue::Uint128(0).is_signed());
        assert!(!IntLiteralValue::Usize(0).is_signed());
    }

    // get_type() and is_signed() must be consistent:
    // if get_type() returns a signed type, is_signed() must be true, and vice versa.
    #[test]
    fn get_type_and_is_signed_are_consistent() {
        let signed_types   = [Type::Int8, Type::Int16, Type::Int32, Type::Int64, Type::Int128];
        let unsigned_types = [Type::Byte, Type::Uint16, Type::Uint32, Type::Uint64, Type::Uint128, Type::Usize];

        let variants = get_int_literals_value();

        for v in variants {
            let ty = v.get_type();
            if v.is_signed() {
                assert!(signed_types.contains(&ty),
                    "{:?}: is_signed=true but get_type()={:?} is not a signed type", v, ty);
            } else {
                assert!(unsigned_types.contains(&ty),
                    "{:?}: is_signed=false but get_type()={:?} is not an unsigned type", v, ty);
            }
        }
    }

    // IntLiteralValue::as_i128 value preservation and boundary checks

    #[test]
    fn as_i128_preserves_values() {
        assert_eq!(IntLiteralValue::Int8(i8::MAX).as_i128(),    i8::MAX as i128);
        assert_eq!(IntLiteralValue::Int8(i8::MIN).as_i128(),    i8::MIN as i128);
        assert_eq!(IntLiteralValue::Int16(i16::MAX).as_i128(),  i16::MAX as i128);
        assert_eq!(IntLiteralValue::Int16(i16::MIN).as_i128(),  i16::MIN as i128);
        assert_eq!(IntLiteralValue::Int32(i32::MAX).as_i128(),  i32::MAX as i128);
        assert_eq!(IntLiteralValue::Int32(i32::MIN).as_i128(),  i32::MIN as i128);
        assert_eq!(IntLiteralValue::Int64(i64::MAX).as_i128(),  i64::MAX as i128);
        assert_eq!(IntLiteralValue::Int64(i64::MIN).as_i128(),  i64::MIN as i128);
        assert_eq!(IntLiteralValue::Int128(i128::MAX).as_i128(), i128::MAX);
        assert_eq!(IntLiteralValue::Int128(i128::MIN).as_i128(), i128::MIN);
        assert_eq!(IntLiteralValue::Int32(0).as_i128(), 0);
        assert_eq!(IntLiteralValue::Int32(-1).as_i128(), -1);
    }

    // Calling as_i128 on ANY unsigned variant must panic.
    // These are testing that the safety guard in the compiler actually fires.

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_byte() {
        IntLiteralValue::Byte(255).as_i128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_uint16() {
        IntLiteralValue::Uint16(u16::MAX).as_i128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_uint32() {
        IntLiteralValue::Uint32(u32::MAX).as_i128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_uint64() {
        IntLiteralValue::Uint64(u64::MAX).as_i128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_uint128() {
        IntLiteralValue::Uint128(u128::MAX).as_i128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_i128_panics_on_usize() {
        IntLiteralValue::Usize(usize::MAX).as_i128();
    }

    // IntLiteralValue::as_u128 value preservation and boundary checks

    #[test]
    fn as_u128_preserves_values() {
        assert_eq!(IntLiteralValue::Byte(u8::MAX).as_u128(),      u8::MAX as u128);
        assert_eq!(IntLiteralValue::Byte(0).as_u128(),            0);
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).as_u128(),   u16::MAX as u128);
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).as_u128(),   u32::MAX as u128);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).as_u128(),   u64::MAX as u128);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).as_u128(), u128::MAX);
        assert_eq!(IntLiteralValue::Usize(usize::MAX).as_u128(),  usize::MAX as u128);
    }

    // Calling as_u128 on ANY signed variant must panic.

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_u128_panics_on_int8() {
        IntLiteralValue::Int8(-1).as_u128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_u128_panics_on_int16() {
        IntLiteralValue::Int16(-1).as_u128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_u128_panics_on_int32() {
        IntLiteralValue::Int32(-1).as_u128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_u128_panics_on_int64() {
        IntLiteralValue::Int64(-1).as_u128();
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn as_u128_panics_on_int128() {
        IntLiteralValue::Int128(-1).as_u128();
    }

}


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
