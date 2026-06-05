/// Internally, the compiler parser stores integer literals as smallest possible integer.
/// Inference, coerces the integer literal into whatever type, and errors if not able.
/// Inference only applies to integers. all other types cannot be inferred.
/// So, this file does 2 things: Ensure internal inference system for integers is correct, AND,
/// ensure the inference system does not behave when used on non-integers.
///
use super::*;

#[cfg(test)]
mod int_literal_internal_inference_tests {
    use super::*;

    // Float literal internal inference.
    // Floating 64 literal cannot be ceorcied into an integer.
    #[test]
    fn test_floating_literal_is_64_but_type_is_integer_errors() {
        let float64_lits = [
            float64_lit(1.0),
            float64_lit(1e12)
        ];

        for l in float64_lits {
            for t in ALL_INT_TYPES_NO_ARR {
                let body = vec![var_decl("x", t.clone(), l.clone())];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Type mismatch"));
            }
        }
    }

    // (signed) integer literal internal inference
    #[test]
    fn test_integer_literal_inferred_to_int8() {
        // if variable is declared with an int8 and the value is a different signed int literal, but it can fit in int8,
        // it shouldn't error
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int8, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Int8(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int8_errors() {
        let edge_cases_numbers = [
            i8::MIN as i16, i8::MAX as i16,
            i16::MIN, i16::MAX
        ];

        for i in edge_cases_numbers {
            let lit = int16_lit(i);
            let body = vec![var_decl("x", Type::Int8, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= i8::MAX as i16) && (i >= i8::MIN as i16) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }



    // Same as above test(s) but for int16
    #[test]
    fn test_integer_literal_inferred_to_int16() {
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int16, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Int16(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int16_errors() {
        let edge_cases_numbers = [
            i16::MIN as i32, i16::MAX as i32,
            i32::MIN, i32::MAX
        ];

        for i in edge_cases_numbers {
            let lit = int32_lit(i);
            let body = vec![var_decl("x", Type::Int16, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= i16::MAX as i32) && (i >= i16::MIN as i32) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }
    

    // Same as above test(s) but for int32
    #[test]
    fn test_integer_literal_inferred_to_int32() {
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int32, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Int32(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int32_errors() {
        let edge_cases_numbers = [
            i32::MIN as i64, i32::MAX as i64,
            i64::MIN, i64::MAX
        ];

        for i in edge_cases_numbers {
            let lit = int64_lit(i);
            let body = vec![var_decl("x", Type::Int32, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= i32::MAX as i64) && (i >= i32::MIN as i64) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }



    // Same as above test(s) but for int64
    #[test]
    fn test_integer_literal_inferred_to_int64() {
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int64, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Int64(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int64_errors() {
        let edge_cases_numbers = [
            i64::MIN as i128, i64::MAX as i128,
            i128::MIN, i128::MAX
        ];

        for i in edge_cases_numbers {
            let lit = int128_lit(i);
            let body = vec![var_decl("x", Type::Int64, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= i64::MAX as i128) && (i >= i64::MIN as i128) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }


    // 
    #[test]
    fn test_integer_literal_inferred_to_int128() {
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int128, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Int128(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int128_errors() {
        let edge_cases_numbers = [
            i128::MAX as u128 + 1, u128::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint128_lit(i);
            let body = vec![var_decl("x", Type::Int128, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("out of range"));
        }

    }




    // (unsigned) integer literal inference
    #[test]
    fn test_integer_literal_inferred_to_byte() {
        // if variable is declared with an byte and the value is a different signed int literal, but it can fit in byte,
        // it shouldn't error
        let literals_unsigned_ints = get_all_unsigned_literals_no_arr();

        for l in literals_unsigned_ints {
            let body = vec![var_decl("x", Type::Byte, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Byte(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_byte_errors() {
        let edge_cases_numbers = [
            u8::MIN as u16, u8::MAX as u16,
            u16::MIN, u16::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint16_lit(i);
            let body = vec![var_decl("x", Type::Byte, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= u8::MAX as u16) && (i >= u8::MIN as u16) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }

    
    // Same as above test(s) but for uint16
    #[test]
    fn test_integer_literal_inferred_to_uint16() {
        let literals_unsigned_ints = get_all_unsigned_literals_no_arr();

        for l in literals_unsigned_ints {
            let body = vec![var_decl("x", Type::Uint16, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Uint16(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_uint16_errors() {
        let edge_cases_numbers = [
            u16::MIN as u32, u16::MAX as u32,
            u32::MIN, u32::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint32_lit(i);
            let body = vec![var_decl("x", Type::Uint16, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= u16::MAX as u32) && (i >= u16::MIN as u32) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }



    // Same as above test(s) but for uint32
    #[test]
    fn test_integer_literal_inferred_to_uint32() {
        let literals_unsigned_ints = get_all_unsigned_literals_no_arr();

        for l in literals_unsigned_ints {
            let body = vec![var_decl("x", Type::Uint32, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Uint32(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_uint32_errors() {
        let edge_cases_numbers = [
            u32::MIN as u64, u32::MAX as u64,
            u64::MIN, u64::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint64_lit(i);
            let body = vec![var_decl("x", Type::Uint32, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= u32::MAX as u64) && (i >= u32::MIN as u64) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }



    // Same as above test(s) but for uint64
    #[test]
    fn test_integer_literal_inferred_to_uint64() {
        let literals_unsigned_ints = get_all_unsigned_literals_no_arr();

        for l in literals_unsigned_ints {
            let body = vec![var_decl("x", Type::Uint64, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Uint64(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_uint64_errors() {
        let edge_cases_numbers = [
            u64::MIN as u128, u64::MAX as u128,
            u128::MIN, u128::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint128_lit(i);
            let body = vec![var_decl("x", Type::Uint64, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= u64::MAX as u128) && (i >= u64::MIN as u128) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }

    // Same as above test(s) but for usize (which is most commonly uint64)
    #[test]
    fn test_integer_literal_inferred_to_usize() {
        let literals_unsigned_ints = get_all_unsigned_literals_no_arr();

        for l in literals_unsigned_ints {
            let body = vec![var_decl("x", Type::Usize, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Usize(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_usize_errors() {
        let edge_cases_numbers = [
            usize::MIN as u128, usize::MAX as u128,
            u128::MIN, u128::MAX
        ];

        for i in edge_cases_numbers {
            let lit = uint128_lit(i);
            let body = vec![var_decl("x", Type::Usize, lit)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if (i <= usize::MAX as u128) && (i >= usize::MIN as u128) {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }


    #[test]
    fn test_integer_literal_inferred_to_uint128() {
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Uint128, l)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Expr::IntLiteral { value: IntLiteralValue::Uint128(1), .. }));
            } else { panic!("Expected VarDecl") }
        }
    }





}
