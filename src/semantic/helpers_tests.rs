use super::*;

use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_TYPES_NO_ARR_NO_INFER,
    ALL_TYPES_NO_ARR_NO_FLOAT
};

use crate::semantic::helpers::{
    assign_default_value_for_type,
    get_bigger_type_of_two_integers,
    get_bigger_type_of_two_floatings,
    resolve_binary_op_types_numeric,
    type_compatible
};

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
        assert!(!Type::Float32.is_integer_type());
        assert!(!Type::Float64.is_integer_type());
        assert!(!Type::Bool.is_integer_type());
        assert!(!Type::String.is_integer_type());
        
        for ty in ALL_TYPES_NO_ARR {
            assert!(!Type::Array(Box::new(ty.clone())).is_integer_type());
        }

        assert!(!Type::Infer.is_integer_type());
    }

    // Type::is_floating_type
    #[test]
    fn floating_type_all_variants() {
        assert!(Type::Float32.is_floating_type());
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

    // get_type must not return Infer, ever.
    #[test]
    fn int_literal_get_type_never_returns_infer() {
        let variants = vec![
            IntLiteralValue::Int8(0),
            IntLiteralValue::Int16(0),
            IntLiteralValue::Int32(0),
            IntLiteralValue::Int64(0),
            IntLiteralValue::Int128(0),
            IntLiteralValue::Byte(0),
            IntLiteralValue::Uint16(0),
            IntLiteralValue::Uint32(0),
            IntLiteralValue::Uint64(0),
            IntLiteralValue::Uint128(0),
            IntLiteralValue::Usize(0),
        ];
        for v in variants {
            assert_ne!(v.get_type(), Type::Infer, "{:?}.get_type() returned Infer", v);
        }
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

        let variants = vec![
            IntLiteralValue::Int8(0),   IntLiteralValue::Int16(0),
            IntLiteralValue::Int32(0),  IntLiteralValue::Int64(0),
            IntLiteralValue::Int128(0), IntLiteralValue::Byte(0),
            IntLiteralValue::Uint16(0), IntLiteralValue::Uint32(0),
            IntLiteralValue::Uint64(0), IntLiteralValue::Uint128(0),
            IntLiteralValue::Usize(0),
        ];

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

    // FloatLiteralValue::get_type

    #[test]
    fn float_literal_get_type_all_variants() {

        for i in 0..=100000 {
            assert_eq!(FloatLiteralValue::Float32(i as f32).get_type(), Type::Float32);
        }

        for i in 0..=100000 {
            assert_eq!(FloatLiteralValue::Float64(i as f64).get_type(), Type::Float64);
        }    
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
    fn default_value_float32_is_zero() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::Float32, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::FloatLiteral { value: FloatLiteralValue::Float32(v), .. } => {
                assert_eq!(v, 0.0f32);
            }
            other => panic!("Expected Float32 literal, got {:?}", other),
        }
    }

    #[test]
    fn default_value_float64_is_zero() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::Float64, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::FloatLiteral { value: FloatLiteralValue::Float64(v), .. } => {
                assert_eq!(v, 0.0f64);
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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let mut expr: Option<Expr> = None;
            let arr_t = Type::Array(Box::new(t.clone()));
            assign_default_value_for_type(&mut expr, &arr_t, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::ArrayLiteral { elements, array_ty, .. } => {
                    assert!(elements.is_empty(), "Default array must be empty");
                    assert_eq!(array_ty, t.clone(), "Inner type must be preserved");
                }
                other => panic!("Expected ArrayLiteral, got {:?}", other),
            }
            
        }
    }

    // Nested array: inner type must also be preserved correctly
    #[test]
    fn default_value_nested_array_preserves_inner_type() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let mut expr: Option<Expr> = None;
            let arr_t = Type::Array(Box::new(Type::Array(Box::new(t.clone()))));
            assign_default_value_for_type(&mut expr, &arr_t, dummy_span()).unwrap();
            match expr.unwrap() {
                Expr::ArrayLiteral { elements, array_ty, .. } => {
                    assert!(elements.is_empty());
                    // inner type should be Array(Float64)
                    assert_eq!(array_ty, Type::Array(Box::new(t.clone())));
                }
                other => panic!("Expected nested ArrayLiteral, got {:?}", other),
            }
        }
    }

    // Calling on an existing Some(...) must overwrite it, not append or ignore
    #[test]
    fn default_value_overwrites_existing_some() {
        let mut expr: Option<Expr> = Some(Expr::IntLiteral {
            value: IntLiteralValue::Int32(999),
            span: dummy_span(),
        });
        assign_default_value_for_type(&mut expr, &Type::Int32, dummy_span()).unwrap();
        match expr.unwrap() {
            Expr::IntLiteral { value: IntLiteralValue::Int32(v), .. } => {
                assert_eq!(v, 0, "Must overwrite previous value with default 0");
            }
            other => panic!("Unexpected: {:?}", other),
        }
    }

    // Infer must panic, this is a compiler bug(s) guard
    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn default_value_panics_on_infer() {
        let mut expr: Option<Expr> = None;
        assign_default_value_for_type(&mut expr, &Type::Infer, dummy_span()).unwrap();
    }

    // type_compatible

    #[test]
    fn type_compatible_same_primitives() {
        assert!(type_compatible(&Type::Int32, &Type::Int32));
        assert!(type_compatible(&Type::Float64, &Type::Float64));
        assert!(type_compatible(&Type::Bool, &Type::Bool));
        assert!(type_compatible(&Type::String, &Type::String));
        assert!(type_compatible(&Type::Usize, &Type::Usize));
    }

    #[test]
    fn type_compatible_same_array_inner_type() {
        let a = Type::Array(Box::new(Type::Int32));
        let b = Type::Array(Box::new(Type::Int32));
        assert!(type_compatible(&a, &b));
    }

    #[test]
    fn type_compatible_different_array_inner_types_are_incompatible() {
        let a = Type::Array(Box::new(Type::Int32));
        let b = Type::Array(Box::new(Type::Int64));
        assert!(!type_compatible(&a, &b));
    }

    #[test]
    fn type_compatible_rejects_all_cross_primitive_combos() {
        // None of these pairs should be compatible with each other
        let primitives = vec![
            Type::Int8, Type::Int16, Type::Int32, Type::Int64, Type::Int128,
            Type::Byte, Type::Uint16, Type::Uint32, Type::Uint64, Type::Uint128,
            Type::Usize, Type::Float32, Type::Float64, Type::Bool, Type::String,
        ];
        for (i, a) in primitives.iter().enumerate() {
            for (j, b) in primitives.iter().enumerate() {
                if i == j {
                    assert!(type_compatible(a, b), "{:?} should be compatible with itself", a);
                } else {
                    assert!(!type_compatible(a, b),
                        "{:?} should NOT be compatible with {:?}", a, b);
                }
            }
        }
    }

    // Signed int is NOT compatible with its unsigned counterpart
    #[test]
    fn type_compatible_signed_vs_unsigned_same_width() {
        assert!(!type_compatible(&Type::Int8,   &Type::Byte));
        assert!(!type_compatible(&Type::Int16,  &Type::Uint16));
        assert!(!type_compatible(&Type::Int32,  &Type::Uint32));
        assert!(!type_compatible(&Type::Int64,  &Type::Uint64));
        assert!(!type_compatible(&Type::Int128, &Type::Uint128));
    }

    // resolve_binary_op_types_numeric
    #[test]
    fn binop_same_signed_types_resolve_correctly() {
        assert_eq!(resolve_binary_op_types_numeric(&Type::Int8,   &Type::Int8,   &dummy_span()).unwrap(), Type::Int8);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Int16,  &Type::Int16,  &dummy_span()).unwrap(), Type::Int16);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Int32,  &Type::Int32,  &dummy_span()).unwrap(), Type::Int32);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Int64,  &Type::Int64,  &dummy_span()).unwrap(), Type::Int64);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Int128, &Type::Int128, &dummy_span()).unwrap(), Type::Int128);
    }

    #[test]
    fn binop_same_unsigned_types_resolve_correctly() {
        assert_eq!(resolve_binary_op_types_numeric(&Type::Byte,    &Type::Byte,    &dummy_span()).unwrap(), Type::Byte);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Uint16,  &Type::Uint16,  &dummy_span()).unwrap(), Type::Uint16);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Uint32,  &Type::Uint32,  &dummy_span()).unwrap(), Type::Uint32);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Uint64,  &Type::Uint64,  &dummy_span()).unwrap(), Type::Uint64);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Uint128, &Type::Uint128, &dummy_span()).unwrap(), Type::Uint128);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Usize,   &Type::Usize,   &dummy_span()).unwrap(), Type::Usize);
    }

    #[test]
    fn binop_same_float_types_resolve_correctly() {
        assert_eq!(resolve_binary_op_types_numeric(&Type::Float32, &Type::Float32, &dummy_span()).unwrap(), Type::Float32);
        assert_eq!(resolve_binary_op_types_numeric(&Type::Float64, &Type::Float64, &dummy_span()).unwrap(), Type::Float64);
    }

    // Mixing signed and unsigned must always be an error
    #[test]
    fn binop_signed_unsigned_mix_is_always_error() {
        let signed   = [Type::Int8, Type::Int16, Type::Int32, Type::Int64, Type::Int128];
        let unsigned = [Type::Byte, Type::Uint16, Type::Uint32, Type::Uint64, Type::Uint128, Type::Usize];

        for s in &signed {
            for u in &unsigned {
                assert!(
                    resolve_binary_op_types_numeric(s, u, &dummy_span()).is_err(),
                    "Mixing {:?} + {:?} should be an error", s, u
                );
                assert!(
                    resolve_binary_op_types_numeric(u, s, &dummy_span()).is_err(),
                    "Mixing {:?} + {:?} should be an error", u, s
                );
            }
        }
    }

    // Mixing int and float must always be an error
    #[test]
    fn binop_int_float_mix_is_always_error() {
        let integers = [Type::Int32, Type::Uint32];
        let floats   = [Type::Float32, Type::Float64];

        for i in &integers {
            for f in &floats {
                assert!(resolve_binary_op_types_numeric(i, f, &dummy_span()).is_err(),
                    "{:?} + {:?} should be an error", i, f);
                assert!(resolve_binary_op_types_numeric(f, i, &dummy_span()).is_err(),
                    "{:?} + {:?} should be an error", f, i);
            }
        }
    }

    // Different-width signed ints must be an error — no implicit widening allowed
    #[test]
    fn binop_different_width_signed_int_is_error() {
        assert!(resolve_binary_op_types_numeric(&Type::Int8,  &Type::Int32, &dummy_span()).is_err());
        assert!(resolve_binary_op_types_numeric(&Type::Int32, &Type::Int64, &dummy_span()).is_err());
        assert!(resolve_binary_op_types_numeric(&Type::Int64, &Type::Int128, &dummy_span()).is_err());
    }

    // Different-width float mix must be an error — no implicit widening allowed
    #[test]
    fn binop_float32_float64_mix_is_error() {
        assert!(resolve_binary_op_types_numeric(&Type::Float32, &Type::Float64, &dummy_span()).is_err());
        assert!(resolve_binary_op_types_numeric(&Type::Float64, &Type::Float32, &dummy_span()).is_err());
    }

    // Bool / String must not be accepted by numeric binop resolver
    #[test]
    fn binop_non_numeric_types_are_error() {
        assert!(resolve_binary_op_types_numeric(&Type::Bool,   &Type::Bool,   &dummy_span()).is_err());
        assert!(resolve_binary_op_types_numeric(&Type::String, &Type::String, &dummy_span()).is_err());
        assert!(resolve_binary_op_types_numeric(&Type::Bool,   &Type::Int32,  &dummy_span()).is_err());
    }

    // Infer on either side must panic — it's a compiler bug to call this before inference is done
    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn binop_infer_left_panics() {
        let _ = resolve_binary_op_types_numeric(&Type::Infer, &Type::Int32, &dummy_span());
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn binop_infer_right_panics() {
        let _ = resolve_binary_op_types_numeric(&Type::Int32, &Type::Infer, &dummy_span());
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn binop_infer_both_panics() {
        let _ = resolve_binary_op_types_numeric(&Type::Infer, &Type::Infer, &dummy_span());
    }

    // -------------------------------------------------------------------------
    // get_bigger_type_of_two_floatings
    // -------------------------------------------------------------------------

    #[test]
    fn bigger_float_float64_beats_float32() {
        assert_eq!(get_bigger_type_of_two_floatings(Type::Float64, Type::Float32), Type::Float64);
        assert_eq!(get_bigger_type_of_two_floatings(Type::Float32, Type::Float64), Type::Float64);
    }

    // When same type, result must still be that type (no garbage returned)
    #[test]
    fn bigger_float_same_type_returns_that_type() {
        assert_eq!(get_bigger_type_of_two_floatings(Type::Float32, Type::Float32), Type::Float32);
        assert_eq!(get_bigger_type_of_two_floatings(Type::Float64, Type::Float64), Type::Float64);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn bigger_float_panics_on_non_float_left() {
        get_bigger_type_of_two_floatings(Type::Int32, Type::Float32);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn bigger_float_panics_on_non_float_right() {
        get_bigger_type_of_two_floatings(Type::Float32, Type::Int32);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn bigger_float_panics_on_both_non_float() {
        get_bigger_type_of_two_floatings(Type::Int32, Type::Int64);
    }

    // -------------------------------------------------------------------------
    // get_bigger_type_of_two_integers
    // -------------------------------------------------------------------------

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

    // Usize scores 8 — same as Uint64. When scores tie, t_2 wins (falls through).
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
    #[should_panic(expected = "Compiler bug")]
    fn bigger_int_panics_on_non_integer_left() {
        get_bigger_type_of_two_integers(Type::Float32, Type::Int32);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn bigger_int_panics_on_non_integer_right() {
        get_bigger_type_of_two_integers(Type::Int32, Type::Float32);
    }

    #[test]
    #[should_panic(expected = "Compiler bug")]
    fn bigger_int_panics_on_both_non_integer() {
        get_bigger_type_of_two_integers(Type::Bool, Type::String);
    }
}
