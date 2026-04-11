use super::*;
use crate::parser::{
    Param, Variable, VariableAssignment, MultiAssignment, 
    IfStmt, WhileStmt, ForStmt, InfiniteStmt, BreakStmt, ContinueStmt
};


use crate::tests_consts::{
    ALL_TYPES_NO_ARR, ALL_TYPES_NO_ARR_SCATTERED, ALL_TYPES_NO_ARR_NO_USIZE, ALL_TYPES_NO_ARR_NO_INFER, ALL_TYPES_NO_INTS_NO_ARR,
    ALL_TYPES_NO_INTS_NO_ARR_NO_INFER,

    ALL_INT_TYPES_NO_ARR_NO_INFER,
    ALL_FLOATS_TYPES,

    ALL_UNSIGNED_TYPES_NO_ARR, ALL_SIGNED_TYPES_NO_ARR,
    ALL_BIN_OP_KIND_ARTH, ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ,
    ALL_BIN_OP_KIND_REAL_ARTH, ALL_BIN_OP_KIND_BIT_ARTH,

    ALL_BIN_OP_KIND,
    ALL_BIN_OP_KIND_LOGIC,
    ALL_BIN_OP_KIND_COMP_ARTH

};


// helper functions




fn get_all_literals_no_arr_bool() -> [Expr; 14] {
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

        str_lit("Hi")
    ];

    return literals;
}

fn get_all_float_literals_no_arr() -> [Expr; 2] {
    let literals = [
        float32_lit(1.0),
        float64_lit(1.0),
    ];

    return literals;
}


fn get_all_literals_no_arr_no_ints() -> [Expr; 4] {
    let literals = [

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}



fn get_all_literals_no_arr_few_ints() -> [Expr; 6] {
    let literals = [
        uint128_lit(1),
        int128_lit(1),

        float32_lit(1.0),
        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}


fn get_all_literals_no_arr_few_ints_scattered() -> [Expr; 6] {
    let literals = [
        str_lit("Hi"),
        float32_lit(1.0),

        int128_lit(1),
        bool_lit(false),
        float64_lit(1.0),
        uint128_lit(1),
    ];

    return literals;
}



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




fn get_all_literals_no_arr_str_bool_float() -> [Expr; 11] {
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


fn var_assign(name: &str, value: Expr) -> Stmt {
    Stmt::VarAssign(VariableAssignment {
        name: name.to_string(),
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


    // Empty functions are not allowed
    #[test]
    fn test_empty_functions_errors() {
        let mut ast = AST {
            functions: vec![
                Function {
                    name: "foo".to_string(),
                    params: vec![],
                    return_type: None,
                    body: vec![

                    ],
                    span: span(),
                }
            ]
        };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("has no statements, empty functions are not allowed!"));
    }


    // duplicate functions are not allowed
    #[test]
    fn test_duplicate_function_name_errors() {
        let f1 = void_func("foo", vec![], vec![]);
        let f2 = void_func("foo", vec![], vec![]);
        let mut ast = AST { functions: vec![f1, f2] };
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate function"));
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


    #[test]
    fn test_non_returning_func_in_expr_errors() {
        let callee = void_func("bar", vec![], vec![]);
        let body = vec![
            var_decl("x", Type::Infer, Some(call_expr("bar", vec![])))
        ];
        let caller = void_func("main", vec![], body);
        let mut ast = AST { functions: vec![callee, caller] };

        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("has no return type declared but is used in an expression"));
    }



    // variables declaration / assignment, and type inference tests

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
                assert_eq!(v.value, Some(l.clone()));
            } else {
                panic!("expected VarDecl");
            }
        }
    }



    #[test]
    fn test_variable_name_taken_by_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let main = void_func("main", vec![], vec![
                var_decl("foo", t.clone(), Some(call_expr("foo", vec![]))),
            ]);

            let foo = void_func("foo", vec![], vec![]);

            let mut ast = AST { functions: vec![main, foo] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());        
            assert!(result.unwrap_err().to_string().contains("Name `foo` is already taken by a function, pick a different name for your variable."));
        }
    }

    #[test]
    fn test_vardecl_uses_non_declared_var_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![var_decl("x", t.clone(), Some(var_expr("y")))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `y`"));
        }
    }


    #[test]
    fn test_vardecl_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),
                var_decl("z", t.clone(), Some(var_expr("x")))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }

    // This test is duplicated but not really.. other tests dont test it all way through.
    #[test]
    fn test_var_decl_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();

        for t in ALL_INT_TYPES_NO_ARR_NO_INFER {
            for l in &literals_no_ints {
                let body = vec![var_decl("x", t.clone(), Some(l.clone()))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());        
                assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
            }
        }
    }



    // Tests the rule:
    // You cannot move an upstream variable multiple times inside a loop.
    //

    #[test]
    fn test_vardecl_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }


    #[test]
    fn test_vardecl_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_vardecl_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl("y", t.clone(), Some(var_expr("x")))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }








    // Tests the rule: 
    // You cannot overshadow variables declared in an upstream scope
    //

    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_for_loop_holder_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
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

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Cannot use variable name `x` in for loop statement as it is already declared"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }

    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }



    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_if_main_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        var_decl("x", t.clone(), Some(l.clone()))
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }


    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_if_else_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),

                    ],
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        var_decl("x", t.clone(), Some(l.clone()))
                    ]),
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }


    #[test]
    fn test_vardecl_overshadowing_upstream_var_in_if_elif_branch_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::If(IfStmt{
                    condition: bool_lit(false),
                    if_branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    elif_branches: vec![
                        (bool_lit(false), vec![
                            var_decl("x", t.clone(), Some(l.clone()))
                        ])
                    ],
                    else_branch: None,
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("you cannot overshadow upstream variables"));
        }
    }







    // Similar tests to above tests, but this for varaible assignment.
    //

    #[test]
    fn test_varassign_assign_to_self_doesnt_move() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), None),
                var_assign("x", var_expr("x")),
                var_assign("x", l.clone()),

            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_ok());
        }
    }




    #[test]
    fn test_varassign_type_mismatch_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;
        
        for l in literals_ints {
            for t in ALL_TYPES_NO_INTS_NO_ARR_NO_INFER {
                let body = vec![
                    var_decl("x", t.clone(), None),
                    var_assign("x", l.clone())
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot assign"));
            }
        }
    }


    #[test]
    fn test_varassign_to_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),

                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `x`"));
        }
    }



    #[test]
    fn test_varassign_uses_non_declared_var_errors() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![
                var_decl("x", t.clone(), None), 
                var_assign("x", var_expr("y"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `y`"));
        }
    }


    #[test]
    fn test_varassign_uses_moved_var_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_INFER.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("y", t.clone(), Some(var_expr("x"))),
                var_decl("z", t.clone(), None),
                var_assign("z", var_expr("x"))
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of moved variable `x`"));
        }
    }



    // Tests the rule:
    // You cannot move an upstream variable multiple times inside a loop.
    //

    #[test]
    fn test_varassign_moving_upstream_var_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }

    #[test]
    fn test_varassign_moving_upstream_var_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }


    #[test]
    fn test_varassign_moving_upstream_var_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "e".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            var_decl("y", t.clone(), Some(l.clone())),
                            var_assign("y", var_expr("x"))
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("variable `x` is potentially moved multiple times"));
        }
    }







    #[should_panic(expected = "Compiler bug")]
    #[test]
    fn test_infer_requires_initializer_or_explicit_type() {
        // Variables declared with Infer type and no value should've been caught by parser phase
        // but if it i didn't, semantic should always panic.
        let body = vec![var_decl("x", Type::Infer, None)];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let _ = check_semantics(&mut ast);
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
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
        }


        for l in literals_ints_floats {
            // Variables declared with explicit type of string, but given an non-string literal is a type mismatch
            let body = vec![var_decl("x", Type::String, Some(l.clone()))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch assigning to"));
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


    #[test]
    fn test_default_nested_array_is_empty() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            for i in 1..=200 {
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
                var_assign("x", l.clone())
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
    fn test_unlock_non_var_expr_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Unlock(vec![l.clone()]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected variable name, instead got"));
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
                var_assign("x", l.clone())
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
        }
    }

    #[test]
    fn test_unlock_upstream_variable_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            Stmt::Unlock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_unlock_upstream_variable_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Unlock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_unlock_upstream_variable_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };


            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),

                Stmt::For(ForStmt{
                    holder_name: "i".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        Stmt::Unlock(vec![var_expr("x")]),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot unlock variable `x` because it is declared upstream"));
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
    fn test_lock_non_var_expr_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![l.clone()]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected variable name, instead got"));
        }
    }



    #[test]
    fn test_lock_repeated_var_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Lock(vec![var_expr("x"), var_expr("x")]),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Lock arguments have duplicated variable"));
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



    #[test]
    fn test_lock_upstream_variable_in_while_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::While(WhileStmt{
                        condition: bool_lit(false),
                        branch: vec![
                            Stmt::Lock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_lock_upstream_variable_in_infinite_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::Lock(vec![var_expr("x")]),
                        ],
                        span: span(),
                    }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }


    #[test]
    fn test_lock_upstream_variable_in_for_loop_errors() {
        let literals = get_all_literals_no_arr();
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };


            let body = vec![
                var_decl("x", t.clone(), Some(l.clone())),
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),

                Stmt::For(ForStmt{
                    holder_name: "i".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        Stmt::Lock(vec![var_expr("x")]),
                    ],
                    span: span(),
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("You cannot lock variable `x` because it is declared upstream"));
        }
    }







    // Test for statements with array variables, no literals.
    #[test]
    fn test_for_statements_with_arrays() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral {
                    elements: elements.clone(),
                    array_ty: t.clone(),
                    span: span(),
                };

                let body = vec![ 
                    var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
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
                let result = check_semantics(&mut ast);

                assert!(result.is_ok())
            }
        }
    }


    // Test for statements with rangecall, with only integer literals, no variables.
    #[test]
    fn test_for_statements_with_range_int_literals() {
        let literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
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
            let result = check_semantics(&mut ast);

            assert!(result.is_ok())
        }
    }


    #[test]
    fn test_for_statements_with_range_non_int_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();


        for (l, t) in literals_no_ints.iter().zip(ALL_TYPES_NO_INTS_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
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
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected range arguments to be any Integer type"));
        }
    }


    #[test]
    fn test_for_statements_with_range_mixed_literals_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();
        let literals = get_all_literals_no_arr();



        for ((l, t), l2) in literals_no_ints.iter()
            .zip(ALL_TYPES_NO_INTS_NO_ARR.iter())
            .zip(literals)
        {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l2.clone()),
                        span: span()
                    },
                    
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
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected range arguments to be of the same type"));
        }
    }



    #[test]
    fn test_for_statements_with_range_holder_name_is_already_taken_errors() {
        let literals = get_all_literals_no_arr_str_bool_float();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                var_decl("x", t.clone(), Some(l.clone())),
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    
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
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Cannot use variable name `x` in for loop statement as it is already declared"));
        }
    }


    #[test]
    fn test_for_statements_with_array_holder_name_is_already_taken_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral {
                    elements: elements.clone(),
                    array_ty: t.clone(),
                    span: span(),
                };

                let body = vec![ 
                    var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                    var_decl("x", t.clone(), Some(l.clone())),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),

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
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Cannot use variable name `x` in for loop statement as it is already declared"));
            }
        }
    }


    #[test]
    fn test_for_statements_with_no_array_no_range() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter())
        {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: l.clone(),
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
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("For loop statement require an expression to be evaulatable to any `Array` type"));
        }
    }



    #[test]
    fn test_infinite_statements_pass() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![ 
                Stmt::Infinite(InfiniteStmt{
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

            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
        }
    }

    // Ensure infinite loops empty branches not allowed
    #[test]
    fn test_infinite_statements_empty_branch_errors() {
        let body = vec![ 
            Stmt::Infinite(InfiniteStmt{
                branch: vec![],
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);

        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Infinite loop branch has no statements"));
    }


    // Ensure while loops empty branches are not allowed
    #[test]
    fn test_while_statements_empty_branch_errors() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in literals_ints_floats {
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
                        branch: vec![],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("While loop branch has no statements"));
            }
        }
    }


    #[test]
    fn test_for_statements_empty_branch_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i];

                let arr_lit = Expr::ArrayLiteral {
                    elements: elements.clone(),
                    array_ty: t.clone(),
                    span: span(),
                };

                let body = vec![ 
                    var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                    Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),

                        branch: vec![],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("For loop branch has no statements"));
            }
        }
    }







    // Tests while statements without booleans, or binop, or anything that could be evaluated to
    // bool, is an error.
    #[test]
    fn test_while_statements_no_bool_eval_expr_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            // Skip bools
            if *t == Type::Bool {
                continue
            }
            let body = vec![ 
                Stmt::While(WhileStmt{
                    condition: l.clone(),
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("While statement require an expression to be evaulatable to type `bool`"));
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


    #[test]
    fn test_break_statement_no_loop_errors() {
        let body = vec![ 
            Stmt::Break(BreakStmt{
                span: span()
            })
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
    }

    #[test]
    fn test_break_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in literals_ints_floats {
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
                            Stmt::Break(BreakStmt{
                                span: span()
                            }),
                        ],
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

    #[test]
    fn test_break_statement_in_if_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition.clone(),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition.clone(),
                                if_branch: vec![
                                    Stmt::Break(BreakStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::While(ws) = &ast.functions[0].body[0] {
                    assert_eq!(ws.condition, condition.clone());
                    assert_eq!(ws.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &ws.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Break(_)), "Expected break statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected While loop statement") }
            }
        }
    }







    #[test]
    fn test_break_statement_outside_while_statements_errors() {
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
                    Stmt::Break(BreakStmt{
                        span: span()
                    }),

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

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
            }
        }
        // Same test, but the `break` is after the while loop

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
                    Stmt::Break(BreakStmt{
                        span: span()
                    }),


                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
            }
        }
    }

    #[test]
    fn test_break_statement_in_infinite_statements() {
        let body = vec![ 
            Stmt::Infinite(InfiniteStmt{
                branch: vec![
                    Stmt::Break(BreakStmt{
                        span: span()
                    }),
                ],
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_ok());
        if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
            assert_eq!(infs.branch.len(), 1);
        
            assert!( matches!(infs.branch[0], Stmt::Break(_)), "Expected break statement");

        } else { panic!("Expected Infinite loop statement") }
    }

    #[test]
    fn test_break_statement_in_if_statement_in_infinite_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition,
                                if_branch: vec![
                                    Stmt::Break(BreakStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
                    assert_eq!(infs.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &infs.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Break(_)), "Expected break statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected Infinite loop statement") }
            }
        }
    }





    #[test]
    fn test_break_statement_outside_infinite_statements_errors() {

        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![ 
                Stmt::Break(BreakStmt{
                    span: span()
                }),

                Stmt::Infinite(InfiniteStmt{
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }

        // Same test, but the `break` is after the infinite loop
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![ 
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),

                    ],
                    span: span(),
                }),
                Stmt::Break(BreakStmt{
                    span: span()
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }
    }



    //
    #[test]
    fn test_break_statement_in_for_statement_with_arr() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::Break(BreakStmt{
                                span: span()
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok());

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, Some(arr_lit));

            } else { panic!("Expected VarDecl statement") }


            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Break(_)), "Expected break statement");

            } else { panic!("Expected For loop statement") }
        }
    }

    #[test]
    fn test_break_statement_in_if_statement_in_for_statements_with_arr() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: bool_lit(false),
                                if_branch: vec![
                                    Stmt::Break(BreakStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, Some(arr_lit));

            } else { panic!("Expected VarDecl statement") }

            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Break(_)), "Expected break statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }






    #[test]
    fn test_break_statement_outside_for_statements_with_arr_errors() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                Stmt::Break(BreakStmt{
                    span: span()
                }),

                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }

        // Same test, but the `break` is after the infinite loop
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    span: span(),
                }),
                Stmt::Break(BreakStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }
    }


    // Same as above for statement tests, but this time with RangeCall
    //


    
    #[test]
    fn test_break_statement_in_for_statement_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::Break(BreakStmt{
                            span: span()
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok());

            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }

                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Break(_)), "Expected break statement");
            } else { panic!("Expected For statement") }
        }
    }


    #[test]
    fn test_break_statement_in_if_statement_in_for_statements_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: bool_lit(false),
                            if_branch: vec![
                                Stmt::Break(BreakStmt{
                                    span: span()
                                }),
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());


            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Break(_)), "Expected break statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }






    #[test]
    fn test_break_statement_outside_for_statements_with_range_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::Break(BreakStmt{
                    span: span()
                }),

                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }

        // Same test, but the `break` is after the infinite loop

        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![  
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    span: span(),
                }),
                Stmt::Break(BreakStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Break can only be used in loops"));
        }
    }















    #[test]
    fn test_continue_statement_no_loop_errors() {
        let body = vec![ 
            Stmt::Continue(ContinueStmt{
                span: span()
            })
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
    }




    #[test]
    fn test_continue_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for l in literals_ints_floats {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition.clone(),
                        branch: vec![
                            Stmt::Continue(ContinueStmt{
                                span: span()
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());

                if let Stmt::While(ws) = &ast.functions[0].body[0] {
                    assert_eq!(ws.condition, condition);
                    assert_eq!(ws.branch.len(), 1);

                    assert!( matches!(ws.branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected While loop statement") }

            }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_while_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::While(WhileStmt{
                        condition: condition.clone(),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition.clone(),
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::While(ws) = &ast.functions[0].body[0] {
                    assert_eq!(ws.condition, condition.clone());
                    assert_eq!(ws.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &ws.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected While loop statement") }
            }
        }
    }





    #[test]
    fn test_continue_statement_outside_while_statements_errors() {
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
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),

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

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
            }
        }
        // Same test, but the `continue` is after the while loop

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
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),


                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);

                let result = check_semantics(&mut ast);

                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
            }
        }
    }

    #[test]
    fn test_continue_statement_in_infinite_statements() {
        let body = vec![ 
            Stmt::Infinite(InfiniteStmt{
                branch: vec![
                    Stmt::Continue(ContinueStmt{
                        span: span()
                    }),
                ],
                span: span(),
            }),
        ];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);

        assert!(result.is_ok());
        if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
            assert_eq!(infs.branch.len(), 1);
        
            assert!( matches!(infs.branch[0], Stmt::Continue(_)), "Expected continue statement");

        } else { panic!("Expected Infinite loop statement") }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_infinite_statements() {
        let literals_ints_floats = get_all_literals_no_arr_str_bool();

        for (l, _) in literals_ints_floats.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_COMP {
                let condition = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };

                let body = vec![ 
                    Stmt::Infinite(InfiniteStmt{
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: condition,
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());

                if let Stmt::Infinite(infs) = &ast.functions[0].body[0] {
                    assert_eq!(infs.branch.len(), 1);
                
                    if let Stmt::If(ifstm) = &infs.branch[0] {
                        assert_eq!(ifstm.if_branch.len(), 1);
                        assert_eq!(ifstm.elif_branches.len(), 0);
                        assert_eq!(ifstm.else_branch, None);

                        assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                    } else { panic!("Expected If statement") }

                } else { panic!("Expected Infinite loop statement") }
            }
        }
    }



    #[test]
    fn test_continue_statement_outside_infinite_statements_errors() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![ 
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                Stmt::Infinite(InfiniteStmt{
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![ 
                Stmt::Infinite(InfiniteStmt{
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),

                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }
    }



    #[test]
    fn test_continue_statement_in_for_statement_with_arr() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::Continue(ContinueStmt{
                                span: span()
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok());

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, Some(arr_lit));

            } else { panic!("Expected VarDecl statement") }


            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Continue(_)), "Expected continue statement");

            } else { panic!("Expected For loop statement") }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_for_statements_with_arr() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
                        branch: vec![
                            Stmt::If(IfStmt{
                                condition: bool_lit(false),
                                if_branch: vec![
                                    Stmt::Continue(ContinueStmt{
                                        span: span()
                                    }),
                                ],
                                elif_branches: vec![],
                                else_branch: None,
                                span: span(),
                            }),
                        ],
                        span: span(),
                    }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());

            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, Type::Array(Box::new(t.clone())) );
                assert_eq!(v.value, Some(arr_lit));

            } else { panic!("Expected VarDecl statement") }

            if let Stmt::For(fs) = &ast.functions[0].body[1] {
                assert_eq!(fs.holder_name, "x");
                assert_eq!(fs.value, var_expr("a"));
                assert_eq!(fs.branch.len(), 1);
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }






    #[test]
    fn test_continue_statement_outside_for_statements_with_arr_errors() {
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                        holder_name: "x".to_string(),
                        value: var_expr("a"),
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![],
                array_ty: t.clone(),
                span: span(),
            };

            let body = vec![
                var_decl("a", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: var_expr("a"),
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }
    }


    // Same as above for statement tests, but this time with RangeCall
    //


    
    #[test]
    fn test_continue_statement_in_for_statement_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::Continue(ContinueStmt{
                            span: span()
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);

            assert!(result.is_ok());

            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }

                assert_eq!(fs.branch.len(), 1);
                assert!( matches!(fs.branch[0], Stmt::Continue(_)), "Expected continue statement");
            } else { panic!("Expected For statement") }
        }
    }


    #[test]
    fn test_continue_statement_in_if_statement_in_for_statements_with_range() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        Stmt::If(IfStmt{
                            condition: bool_lit(false),
                            if_branch: vec![
                                Stmt::Continue(ContinueStmt{
                                    span: span()
                                }),
                            ],
                            elif_branches: vec![],
                            else_branch: None,
                            span: span(),
                        }),
                    ],
                    span: span(),
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_ok());


            if let Stmt::For(fs) = &ast.functions[0].body[0] {
                assert_eq!(fs.holder_name, "x");

                if let Expr::RangeCall { start, end, .. } = &fs.value {
                    assert!(matches!(start.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                    assert!(matches!(end.as_ref(), Expr::IntLiteral { value, .. } if value.get_type() == t.clone()));
                } else { panic!("Expected RangeCall expression, instead got {:?}", fs.value) }
            
                if let Stmt::If(ifstm) = &fs.branch[0] {
                    assert_eq!(ifstm.if_branch.len(), 1);
                    assert_eq!(ifstm.elif_branches.len(), 0);
                    assert_eq!(ifstm.else_branch, None);

                    assert!( matches!(ifstm.if_branch[0], Stmt::Continue(_)), "Expected continue statement");

                } else { panic!("Expected If statement") }

            } else { panic!("Expected For loop statement") }
        }
    }






    #[test]
    fn test_continue_statement_outside_for_statements_with_range_errors() {
        let literals_ints = get_all_literals_no_arr_str_bool_float();
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![ 
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),

                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
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

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
        }

        // Same test, but the `continue` is after the infinite loop

        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let body = vec![  
                Stmt::For(ForStmt{
                    holder_name: "x".to_string(),
                    value: Expr::RangeCall{
                        start: Box::new(l.clone()),
                        end: Box::new(l.clone()),
                        span: span()
                    },
                    branch: vec![
                        // Just dummy declaration, so we don't get flagged by dead code because
                        // of empty branch.
                        var_decl("z", t.clone(), None),
                    ],
                    span: span(),
                }),
                Stmt::Continue(ContinueStmt{
                    span: span()
                }),
            ];

            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Continue can only be used in loops"));
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
                let result = check_semantics(&mut ast);

                assert!(result.is_ok());
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
    fn test_call_wrong_return_arity_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            let callee = returning_func("bar", vec![], vec![t.clone(), t.clone()], vec![
                return_stmt(vec![l.clone(), l.clone()])
            ]);
            let body = vec![
                var_decl("x", t.clone(), Some(
                            call_expr("bar", vec![])
                        )
                )
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Call to function `bar` returns 2 values but is used in a single-value expression"));
        }
    }


    #[test]
    fn test_call_assign_from_non_returning_func_errors() {
        for t in ALL_TYPES_NO_ARR {
            let callee = void_func("bar", vec![], vec![]);
            let body = vec![
                var_decl("x", t.clone(), Some(
                            call_expr("bar", vec![])
                        )
                )
            ];
            let caller = void_func("main", vec![], body);
            let mut ast = AST { functions: vec![callee, caller] };

            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("has no return type declared but is used in an expression"));
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
    // with multi-assignments
    #[test]
    fn test_multi_return_assign_correct() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { 
        //  own a t1
        //  own b t2
        //  a, b = pair() 
        //  }

        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            check_semantics(&mut ast).unwrap();

            if let Stmt::VarDecl(v) = &ast.functions[1].body[0] {
                assert_eq!(v.name, "a");
                assert_eq!(v.type_name, t1.clone());
                assert_ne!(v.value, None);
            } else { panic!("Expected VarDecl") }


            if let Stmt::VarDecl(v) = &ast.functions[1].body[1] {
                assert_eq!(v.name, "b");
                assert_eq!(v.type_name, t2.clone());
                assert_ne!(v.value, None);
            } else { panic!("Expected VarDecl") }

            if let Stmt::VarAssignMulti(ma) = &ast.functions[1].body[2] {
                assert_eq!(ma.names.len(), 2, "Expected 2 variable names");
                assert_eq!(ma.names[0], "a");
                assert_eq!(ma.names[1], "b");

                if let Expr::Call { name, .. } = &ma.value {
                    assert_eq!(name, "pair");
                } else { panic!("Expected Call expression, instead got {:?}", ma.value) }

            } else { panic!("Expected VarAssignMulti") }
        }
    }


    #[test]
    fn test_multi_return_assign_type_mismatch_errors() {
        let literals = get_all_literals_no_arr();


        // a is mismatch, b is correct
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t2.clone(), None),
                var_decl("b", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));

        }

        // now b is mismatched while a is correct

        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));

        }



        // Both mismatched
    
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let body = vec![
                var_decl("a", t2.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));

        }
    }



    #[test]
    fn test_multi_assign_func_not_return_errors() {
        for (t1, t2) in ALL_TYPES_NO_ARR_NO_INFER.iter().zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair = void_func("pair", vec![], vec![]);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Function `pair` has no return type declared but is used in an expression"));

        }
    }


    #[test]
    fn test_multi_assign_undeclared_vars_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `b`"));
        }


        // Same as above, but `a` is undeclared instead of `b`
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("b", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `a`"));
        }


        // Same as above, but both are undeclared
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Use of undeclared variable `b`"));
        }
    }


    #[test]
    fn test_multi_assign_use_of_moved_vars_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                var_decl("c", t1.clone(), Some(var_expr("a"))),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `a`"));
        }


        // Same as above, but this time we move "b" instead of "a"
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                var_decl("c", t2.clone(), Some(var_expr("b"))),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `b`"));
        }


        // Same as above, but this time we move both "a" and "b"
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                var_decl("c", t1.clone(), Some(var_expr("a"))),
                var_decl("d", t2.clone(), Some(var_expr("b"))),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `a`"));
            // assert!(result.unwrap_err().to_string().contains("Value assignment to moved variable `b`"));
        }
    }


    #[test]
    fn test_multi_assign_locked_vars_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::Lock(vec![var_expr("a")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
        }


        // Same as above, but this locks "b" instead of "a"
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::Lock(vec![var_expr("b")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }

        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::Lock(vec![var_expr("a"), var_expr("b")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
            // assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }
    }


    #[test]
    fn test_multi_assign_multi_return_not_func_call_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();


        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),

                Stmt::Lock(vec![var_expr("a")]),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string()],
                    value: l1.clone(),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Multi-assignment requires only a single function call on the right-hand side"));
        }
    }




    #[test]
    fn test_multi_assign_return_count_mismatch_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));

        }



        // Same test b ut this time extra variable
        for (((l1, t1), l2), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(literals_scattered.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l2.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t2.clone()], pair_body);

            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t2.clone(), None),
                var_decl("c", t2.clone(), None),

                Stmt::VarAssignMulti(MultiAssignment{
                    names: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    value: call_expr("pair", vec![]),
                    span: span()
                })
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Return length mismatch"));

        }


    }

     






    // return statement with multiple values (aka multi-return)
    // with multi-declaration
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

            if let Stmt::VarDeclMulti(vs, ce) = &ast.functions[1].body[0] {
                assert_eq!(vs.len(), 2, "Expected 2 variable declarations");
                assert_eq!(vs[0].type_name, t1.clone());
                assert_eq!(vs[1].type_name, t2.clone());
            
                if let Expr::Call { name, .. } = ce {
                    assert_eq!(name, "pair");
                } else { panic!("Expected Call expression, instead got {:?}", ce) }

            } else { panic!("Expected VarDecl") }
        }
    }


    // Same as above test, but this lets variables infer their types from return types
    #[test]
    fn test_multi_return_decl_infer_correct() {
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
                Variable { name: "a".to_string(), type_name: Type::Infer, value: None, span: span() },
                Variable { name: "b".to_string(), type_name: Type::Infer, value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            check_semantics(&mut ast).unwrap();

        
            if let Stmt::VarDeclMulti(vs, ce) = &ast.functions[1].body[0] {
                assert_eq!(vs.len(), 2, "Expected 2 variable declarations");
                assert_eq!(vs[0].type_name, t1.clone());
                assert_eq!(vs[1].type_name, t2.clone());
            
                if let Expr::Call { name, .. } = ce {
                    assert_eq!(name, "pair");
                } else { panic!("Expected Call expression, instead got {:?}", ce) }

            } else { panic!("Expected VarDecl") }
        }
    }

    #[test]
    fn test_multi_return_decl_one_variable_locked_errors() {
        // func pair() (t1, t2,) { return l1, l2 }
        // func main() { 
        // own a t1
        // lock a
        // own a, b = pair() }

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
            let body = vec![
                var_decl("a", t1.clone(), None),
                Stmt::Lock(vec![var_expr("a")]),

                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
        }



        // Same test, but this time `b` is locked instead
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
            let body = vec![
                var_decl("b", t1.clone(), None),
                Stmt::Lock(vec![var_expr("b")]),

                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }
    }


    // Same test, but both `a` and `b` are locked
    #[test]
    fn test_multi_return_decl_two_variables_locked_errors() {
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
            let body = vec![
                var_decl("a", t1.clone(), None),
                var_decl("b", t1.clone(), None),
                Stmt::Lock(vec![var_expr("a"), var_expr("b")]),

                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is locked"));
            // assert!(result.unwrap_err().to_string().contains("Variable `b` is locked"));
        }
    }



    // Cuz u can't overshadow variables declared in upstream scopes
    #[test]
    fn test_multi_return_decl_one_variable_upstream_errors() {
        // This tests against function arguments considering they are upstream
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();
        
        // Lets leave them locked.
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
            let body = vec![
                Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))
            ];
            let main = void_func("main", vec![param("a", t1.clone()), param("b", t2.clone())], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Variable `a` is already defined upstream"));
            // assert!(result.unwrap_err().to_string().contains("Variable `b` is already defined upstream"));
        }
    }


    #[test]
    fn test_multi_return_decl_not_func_call_errors() {
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
            let body = vec![
                Stmt::VarDeclMulti(vars, l1.clone())
            ];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Multi-declarement requires only a single function call on the right-hand side"));
        }
    }








    #[test]
    fn test_multi_return_decl_typemismatch_errors() {
        let literals = get_all_literals_no_arr();
        let literals_scattered = get_all_literals_no_arr_scattered_order();



        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `b`"));
        }


        // Same test but the mismatch is in "a" instead of "b"
        for ((l1, t1), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t2.clone(), t2.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t1.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
        }


        // Same test but the mismatch is in both "a" and "b"
        for ((l1, t1), t2) in literals.iter()
            .zip(ALL_TYPES_NO_ARR.iter())
            .zip(ALL_TYPES_NO_ARR_SCATTERED)
        {
            let pair_body = vec![return_stmt(vec![l1.clone(), l1.clone()])];
            let pair = returning_func("pair", vec![], vec![t1.clone(), t1.clone()], pair_body);

            let vars = vec![
                Variable { name: "a".to_string(), type_name: t2.clone(), value: None, span: span() },
                Variable { name: "b".to_string(), type_name: t2.clone(), value: None, span: span() },
            ];
            let body = vec![Stmt::VarDeclMulti(vars, call_expr("pair", vec![]))];
            let main = void_func("main", vec![], body);

            let mut ast = AST { functions: vec![pair, main] };
            let result = check_semantics(&mut ast);

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Type mismatch for variable `a`"));
        }




    }






    #[test]
    fn test_multidec_return_count_mismatch_errors() {
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


    // Invalid array construction (element types mismatch)
    #[test]
    fn test_array_element_type_mismatch_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints();
        let literals_scattered = get_all_literals_no_arr_scattered_order();

        
        // We use no_ints here because if we included int literals, they would get inferred to
        // correct type if they fit, and since functions return 1 for all ints, they would always
        // fit.
        for ((l1, t1), l2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(literals_no_ints.iter())
        {
            for i in 0..=100 {
                let mut elements = vec![l1.clone(); i];

                elements.push(l2.clone());
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements.clone(),
                    array_ty: t1.clone(),
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArraySingleAccess {
                        array: Box::new(var_expr("x")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("x", Type::Array(Box::new(t1.clone())), Some(arr_lit.clone())),
                        var_decl("y", t1.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array element type mismatch: expected"));
                }
            }       
        }


        // Same as above, but this time we test with a variable. All literals.
        let literals = get_all_literals_no_arr();
        for (((l1, t1), l2), t2) in literals_scattered.iter()
            .zip(ALL_TYPES_NO_ARR_SCATTERED.iter())
            .zip(literals.iter())
            .zip(ALL_TYPES_NO_ARR)
        {
            for i in 0..=100 {
                let mut elements = vec![l1.clone(); i];

                elements.push(var_expr("e"));
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements.clone(),
                    array_ty: t1.clone(),
                    span: span(),
                };

                for i2 in 0..i+1 {
                    let access = Expr::ArraySingleAccess {
                        array: Box::new(var_expr("x")),
                        index: Box::new(usize_lit(i2)),
                        span: span(),
                    };
                    let body = vec![
                        var_decl("e", t2.clone(), Some(l2.clone())),
                        var_decl("x", Type::Array(Box::new(t1.clone())), Some(arr_lit.clone())),
                        var_decl("y", t1.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array element type mismatch: expected"));
                }
            }       
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
            for i in 3..=10000 {
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
    fn test_array_access_not_usize_var_errors() {
        let literals = get_all_literals_no_arr_no_usize();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR_NO_USIZE.iter()) {
            let arr_lit = Expr::ArrayLiteral {
                elements: vec![l.clone(), l.clone(), l.clone()],
                array_ty: t.clone(),
                span: span(),
            };
            let access = Expr::ArraySingleAccess {
                array: Box::new(var_expr("arr")),
                index: Box::new(var_expr("e")),
                span: span(),
            };
            let body = vec![
                var_decl("e", t.clone(), Some(l.clone())),
                var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit)),
                var_decl("x", t.clone(), Some(access)),
            ];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Expected array index to be of type"));
        }
    }





    #[test]
    fn test_array_out_of_bounds_multiple_access_errors() {
        // own arr t[] = [l, l, l]
        // own x t = arr[0:i]  (out of bounds)
        // i starts from 3 up to 10k

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 3..=10000 {
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
            for i in 0..=100 {
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


    // Because array access index variables are always copied.
    #[test]
    fn test_array_valid_access_variable_copy_errors() {

        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=100 {
                let elements = vec![l.clone(); i + 1];
                
            
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                for i2 in 0..i+1 {
                
                    let copy_var = Expr::CopyCall { expr: Box::new(var_expr("e")), span: span() };
                    let access = Expr::ArraySingleAccess {
                        array: Box::new(var_expr("arr")),
                        index: Box::new(copy_var),
                        span: span(),
                    };

                    let body = vec![
                        var_decl("e", Type::Usize, Some(usize_lit(i2))),
                        var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                        var_decl("x", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("You do not need to Copy an index when you are accessing an array, it is always copied. Remove the copy call"));
                }
            }       
        }
    }



    // i.e. "hi"[0] is an error. You can only access variables, of type array, not literals.
    #[test]
    fn test_array_access_on_literals_errors() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 0..=1000 {
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
            for i in 0..=1000 {
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
            for i in 0..=1000 {
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
            for i in 0..=100 {
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



    // We dont use should_panic here because we test multiplecases like literals, arrays of
    // different types, etc.
    #[test]
    fn test_array_multiple_access_without_start_and_end_panics() {
        let literals = get_all_literals_no_arr();
        
        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for i in 1..=100 {
                let elements = vec![l.clone(); i + 1];
                
                let arr_lit = Expr::ArrayLiteral {
                    elements: elements,
                    array_ty: t.clone(),
                    span: span(),
                };

                let access = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("arr")),
                    start: None,
                    end: None,
                    span: span(),
                };
                let body = vec![
                    var_decl("arr", Type::Array(Box::new(t.clone())), Some(arr_lit.clone())),
                    var_decl("x", Type::Array(Box::new(t.clone())), Some(access)),
                ];
                let func = void_func("foo", vec![], body);
                let result = std::panic::catch_unwind(|| { 
                    let mut ast = ast_one(func);
                    let _ = check_semantics(&mut ast);
                });

                assert!(result.is_err(), "Expected panic for: {:?} {:?}", t, l);
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
                        // move arr to x
                        var_decl("x", Type::Array(Box::new(t.clone())), Some(var_expr("arr"))), 
                        var_decl("y", t.clone(), Some(access)),
                    ];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().starts_with("Semantic error: Array access on moved variable `arr`"));
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

            let assert_cond = result.unwrap_err().to_string();
            let assert_cond = assert_cond.contains("Expected numeric types in binary arithmetic operation") |
                                assert_cond.contains("You cannot perform bitwise operations on non-integer types");

            assert!(assert_cond);
        }
    }



    // (includes strings)
    #[test]
    fn test_all_literals_binop_comp_eq_passes() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());
            }
        }
    }



    // Same as above test, but its mixed types
    #[test]
    fn test_all_literals_binop_comp_eq_errors() {
        let literals = get_all_literals_no_arr_few_ints();
        let literals_scattered = get_all_literals_no_arr_few_ints_scattered();

        for (l, ls) in literals.iter().zip(literals_scattered.iter()) {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(ls.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
            }
        }
    }


    #[test]
    fn test_all_literals_binop_comp_arth_errors() {
        let literals_str_bool = [str_lit("hi"), bool_lit(false)];

        for l in literals_str_bool {
            for b in ALL_BIN_OP_KIND_COMP_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("You cannot perform arithmetic comparison on non-numeric types"));
            }
        }
    }


    #[test]
    fn test_all_literals_binop_comp_arth_mixed_errors() {
        let literals = get_all_literals_no_arr_few_ints();
        let literals_scattered = get_all_literals_no_arr_few_ints_scattered();

        for (l, ls) in literals.iter().zip(literals_scattered.iter()) {
            for b in ALL_BIN_OP_KIND_COMP_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(ls.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Type mismatch in binary comparison operation"));
            }
        }
    }




    #[test]
    fn test_all_literals_binop_comp_eq_for_binop_logical_passes() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            for b in ALL_BIN_OP_KIND_COMP_EQ {
                let bin_bool = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b,
                        right: Box::new(l.clone()),
                        span: span(),
                    };

                for bl in ALL_BIN_OP_KIND_LOGIC {
                    let bin = Expr::BinOp {
                        left: Box::new(bin_bool.clone()),
                        op: bl,
                        right: Box::new(bin_bool.clone()),
                        span: span(),
                    };


                    let body = vec![var_decl("s", Type::Infer, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_ok());
                }
            }
        }
    }




    #[test]
    fn test_non_boolean_binop_logical_passes() {
        let bools = [bool_lit(false), bool_lit(true)];

        for bv in &bools {
            for b in ALL_BIN_OP_KIND_LOGIC {
                let bin = Expr::BinOp {
                    left: Box::new(bv.clone()),
                    op: b,
                    right: Box::new(bv.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_ok());
            }
        }
    }



    #[test]
    fn test_binop_copy_var_errors() {
        let literals = get_all_literals_no_arr();

        for (l, t) in literals.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND {
                let copy_var = Expr::CopyCall { expr: Box::new(var_expr("a")), span: span() };

                let bin = Expr::BinOp {
                    left: Box::new(copy_var),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![
                    var_decl("a", t.clone(), Some(l.clone())),
                    var_decl("s", Type::Infer, Some(bin))
                ];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Copying is not needed for variables in binary operations"));
            }
        }
    }






    // Non boolean left or right with logical "AND" or "OR" should error
    #[test]
    fn test_non_boolean_binop_logical_errors() {
        let literals = get_all_literals_no_arr_bool();

        for l in &literals {
            for b in ALL_BIN_OP_KIND_LOGIC {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b,
                    right: Box::new(l.clone()),
                    span: span(),
                };
                let body = vec![var_decl("s", Type::Infer, Some(bin))];
                let func = void_func("foo", vec![], body);
                let mut ast = ast_one(func);
                let result = check_semantics(&mut ast);
                assert!(result.is_err());
                assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
            }
        }


        let bool_values = [false, true];
        // Same test, but left is a bool (false and true)
        for l in &literals {
            for b in ALL_BIN_OP_KIND_LOGIC {
                for bv in bool_values {
                    let bin = Expr::BinOp {
                        left: Box::new(bool_lit(bv)),
                        op: b.clone(),
                        right: Box::new(l.clone()),
                        span: span(),
                    };
                    let body = vec![var_decl("s", Type::Infer, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
                }
            }
        }


        // Same test, but right is a bool (false and true)
        for l in &literals {
            for b in ALL_BIN_OP_KIND_LOGIC {
                for bv in bool_values {
                    let bin = Expr::BinOp {
                        left: Box::new(l.clone()),
                        op: b.clone(),
                        right: Box::new(bool_lit(bv)),
                        span: span(),
                    };
                    let body = vec![var_decl("s", Type::Infer, Some(bin))];
                    let func = void_func("foo", vec![], body);
                    let mut ast = ast_one(func);
                    let result = check_semantics(&mut ast);
                    assert!(result.is_err());
                    assert!(result.unwrap_err().to_string().contains("Logical binary operation require both expressions to be evalutable to type `bool`"));
                }
            }
        }
    }





    #[test]
    fn test_integers_binop_arth_passes() {
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;
        
        for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
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


    #[test]
    fn test_floating_binop_real_arth_passes() {
        let literals_floats = get_all_float_literals_no_arr();

        for (l, t) in literals_floats.iter().zip(ALL_FLOATS_TYPES.iter()) {
            for b in ALL_BIN_OP_KIND_REAL_ARTH {
                let bin = Expr::BinOp {
                    left: Box::new(l.clone()),
                    op: b.clone(),
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


    #[test]
    fn test_non_int_binop_bit_arth_errors() {
        let literals_no_ints = get_all_literals_no_arr_no_ints(); 

        for (l, t) in literals_no_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
            for b in ALL_BIN_OP_KIND_BIT_ARTH {
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
                assert!(result.is_err());
            }
        }
    }





    // float literal inference


    // All floating 32 literals can be safely converted to floating 64 .
    #[test]
    fn test_floating_literal_is_32_but_type_is_64() {
        // if variable is declared with an int8 and the value is an int32, but it can fit in int8,
        // it shouldn't error
        let lit = float32_lit(1.0); 
        let body = vec![var_decl("x", Type::Float64, Some(lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        check_semantics(&mut ast).unwrap();
        if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
            assert!(matches!(v.value, Some(Expr::FloatLiteral { value: FloatLiteralValue::Float64(1.0), .. })));
        } else { panic!("Expected VarDecl") }
    }

    #[test]
    fn test_float32_cannot_accept_float64_errors() {
        let lit = Expr::FloatLiteral { value: FloatLiteralValue::Float64(3.14), span: span() };
        let body = vec![var_decl("f", Type::Float32, Some(lit))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is of type `float64`, but we expected type `float32`"));
    }




    // (signed) integer literal inference
    #[test]
    fn test_integer_literal_inferred_to_int8() {
        // if variable is declared with an int8 and the value is a different signed int literal, but it can fit in int8,
        // it shouldn't error
        let literals_signed_ints = get_all_signed_literals_no_arr_no_float();

        for l in literals_signed_ints {
            let body = vec![var_decl("x", Type::Int8, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int8(1), .. })));
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
            let body = vec![var_decl("x", Type::Int8, Some(lit))];
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
            let body = vec![var_decl("x", Type::Int16, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int16(1), .. })));
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
            let body = vec![var_decl("x", Type::Int16, Some(lit))];
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
            let body = vec![var_decl("x", Type::Int32, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int32(1), .. })));
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
            let body = vec![var_decl("x", Type::Int32, Some(lit))];
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
            let body = vec![var_decl("x", Type::Int64, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int64(1), .. })));
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
            let body = vec![var_decl("x", Type::Int64, Some(lit))];
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
            let body = vec![var_decl("x", Type::Int128, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Int128(1), .. })));
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
            let body = vec![var_decl("x", Type::Int128, Some(lit))];
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
            let body = vec![var_decl("x", Type::Byte, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Byte(1), .. })));
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
            let body = vec![var_decl("x", Type::Byte, Some(lit))];
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
            let body = vec![var_decl("x", Type::Uint16, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint16(1), .. })));
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
            let body = vec![var_decl("x", Type::Uint16, Some(lit))];
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
            let body = vec![var_decl("x", Type::Uint32, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint32(1), .. })));
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
            let body = vec![var_decl("x", Type::Uint32, Some(lit))];
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
            let body = vec![var_decl("x", Type::Uint64, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint64(1), .. })));
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
            let body = vec![var_decl("x", Type::Uint64, Some(lit))];
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
            let body = vec![var_decl("x", Type::Usize, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                // because all literals in that func return int literals with value of 1
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Usize(1), .. })));
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
            let body = vec![var_decl("x", Type::Usize, Some(lit))];
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
            let body = vec![var_decl("x", Type::Uint128, Some(l))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            check_semantics(&mut ast).unwrap();
            if let Stmt::VarDecl(v) = &ast.functions[0].body[0] {
                assert!(matches!(v.value, Some(Expr::IntLiteral { value: IntLiteralValue::Uint128(1), .. })));
            } else { panic!("Expected VarDecl") }
        }
    }




    
    // binary operation type mismatch 

    #[test]
    fn test_binop_int_non_int_mixed_types_errors() {
        // int literals are not allowed to mix with literals of non-int type
        //
        
        let int_literals = get_all_literals_no_arr_str_bool_float();

        let non_int_literals = get_all_literals_no_arr_no_ints();

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
                    let assert_condition = assert_condition.contains("Type mismatch in binary") 
                                           || assert_condition.contains("You cannot perform arithmetic");

                    assert!(assert_condition);
                }
            }
        }
        
        // Same as above, but this switches non_int to left, and int to right
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
                    let assert_condition = assert_condition.contains("Type mismatch in binary") 
                                           || assert_condition.contains("You cannot perform arithmetic");

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
                let assert_condition = assert_condition.contains("Type mismatch in binary arithmetic operation") |
                                        assert_condition.contains("Type mismatch in binary bitwise operation");

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
    fn test_copy_of_func_call_errors() {
        let call_expr = Expr::Call{
            name: "x".to_string(),
            args: vec![],
            span: span()
        };

        let copy_expr = Expr::CopyCall { expr: Box::new(call_expr), span: span() };
        let body = vec![var_decl("x", Type::Infer, Some(copy_expr))];
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Copy call expects a variable"));
    }


    #[test]
    fn test_copy_of_array_access_errors() {
        for i in 0..1000 {
            let array_expr = Expr::ArraySingleAccess {
                array: Box::new(var_expr("e")),
                index: Box::new(usize_lit(i)),
                span: span(),
            };


            let copy_expr = Expr::CopyCall { expr: Box::new(array_expr), span: span() };
            let body = vec![var_decl("x", Type::Infer, Some(copy_expr))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Copying is not needed for array access, when you access or slice an array or a string, a new copy is made. Remove the copy call and use operation directly."));
        }
    }


    #[test]
    fn test_copy_of_array_multiple_access_errors() {
        for i in 0..=1000 {
            let array_expr = Expr::ArrayMultipleAccess {
                    array: Box::new(var_expr("arr")),
                    start: Some(Box::new(usize_lit(0))),
                    end: Some(Box::new(usize_lit(i))),
                    span: span(),
                };


            let copy_expr = Expr::CopyCall { expr: Box::new(array_expr), span: span() };
            let body = vec![var_decl("x", Type::Infer, Some(copy_expr))];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Copying is not needed for array access, when you access or slice an array or a string, a new copy is made. Remove the copy call and use operation directly."));
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
    fn test_use_of_undeclared_variable_errors() {
        // Try referencing non-existent variable "x"
        let body = vec![Stmt::Expr(var_expr("x"))]; // x not declared
        let func = void_func("foo", vec![], body);
        let mut ast = ast_one(func);
        let result = check_semantics(&mut ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("undeclared variable"));
    }


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


    #[test]
    fn test_assignment_of_undeclared_variable_errors() {
        let literals = get_all_literals_no_arr();

        for l in &literals {
            let body = vec![var_assign("x", l.clone())];
            let func = void_func("foo", vec![], body);
            let mut ast = ast_one(func);
            let result = check_semantics(&mut ast);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("undeclared variable"));
        }
    }

    #[test]
    fn test_assignment_of_undeclared_variable_other_errors() {
        for t in ALL_TYPES_NO_ARR {
            let body = vec![
                var_decl("x", t.clone(), Some(var_expr("y"))),
            ]; 
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
        for t in ALL_TYPES_NO_ARR_NO_INFER {
            let body = vec![return_stmt(vec![var_expr("n")])];
            let func = returning_func("foo", vec![param("n", t.clone())], vec![t.clone()], body);
            let mut ast = ast_one(func);

            let result = check_semantics(&mut ast);
            assert!(result.is_ok());
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
    fn test_full_valid_program_integers() {
        // This program tests all integers / floating points and all arthemtic binary operations
        // it also tests variable declaration, function declaration, and function calling 
        let literals_ints = get_all_literals_no_arr_str_bool_float() ;

        for b in ALL_BIN_OP_KIND_ARTH {
            let add_body = vec![return_stmt(vec![
                Expr::BinOp {
                    left: Box::new(var_expr("a")),
                    op: b,
                    right: Box::new(var_expr("b")),
                    span: span(),
                }
            ])];
            

            for (l, t) in literals_ints.iter().zip(ALL_TYPES_NO_ARR.iter()) {
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
