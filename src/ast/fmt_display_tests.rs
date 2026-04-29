use super::*;

#[cfg(test)]
mod fmt_display_tests {
    use super::*;

    #[test]
    fn type_display() {
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
}
