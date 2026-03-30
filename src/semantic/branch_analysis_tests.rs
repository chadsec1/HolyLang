use super::*;

use crate::parser::{
    IfStmt, BreakStmt
};

use crate::semantic::branch_analysis::{
    block_always_terminates,
};

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

fn make_return_stmt(exprs: Vec<Expr>) -> Stmt {
    Stmt::Return(exprs)
}

fn make_break_stmt() -> Stmt {
    Stmt::Break(BreakStmt { span: span() })
}

/*
fn var_expr(name: &str) -> Expr {
    Expr::Var { name: name.to_string(), span: span() }
}*/

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


#[cfg(test)]
mod test_block_always_terminates {
    use super::*;
    

    #[test]
    fn empty_if_statement_branch() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }



    #[test]
    fn empty_if_statement_branch_and_empty_else_branch() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l,
                    if_branch: vec![],
                    elif_branches: vec![],
                    else_branch: Some(vec![]),
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts);
            // Branches do not terminate
            assert_eq!(result, false);
        }
    }


    // Main if branch terminates via return, but it has no else branch therefore the block does not
    // always terminate.
    #[test]
    fn if_statement_main_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_return_stmt(vec![l])
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }



    // Same as the above test, but this terminates via break
    #[test]
    fn if_statement_main_branch_break_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![
                        make_break_stmt()
                    ],
                    elif_branches: vec![],
                    else_branch: None,
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }


    // else branch terminates via return, but main branch does not terminate, therefore the block does not
    // always terminate.
    #[test]
    fn if_statement_else_branch_return_not_terminates() {
        let literals = get_all_literals_no_arr();

        for l in literals {
            let stmts: Vec<Stmt> = vec![
                Stmt::If(IfStmt{
                    condition: l.clone(),
                    if_branch: vec![], 
                    elif_branches: vec![],
                    else_branch: Some(vec![
                        make_return_stmt(vec![l])
                    ]),
                    span: span(),
                })
            ];

            let result: bool = block_always_terminates(&stmts);
            // Branch does not terminate
            assert_eq!(result, false);
        }
    }


}

