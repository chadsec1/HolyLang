use super::*;
use std::sync::LazyLock;
use crate::tests_consts::{
    // ALL_TYPES_NO_ARR, 
    ALL_BIN_OP_KIND, ALL_BIN_OP_KIND_COMP, ALL_BIN_OP_KIND_COMP_EQ
};

use crate::ast::{
    Type, Span, Stmt, Expr, Param,
    IntLiteralValue,
    VariableDeclaration, MultiVariableDeclaration,
    VariableAssignment, MultiAssignment
};

mod const_tests;
mod var_decl_tests;
mod var_assign_tests;
mod multi_return_tests;
mod lock_tests;

// With dynamic array types 
static ALL_TYPES_WITH_DYN_ARR: LazyLock<Vec<Type>> = LazyLock::new(|| {
    vec![
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
        Type::Float64,
        Type::Bool,
        Type::String,

        Type::Array(Box::new(Type::Int8)),
        Type::Array(Box::new(Type::Int16)),
        Type::Array(Box::new(Type::Int32)),
        Type::Array(Box::new(Type::Int64)),
        Type::Array(Box::new(Type::Int128)),

        Type::Array(Box::new(Type::Byte)),
        Type::Array(Box::new(Type::Uint16)),
        Type::Array(Box::new(Type::Uint32)),
        Type::Array(Box::new(Type::Uint64)),
        Type::Array(Box::new(Type::Uint128)),
        Type::Array(Box::new(Type::Usize)),

        Type::Array(Box::new(Type::Float64)),
        Type::Array(Box::new(Type::Bool)),
        Type::Array(Box::new(Type::String)),
    ]
});

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

fn call_expr(name: &str, args: Vec<Expr>) -> Expr {
    Expr::Call { name: name.to_string(), args, span: span() }
}

fn return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}

fn param(name: &str, ty: Type) -> Param {
    Param { name: name.to_string(), type_name: ty, span: span() }
}

fn const_define_locally(name: &str, ty: Type, value: Expr) -> Stmt {
    Stmt::Const(Constant {
        name: name.to_string(),
        type_name: ty,
        value,
        span: span(),
    })
}

fn const_define_globally(name: &str, ty: Type, value: Expr) -> GlobalStmt {
    GlobalStmt::Const(Constant {
        name: name.to_string(),
        type_name: ty,
        value,
        span: span(),
    })
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

fn array_lit(exprs: Vec<Expr>, type_name: Option<Type>) -> Expr {
    Expr::ArrayLiteral { elements: exprs, type_name, span: span() }
}

fn get_many_boolean_conditions() -> Vec<Expr> {
    let literals = get_all_literals();
    
    let mut boolean_conds = vec![
        bool_lit(true),
        bool_lit(false),
    ];

    for l in literals {
        for b in ALL_BIN_OP_KIND_COMP {
            // So that >= > <= < doesnt get performed on non integer/floats.
            if !ALL_BIN_OP_KIND_COMP_EQ.contains(&b) {
                match l {
                    Expr::StringLiteral { .. } | Expr::BoolLiteral { .. } | Expr::ArrayLiteral { .. } => {
                        continue
                    },
                    _ => {}
                }
            }

            let bin = Expr::BinOp {
                left: Box::new(l.clone()),
                right: Box::new(l.clone()),
                op: b,
                span: span(),
            };

            boolean_conds.push(bin);
        }
    }

    return boolean_conds;
}


/*
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
*/

fn get_all_literals() -> [Expr; 28] {
    return [
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
        str_lit("Hi"),

        array_lit(vec![int8_lit(1), int8_lit(i8::MIN), int8_lit(i8::MAX) ], Some(Type::Array(Box::new(Type::Int8)))),
        array_lit(vec![int16_lit(1), int16_lit(i16::MIN), int16_lit(i16::MAX) ], Some(Type::Array(Box::new(Type::Int16)))),
        array_lit(vec![int32_lit(1), int32_lit(i32::MIN), int32_lit(i32::MAX) ], Some(Type::Array(Box::new(Type::Int32)))),
        array_lit(vec![int64_lit(1), int64_lit(i64::MIN), int64_lit(i64::MAX) ], Some(Type::Array(Box::new(Type::Int64)))),
        array_lit(vec![int128_lit(1), int128_lit(i128::MIN), int128_lit(i128::MAX) ], Some(Type::Array(Box::new(Type::Int128)))),

        array_lit(vec![byte_lit(1), byte_lit(u8::MIN), byte_lit(u8::MAX) ], Some(Type::Array(Box::new(Type::Byte)))),
        array_lit(vec![uint16_lit(1), uint16_lit(u16::MIN), uint16_lit(u16::MAX) ], Some(Type::Array(Box::new(Type::Uint16)))),
        array_lit(vec![uint32_lit(1), uint32_lit(u32::MIN), uint32_lit(u32::MAX) ], Some(Type::Array(Box::new(Type::Uint32)))),
        array_lit(vec![uint64_lit(1), uint64_lit(u64::MIN), uint64_lit(u64::MAX) ], Some(Type::Array(Box::new(Type::Uint64)))),
        array_lit(vec![uint128_lit(1), uint128_lit(u128::MIN), uint128_lit(u128::MAX) ], Some(Type::Array(Box::new(Type::Uint128)))),
        array_lit(vec![usize_lit(1), usize_lit(usize::MIN), usize_lit(usize::MAX) ], Some(Type::Array(Box::new(Type::Usize)))),

        array_lit(vec![float64_lit(1.0), float64_lit(f64::MIN), float64_lit(f64::MAX) ], Some(Type::Array(Box::new(Type::Float64)))),
        array_lit(vec![bool_lit(false), bool_lit(true)], Some(Type::Array(Box::new(Type::Bool)))),
        array_lit(vec![str_lit(""), str_lit("Hi"), str_lit(" !")], Some(Type::Array(Box::new(Type::String))))
    ];
}

