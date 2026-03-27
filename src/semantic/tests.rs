use super::*;
use crate::parser::{
    BinOpKind, VariableAssignment
};


const AllBinOpKindArth: [BinOpKind; 4] = [
            BinOpKind::Add,
            BinOpKind::Subtract,
            BinOpKind::Multiply,
            BinOpKind::Divide,
        ];

const AllBinOpKindComp: [BinOpKind; 6] = [
            BinOpKind::Equal,
            BinOpKind::NotEqual,
            BinOpKind::Greater,
            BinOpKind::GreaterEqual,
            BinOpKind::Less,
            BinOpKind::LessEqual,
        ];


const AllBinOpKindCompEq: [BinOpKind; 2] = [
            BinOpKind::Equal,
            BinOpKind::NotEqual,
        ];


// No array type
const ALL_TYPES_NO_ARR: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Float32,
    Type::Float64,
    Type::Bool,
    Type::String,
    Type::Infer,
];

const ALL_TYPES_NO_ARR_SCATTERED: &[Type] = &[
    Type::Int128,
    Type::Int8,
    Type::Uint64,
    Type::Int64,
    Type::Float32,
    Type::Byte,
    Type::Uint16,
    Type::String,
    Type::Uint128,
    Type::Float64,
    Type::Uint32,
    Type::Int16,
    Type::Bool,
    Type::Int32,
    Type::Usize,

    Type::Infer,
];




const ALL_SIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Float32,
    Type::Float64,
    Type::Infer,
];


const ALL_UNSIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Infer,
];

#[cfg(test)]
mod tests {
    use super::*;


    

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
            int64_lit(1),
            float32_lit(1.0),
            byte_lit(1),
            uint16_lit(1),
            str_lit("Hi"),
            uint128_lit(1),
            float64_lit(1.0),
            uint32_lit(1),
            int16_lit(1),
            bool_lit(false),
            int32_lit(1),
            usize_lit(1)
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
        // Use int32 returning func: return then another return.
        let body = vec![
            return_stmt(vec![int32_lit(1)]),
            var_decl("x", Type::Int32, None),
        ];
        let func = returning_func("foo", vec![], vec![Type::Int32], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Code after `return`"));
    }

    // missing return
    //
    #[test]
    fn test_missing_return_in_typed_function_errors() {
        // Function declares return type but body has no return statement.
        let body = vec![var_decl("x", Type::Int32, Some(int32_lit(5)))];
        let func = returning_func("foo", vec![], vec![Type::Int32], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with("Semantic error: Function `foo` declares return type(s) `[Int32]`, but statement branch body does not end with a return statement"));
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

    #[test]
    fn test_type_mismatch_int_bool_errors() {
        // Variables declared with explicit type of bool, but given an int32 literal is a type mismatch
        let body = vec![var_decl("x", Type::Bool, Some(int32_lit(5)))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().starts_with("Semantic error: Cannot assign integer literal to non-integer type `bool`"));
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
        let body = vec![return_stmt(vec![int32_lit(1)])];
        let func = returning_func("foo", vec![], vec![Type::Int32, Type::Int32], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
    }

    // default values assigning tests
    //

    #[test]
    fn test_default_int8_zero() {
        // `own x int8` value should default to an Int literal with type Int32 and value of 0
        let body = vec![var_decl("x", Type::Int8, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(0), .. })));
        }
    }

    #[test]
    fn test_default_int16_zero() {
        // `own x int16` value should default to an Int literal with type Int32 and value of 0
        let body = vec![var_decl("x", Type::Int16, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int16(0), .. })));
        }
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
        }
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
        }
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
        }
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
        let body = vec![var_decl("arr", Type::Array(Box::new(Type::Int32)), None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                assert!(elements.is_empty());
            } else {
                panic!("expected empty ArrayLiteral");
            }
        }
    }


    #[test]
    fn test_default_nested_array_is_empty() {
        let body = vec![var_decl("nested_array", Type::Array(Box::new(Type::Array(Box::new(Type::Int32)))), None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            if let Some(Expr::ArrayLiteral { elements, .. }) = &v.value {
                assert!(elements.is_empty());
            } else {
                panic!("expected empty ArrayLiteral");
            }
        }
    }

    // move semantics 

    #[test]
    fn test_use_after_move_errors_explicit_type() {
        // own a int32 = 5
        // own b int32 = a   (moves `a`)
        // own c int32 = a   (this must error because `a` already moved)
        let body = vec![
            var_decl("a", Type::Int32, Some(int32_lit(5))),
            var_decl("b", Type::Int32, Some(var_expr("a"))),
            var_decl("c", Type::Int32, Some(var_expr("a"))),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("moved"));
    }

    #[test]
    fn test_use_after_move_errors_infer_type() {
        // own a = 5
        // own b = a   (moves `a`)
        // own c = a   (this must error because `a` already moved)
        let body = vec![
            var_decl("a", Type::Infer, Some(int32_lit(5))),
            var_decl("b", Type::Infer, Some(var_expr("a"))),
            var_decl("c", Type::Infer, Some(var_expr("a"))),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("moved"));
    }

    #[test]
    fn test_use_after_move_errors_explicit_and_infer_type() {
        // own a int32 = 5
        // own b = a   (moves `a`)
        // own c int32 = a   (this must error because `a` already moved)
        let body = vec![
            var_decl("a", Type::Int32, Some(int32_lit(5))),
            var_decl("b", Type::Infer, Some(var_expr("a"))),
            var_decl("c", Type::Int32, Some(var_expr("a"))),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("moved"));
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
        // bar takes one int32.
        // own a T = Some Literal
        // bar(a)       (moves a)
        // own b T = a  (error)
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
        let callee = void_func("bar", vec![param("a", Type::Int32)], vec![]);
        let body = vec![Stmt::Expr(call_expr("bar", vec![bool_lit(true)]))];
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![callee, caller] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("type mismatch"));
    }

    #[test]
    fn test_correct_call_passes() {
        let callee = void_func("bar", vec![param("a", Type::Int32)], vec![]);
        let body = vec![Stmt::Expr(call_expr("bar", vec![int32_lit(42)]))];
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![callee, caller] };
        check_semantics(&mut ast).unwrap();
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
        // i starts from 3 up to 100k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..100000 {
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
        // i starts from 3 up to 100k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..100000 {
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

                let mut elements = vec![];

                for _ in 0..i+1 {
                    elements.push(l.clone());
                }
                
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

    #[test]
    fn test_array_valid_multiple_access_both_ends_passes() {

        // This is no black magic voodooo.. not too much of if..
        // This is just creating an array of dynamic sizes, and testing slicing it aka multiple
        // access
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 2..100 {
                let mut elements = vec![];

                for _ in 0..i+1 {
                    elements.push(l.clone());
                }
                
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
        for b in AllBinOpKindArth {
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
        for b in AllBinOpKindCompEq {
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
            for b in AllBinOpKindArth {
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
        assert!(result.unwrap_err().to_string().contains("float64"));
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
                for b in AllBinOpKindArth {
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
                for b in AllBinOpKindArth {
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
            for b in AllBinOpKindArth {
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

    // happy-path integration 

    #[test]
    fn test_full_valid_program() {
        // This program tests all integers / floating points and all arthemtic binary operations
        // it also tests variable declaration, function declaration, and function calling 
        let literals = get_all_literals_no_arr();

        for b in AllBinOpKindArth {
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
