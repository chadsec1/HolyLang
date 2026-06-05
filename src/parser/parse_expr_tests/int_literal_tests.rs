use super::*;


#[cfg(test)]
mod int_literals_tests {
    use super::*;


    // Tests integer literals boundaries from MIN to MAX, with (and without) whitespaces on
    // left/right sides.
    //
    
    #[test]
    fn test_int8_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            assert_int_literal(&format!("{}{}", spaces, i8::MIN.to_string()), IntLiteralValue::Int8(i8::MIN));
            assert_int_literal(&format!("{}{}", spaces, i8::MAX.to_string()), IntLiteralValue::Int8(i8::MAX));

            assert_int_literal(&format!("{}{}", i8::MIN.to_string(), spaces), IntLiteralValue::Int8(i8::MIN));
            assert_int_literal(&format!("{}{}", i8::MAX.to_string(), spaces), IntLiteralValue::Int8(i8::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, i8::MIN.to_string(), spaces), IntLiteralValue::Int8(i8::MIN));
            assert_int_literal(&format!("{}{}{}", spaces, i8::MAX.to_string(), spaces), IntLiteralValue::Int8(i8::MAX));
           
            spaces.push(' ');
        }   
    }

    #[test]
    fn test_int16_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            assert_int_literal(&format!("{}{}", spaces, i16::MIN.to_string()), IntLiteralValue::Int16(i16::MIN));
            assert_int_literal(&format!("{}{}", spaces, i16::MAX.to_string()), IntLiteralValue::Int16(i16::MAX));

            assert_int_literal(&format!("{}{}", i16::MIN.to_string(), spaces), IntLiteralValue::Int16(i16::MIN));
            assert_int_literal(&format!("{}{}", i16::MAX.to_string(), spaces), IntLiteralValue::Int16(i16::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, i16::MIN.to_string(), spaces), IntLiteralValue::Int16(i16::MIN));
            assert_int_literal(&format!("{}{}{}", spaces, i16::MAX.to_string(), spaces), IntLiteralValue::Int16(i16::MAX));
           
            spaces.push(' ');
        } 
    }

    #[test]
    fn test_int32_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            assert_int_literal(&format!("{}{}", spaces, i32::MIN.to_string()), IntLiteralValue::Int32(i32::MIN));
            assert_int_literal(&format!("{}{}", spaces, i32::MAX.to_string()), IntLiteralValue::Int32(i32::MAX));

            assert_int_literal(&format!("{}{}", i32::MIN.to_string(), spaces), IntLiteralValue::Int32(i32::MIN));
            assert_int_literal(&format!("{}{}", i32::MAX.to_string(), spaces), IntLiteralValue::Int32(i32::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, i32::MIN.to_string(), spaces), IntLiteralValue::Int32(i32::MIN));
            assert_int_literal(&format!("{}{}{}", spaces, i32::MAX.to_string(), spaces), IntLiteralValue::Int32(i32::MAX));
           
            spaces.push(' ');
        }
    }

    #[test]
    fn test_int64_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            assert_int_literal(&format!("{}{}", spaces, i64::MIN.to_string()), IntLiteralValue::Int64(i64::MIN));
            assert_int_literal(&format!("{}{}", spaces, i64::MAX.to_string()), IntLiteralValue::Int64(i64::MAX));

            assert_int_literal(&format!("{}{}", i64::MIN.to_string(), spaces), IntLiteralValue::Int64(i64::MIN));
            assert_int_literal(&format!("{}{}", i64::MAX.to_string(), spaces), IntLiteralValue::Int64(i64::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, i64::MIN.to_string(), spaces), IntLiteralValue::Int64(i64::MIN));
            assert_int_literal(&format!("{}{}{}", spaces, i64::MAX.to_string(), spaces), IntLiteralValue::Int64(i64::MAX));
           
            spaces.push(' ');
        }
    }

    #[test]
    fn test_int128_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            assert_int_literal(&format!("{}{}", spaces, i128::MIN.to_string()), IntLiteralValue::Int128(i128::MIN));
            assert_int_literal(&format!("{}{}", spaces, i128::MAX.to_string()), IntLiteralValue::Int128(i128::MAX));

            assert_int_literal(&format!("{}{}", i128::MIN.to_string(), spaces), IntLiteralValue::Int128(i128::MIN));
            assert_int_literal(&format!("{}{}", i128::MAX.to_string(), spaces), IntLiteralValue::Int128(i128::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, i128::MIN.to_string(), spaces), IntLiteralValue::Int128(i128::MIN));
            assert_int_literal(&format!("{}{}{}", spaces, i128::MAX.to_string(), spaces), IntLiteralValue::Int128(i128::MAX));
           
            spaces.push(' ');
        }
    }

    #[test]
    fn test_uint128_boundary() {
        let mut spaces = String::with_capacity(5000);
        for _ in 1..=5000 {
            // This is because 0 can fit into int8
            assert_int_literal(&format!("{}{}", spaces, u128::MIN.to_string()), IntLiteralValue::Int8(u128::MIN as i8));
            assert_int_literal(&format!("{}{}", spaces, u128::MAX.to_string()), IntLiteralValue::Uint128(u128::MAX));

            assert_int_literal(&format!("{}{}", u128::MIN.to_string(), spaces), IntLiteralValue::Int8(u128::MIN as i8));
            assert_int_literal(&format!("{}{}", u128::MAX.to_string(), spaces), IntLiteralValue::Uint128(u128::MAX));

            assert_int_literal(&format!("{}{}{}", spaces, u128::MIN.to_string(), spaces), IntLiteralValue::Int8(u128::MIN as i8));
            assert_int_literal(&format!("{}{}{}", spaces, u128::MAX.to_string(), spaces), IntLiteralValue::Uint128(u128::MAX));
           
            spaces.push(' ');
        }
    }
}
