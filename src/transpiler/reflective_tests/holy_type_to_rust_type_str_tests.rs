/// This is manual verification tests to ensure that
/// `holy_type_to_rust_type_str` results are correct, by manually checking expected
/// results
///
use super::*;


#[cfg(test)]
mod holy_type_to_rust_type_str_non_array_types_tests {
    use super::*;

    #[test]
    fn string() {
        let t = Type::String;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "String")
    }

    #[test]
    fn float64() {
        let t = Type::Float64;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "f64")
    }

    #[test]
    fn bool() {
        let t = Type::Bool;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "bool")
    }

    #[test]
    fn int8() {
        let t = Type::Int8;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "i8")
    }

    #[test]
    fn int16() {
        let t = Type::Int16;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "i16")
    }
 
    #[test]
    fn int32() {
        let t = Type::Int32;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "i32")
    }   

    #[test]
    fn int64() {
        let t = Type::Int64;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "i64")
    }

    #[test]
    fn int128() {
        let t = Type::Int128;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "i128")
    }


    #[test]
    fn byte() {
        let t = Type::Byte;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "u8")
    }

    #[test]
    fn uint16() {
        let t = Type::Uint16;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "u16")
    }

    #[test]
    fn uint32() {
        let t = Type::Uint32;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "u32")
    }

    #[test]
    fn uint64() {
        let t = Type::Uint64;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "u64")
    }

    #[test]
    fn uint128() {
        let t = Type::Uint128;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "u128")
    }

    #[test]
    fn usize() {
        let t = Type::Usize;
        let t_str = holy_type_to_rust_type_str(&t);

        assert_eq!(t_str, "usize")
    }
}


#[cfg(test)]
mod holy_type_to_rust_type_str_dyn_arr_types_tests {
    use super::*;

    #[test]
    fn array_of_string() {
        let plain_t = Type::String;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_float64() {
        let plain_t = Type::Float64;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_bool() {
        let plain_t = Type::Bool;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_int8() {
        let plain_t = Type::Int8;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_int16() {
        let plain_t = Type::Int16;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }
 
    #[test]
    fn array_of_int32() {
        let plain_t = Type::Int32;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_int64() {
        let plain_t = Type::Int64;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_int128() {
        let plain_t = Type::Int128;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }


    #[test]
    fn array_of_byte() {
        let plain_t = Type::Byte;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_uint16() {
        let plain_t = Type::Uint16;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_uint32() {
        let plain_t = Type::Uint32;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_uint64() {
        let plain_t = Type::Uint64;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_uint128() {
        let plain_t = Type::Uint128;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }

    #[test]
    fn array_of_usize() {
        let plain_t = Type::Usize;
        let plain_t_str = holy_type_to_rust_type_str(&plain_t);

        let mut vec_str = plain_t_str;
        let mut arr_t = plain_t;

        for _ in 1..=1000 {
            arr_t = Type::Array(Box::new(arr_t));
            vec_str = format!("Vec<{}>", vec_str);

            let arr_t_str = holy_type_to_rust_type_str(&arr_t);

            assert_eq!(arr_t_str, vec_str)
        }
    }
}
