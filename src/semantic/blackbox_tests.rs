use super::*;
use crate::parser::{
    Param, Variable, VariableAssignment, 
    IfStmt, WhileStmt
};


use crate::tests_consts::{
    ALL_TYPES_NO_ARR, ALL_TYPES_NO_ARR_SCATTERED, ALL_TYPES_NO_ARR_NO_USIZE, ALL_TYPES_NO_ARR_NO_INFER,
    ALL_UNSIGNED_TYPES_NO_ARR, ALL_SIGNED_TYPES_NO_ARR,
    ALL_BIN_OP_KIND_ARTH, ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ,
};


// helper functions

fn get_all_signed_literals_no_arr() -> [Expr; 7] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),
    ];

    return literals;
}


fn get_all_signed_literals_no_arr_no_float() -> [Expr; 5] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),
    ];

    return literals;
}





fn get_all_unsigned_literals_no_arr() -> [Expr; 6] {
    let literals = [
        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),
        usize_lit(1)
    ];

    return literals;
}


fn get_all_literals_no_arr_str_bool() -> [Expr; 13] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),
    ];

    return literals;
}



fn get_all_literals_no_arr_str_bool_scattered() -> [Expr; 13] {
    let literals = [
        uint32_lit(1),
        int8_lit(1),
        int64_lit(1),
        uint128_lit(1),
        float32_lit(1.0),

        uint16_lit(1),
        usize_lit(1),
        int16_lit(1),
        byte_lit(1),
        float64_lit(1.0),
        uint64_lit(1),
        int128_lit(1),
        int32_lit(1),

    ];

    return literals;
}






fn get_all_literals_no_arr() -> [Expr; 15] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        usize_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}

fn get_all_literals_no_arr_scattered_order() -> [Expr; 15] {
    let literals = [
        int128_lit(1),
        int8_lit(1),
        uint64_lit(1),
        float32_lit(1.0),
        int64_lit(1),
        uint16_lit(1),
        str_lit("Hi"),
        uint128_lit(1),
        float64_lit(1.0),
        uint32_lit(1),
        int16_lit(1),
        bool_lit(false),
        byte_lit(1),
        int32_lit(1),
        usize_lit(1)
    ];

    return literals;
}



fn get_all_literals_no_arr_no_usize() -> [Expr; 14] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

        byte_lit(1),
        uint16_lit(1),
        uint32_lit(1),
        uint64_lit(1),
        uint128_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}



fn span() -> Span {
    Span { line: 1, column: 0 }
}

/// Build an AST that contains exactly one function.
fn ast_one(func: Function) -> AST {
    AST { functions: vec![func] }
}

/// Build a void function (no return type) with the given body.
fn void_func(name: &str, params: Vec<Param>, mut body: Vec<Stmt>) -> Function {
    if body.len() == 0 {
        // Dummy body because empty branches are not allowed.
        body = vec![var_decl("x", Type::Int8, Some(int32_lit(69)))];
    }

    Function {
        name: name.to_string(),
        params,
        return_type: None,
        body,
        span: span(),
    }
}

/// Build a function that returns a single type.
fn returning_func(name: &str, params: Vec<Param>, ret: Vec<Type>, body: Vec<Stmt>) -> Function {
    Function {
        name: name.to_string(),
        params,
        return_type: Some(ret),
        body,
        span: span(),
    }
}

fn param(name: &str, ty: Type) -> Param {
    Param { name: name.to_string(), type_name: ty, span: span() }
}

fn var_decl(name: &str, ty: Type, value: Option<Expr>) -> Stmt {
    Stmt::VarDecl(Variable {
        name: name.to_string(),
        type_name: ty,
        value,
        span: span(),
    })
}

fn int8_lit(n: i8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int8(n), span: span() }
}

fn int16_lit(n: i16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int16(n), span: span() }
}

fn int32_lit(n: i32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int32(n), span: span() }
}

fn int64_lit(n: i64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int64(n), span: span() }
}

fn int128_lit(n: i128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Int128(n), span: span() }
}



fn byte_lit(b: u8) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Byte(b), span: span() }
}

fn uint16_lit(n: u16) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint16(n), span: span() }
}

fn uint32_lit(n: u32) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint32(n), span: span() }
}

fn uint64_lit(n: u64) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint64(n), span: span() }
}

fn uint128_lit(n: u128) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Uint128(n), span: span() }
}


fn usize_lit(n: usize) -> Expr {
    Expr::IntLiteral { value: IntLiteralValue::Usize(n), span: span() }
}


fn float32_lit(f: f32) -> Expr {
    Expr::FloatLiteral { value: FloatLiteralValue::Float32(f), span: span() }
}


fn float64_lit(f: f64) -> Expr {
    Expr::FloatLiteral { value: FloatLiteralValue::Float64(f), span: span() }
}


fn bool_lit(b: bool) -> Expr {
    Expr::BoolLiteral { value: b, span: span() }
}

fn str_lit(s: &str) -> Expr {
    Expr::StringLiteral { value: s.to_string(), span: span() }
}

fn var_expr(name: &str) -> Expr {
    Expr::Var { name: name.to_string(), span: span() }
}

fn call_expr(name: &str, args: Vec<Expr>) -> Expr {
    Expr::Call { name: name.to_string(), args, span: span() }
}

fn return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}


#[cfg(test)]
mod blackbox_tests {
    use super::*;


    // duplicate functions are not allowed
    #[test]
    fn test_duplicate_function_name_errors() {
        let f1 = void_func("foo", vec![], vec![]);
        let f2 = void_func("foo", vec![], vec![]);
        let mut ast = AST { functions: vec![f1, f2] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Duplicate function"));
    }

    // Code after return is not allowed

    #[test]
    fn test_code_after_return_errors() {
        // returning func: return then another return.
        //
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                return_stmt(vec![l.clone()]),
                var_decl("x", t.clone(), None),
            ];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Dead code detected"));
        }
    }

    // missing return
    //
    #[test]
    fn test_missing_return_in_typed_function_errors() {
        // Function declares return type but body has no return statement.

        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![var_decl("x", t.clone(), Some(l.clone()))];
            let func = returning_func("foo", vec![], vec![t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();
            assert!(err.starts_with("Semantic error: Function `foo` declares return type(s)"));
            assert!(err.contains("but statement branch body does not end with a return statement"));
        }
    }

    #[test]
    fn test_return_in_void_function_errors() {
        let literals = get_all_literals_no_arr();
        
        for l in literals {
            // Void function that tries to return a value.
            let body = vec![return_stmt(vec![l.clone()])];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("no declared return type"));
        }
    }

    // type inference tests

    #[test]
    fn test_infer_type_literal() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            // a literal of type T with infer type should be inferred correctly as T
            let body = vec![var_decl("x", Type::Infer, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            // After check, the VarDecl type should be T
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.type_name, t.clone());
            } else {
                panic!("expected VarDecl");
            }
        }
    }

    #[test]
    fn test_infer_requires_initializer_or_explicit_type() {
        // Variables declared with Infer type and no value must error
        let body = vec![var_decl("x", Type::Infer, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires explicit type"));
    }

    // type mismatch tests


    // This tests  integers / floats only, against Bool / String
    #[test]
    fn test_vardecl_type_mismatch_int_bool_errors() {

        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in &literals_ints_floats {
            // Variables declared with explicit type of bool, but given an non-bool literal is a type mismatch
            let body = vec![var_decl("x", Type::Bool, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());

            let err = result.unwrap_err().to_string();
            let err = err.starts_with("Semantic error: Cannot assign integer literal to non-integer type `bool`")
                     || err.starts_with("Semantic error: Cannot assign float literal to non-float type `bool`");

            assert!(err); 
        }


        for l in literals_ints_floats {
            // Variables declared with explicit type of string, but given an non-string literal is a type mismatch
            let body = vec![var_decl("x", Type::String, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());

            let err = result.unwrap_err().to_string();
            let err = err.starts_with("Semantic error: Cannot assign integer literal to non-integer type `string`")
                     || err.starts_with("Semantic error: Cannot assign float literal to non-float type `string`");

            assert!(err);                
        }

    }

    #[test]
    fn test_type_mismatch_return_errors() {
        // Function returns Int32 but body returns Bool.
        let body = vec![return_stmt(vec![bool_lit(true)])];
        let func = returning_func("foo", vec![], vec![Type::Int32], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Return type mismatch"));
    }

    #[test]
    fn test_return_count_mismatch_errors() {
        // Declares two return types but returns one value.

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![return_stmt(vec![l.clone()])];
            let func = returning_func("foo", vec![], vec![t.clone(), t.clone()], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }

    // default values assigning tests
    //

    #[test]
    fn test_default_int8_zero() {
        // `own x int8` value should default to an Int literal with type Int8 and value of 0
        let body = vec![var_decl("x", Type::Int8, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int16_zero() {
        // `own x int16` value should default to an Int literal with type Int16 and value of 0
        let body = vec![var_decl("x", Type::Int16, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int16(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_int32_zero() {
        // `own x int32` value should default to an Int literal with type Int32 and value of 0
        let body = vec![var_decl("x", Type::Int32, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int32(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int64_zero() {
        // `own x int64` value should default to an Int literal with type Int64 and value of 0
        let body = vec![var_decl("x", Type::Int64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int64(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_int128_zero() {
        // `own x int128` value should default to an Int literal with type Int128 and value of 0
        let body = vec![var_decl("x", Type::Int128, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int128(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


//
    
    #[test]
    fn test_default_byte_zero() {
        // `own x byte` value should default to an Int literal with type Byte and value of 0
        let body = vec![var_decl("x", Type::Byte, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Byte(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_uint16_zero() {
        // `own x uint16` value should default to an Int literal with type Uint16 and value of 0
        let body = vec![var_decl("x", Type::Uint16, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint16(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_uint32_zero() {
        // `own x uint32` value should default to an Int literal with type Uint32 and value of 0
        let body = vec![var_decl("x", Type::Uint32, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint32(0), .. })));
        } else { panic!("expected VarDecl") }    
    }

    #[test]
    fn test_default_uint64_zero() {
        // `own x uint64` value should default to an Int literal with type Uint64 and value of 0
        let body = vec![var_decl("x", Type::Uint64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint64(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_uint128_zero() {
        // `own x uint128` value should default to an Int literal with type Uint128 and value of 0
        let body = vec![var_decl("x", Type::Uint128, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint128(0), .. })));
        } else { panic!("expected VarDecl") }    
    }


    #[test]
    fn test_default_usize_zero() {
        // `own x usize` value should default to an Int literal with type Usize and value of 0
        let body = vec![var_decl("x", Type::Usize, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Usize(0), .. })));
        
        } else { panic!("expected VarDecl") }    
    }







    #[test]
    fn test_default_bool_false() {
        // `own x bool` value should default to a Bool literal with value of false
        let body = vec![var_decl("flag", Type::Bool, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::BoolLiteral { value: false, .. })));
        }
    }


    #[test]
    fn test_default_string_empty() {
        // `own x bool` value should default to a Bool literal with value of false
        let body = vec![var_decl("str", Type::String, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(&v.value, Some(Expr::StringLiteral { value, .. }) if value == ""));
        }
    }



    #[test]
    fn test_default_float32_zero() {
        // `own x float64` value should default to a Float literal with value of 0.0
        let body = vec![var_decl("f", Type::Float32, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::FloatLiteral { value: FloatLiteralValue::Float32(0.0), .. })));
        }
    }

    #[test]
    fn test_default_float64_zero() {
        // `own x float64` value should default to a Float literal with value of 0.0
        let body = vec![var_decl("f", Type::Float64, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::FloatLiteral { value: FloatLiteralValue::Float64(0.0), .. })));
        }
    }

    #[test]
    fn test_default_array_is_empty() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![var_decl("arr", Type::Array(Box::new(t.clone())), None)];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())));
                if let Some(Expr::ArrayLiteral { elements, array_ty, .. }) = &v.value {
                    assert!(elements.is_empty());
                    assert_eq!(array_ty, t);
                } else {
                    panic!("expected empty ArrayLiteral");
                }
            }
        }
    }


    // TODO: Improve this test to have variable length of nested arrays.
    //
    #[test]
    fn test_default_nested_array_is_empty() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            for i in 1..=100 {
                let mut nested_ty = Type::Array(Box::new(t.clone()));

                for _ in 0..=i {
                    nested_ty = Type::Array(Box::new(nested_ty));
                }

                let body = vec![var_decl("nested_array", nested_ty.clone(), None)];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                check_semantics(&mut ast).unwrap();
                if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                    assert_eq!(v.type_name, nested_ty);
                    if let Some(Expr::ArrayLiteral { elements, array_ty, .. }) = &v.value {
                        assert!(elements.is_empty());
                        // This is to add the outer most array type wrapping, so variable 
                        // type == array_ty
                        //
                        let array_ty_wraped = Type::Array(Box::new(array_ty.clone()));
                        assert_eq!(array_ty_wraped, nested_ty);

                    } else {
                        panic!("expected empty ArrayLiteral");
                    }
                }
            }
                
        }
    }

    // move semantics 

    #[test]
    fn test_use_after_move_errors_explicit_type() {
        // own a t = 5
        // own b t = a   (moves `a`)
        // own c t = a   (this must error because `a` already moved)

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", t.clone(), Some(var_expr("a"))),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn test_use_after_move_errors_infer_type() {
        // own a = 5
        // own b = a   (moves `a`)
        // own c = a   (this must error because `a` already moved)

        let literals = get_all_literals_no_arr();
        
        for l in literals {
            let body = vec![
                var_decl("a", Type::Infer, Some(l.clone())),
                var_decl("b", Type::Infer, Some(var_expr("a"))),
                var_decl("c", Type::Infer, Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn test_use_after_move_errors_explicit_and_infer_type() {
        // own a T = 5
        // own b = a   (moves `a`)
        // own c T = a   (this must error because `a` already moved)
        //

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", Type::Infer, Some(var_expr("a"))),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
        }
    }

    #[test]
    fn test_copy_call_allows_reuse() {
        // own a T = Some Literal
        // own b T = copy(a)  (copies, does not move)
        // own c T = a        (valid, because no moves happened)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let copy_a = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                var_decl("b", t.clone(), Some(copy_a)),
                var_decl("c", t.clone(), Some(var_expr("a"))),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_pass_variable_to_call_marks_it_moved() {
        // bar takes one t.
        // own a t = Some Literal
        // bar(a)       (moves a)
        // own b t = a  (error)
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let bar = void_func("bar", vec![param("p", t.clone())], vec![]);
            let body = vec![
                var_decl("a", t.clone(), Some(l.clone())),
                Stmt::Expr(call_expr("bar", vec![var_expr("a")])),
                var_decl("b", t.clone(), Some(var_expr("a"))),
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![bar, caller] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("moved"));
            
        }
    }

    // locking / unlocking variables

    #[test]
    fn test_assign_to_locked_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::VarAssign(VariableAssignment {
                    name: "x".to_string(),
                    value: l.clone(),
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("locked"));
        }
    }

    #[test]
    fn test_overshadow_locked_variable_same_type_and_literal_errors() {
        let literals = get_all_literals_no_arr();
       
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                var_decl("x", t.clone(), Some(l.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("locked"));
        }
    }

    // Same test as above, but re-declartion use a different type and literal
    #[test]
    fn test_overshadow_locked_variable_different_type_and_literal_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();


        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let body = vec![
                var_decl("x", t1.clone(), Some(l1.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                var_decl("x", t2.clone(), Some(l2.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("locked"));
        }
    }


    #[test]
    fn test_unlock_allows_redeclare_same_type() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_decl("x", t.clone(), Some(l.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    // Same test as above, but re-declartion use a different type and literal
    #[test]
    fn test_unlock_allows_redeclare_different_type() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let body = vec![
                var_decl("x", t1.clone(), Some(l1.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_decl("x", t2.clone(), Some(l2.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }


    #[test]
    fn test_unlock_allows_reassign() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                Stmt::VarAssign(VariableAssignment {
                    name: "x".to_string(),
                    value: l.clone(),
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }


    #[test]
    fn test_lock_unlock_lock_unlock_variable() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Unlock(vec![var_expr("x")]),
                var_decl("x", t.clone(), Some(l.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }


    #[test]
    fn test_double_lock_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                Stmt::Lock(vec![var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("already locked"));
        }
    }

    #[test]
    fn test_unlock_unlocked_variable_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Unlock(vec![var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is already unlocked"));
        }
    }

    #[test]
    fn test_shadowing_locked_variable_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x")]),
                var_decl("x", t.clone(), Some(l.clone())),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("is locked, therefore you cannot overshadow it"));
        }
    }

    // Test while statements with only literals, no strings/bools
    #[test]
    fn test_while_statements_ints_floats_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test while statements with only variables, no strings/bools
    #[test]
    fn test_while_statements_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test while statements with literals and variables mixed  (left & right side), no strings/bools
    #[test]
    fn test_while_statements_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        // Variable left side, Literal right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }

        // Literal left side, Variable right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::While(WhileStmt{
                        condition: condition,
                        branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }



    }





    // Test if statements with only literals, with no else, no elif, and no string/bool literals
    #[test]
    fn test_if_statements_ints_floats_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }



    // Test if statements with only variables with same type, with no else, no elif, and no string/bool variables
    #[test]
    fn test_if_statements_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with literals and variables mixed (left & right side), with no else, no elif, and no string/bool literals
    #[test]
    fn test_if_statements_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        // Variable left side, Literal right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }

        // Literal left side, Variable right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("y", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }

    //////////////////////////////////  If statements with only elif /////////////////////////

    // Test if statements with only literals, with elif. but no else, and no string/bool literals
    #[test]
    fn test_if_statements_with_elif_ints_floats_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with only variables, with elif. but no else, and no string/bool variables
    #[test]
    fn test_if_statements_with_elif_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };


                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }
    }



    // Test if statements with literals and variables mixed (left & right side), with elif. but no else, and no string/bool literals
    #[test]
    fn test_if_statements_with_elif_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }


        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let elif_condition = condition.clone();

                let body = vec![ 
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![(elif_condition, vec![
                            // For above reason
                            var_decl("e", t.clone(), None),
                        ])],
                        else_branch: None,
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
            }
        }
    }




    ////////////////////////////// end /////////////////////




    //////////////////////////////////  If statements with only else /////////////////////////

    // Test if statements with only literals, with else. but no elif, and no string/bool literals
    #[test]
    fn test_if_statements_with_else_ints_floats_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with only variables with same type with else. but no elif, and no string/bool variables
    #[test]
    fn test_if_statements_with_else_ints_floats_vars_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    var_decl("y", t.clone(), Some(l.clone())),

                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }


    // Test if statements with literals and variables mixed (left & right side), with else. but no elif, and no string/bool literals
    #[test]
    fn test_if_statements_with_else_ints_floats_vars_literals_same_type() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        // Variable left side, Literal right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(var_expr("x")),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("x", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],
                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }

        // Literal left side, Variable right side
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(var_expr("y")),
                        span: span(),
                    };

                let body = vec![ 
                    var_decl("y", t.clone(), Some(l.clone())),
                    Stmt::If(IfStmt{
                        condition: condition,
                        if_branch: vec![
                            // Just dummy declaration, so we don't get flagged by dead code because
                            // of empty branch.
                            var_decl("z", t.clone(), None),
                        ],
                        elif_branches: vec![],

                        else_branch: Some(vec![
                            // For above reason
                            var_decl("q", t.clone(), None)
                        ]),
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }

    ////////////////////////////// end /////////////////////


    // function calls 

    #[test]
    fn test_call_unknown_function_errors() {
        let body = vec![Stmt::Expr(call_expr("nonexistent", vec![]))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown function"));
    }

    #[test]
    fn test_call_wrong_arity_errors() {
        for t in ALL_TYPES_NO_ARR {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![]))]; // 0 args instead of 1
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("expects 1 arguments, got 0"));
        }
    }

    #[test]
    fn test_call_wrong_arg_type_errors() {
        let literals_scattered = get_all_literals_no_arr_scattered_order();


        for ((l, t1), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(ALL_TYPES_NO_ARR)
        {
            let callee = void_func("bar", vec![param("a", t2.clone())], vec![]);

            let body = vec![
                var_decl("x", t1.clone(), Some(l.clone())),

                Stmt::Expr(call_expr("bar", vec![var_expr("x")]))
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("type mismatch"));
        }
    }

    #[test]
    fn test_correct_call_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![l.clone()]))];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };
            check_semantics(&mut ast).unwrap();
        }
    }


    // All signed literals whose value equal to or more than `0` can be safely converted to uint,
    // so passing integer literals directly to functions should always work
    #[test]
    fn test_correct_call_literal_inference_passes() {
        let signed_literals = get_all_signed_literals_no_arr_no_float();
        
        for (sl, t) in signed_literals.iter().zip(ALL_UNSIGNED_TYPES_NO_ARR.iter()) {
            let callee = void_func("bar", vec![param("a", t.clone())], vec![]);
            let body = vec![Stmt::Expr(call_expr("bar", vec![sl.clone()]))];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };
            check_semantics(&mut ast).unwrap();
        }
    }
    // return statement with multiple values (aka multi-return)

    #[test]
    fn test_multi_return_decl_correct() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { own a, b = pair() }

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_multi_return_count_mismatch_errors() {
        // pair returns 2 values, but programmer only binds 1 variable

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
        }
    }

    // array invalid access patterns errors checks
    #[test]
    fn test_array_out_of_bounds_single_access_errors() {
        // own arr t[] = [l, l, l]
        // own x t = arr[i]  (out of bounds)
        // i starts from 3 up to 10k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..10000 {
                let arr_lit = Expr::ArrayLiteral {
                    elements: vec![l.clone(), l.clone(), l.clone()],
                    array_ty: t.clone(),
                    span: span(),
                };
                let access = Expr::ArraySingleAccess {
                    array: Box::new(var_expr("arr")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out-of-bounds"));
            }
            
        }
    }



    #[test]
    fn test_array_out_of_bounds_multiple_access_errors() {
        // own arr t[] = [l, l, l]
        // own x t = arr[0:i]  (out of bounds)
        // i starts from 3 up to 10k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..10000 {
                let arr_lit = Expr::ArrayLiteral {
                    elements: vec![l.clone(), l.clone(), l.clone()],
                    array_ty: t.clone(),
                    span: span(),
                };

                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("arr")),
                    start: Some(Box::new(usize_lit(0))),
                    end: Some(Box::new(usize_lit(i))),
                    span: span(),
                };
                let body = vec![
                    var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out-of-bounds"));
            }
        }
    }


    #[test]
    fn test_array_valid_access_passes() {

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArraySingleAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }
            }       
        }
    }


    // i.e. "hi"[0] is an error. You can only access variables, of type array, not literals.
    #[test]
    fn test_array_access_on_literals_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..1000 {
                let access = Expr::ArraySingleAccess {
                    array: Box::new(l.clone()),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected variable of any `array` type"));

            }       
        }
    }




    // Array access on undeclared variable
    #[test]
    fn test_array_access_on_undeclared_var_errors() {
        for t in ALL_TYPES_NO_ARR {
            for i in 0..1000 {
                let access = Expr::ArraySingleAccess {
                    array: Box::new(var_expr("e")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on undeclared variable `e`"));
            }       
        }
    }

    // Array access on non-array variable
    #[test]
    fn test_array_access_on_non_array_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..1000 {
                let access = Expr::ArraySingleAccess {
                    array: Box::new(var_expr("e")),
                    index: Box::new(usize_lit(i)),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", t.clone(), Some(l.clone())),
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on non-array variable `e`"));
            }       
        }
    }


    #[test]
    fn test_array_access_on_moved_variable_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArraySingleAccess {
                        array: Box::new(var_expr("a")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        // move a to x
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(var_expr("a"))), 
                        var_decl("y", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on moved variable `a`"));
                }
            }       
        }
    }



    #[test]
    fn test_array_valid_multiple_access_both_ends_passes() {

        // This is no black magic voodooo.. not too much of it at least.. idk..
        // This is just creating an array of dynamic sizes, and testing slicing it aka multiple
        // access
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArrayMultipleAccess {
                        array: Box::new(var_expr("arr")),
                        start: Some(Box::new(usize_lit(1))),
                        end: Some(Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }       
            }
        }
    }


    // Same as above test, but this makes start and end variables instead of literals
    #[test]
    fn test_array_valid_multiple_access_both_ends_vars_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArrayMultipleAccess {
                        array: Box::new(var_expr("arr")),
                        start: Some(Box::new(var_expr("e"))),
                        end: Some(Box::new(var_expr("h"))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", Type::Usize, Some(usize_lit(1))),
                        var_decl("h", Type::Usize, Some(usize_lit(i2+1))),
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }       
            }
        }
    }

    // Same as above test, but this makes start usize var, but end is not usize
    // and vice versa.
    #[test]
    fn test_array_valid_multiple_access_both_ends_vars_start_not_usize_errors() {
        let literals_no_usize = get_all_literals_no_arr_no_usize();
        
        for (l, t) in literals_no_usize.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArrayMultipleAccess {
                        array: Box::new(var_expr("arr")),
                        start: Some(Box::new(var_expr("e"))),
                        end: Some(Box::new(var_expr("h"))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", t.clone(), Some(l.clone())),
                        var_decl("h", Type::Usize, Some(usize_lit(i2+1))),
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected start index to be of type `usize` for array"));

                }       
            }
        }

        // Same as above, but a little weaker because we can't do i2+1 for l.. its just always 1.

        for (l, t) in literals_no_usize.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("arr")),
                    start: Some(Box::new(var_expr("e"))),
                    end: Some(Box::new(var_expr("h"))),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", Type::Usize, Some(usize_lit(1))),
                    var_decl("h", t.clone(), Some(l.clone())),
                    var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                    var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                println!("niggers master {:?}", result);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected end index to be of type `usize` for array"));
            }
        }
    }





    // Similar to above test(s), except here we attempt to access a literal instead of array variable, which
    // should always error
    #[test]
    fn test_array_multiple_access_on_literals_both_ends_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..100 {
                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(l.clone()),
                    start: Some(Box::new(usize_lit(1))),
                    end: Some(Box::new(usize_lit(i+1))),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Expected variable of any `array` type"));
            }
        }
    }


    // Array access on non-array variable
    #[test]
    fn test_array_multiple_access_on_non_array_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..1000 {
                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("e")),
                    start: Some(Box::new(usize_lit(1))),
                    end: Some(Box::new(usize_lit(i+1))),
                    span: span(),
                };
                let body = vec![
                    var_decl("e", t.clone(), Some(l.clone())),
                    var_decl("x", t.clone(), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on non-array variable `e`"));
            }       
        }
    }



    #[test]
    fn test_array_valid_multiple_access_both_ends_on_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i-1 {
                    let access = Expr::ArrayMultipleAccess {
                        array: Box::new(var_expr("arr")),
                        start: Some(Box::new(usize_lit(1))),
                        end: Some(Box::new(usize_lit(i2+1))),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    check_semantics(&mut ast).unwrap();
                }       
            }
        }
    }






    // Array access on undeclared variable
    #[test]
    fn test_array_multiple_access_on_undeclared_var_errors() {
        
        for t in ALL_TYPES_NO_ARR {
            for i in 1..100 {
                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("e")),
                    start: Some(Box::new(usize_lit(1))),
                    end: Some(Box::new(usize_lit(i))),
                    span: span(),
                };
                let body = vec![
                    var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on undeclared variable `e`"));
            }
        }

    }
     






    #[test]
    fn test_array_valid_multiple_access_start_only_passes() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone()],
                array_ty: t.clone(),
                span: span(),
            };
            let access = Expr::ArrayMultipleAccess {
                array: Box::new(var_expr("arr")),
                start: Some(Box::new(usize_lit(1))),
                end: None,
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_array_valid_multiple_access_end_only_passes() {

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone()],
                array_ty: t.clone(),
                span: span(),
            };
            let access = Expr::ArrayMultipleAccess {
                array: Box::new(var_expr("arr")),
                start: None,
                end: Some(Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }



    #[test]
    fn test_array_slice_start_greater_than_end_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone(), l.clone()],
                array_ty: t.clone(),
                span: span(),
            };
            let slice = Expr::ArrayMultipleAccess {
                array: Box::new(var_expr("arr")),
                start: Some(Box::new(usize_lit(3))),
                end: Some(Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                var_decl("s", Type::Array(Box::new(t.clone())), Some(slice)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Start index"));
        }


        // Same test but for arrays of infer
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone(), l.clone()],
                array_ty: t.clone(),
                span: span(),
            };
            let slice = Expr::ArrayMultipleAccess {
                array: Box::new(var_expr("arr")),
                start: Some(Box::new(usize_lit(3))),
                end: Some(Box::new(usize_lit(1))),
                span: span(),
            };
            let body = vec![
                var_decl("arr", Type::Infer, Some(arr_lit)),
                var_decl("s", Type::Infer, Some(slice)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Start index"));
        }

    }

    // unary operations 
    #[test]
    fn test_negate_unsigned_errors() {
        let unsigned_literals = get_all_unsigned_literals_no_arr();

        for (ul, t) in unsigned_literals.iter().zip(ALL_UNSIGNED_TYPES_NO_ARR.iter()) {
            let neg = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(ul.clone()),
                span: span(),
            };
            let body = vec![var_decl("x", t.clone(), Some(neg))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("negate"));
        }
    }

    #[test]
    fn test_negate_signed_passes() {
        let signed_literals = get_all_signed_literals_no_arr();

        for (sl, t) in signed_literals.iter().zip(ALL_SIGNED_TYPES_NO_ARR.iter()) {
            let neg = Expr::UnaryOp {
                op: UnaryOpKind::Negate,
                expr: Box::new(sl.clone()),
                span: span(),
            };
            let body = vec![var_decl("x", t.clone(), Some(neg))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_string_binop_arth_errors() {
        for b in ALL_BIN_OP_KIND_ARTH {
            // Strings may not be ever wrapped in ANY BinOpKind (except *some* comparison operators like == and !=), we use format() instead.
            let bin = Expr::BinOp {
                left: Box::new(str_lit("hello")),
                op: b,
                right: Box::new(str_lit("world")),
                span: span(),
            };
            let body = vec![var_decl("s", Type::String, Some(bin))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: You cannot perform arithmetic on types: `string` vs `string`"));
        }
    }



    #[test]
    fn test_string_binop_comp_eq_passes() {
        for b in ALL_BIN_OP_KIND_COMP_EQ {
            let bin = Expr::BinOp {
                left: Box::new(str_lit("hello")),
                op: b,
                right: Box::new(str_lit("world")),
                span: span(),
            };
            let body = vec![var_decl("s", Type::Bool, Some(bin))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_integers_and_floats_binop_arth_passes() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();
        
        for (l, t) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", t.clone(), Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());
            }
        }
    }

    // integer literal inference 

    #[test]
    fn test_integer_literal_inferred_to_int8() {
        // if variable is declared with an int8 and the value is an int32, but it can fit in int8,
        // it shouldn't error
        let lit = int32_lit(100); 
        let body = vec![var_decl("x", Type::Int8, Some(lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(100), .. })));
        }
    }

    #[test]
    fn test_integer_literal_out_of_range_for_int8_errors() {
        for i in 0..32767 {
            let lit = int16_lit(i);
            let body = vec![var_decl("x", Type::Int8, Some(lit))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            if i <= i8::MAX as i16 {
                assert!(result.is_ok());

            } else {
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("out of range"));
            }
        }
    }

    #[test]
    fn test_float32_cannot_accept_float64_errors() {
        let lit = Expr::FloatLiteral { value: FloatLiteralValue::Float64(3.14), span: span() };
        let body = vec![var_decl("f", Type::Float32, Some(lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with("Semantic error: Float literal has a float64 value but we expected a float32 value"));
    }


    // binary operation type mismatch 

    #[test]
    fn test_binop_int_non_int_mixed_types_errors() {
        // int literals are not allowed to mix with literals of non-int type
        //
        
        let int_literals = [
            int8_lit(1),
            int16_lit(1),
            int32_lit(1),
            int64_lit(1),
            int128_lit(1),

            byte_lit(1),
            uint16_lit(1),
            uint32_lit(1),
            uint64_lit(1),
            uint128_lit(1),

            usize_lit(1),
        ];


        let non_int_literals = [
            float32_lit(1.0),
            float64_lit(1.0),
            bool_lit(false),
            str_lit("Hi")
        ];

        for int in &int_literals {
            for non_int in &non_int_literals {
                for b in ALL_BIN_OP_KIND_ARTH {
                    let bin = Expr::BinOp {
                        left: Box::new(int.clone()),
                        right: Box::new(non_int.clone()),
                        op: b,
                        span: span(),
                    };
                    let body = vec![var_decl("x", Type::Infer, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    let assert_condition = result.unwrap_err().to_string();
                    let assert_condition = assert_condition.starts_with("Semantic error: Type mismatch in binary operation: ") 
                                           || assert_condition.starts_with("Semantic error: You cannot perform arithmetic on types: ");

                    assert!(assert_condition);
                }
            }
        }
        


        for int in &int_literals {
            for non_int in &non_int_literals {
                for b in ALL_BIN_OP_KIND_ARTH {
                    let bin = Expr::BinOp {
                        left: Box::new(non_int.clone()),
                        right: Box::new(int.clone()),
                        op: b,
                        span: span(),
                    };
                    let body = vec![var_decl("x", Type::Infer, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    let assert_condition = result.unwrap_err().to_string();
                    let assert_condition = assert_condition.starts_with("Semantic error: Type mismatch in binary operation: ") 
                                           || assert_condition.starts_with("Semantic error: You cannot perform arithmetic on types: ");

                    assert!(assert_condition);
                }
            }
        }
    }


    // Mixing int32, int16, float32, float64, etc should always return an error.
    //
    #[test]
    fn test_binop_arth_mixed_types_errors() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        let literals_ints_floats_scat = get_all_literals_no_arr_str_bool_scattered();

        for (l1, l2) in literals_ints_floats.iter().zip(literals_ints_floats_scat.iter()) {
            for b in ALL_BIN_OP_KIND_ARTH {
                // We declare variables here, because had we used literals, it would get inferred
                // in the binary operation expression
                //
                let bin = Expr::BinOp {
                    left: Box::new(var_expr("x")),
                    right: Box::new(var_expr("y")),
                    op: b,
                    span: span(),
                };
                let body = vec![
                    var_decl("x", Type::Infer, Some(l1.clone())),
                    var_decl("y", Type::Infer, Some(l2.clone())),
                    var_decl("z", Type::Infer, Some(bin))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                let assert_condition = result.unwrap_err().to_string();
                let assert_condition = assert_condition.starts_with("Semantic error: Type mismatch in binary operation: ");

                assert!(assert_condition);
            }
        }
    }



        
    // copy call guards 

    #[test]
    fn test_copy_of_literals_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let copy_lit = Expr::CopyCall { expr: Box::new(l.clone()), span: span() };
            let body = vec![var_decl("x", Type::Infer, Some(copy_lit))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Copying a literal"));
        }
    }

    #[test]
    fn test_double_copy_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let body = vec![
                var_decl("a", Type::Infer, Some(l.clone())),
                var_decl("b", Type::Infer, Some(
                    Expr::CopyCall {
                        expr: Box::new(Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() }),
                        span: span(),
                    }
                )),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Double copying is not needed. Remove the extra copy call. "));
            
        }
    }

    // undeclared variable usage tests

    #[test]
    fn test_use_of_undeclared_variable_other_errors() {
        // Try referencing non-existent variable "y"
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("y")))]; // y not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared variable"));
        }
    }


    #[test]
    fn test_use_of_undeclared_variable_ourself_errors() {
        // Try referencing non-existent variable "x" aka ourselves.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("x")))]; // x not declared
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared variable"));
        }
    }


    // function parameters
    //
    #[test]
    fn test_params_are_in_scope_basic() {
        // Checks if function parameters are in scope, without testing for inner scopes.
        for t in ALL_TYPES_NO_ARR {
            let body = vec![return_stmt(vec![var_expr("n")])];
            let func = returning_func("foo", vec![param("n", t.clone())], vec![t.clone()], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    // format call guards 
    #[test]
    fn test_format_call_with_literal_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![l.clone()], // plain literals not allowed
                span: span(),
            };
            let body = vec![var_decl("s", Type::String, Some(fmt))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Plain literals are not allowed in formating! Remove the format placeholders and use the literal directly!"));
        }
    } 


    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn test_format_call_without_any_template_placeholders_panics() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value".to_string(),
                expressions: vec![l.clone()], 
                span: span(),
            };
            let body = vec![var_decl("s", Type::String, Some(fmt))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let _ = check_semantics(&mut ast);
        }
    }

    #[test]
    fn test_format_call_with_variable_passes() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![var_expr("n")],
                span: span(),
            };
            let body = vec![
                var_decl("n", Type::Infer, Some(l.clone())),
                var_decl("s", Type::String, Some(fmt)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }


    // Format calls copies expressions passed to it, if you attempt to copy manually, that's an
    // error.
    #[test]
    fn test_format_call_with_expressions_copied_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            
            let copy_n = Expr::CopyCall { expr: Box::new(var_expr("n")), span: span() };
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![copy_n],
                span: span(),
            };
            let body = vec![
                var_decl("n", Type::Infer, Some(l.clone())),
                var_decl("s", Type::String, Some(fmt)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Format calls copy by default, Remove the extra copy call."));
        }
    }



    #[test]
    fn test_nested_format_call_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![var_expr("n")], 
                span: span(),
            };

            let fmt = Expr::FormatCall {
                template: "value: {}".to_string(),
                expressions: vec![fmt], 
                span: span(),
            };


            let body = vec![
                var_decl("n", Type::Infer, Some(l.clone())),
                var_decl("s", Type::String, Some(fmt)),
            ];
            
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().starts_with("Semantic error: Nested FormatCalls are not allowed."));
            
        }
    }


    // happy-path integration 

    #[test]
    fn test_full_valid_program() {
        // This program tests all integers / floating points and all arthemtic binary operations
        // it also tests variable declaration, function declaration, and function calling 
        let literals = get_all_literals_no_arr();

        for b in ALL_BIN_OP_KIND_ARTH {
            let add_body = vec![return_stmt(vec![
                Expr::BinOp {
                    left: Box::new(var_expr("a")),
                    op: b,
                    right: Box::new(var_expr("b")),
                    span: span(),
                }
            ])];
            

            for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
                if matches!(t.clone(), Type::Bool | Type::String) {
                    continue;
                }

                let add = returning_func(
                    "add",
                    vec![param("a", t.clone()), param("b", t.clone())],
                    vec![t.clone()],
                    add_body.clone(),
                );

                let main_body = vec![
                    var_decl("r", t.clone(), Some(call_expr("add", vec![l.clone(), l.clone()]))),
                ];
                let main = void_func("main", vec![], main_body);

                let mut ast = AST { functions: vec![add, main] };
                
                assert!(check_semantics(&mut ast).is_ok());
            }
        }
    }
}
