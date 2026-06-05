use super::*;

#[cfg(test)]
mod fmt_display_tests {
    use super::*;

    #[test]
    fn type_display_no_arr() {
        assert_eq!(Type::Int8.to_string(), "int8");
        assert_eq!(Type::Int16.to_string(), "int16");
        assert_eq!(Type::Int32.to_string(), "int32");
        assert_eq!(Type::Int64.to_string(), "int64");
        assert_eq!(Type::Int128.to_string(), "int128");

        assert_eq!(Type::Byte.to_string(), "byte");
        assert_eq!(Type::Uint16.to_string(), "uint16");
        assert_eq!(Type::Uint32.to_string(), "uint32");
        assert_eq!(Type::Uint64.to_string(), "uint64");
        assert_eq!(Type::Uint128.to_string(), "uint128");
        
        assert_eq!(Type::Usize.to_string(), "usize");

        assert_eq!(Type::Float64.to_string(), "float64");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::String.to_string(), "string");
    }

    #[test]
    fn int_literal_value_display() {
        assert_eq!(IntLiteralValue::Int8(i8::MAX).to_string(), i8::MAX.to_string());
        assert_eq!(IntLiteralValue::Int16(i16::MAX).to_string(), i16::MAX.to_string());
        assert_eq!(IntLiteralValue::Int32(i32::MAX).to_string(), i32::MAX.to_string());
        assert_eq!(IntLiteralValue::Int64(i64::MAX).to_string(), i64::MAX.to_string());
        assert_eq!(IntLiteralValue::Int128(i128::MAX).to_string(), i128::MAX.to_string());

        assert_eq!(IntLiteralValue::Byte(u8::MAX).to_string(), u8::MAX.to_string());
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).to_string(), u16::MAX.to_string());
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).to_string(), u32::MAX.to_string());
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).to_string(), u64::MAX.to_string());
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).to_string(), u128::MAX.to_string());
        assert_eq!(IntLiteralValue::Usize(usize::MAX).to_string(), usize::MAX.to_string());

    }
    
    #[test]
    fn type_display_dynamic_arrays() {
        assert_eq!(Type::Array(Box::new(Type::Int8)).to_string(), "[]int8");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int8)))).to_string(), "[][]int8");

        assert_eq!(Type::Array(Box::new(Type::Int16)).to_string(), "[]int16");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int16)))).to_string(), "[][]int16");

        assert_eq!(Type::Array(Box::new(Type::Int32)).to_string(), "[]int32");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int32)))).to_string(), "[][]int32");

        assert_eq!(Type::Array(Box::new(Type::Int64)).to_string(), "[]int64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int64)))).to_string(), "[][]int64");

        assert_eq!(Type::Array(Box::new(Type::Int128)).to_string(), "[]int128");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Int128)))).to_string(), "[][]int128");


        assert_eq!(Type::Array(Box::new(Type::Byte)).to_string(), "[]byte");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Byte)))).to_string(), "[][]byte");

        assert_eq!(Type::Array(Box::new(Type::Uint16)).to_string(), "[]uint16");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint16)))).to_string(), "[][]uint16");

        assert_eq!(Type::Array(Box::new(Type::Uint32)).to_string(), "[]uint32");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint32)))).to_string(), "[][]uint32");

        assert_eq!(Type::Array(Box::new(Type::Uint64)).to_string(), "[]uint64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint64)))).to_string(), "[][]uint64");

        assert_eq!(Type::Array(Box::new(Type::Uint128)).to_string(), "[]uint128");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Uint128)))).to_string(), "[][]uint128");

        assert_eq!(Type::Array(Box::new(Type::Usize)).to_string(), "[]usize");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Usize)))).to_string(), "[][]usize");


        assert_eq!(Type::Array(Box::new(Type::Float64)).to_string(), "[]float64");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Float64)))).to_string(), "[][]float64");


        assert_eq!(Type::Array(Box::new(Type::String)).to_string(), "[]string");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::String)))).to_string(), "[][]string");

        assert_eq!(Type::Array(Box::new(Type::Bool)).to_string(), "[]bool");
        assert_eq!(Type::Array(Box::new(Type::Array(Box::new(Type::Bool)))).to_string(), "[][]bool");
    }


    #[test]
    fn type_display_fixed_arrays() {
        for i in 1usize..=1000usize {
            assert_eq!(Type::FixedArray(Box::new(Type::Int8), FixedArraySize::Literal(i)).to_string(), format!("[{}]int8", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Int8), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]int8", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Int16), FixedArraySize::Literal(i)).to_string(), format!("[{}]int16", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Int16), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]int16", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Int32), FixedArraySize::Literal(i)).to_string(), format!("[{}]int32", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Int32), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]int32", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Int64), FixedArraySize::Literal(i)).to_string(), format!("[{}]int64", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Int64), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]int64", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Int128), FixedArraySize::Literal(i)).to_string(), format!("[{}]int128", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Int128), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]int128", i, i + 1));


            assert_eq!(Type::FixedArray(Box::new(Type::Byte), FixedArraySize::Literal(i)).to_string(), format!("[{}]byte", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Byte), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]byte", i, i + 1));


            assert_eq!(Type::FixedArray(Box::new(Type::Uint16), FixedArraySize::Literal(i)).to_string(), format!("[{}]uint16", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Uint16), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]uint16", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Uint32), FixedArraySize::Literal(i)).to_string(), format!("[{}]uint32", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Uint32), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]uint32", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Uint64), FixedArraySize::Literal(i)).to_string(), format!("[{}]uint64", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Uint64), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(),  format!("[{}][{}]uint64", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Uint128), FixedArraySize::Literal(i)).to_string(), format!("[{}]uint128", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Uint128), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]uint128", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Usize), FixedArraySize::Literal(i)).to_string(), format!("[{}]usize", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Usize), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]usize", i, i + 1));


            assert_eq!(Type::FixedArray(Box::new(Type::Float64), FixedArraySize::Literal(i)).to_string(), format!("[{}]float64", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Float64), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]float64", i, i + 1));


            assert_eq!(Type::FixedArray(Box::new(Type::String), FixedArraySize::Literal(i)).to_string(), format!("[{}]string", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::String), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]string", i, i + 1));

            assert_eq!(Type::FixedArray(Box::new(Type::Bool), FixedArraySize::Literal(i)).to_string(), format!("[{}]bool", i));
            assert_eq!(Type::FixedArray(Box::new(Type::FixedArray(Box::new(Type::Bool), FixedArraySize::Literal(i + 1))), FixedArraySize::Literal(i)).to_string(), format!("[{}][{}]bool", i, i + 1));

        }
    }

    #[test]
    fn display_fixed_array_size() {
        for i in 0usize..=10000usize {
            assert_eq!(FixedArraySize::Literal(i).to_string(), i.to_string());
            assert_eq!(FixedArraySize::Const(i.to_string()).to_string(), i.to_string());
        }
    }

}
