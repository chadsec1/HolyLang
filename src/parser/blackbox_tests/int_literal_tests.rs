use super::*;

#[cfg(test)]
mod blackbox_int_literals_tests {
    use super::*;

    #[test]
    fn integer_literal_fits_int8() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 1", t));
                
            assert_eq!(stmts.len(), 1);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int8(1)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int8_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 127 fits int8, 128 does not
            let stmts = parse_body(&format!("own a {} = 127\nown b {} = 128", t, t));
            
            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int8(127)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
            
            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(128)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int16() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 128", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(128)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int16_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 32767 fits int16, 32768 does not
            let stmts = parse_body(&format!("own a {} = 32767\nown b {} = 32768", t, t));

            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int16(32767)));
                } else { panic!("Expected IntLiteral"); }

            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(32768)));
                } else { panic!("Expected IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int32() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 32768", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(32768)));
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int32_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 2147483647 fits int32, 2147483648 does not
            let stmts = parse_body(&format!("own a {} = 2147483647\nown b {} = 2147483648", t, t));

            assert_eq!(stmts.len(), 2);

            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int32(2147483647)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
            
            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(2147483648)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn integer_literal_fits_int64() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 2147483648", t));
            
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(2147483648)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_int64_boundary() {
        for t in ALL_TYPES_NO_ARR {
            // 9223372036854775807 fits int64, 9223372036854775808 does not
            let stmts = parse_body(&format!("own a {} = 9223372036854775807\nown b {} = 9223372036854775808", t, t));
            
            assert_eq!(stmts.len(), 2);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int64(9223372036854775807)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(9223372036854775808)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_int128() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 9223372036854775808", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(9223372036854775808)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        } 
    }

    #[test]
    fn integer_literal_int128_boundary() {
        // 170141183460469231731687303715884105727 fits int128,  170141183460469231731687303715884105728 does not
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own a {} = 170141183460469231731687303715884105727\nown b {} = 170141183460469231731687303715884105728", t, t));
            
            assert_eq!(stmts.len(), 2);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Int128(170141183460469231731687303715884105727)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }

            if let Stmt::VarDecl(v) = &stmts[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Uint128(170141183460469231731687303715884105728)));
                } else { panic!("Expcted IntLiteral"); }
            } else { panic!("Expected VarDecl"); }
        }
    }

    #[test]
    fn integer_literal_fits_uint128() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = 340282366920938463463374607431768211455", t));
            assert_eq!(stmts.len(), 1);
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                if let Some(Expr::IntLiteral { value, .. }) = &v.value {
                    assert!(matches!(value, IntLiteralValue::Uint128(340282366920938463463374607431768211455)));
                } else { panic!(); }
            } else { panic!("Expected VarDecl"); }
        }
    }



    #[test]
    fn integer_literal_negative() {
        for t in ALL_TYPES_NO_ARR {
            let stmts = parse_body(&format!("own x {} = -128", t));
            assert_eq!(stmts.len(), 1);
            
            if let Stmt::VarDecl(v) = &stmts[0] {
                assert_eq!(v.name, "x");
                assert_eq!(v.type_name, t.clone());

                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(-128), .. })))
            } else { panic!("Expected VarDecl"); }
        }
    }


    #[test]
    fn int_literal_get_type() {
        assert_eq!(IntLiteralValue::Int8(1).get_type(), Type::Int8);
        assert_eq!(IntLiteralValue::Int32(1).get_type(), Type::Int32);
        assert_eq!(IntLiteralValue::Int64(1).get_type(), Type::Int64);
        assert_eq!(IntLiteralValue::Int128(1).get_type(), Type::Int128);
        assert_eq!(IntLiteralValue::Byte(1).get_type(), Type::Byte);
        assert_eq!(IntLiteralValue::Uint16(1).get_type(), Type::Uint16);
        assert_eq!(IntLiteralValue::Uint32(1).get_type(), Type::Uint32);
        assert_eq!(IntLiteralValue::Uint64(1).get_type(), Type::Uint64);
        assert_eq!(IntLiteralValue::Uint128(1).get_type(), Type::Uint128);
        assert_eq!(IntLiteralValue::Usize(1).get_type(), Type::Usize);
    }

    // Testing IntLiteralValue helpers
    //
    #[test]
    fn int_literal_as_i128() {
        assert_eq!(IntLiteralValue::Int8(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int8(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int16(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int16(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int32(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int32(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int64(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int64(100).as_i128(), 100i128);
        assert_eq!(IntLiteralValue::Int128(-1).as_i128(), -1i128);
        assert_eq!(IntLiteralValue::Int128(100).as_i128(), 100i128);
    }

    #[test]
    fn int_literal_as_u128() {
        assert_eq!(IntLiteralValue::Usize(usize::MAX).as_u128(), usize::MAX as u128);
        assert_eq!(IntLiteralValue::Byte(255).as_u128(), 255u128);
        assert_eq!(IntLiteralValue::Uint16(u16::MAX).as_u128(), u16::MAX as u128);
        assert_eq!(IntLiteralValue::Uint32(u32::MAX).as_u128(), u32::MAX as u128);
        assert_eq!(IntLiteralValue::Uint64(u64::MAX).as_u128(), u64::MAX as u128);
        assert_eq!(IntLiteralValue::Uint128(u128::MAX).as_u128(), u128::MAX);
    }

    // Signed literals casted as u128 should trigger a safety panic
    #[test]
    #[should_panic]
    fn int_literal_int8_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int8(i8::MIN).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int8_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int8(i8::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int16_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int16(i16::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int16_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int16(i16::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int32_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int32(i32::MIN).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int32_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int32(i32::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int64_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int64(i64::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int64_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int64(i64::MAX).as_u128();
    }


    #[test]
    #[should_panic]
    fn int_literal_int128_min_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int128(i128::MIN).as_u128();
    }

    #[test]
    #[should_panic]
    fn int_literal_int128_max_as_u128_unsafe_panics_on_signed() {
        IntLiteralValue::Int128(i128::MAX).as_u128();
    }


    // Unsigned literals casted as i128 should trigger a safety panic
    #[test]
    #[should_panic]
    fn int_literal_byte_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Byte(u8::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_byte_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Byte(u8::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint16_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint16(u16::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_uint16_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint16(u16::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint32_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint32(u32::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint32_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint32(u32::MAX).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_uint64_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint64(u64::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint64_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint64(u64::MAX).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint128_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint128(u128::MIN).as_i128();
    }

    #[test]
    #[should_panic]
    fn int_literal_uint128_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Uint128(u128::MAX).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_usize_min_as_i128_panics_on_unsigned() {
        IntLiteralValue::Usize(usize::MIN).as_i128();
    }


    #[test]
    #[should_panic]
    fn int_literal_usize_max_as_i128_panics_on_unsigned() {
        IntLiteralValue::Usize(usize::MAX).as_i128();
    }

}
