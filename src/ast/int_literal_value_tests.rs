use super::*;

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
mod int_literal_value_tests {
    use super::*;

    // IntLiteralValue::get_type: every variant maps to exactly its own Type

    #[test]
    fn int_literal_get_type_all_variants() {
        assert_eq!(IntLiteralValue::Int8(i8::MIN).get_type(),    Type::Int8);
        assert_eq!(IntLiteralValue::Int8(i8::MAX).get_type(),    Type::Int8);
        assert_eq!(IntLiteralValue::Int16(i16::MIN).get_type(),   Type::Int16);
        assert_eq!(IntLiteralValue::Int16(i16::MAX).get_type(),   Type::Int16);
        assert_eq!(IntLiteralValue::Int32(i32::MIN).get_type(),   Type::Int32);
        assert_eq!(IntLiteralValue::Int32(i32::MAX).get_type(),   Type::Int32);
        assert_eq!(IntLiteralValue::Int64(i64::MIN).get_type(),   Type::Int64);
        assert_eq!(IntLiteralValue::Int64(i64::MIN).get_type(),   Type::Int64);
        assert_eq!(IntLiteralValue::Int128(i128::MIN).get_type(),  Type::Int128);
        assert_eq!(IntLiteralValue::Int128(i128::MAX).get_type(),  Type::Int128);

        assert_eq!(IntLiteralValue::Byte(u8::MIN).get_type(),    Type::Byte);
        assert_eq!(IntLiteralValue::Byte(u8::MAX).get_type(),    Type::Byte);
        assert_eq!(IntLiteralValue::Uint16(u16::MIN).get_type(),  Type::Uint16);
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).get_type(),  Type::Uint16);
        assert_eq!(IntLiteralValue::Uint32(u32::MIN).get_type(),  Type::Uint32);
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).get_type(),  Type::Uint32);
        assert_eq!(IntLiteralValue::Uint64(u64::MIN).get_type(),  Type::Uint64);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).get_type(),  Type::Uint64);
        assert_eq!(IntLiteralValue::Uint128(u128::MIN).get_type(), Type::Uint128);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).get_type(), Type::Uint128);
        assert_eq!(IntLiteralValue::Usize(usize::MIN).get_type(),   Type::Usize);
        assert_eq!(IntLiteralValue::Usize(usize::MAX).get_type(),   Type::Usize);
    }


    // IntLiteralValue::bit_width
    #[test]
    fn int_literal_get_bit_width_all_variants() {
        assert_eq!(IntLiteralValue::Int8(i8::MIN).bit_width(), i8::BITS);
        assert_eq!(IntLiteralValue::Int8(i8::MAX).bit_width(), i8::BITS);

        assert_eq!(IntLiteralValue::Int16(i16::MIN).bit_width(), i16::BITS);
        assert_eq!(IntLiteralValue::Int16(i16::MAX).bit_width(), i16::BITS);

        assert_eq!(IntLiteralValue::Int32(i32::MIN).bit_width(), i32::BITS);
        assert_eq!(IntLiteralValue::Int32(i32::MAX).bit_width(), i32::BITS);

        assert_eq!(IntLiteralValue::Int64(i64::MIN).bit_width(), i64::BITS);
        assert_eq!(IntLiteralValue::Int64(i64::MAX).bit_width(), i64::BITS);

        assert_eq!(IntLiteralValue::Int128(i128::MIN).bit_width(), i128::BITS);
        assert_eq!(IntLiteralValue::Int128(i128::MAX).bit_width(), i128::BITS);

        assert_eq!(IntLiteralValue::Byte(u8::MIN).bit_width(), u8::BITS);
        assert_eq!(IntLiteralValue::Byte(u8::MAX).bit_width(), u8::BITS);

        assert_eq!(IntLiteralValue::Uint16(u16::MIN).bit_width(), u16::BITS);
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).bit_width(), u16::BITS);

        assert_eq!(IntLiteralValue::Uint32(u32::MIN).bit_width(), u32::BITS);
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).bit_width(), u32::BITS);

        assert_eq!(IntLiteralValue::Uint64(u64::MIN).bit_width(), u64::BITS);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).bit_width(), u64::BITS);

        assert_eq!(IntLiteralValue::Uint128(u128::MIN).bit_width(), u128::BITS);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).bit_width(), u128::BITS);

        assert_eq!(IntLiteralValue::Usize(usize::MIN).bit_width(), usize::BITS);
        assert_eq!(IntLiteralValue::Usize(usize::MAX).bit_width(), usize::BITS);
    }


    // IntLiteralValue::is_signed

    #[test]
    fn is_signed_true_for_all_signed_variants() {
        assert!(IntLiteralValue::Int8(i8::MIN).is_signed());
        assert!(IntLiteralValue::Int8(i8::MAX).is_signed());

        assert!(IntLiteralValue::Int16(i16::MIN).is_signed());
        assert!(IntLiteralValue::Int16(i16::MAX).is_signed());

        assert!(IntLiteralValue::Int32(i32::MIN).is_signed());
        assert!(IntLiteralValue::Int32(i32::MAX).is_signed());

        assert!(IntLiteralValue::Int64(i64::MIN).is_signed());
        assert!(IntLiteralValue::Int64(i64::MAX).is_signed());

        assert!(IntLiteralValue::Int128(i128::MIN).is_signed());
        assert!(IntLiteralValue::Int128(i128::MAX).is_signed());
    }

    #[test]
    fn is_signed_false_for_all_unsigned_variants() {
        assert!(!IntLiteralValue::Byte(u8::MIN).is_signed());
        assert!(!IntLiteralValue::Byte(u8::MAX).is_signed());

        assert!(!IntLiteralValue::Uint16(u16::MIN).is_signed());
        assert!(!IntLiteralValue::Uint16(u16::MAX).is_signed());

        assert!(!IntLiteralValue::Uint32(u32::MIN).is_signed());
        assert!(!IntLiteralValue::Uint32(u32::MAX).is_signed());

        assert!(!IntLiteralValue::Uint64(u64::MIN).is_signed());
        assert!(!IntLiteralValue::Uint64(u64::MAX).is_signed());

        assert!(!IntLiteralValue::Uint128(u128::MIN).is_signed());
        assert!(!IntLiteralValue::Uint128(u128::MAX).is_signed());

        assert!(!IntLiteralValue::Usize(usize::MIN).is_signed());
        assert!(!IntLiteralValue::Usize(usize::MAX).is_signed());
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

