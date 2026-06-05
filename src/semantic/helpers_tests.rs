use super::*;

use crate::tests_consts::{
    ALL_TYPES_NO_INTS_NO_ARR,
    ALL_INT_TYPES_NO_ARR

};

use crate::semantic::helpers::{
    get_bigger_type_of_two_integers
};


#[cfg(test)]
mod helpers_tests {
    use super::*;

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

    // Same type result must be that type (not garbage)
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

    // FOOTGUN: this function does not reject signed+unsigned mixing.
    // It will happily compare Int32 vs Uint64 and return one of them.
    #[test]
    fn bigger_int_does_not_reject_signed_unsigned_mix_footgun() {
        // Int32 scores 5, Uint64 scores 8 so Uint64 wins.
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
