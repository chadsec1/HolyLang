use super::*;

use crate::ast::{
    IntLiteralValue, 
    ForStmt, IfStmt, WhileStmt, InfiniteStmt, BreakStmt
};

use crate::semantic::branch_analysis::{
    dead_code_analysis,
    return_branch_analysis
};


mod dead_code_analysis_tests;
mod return_branch_analysis_tests;

// Test Helpers

fn span() -> Span {
    Span { line: 1, column: 1 }
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

fn make_dummy_func(name: String, body: Option<Vec<Stmt>>) -> Function {
    if body.is_none() {
        return Function { 
            name: name, params: vec![], return_type: Some(vec![Type::Int32]), body: vec![Stmt::Expr(int64_lit(69))], span: span()
        }; 
    } else {
        return Function { 
            name: name, params: vec![], return_type: Some(vec![Type::Int32]), body: body.unwrap(), span: span()
        };
    }
}


fn make_return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}

fn make_break_stmt() -> Stmt {
    Stmt::Break(BreakStmt { span: span() })
}

fn var_expr(name: &str) -> Expr {
    Expr::Var { name: name.to_string(), span: span() }
}


fn get_all_literals_with_var_no_arr() -> [Expr; 15] {
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
        str_lit("Hi"),
        var_expr("a")
    ];

    return literals;
}

