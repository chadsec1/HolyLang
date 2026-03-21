use super::*;

// helper to get the span of a statement (so we can point to offending code)
pub fn stmt_span(s: &Stmt) -> Span {
    match s {
        Stmt::VarDecl(v) => v.span,
        Stmt::VarAssign(a) => a.span,
        Stmt::Expr(e) => expr_span(e),
        Stmt::Lock(e) => expr_span(&e[0]),  // At least one lock element is always present
        Stmt::Unlock(e) => expr_span(&e[0]),  // At least one unlock element is always present
        Stmt::Return(e) => expr_span(&e[0]), // First return element is always present
                                            // if there is a return
        Stmt::While(w) => w.span,
        Stmt::If(i) => i.span,
        Stmt::Func(f) => f.span,
        Stmt::VarDeclMulti(_, v) => expr_span(v), 
        Stmt::VarAssignMulti(ma) => ma.span,
    }
}

fn expr_span(e: &Expr) -> Span {
    match e {
        Expr::IntLiteral { span, .. } => *span,
        Expr::FloatLiteral { span, .. } => *span,
        Expr::BoolLiteral { span, .. } => *span,
        Expr::ArrayLiteral { span, .. } => *span,
        Expr::StringLiteral { span, .. } => *span,

        Expr::ArraySingleAccess { span, .. } => *span,
        Expr::ArrayMultipleAccess { span, .. } => *span,
        Expr::Var { span, .. } => *span,
        Expr::BinOp { span, .. } => *span,
        Expr::UnaryOp { span, .. } => *span,
        Expr::CopyCall { span, .. } => *span,
        Expr::FormatCall { span, .. } => *span,
        Expr::Call { span, .. } => *span,
    }
}

