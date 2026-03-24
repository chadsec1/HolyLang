use super::*;


pub fn get_bigger_type_of_two(t_1: Type, t_2: Type) -> Type {
    if !t_1.is_integer_type() || !t_2.is_integer_type() {
        panic!("(Compiler bug) you should not call this function unless you are sure both types are integer type. We got {:?} {:?}", t_1, t_2);
    }


    let t_1_score = match t_1 {
            Type::Int8 => 1,
            Type::Int16 => 3,
            Type::Int32 => 5,
            Type::Int64 => 7,
            Type::Int128 => 9,

            Type::Byte => 2,
            Type::Uint16 => 4,
            Type::Uint32 => 6,
            Type::Uint64 => 8,
            Type::Uint128 => 10,
            
            Type::Usize => 8,

            other => panic!("Shouldve been an integer, instead its {:?}", other)
    };

    let t_2_score = match t_2 {
            Type::Int8 => 1,
            Type::Int16 => 3,
            Type::Int32 => 5,
            Type::Int64 => 7,
            Type::Int128 => 9,

            Type::Byte => 2,
            Type::Uint16 => 4,
            Type::Uint32 => 6,
            Type::Uint64 => 8,
            Type::Uint128 => 10,
            
            Type::Usize => 8,

            other => panic!("Shouldve been an integer, instead its {:?}", other)
    };


    if t_1_score > t_2_score {
        return t_1
    }

    return t_2

}





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
        Stmt::For(f) => f.span,
        Stmt::While(w) => w.span,
        Stmt::Break(b) => b.span,
        Stmt::Continue(c) => c.span,
        Stmt::Forever(f) => f.span,
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
        Expr::Call { span, .. } => *span,
        Expr::CopyCall { span, .. } => *span,
        Expr::FormatCall { span, .. } => *span,
        Expr::RangeCall { span, .. } => *span,
    }
}

