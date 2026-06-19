use super::*;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR,
    ALL_TYPES_NO_ARR_NO_FLOAT
}; 


// Helper function
fn span() -> Span {
    Span { line: 1, column: 0 }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    // Type::get_default_value

    #[test]
    fn default_value_all_valid_variants() {
        assert_eq!(Type::Int8.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Int8(0), span: span() });
        assert_eq!(Type::Int16.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Int16(0), span: span() });
        assert_eq!(Type::Int32.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Int32(0), span: span() });
        assert_eq!(Type::Int64.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Int64(0), span: span() });
        assert_eq!(Type::Int128.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Int128(0), span: span() });

        assert_eq!(Type::Byte.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Byte(0), span: span() });
        assert_eq!(Type::Uint16.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Uint16(0), span: span() });
        assert_eq!(Type::Uint32.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Uint32(0), span: span() });
        assert_eq!(Type::Uint64.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Uint64(0), span: span() });
        assert_eq!(Type::Uint128.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Uint128(0), span: span() });
        assert_eq!(Type::Usize.get_default_value(span()).unwrap(), Expr::IntLiteral { value: IntLiteralValue::Usize(0), span: span() });
        
        assert_eq!(Type::Float64.get_default_value(span()).unwrap(), Expr::Float64Literal { value: 0.0, span: span() });
        assert_eq!(Type::String.get_default_value(span()).unwrap(), Expr::StringLiteral { value: "".to_string(), span: span() });
        assert_eq!(Type::Bool.get_default_value(span()).unwrap(), Expr::BoolLiteral { value: false, span: span() });


        assert!(Type::Char.get_default_value(span()).is_err());

        for t in ALL_TYPES_NO_ARR {
            let mut arr = Type::Array(Box::new(t.clone()));

            for _ in 1..=500 {
                assert_eq!(arr.get_default_value(span()).unwrap(), Expr::ArrayLiteral { elements: vec![], type_name: None, span: span() });
                arr = Type::Array(Box::new(arr));
            }
        }

        for t in ALL_TYPES_NO_ARR {
            let mut arr = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));

            for _ in 1..=500 {
                assert!(arr.get_default_value(span()).is_err());

                arr = Type::FixedArray(Box::new(arr), FixedArraySize::Literal(1));
            }
        }



    }



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

    // We dont use should_panic here because we test multiple types
    //
    #[test]
    fn fixed_array_to_dynamic_array_type_full() {
        for t in ALL_TYPES_NO_ARR {
            let result = std::panic::catch_unwind(|| { 
                let _ = t.fixed_array_to_dynamic_array_type_full();
            });

            assert!(result.is_err(), "Expected panic for: {:?}", t);
        }
    }


    #[test]
    fn get_array_inner_most_type_passes() {
        for t in ALL_TYPES_NO_ARR {
            let mut dyn_arr_t = Type::Array(Box::new(t.clone()));
            let mut fixed_arr_t = Type::FixedArray(Box::new(t.clone()), FixedArraySize::Literal(1));
            
            let mut mixed_arr_1_t = Type::FixedArray(Box::new(dyn_arr_t.clone()), FixedArraySize::Literal(1));
            let mut mixed_arr_2_t = Type::Array(Box::new(fixed_arr_t.clone()));

            let mut switch = false;

            for i in 2usize..=600 {
                assert_eq!(dyn_arr_t.get_array_inner_most_type(), t);
                assert_eq!(fixed_arr_t.get_array_inner_most_type(), t);
                assert_eq!(mixed_arr_1_t.get_array_inner_most_type(), t);
                assert_eq!(mixed_arr_2_t.get_array_inner_most_type(), t);

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
    fn get_array_inner_most_type_on_non_array_panics() {
        for t in ALL_TYPES_NO_ARR {
            let result = std::panic::catch_unwind(|| { 
                t.get_array_inner_most_type()
            });

            assert!(result.is_err(), "Expected panic for: {:?}", t);
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
}
