use super::*;
use crate::tests_consts::{
    ALL_TYPES_NO_ARR, 
};

use crate::ast::{
    Type, Span, Stmt, Expr, Param,
    IntLiteralValue,
    VariableDeclaration
};

mod var_decl_tests;

fn span() -> Span {
    Span { line: 1, column: 0 }
}

/// Build an AST that contains exactly one function.
fn ast_one(func: Function) -> AST {
    AST { functions: vec![func], globals: vec![] }
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

fn var_decl(name: &str, ty: Type, value: Expr, explicitly_initialized: bool) -> Stmt {
    Stmt::VarDecl(VariableDeclaration {
        name: name.to_string(),
        type_name: ty,
        value,
        explicitly_initialized,
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
