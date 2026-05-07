use super::*;
use crate::ast::{
    FixedArraySize, IntLiteralValue,
    UnaryOpKind,
    Param, Variable, VariableAssignment, MultiAssignment, 
    IfStmt, WhileStmt, ForStmt, InfiniteStmt, BreakStmt, ContinueStmt
};

use crate::tests_consts::{
    ALL_TYPES_NO_ARR, ALL_TYPES_NO_ARR_SCATTERED, ALL_TYPES_NO_ARR_NO_USIZE, ALL_TYPES_NO_INTS_NO_ARR,
 
    ALL_TYPES_NO_ARR_NO_BOOL,
    ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING, ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING_SCATTERED,

    ALL_INT_TYPES_NO_ARR,

    ALL_UNSIGNED_TYPES_NO_ARR, ALL_SIGNED_TYPES_NO_ARR,
    ALL_BIN_OP_KIND_ARTH, ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ,
    ALL_BIN_OP_KIND_REAL_ARTH, ALL_BIN_OP_KIND_BIT_ARTH,

    ALL_BIN_OP_KIND,
    ALL_BIN_OP_KIND_LOGIC,
    ALL_BIN_OP_KIND_COMP_ARTH
};

mod var_decl_tests;
mod var_assign_tests;
mod ownership_tests;
mod expr_tests;
mod copy_tests;
mod format_tests;

mod int_literals_internal_inference_tests;

mod return_tests;
mod multi_return_tests;

mod function_tests;
mod function_call_tests;

mod locking_unlocking_tests;

mod bin_op_tests;
mod unary_op_tests;

mod array_tests;
mod dyn_array_tests;
mod dyn_array_access_tests;
mod dyn_array_slicing_tests;

mod fixed_array_tests;
mod fixed_array_access_tests;
mod fixed_array_slicing_tests;



mod if_stmt_tests;

mod for_stmt_tests;
mod while_stmt_tests;
mod infinite_stmt_tests;
mod break_stmt_tests;
mod continue_stmt_tests;

mod happy_path_tests;




// Helper functions for all the test submodules


fn get_all_literals_no_arr_bool() -> [Expr; 13] {
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

        float64_lit(1.0),

        str_lit("Hi")
    ];

    return literals;
}


fn get_all_literals_no_arr_no_ints() -> [Expr; 3] {
    let literals = [

        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}



fn get_all_literals_no_arr_few_ints() -> [Expr; 5] {
    let literals = [
        uint128_lit(1),
        int128_lit(1),

        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}


fn get_all_literals_no_arr_few_ints_scattered() -> [Expr; 5] {
    let literals = [
        str_lit("Hi"),

        bool_lit(false),
        int128_lit(1),
        float64_lit(1.0),
        uint128_lit(1),
    ];

    return literals;
}



fn get_all_signed_literals_no_arr() -> [Expr; 6] {
    let literals = [
        int8_lit(1),
        int16_lit(1),
        int32_lit(1),
        int64_lit(1),
        int128_lit(1),

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


fn get_all_literals_no_arr_str_bool() -> [Expr; 12] {
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

        float64_lit(1.0),
    ];

    return literals;
}



fn get_all_literals_no_arr_str_bool_scattered() -> [Expr; 12] {
    let literals = [
        uint32_lit(1),
        int8_lit(1),
        int64_lit(1),
        uint128_lit(1),

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


fn get_all_literals_no_arr() -> [Expr; 14] {
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

        float64_lit(1.0),

        bool_lit(false),
        str_lit("Hi")
    ];

    return literals;
}

fn get_all_literals_no_arr_scattered_order() -> [Expr; 14] {
    let literals = [
        int128_lit(1),
        int8_lit(1),
        uint64_lit(1),
        uint16_lit(1),
        int64_lit(1),
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



fn get_all_literals_no_arr_no_usize() -> [Expr; 13] {
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
    AST { functions: vec![func], globals: vec![] }
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



fn float64_lit(f: f64) -> Expr {
    Expr::Float64Literal { value: f, span: span() }
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

