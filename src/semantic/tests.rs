use super::*;
use crate::parser::{
    BinOpKind, VariableAssignment
};




#[cfg(test)]
mod tests {
    use super::*;

    // helper functions

    fn span() -> Span {
        Span { line: 1, column: 0 }
    }

    /// Build an AST that contains exactly one function.
    fn ast_one(func: Function) -> AST {
        AST { functions: vec![func] }
    }

    /// Build a void function (no return type) with the given body.
    fn void_func(name: &str, params: Vec<Param>, body: Vec<Stmt>) -> Function {
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

    fn int32_lit(n: i32) -> Expr {
        Expr::IntLiteral { value: IntLiteralValue::Int32(n), span: span() }
    }

    fn usize_lit(n: usize) -> Expr {
        Expr::IntLiteral { value: IntLiteralValue::Usize(n), span: span() }
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
        assert!(result.unwrap_err().to_string().contains("does not end with a `return`"));
    }

    #[test]
    fn test_return_in_void_function_errors() {
        // Void function that tries to return a value.
        let body = vec![return_stmt(vec![int32_lit(42)])];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no declared return type"));
    }

    // type inference tests

    #[test]
    fn test_infer_type_from_int32_literal() {
        // `own x = 5` type should be inferred as Int32
        let body = vec![var_decl("x", Type::Infer, Some(int32_lit(5)))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        // After check, the VarDecl type should be Int32
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert_eq!(v.type_name, Type::Int32);
        } else {
            panic!("expected VarDecl");
        }
    }

    #[test]
    fn test_infer_requires_initializer_or_explicit_type() {
        // `own x` with Infer type and no value must error
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
        // `own x bool = 5` is a type mismatch
        let body = vec![var_decl("x", Type::Bool, Some(int32_lit(5)))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Semantic error: Cannot assign integer literal to non-integer type `bool` (line 1 column 0) in function `foo`"));
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

    // move semantics 

    #[test]
    fn test_use_after_move_errors() {
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
    fn test_copy_call_allows_reuse() {
        // own a int32 = 5
        // own b int32 = copy(a)  (copies, does not move)
        // own c int32 = a        (valid, because no moves happened)
        let copy_a = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };
        let body = vec![
            var_decl("a", Type::Int32, Some(int32_lit(5))),
            var_decl("b", Type::Int32, Some(copy_a)),
            var_decl("c", Type::Int32, Some(var_expr("a"))),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    #[test]
    fn test_pass_variable_to_call_marks_it_moved() {
        // bar takes one int32.
        // own a int32 = 0
        // bar(a)       (moves a)
        // own b int32 = a  (error)
        let bar = void_func("bar", vec![param("p", Type::Int32)], vec![]);
        let body = vec![
            var_decl("a", Type::Int32, Some(int32_lit(0))),
            Stmt::Expr(call_expr("bar", vec![var_expr("a")])),
            var_decl("b", Type::Int32, Some(var_expr("a"))),
        ];
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![bar, caller] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("moved"));
    }

    // locking / unlocking variables

    #[test]
    fn test_assign_to_locked_variable_errors() {
        let body = vec![
            var_decl("x", Type::Int32, Some(int32_lit(1))),
            Stmt::Lock(vec![var_expr("x")]),
            Stmt::VarAssign(VariableAssignment {
                name: "x".to_string(),
                value: int32_lit(2),
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("locked"));
    }

    #[test]
    fn test_unlock_allows_reassign() {
        let body = vec![
            var_decl("x", Type::Int32, Some(int32_lit(1))),
            Stmt::Lock(vec![var_expr("x")]),
            Stmt::Unlock(vec![var_expr("x")]),
            Stmt::VarAssign(VariableAssignment {
                name: "x".to_string(),
                value: int32_lit(99),
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    #[test]
    fn test_double_lock_errors() {
        let body = vec![
            var_decl("x", Type::Int32, Some(int32_lit(0))),
            Stmt::Lock(vec![var_expr("x")]),
            Stmt::Lock(vec![var_expr("x")]),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already locked"));
    }

    #[test]
    fn test_unlock_unlocked_variable_errors() {
        let body = vec![
            var_decl("x", Type::Int32, Some(int32_lit(0))),
            Stmt::Unlock(vec![var_expr("x")]),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already unlocked"));
    }

    #[test]
    fn test_shadowing_locked_variable_errors() {
        let body = vec![
            var_decl("x", Type::Int32, Some(int32_lit(1))),
            Stmt::Lock(vec![var_expr("x")]),
            var_decl("x", Type::Int32, Some(int32_lit(2))),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("locked"));
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
        let callee = void_func("bar", vec![param("a", Type::Int32)], vec![]);
        let body = vec![Stmt::Expr(call_expr("bar", vec![]))]; // 0 args instead of 1
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![callee, caller] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expects 1 arguments, got 0"));
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
        // fn pair() -> (int32, bool) { return 1, true }
        // fn main() { own a int32, b bool = pair() }
        let pair_body = vec![return_stmt(vec![int32_lit(1), bool_lit(true)])];
        let pair = returning_func("pair", vec![], vec![Type::Int32, Type::Bool], pair_body);

        let vars = vec![
            Variable { name: "a".to_string(), type_name: Type::Int32, value: None, span: span() },
            Variable { name: "b".to_string(), type_name: Type::Bool, value: None, span: span() },
        ];
        let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
        let main = void_func("main", vec![], body);

        let mut ast = AST { functions: vec![pair, main] };
        check_semantics(&mut ast).unwrap();
    }

    #[test]
    fn test_multi_return_count_mismatch_errors() {
        // pair returns 2 values; we only bind 1
        let pair_body = vec![return_stmt(vec![int32_lit(1), bool_lit(true)])];
        let pair = returning_func("pair", vec![], vec![Type::Int32, Type::Bool], pair_body);

        let vars = vec![
            Variable { name: "a".to_string(), type_name: Type::Int32, value: None, span: span() },
        ];
        let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
        let main = void_func("main", vec![], body);

        let mut ast = AST { functions: vec![pair, main] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Return length mismatch"));
    }

    // basic array out-of-bounds checks

    #[test]
    fn test_array_out_of_bounds_single_access_errors() {
        // own arr int32[] = [0, 1]
        // own x int32 = arr[3]  (out of bounds)
        let arr_lit = Expr::ArrayLiteral {
            elements: vec![int32_lit(0), int32_lit(1), int32_lit(2)],
            array_ty: Type::Int32,
            span: span(),
        };
        let access = Expr::ArraySingleAccess {
            array: Box::new(var_expr("arr")),
            index: Box::new(usize_lit(5)),
            span: span(),
        };
        let body = vec![
            var_decl("arr", Type::Array(Box::new(Type::Int32)), Some(arr_lit)),
            var_decl("x", Type::Int32, Some(access)),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out-of-bounds"));
    }

    #[test]
    fn test_array_valid_access_passes() {
        let arr_lit = Expr::ArrayLiteral {
            elements: vec![int32_lit(10), int32_lit(20), int32_lit(30)],
            array_ty: Type::Int32,
            span: span(),
        };
        let access = Expr::ArraySingleAccess {
            array: Box::new(var_expr("arr")),
            index: Box::new(usize_lit(2)),
            span: span(),
        };
        let body = vec![
            var_decl("arr", Type::Array(Box::new(Type::Int32)), Some(arr_lit)),
            var_decl("x", Type::Int32, Some(access)),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    #[test]
    fn test_array_slice_start_greater_than_end_errors() {
        let arr_lit = Expr::ArrayLiteral {
            elements: vec![int32_lit(1), int32_lit(2), int32_lit(3), int32_lit(4)],
            array_ty: Type::Int32,
            span: span(),
        };
        let slice = Expr::ArrayMultipleAccess {
            array: Box::new(var_expr("arr")),
            start: Some(Box::new(usize_lit(3))),
            end: Some(Box::new(usize_lit(1))),
            span: span(),
        };
        let body = vec![
            var_decl("arr", Type::Array(Box::new(Type::Int32)), Some(arr_lit)),
            var_decl("s", Type::Array(Box::new(Type::Int32)), Some(slice)),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Start index"));
    }

    // unary operations 

    #[test]
    fn test_negate_unsigned_errors() {
        let neg = Expr::UnaryOp {
            op: UnaryOpKind::Negate,
            expr: Box::new(Expr::IntLiteral { value: IntLiteralValue::Usize(5), span: span() }),
            span: span(),
        };
        let body = vec![var_decl("x", Type::Usize, Some(neg))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("negate"));
    }

    #[test]
    fn test_negate_signed_passes() {
        let neg = Expr::UnaryOp {
            op: UnaryOpKind::Negate,
            expr: Box::new(int32_lit(7)),
            span: span(),
        };
        let body = vec![var_decl("x", Type::Int32, Some(neg))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    // string concatantion via binary operation
    //
    #[test]
    fn test_string_binop_errors() {
        // Strings may not be added with `+`, we use format() instead.
        let bin = Expr::BinOp {
            left: Box::new(str_lit("hello")),
            op: crate::parser::BinOpKind::Add,
            right: Box::new(str_lit(" world")),
            span: span(),
        };
        let body = vec![var_decl("s", Type::String, Some(bin))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("concatnate strings"));
    }

    // integer literal inference 

    #[test]
    fn test_integer_literal_inferred_to_int8() {
        // own x int8 = 100  (fits in i8)
        let lit = Expr::IntLiteral { value: IntLiteralValue::Int32(100), span: span() };
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
        let lit = Expr::IntLiteral { value: IntLiteralValue::Int32(200), span: span() };
        let body = vec![var_decl("x", Type::Int8, Some(lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range"));
    }

    #[test]
    fn test_float32_cannot_accept_float64_literal_errors() {
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
    fn test_binop_mixed_int_types_errors() {
        // int32 + int64 is not allowed (we don't allow any different types mixing in expressions)
        let bin = Expr::BinOp {
            left: Box::new(int32_lit(1)),
            right: Box::new(Expr::IntLiteral { value: IntLiteralValue::Int64(2), span: span() }),
            op: crate::parser::BinOpKind::Add,
            span: span(),
        };
        let body = vec![var_decl("x", Type::Infer, Some(bin))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Type mismatch"));
    }

    // copy call guards 

    #[test]
    fn test_copy_of_literal_errors() {
        let copy_lit = Expr::CopyCall { expr: Box::new(int32_lit(5)), span: span() };
        let body = vec![var_decl("x", Type::Int32, Some(copy_lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Copying a literal"));
    }

    #[test]
    fn test_double_copy_errors() {
        let body = vec![
            var_decl("a", Type::Int32, Some(int32_lit(1))),
            var_decl("b", Type::Int32, Some(
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
        assert!(result.unwrap_err().to_string().contains("Double copying"));
    }

    // undeclared variable usage tests

    #[test]
    fn test_use_of_undeclared_variable_errors() {
        let body = vec![var_decl("x", Type::Int32, Some(var_expr("y")))]; // y not declared
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("undeclared variable"));
    }

    // params are in scope

    #[test]
    fn test_params_are_in_scope() {
        // fn foo(n int32) -> int32 { return n }
        let body = vec![return_stmt(vec![var_expr("n")])];
        let func = returning_func("foo", vec![param("n", Type::Int32)], vec![Type::Int32], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    // format call guards 
    #[test]
    fn test_format_call_with_literal_errors() {
        let fmt = Expr::FormatCall {
            template: "value: {}".to_string(),
            expressions: vec![int32_lit(42)], // plain literal not allowed
            span: span(),
        };
        let body = vec![var_decl("s", Type::String, Some(fmt))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Plain literals are not allowed"));
    }

    #[test]
    fn test_format_call_with_variable_passes() {
        let fmt = Expr::FormatCall {
            template: "value: {}".to_string(),
            expressions: vec![var_expr("n")],
            span: span(),
        };
        let body = vec![
            var_decl("n", Type::Int32, Some(int32_lit(7))),
            var_decl("s", Type::String, Some(fmt)),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
    }

    // happy-path integration 

    #[test]
    fn test_full_valid_program() {
        // fn add(a int32, b int32) -> int32 { return a + b }
        // fn main() { own r int32 = add(3, 4) }
        let add_body = vec![return_stmt(vec![
            Expr::BinOp {
                left: Box::new(var_expr("a")),
                op: crate::parser::BinOpKind::Add,
                right: Box::new(var_expr("b")),
                span: span(),
            }
        ])];
        let add = returning_func(
            "add",
            vec![param("a", Type::Int32), param("b", Type::Int32)],
            vec![Type::Int32],
            add_body,
        );

        let main_body = vec![
            var_decl("r", Type::Int32, Some(call_expr("add", vec![int32_lit(3), int32_lit(4)]))),
        ];
        let main = void_func("main", vec![], main_body);

        let mut ast = AST { functions: vec![add, main] };
        check_semantics(&mut ast).unwrap();
    }
}
