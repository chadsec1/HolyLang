use super::*;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_TYPES_NO_ARR_NO_FLOAT
}; 

// Test helper function
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
mod types_tests {
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

            for i in 1usize..100usize {
                assert!(!Type::FixedArray(Box::new(ty.clone()), FixedArraySize::Literal(i)).is_integer_type());
            }
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

            assert!(!Type::Array(Box::new(t.clone())).is_floating_type());

            for i in 1usize..100usize {
                assert!(!Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)).is_floating_type());
            }
        }
    }

    // Tests against arrays and nested arrays, both dynamic and fixed
    // to see if is_array_type passes or not. it must pass.
    //
    #[test]
    fn is_array_type_pass() {
        for t in ALL_TYPES_NO_ARR {
            assert!(!t.is_array_type());

            let mut dyn_arr_t = Type::Array(Box::new(t.clone()));
            let mut fixed_arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for i in 2usize..=200 {
                assert!(dyn_arr_t.is_array_type());
                assert!(fixed_arr_t.is_array_type());

                dyn_arr_t = Type::Array(Box::new(dyn_arr_t));
                fixed_arr_t = Type::FixedArray(Box::new(fixed_arr_t), FixedArraySize::Literal(i));
            }
        }
    }

    // Tests against dynamic arrays, and nested dynmaic arrays
    // and fixed arrays and nested fixed arrays
    // and mixed arrays of both dynamic and fixed, and nested mixed arrays
    #[test]
    fn is_fully_fixed_array_type_pass() {
        for t in ALL_TYPES_NO_ARR {
            let mut dyn_arr_t = Type::Array(Box::new(t.clone()));
            let mut fixed_arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));
            
            let mut mixed_arr_1_t = Type::FixedArray(Box::new(dyn_arr_t.clone()), FixedArraySize::Literal(1));
            let mut mixed_arr_2_t = Type::Array(Box::new(fixed_arr_t.clone()));

            let mut switch = false;

            for i in 2usize..=600 {
                assert!(!dyn_arr_t.is_fully_fixed_array_type());
                assert!(!mixed_arr_1_t.is_fully_fixed_array_type());
                assert!(!mixed_arr_2_t.is_fully_fixed_array_type());
                assert!(fixed_arr_t.is_fully_fixed_array_type());

                dyn_arr_t = Type::Array(Box::new(dyn_arr_t));
                fixed_arr_t = Type::FixedArray(Box::new(fixed_arr_t), FixedArraySize::Literal(i));
                if switch {
                    mixed_arr_1_t = Type::Array(Box::new(mixed_arr_1_t));
                    mixed_arr_2_t = Type::FixedArray(Box::new(mixed_arr_2_t), FixedArraySize::Literal(i));
                } else {
                    mixed_arr_1_t = Type::FixedArray(Box::new(mixed_arr_1_t), FixedArraySize::Literal(i));
                    mixed_arr_2_t = Type::Array(Box::new(mixed_arr_2_t));
                }

                switch = !switch;
            }
        }
    }

    // We dont use should_panic here because we test multiple types
    //
    #[test]
    fn is_fully_fixed_array_type_on_non_array_panics() {
        for t in ALL_TYPES_NO_ARR {
            let result = std::panic::catch_unwind(|| { 
                t.is_fully_fixed_array_type()
            });

            assert!(result.is_err(), "Expected panic for: {:?}", t);
        }
    }


    #[test]
    fn fixed_array_to_dynamic_array_type_full_pass() {
        for t in ALL_TYPES_NO_ARR {
            let mut dyn_arr_t = Type::Array(Box::new(t.clone()));
            let mut fixed_arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));
            
            let mut mixed_arr_1_t = Type::FixedArray(Box::new(dyn_arr_t.clone()), FixedArraySize::Literal(1));
            let mut mixed_arr_2_t = Type::Array(Box::new(fixed_arr_t.clone()));

            let mut switch = false;

            for i in 2usize..=600 {
                // Cuz `dyn_arr_t` array is already fully dynamic
                assert_eq!(dyn_arr_t.fixed_array_to_dynamic_array_type_full(), dyn_arr_t);

                // Should be exactly equal to `dyn_arr_t` once it is converted to dynamic array.
                assert_eq!(fixed_arr_t.fixed_array_to_dynamic_array_type_full(), dyn_arr_t);
                

                dyn_arr_t = Type::Array(Box::new(dyn_arr_t));
                fixed_arr_t = Type::FixedArray(Box::new(fixed_arr_t), FixedArraySize::Literal(i));

                assert_eq!(mixed_arr_1_t.fixed_array_to_dynamic_array_type_full(), dyn_arr_t);
                assert_eq!(mixed_arr_2_t.fixed_array_to_dynamic_array_type_full(), dyn_arr_t);

                if switch {
                    mixed_arr_1_t = Type::Array(Box::new(mixed_arr_1_t));
                    mixed_arr_2_t = Type::FixedArray(Box::new(mixed_arr_2_t), FixedArraySize::Literal(i));
                } else {
                    mixed_arr_1_t = Type::FixedArray(Box::new(mixed_arr_1_t), FixedArraySize::Literal(i));
                    mixed_arr_2_t = Type::Array(Box::new(mixed_arr_2_t));
                }

                switch = !switch;
            }
        }
    }



    // Check if type is numeric, and test is_numeric_type on it.
    // if type is not numeric, test !is_numeric_type on it.
    //
    // We also test array types dynamic and fixed, against !is_numeric_type
    //
    #[test]
    fn get_type_and_is_numeric_are_consistent() {
        let numeric_types = [
            Type::Int8, Type::Int16, Type::Int32, Type::Int64, Type::Int128,
            Type::Byte, Type::Uint16, Type::Uint32, Type::Uint64, Type::Uint128, Type::Usize,

            Type::Float64
        ];

        for t in ALL_TYPES_NO_ARR {
            if numeric_types.contains(t) {
                assert!(t.is_numeric_type());
            } else {
                assert!(!t.is_numeric_type());
            }

            assert!(!Type::Array(Box::new(t.clone())).is_numeric_type());

            for i in 1usize..100usize {
                assert!(!Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(i)).is_numeric_type());
            }
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

            let dyn_arr_ty = Type::Array(Box::new(ty.clone()));

            assert!(
                !(dyn_arr_ty.is_integer_type() && dyn_arr_ty.is_floating_type()),
                "Array of type {:?} is claiming to be both integer AND float, which is impossible", ty
            );
            
            for i in 1usize..100usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(ty.clone()), FixedArraySize::Literal(i));

                assert!(
                    !(fixed_arr_ty.is_integer_type() && fixed_arr_ty.is_floating_type()),
                    "Array of type {:?} is claiming to be both integer AND float, which is impossible", ty
                );
            }
        }
    }


    // No type can be both an integer and an array. Ever.
    //
    #[test]
    fn no_type_is_both_integer_and_array() {
        for ty in ALL_TYPES_NO_ARR {
            assert!(
                !(ty.is_integer_type() && ty.is_array_type()),
                "Type {:?} is claiming to be both integer AND array, which is impossible", ty
            );

            let dyn_arr_ty = Type::Array(Box::new(ty.clone()));

            assert!(
                !(dyn_arr_ty.is_integer_type() && dyn_arr_ty.is_array_type()),
                "Dynamic array of Type {:?} is claiming to be both integer AND array, which is impossible", ty
            );
            
            for i in 1usize..100usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(ty.clone()), FixedArraySize::Literal(i));

                assert!(
                    !(fixed_arr_ty.is_integer_type() && fixed_arr_ty.is_array_type()),
                    "Fixed array of Type {:?} is claiming to be both integer AND array, which is impossible", ty
                );
            }
        }
    }


    // No type can be both a float and an array. Ever.
    //
    #[test]
    fn no_type_is_both_float_and_array() {
        for ty in ALL_TYPES_NO_ARR {
            assert!(
                !(ty.is_floating_type() && ty.is_array_type()),
                "Type {:?} is claiming to be both floating AND array, which is impossible", ty
            );

            let dyn_arr_ty = Type::Array(Box::new(ty.clone()));

            assert!(
                !(dyn_arr_ty.is_floating_type() && dyn_arr_ty.is_array_type()),
                "Dynamic array of Type {:?} is claiming to be both floating AND array, which is impossible", ty
            );
            
            for i in 1usize..100usize {
                let fixed_arr_ty = Type::FixedArray(Box::new(ty.clone()), FixedArraySize::Literal(i));

                assert!(
                    !(fixed_arr_ty.is_floating_type() && fixed_arr_ty.is_array_type()),
                    "Fixed array of Type {:?} is claiming to be both floating AND array, which is impossible", ty
                );
            }
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
