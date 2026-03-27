use super::*;

// When variable is declared like
// own x int32
// its value is None, so we need to set it to a default value based on its type.
// ints default are 0, floats are 0.0, string is "", etc.
// Only primitives listed above though. Everything else needs a value.
//
// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
pub fn assign_default_value_for_type(expr: &mut Option<Expr>, ty: &Type, span: Span) -> Result<(), HolyError> {
    if *ty == Type::Infer {
        // To ensure that I dont do mistakes and call this function on things I am not supposed to.

        panic!("(Compiler bug) Cannot assign default value for type `{}` because the expression holder type is Infer. Expression: {:?}\nDont call this on things of type Infer.", 
                        ty, expr);
    }
    
    match ty {
        Type::Int8 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Int8(0), span: span })
        }
        Type::Int16 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Int16(0), span: span })
        }
        Type::Int32 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Int32(0), span: span })
        }
        Type::Int64 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Int64(0), span: span })
        }
        Type::Int128 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Int128(0), span: span })
        }
       
        Type::Usize => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Usize(0), span: span })
        }
        Type::Byte => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Byte(0), span: span })
        }
        Type::Uint16 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Uint16(0), span: span })
        }
        Type::Uint32 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Uint32(0), span: span })
        }
        Type::Uint64 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Uint64(0), span: span })
        }
        Type::Uint128 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Uint128(0), span: span })
        }

        Type::Float32 => {
            *expr = Some(Expr::FloatLiteral { value: FloatLiteralValue::Float32(0.0), span: span })
        }

        Type::Float64 => {
            *expr = Some(Expr::FloatLiteral { value: FloatLiteralValue::Float64(0.0), span: span })
        }

        Type::Bool => {
            *expr = Some(Expr::BoolLiteral { value: false, span: span })
        }

        Type::Array(inner) => {
            let inner_ty = inner.clone();
            *expr = Some(Expr::ArrayLiteral { elements: Vec::new(), array_ty: *inner_ty, span: span })
        }

        Type::String => {
            *expr = Some(Expr::StringLiteral { value: "".to_string(), span: span })
        }




        _ => {
            panic!("(Compiler bug) Cannot assign default value for type `{}` because the expression holder type is Non-literal. Expression: {:?}\nDont call this on non-literals.", 
                        ty, expr);
        }
    }

    Ok(())
}


/// Decide whether two types are compatible for assignment / matching.
/// so we don't have to dereference types all times.
pub fn type_compatible(a: &Type, b: &Type) -> bool {
    a == b
}


/// Resolve binary operation for numeric types and operations. Rules:
/// - both must be numeric and same kind (int/int or float/float)
/// - mixing signed and unsigned is an error
/// - return the resulting type
pub fn resolve_binary_op_types_numeric(a: &Type, b: &Type, span: &Span) -> Result<Type, HolyError> {
    use Type::*;
    match (a, b) {
        (Int8, Int8) => Ok(Int8),
        (Int16, Int16) => Ok(Int16),
        (Int32, Int32) => Ok(Int32),
        (Int64, Int64) => Ok(Int64),
        (Int128, Int128) => Ok(Int128),

        (Usize, Usize) => Ok(Usize),

        (Byte, Byte) => Ok(Byte),
        (Uint16, Uint16) => Ok(Uint16),
        (Uint32, Uint32) => Ok(Uint32),
        (Uint64, Uint64) => Ok(Uint64),
        (Uint128, Uint128) => Ok(Uint128),

        (Float32, Float32) => Ok(Float32),
        (Float64, Float64) => Ok(Float64),


        // If one side is Infer, prefer the other side if concrete
        // 
        // Perhaps it is wiser to panic here as a compiler bug ? It make no sense that any side is infer
        // because it should've been assigned a type by now. 
        //
        /*
        (Infer, t @ _) if *t != Infer => Ok(t.clone()),
        (t @ _, Infer) if *t != Infer => Ok(t.clone()),

        // both infer, we default to default to Int32
        (Infer, Infer) => Ok(Int32),
        */

        (t1 @ _, t2 @ _) if (*t1 == Infer || *t2 == Infer) => panic!("(Compiler bug) We received a numeric binary operation with at least one side being of type infer. A: {:?}, B: {:?}", a, b),

        // mixed signed/unsigned or int/float combos -> error
        _ => Err(HolyError::Semantic(format!("Type mismatch in binary operation: `{}` vs `{}` (line {} column {})", a, b, span.line, span.column))),
    }
}


/// Takes 2 floating point types, determines which type can hold more than the other
///
pub fn get_bigger_type_of_two_floatings(t_1: Type, t_2: Type) -> Type {
    if !t_1.is_floating_type() || !t_2.is_floating_type() {
        panic!("(Compiler bug) you should not call this function unless you are sure both types are floating type. We got {:?} {:?}", t_1, t_2);
    }


    let t_1_score = match t_1 {
            Type::Float32 => 1,
            Type::Float64 => 2,

            other => panic!("Shouldve been a float, instead its {:?}", other)
    };



    let t_2_score = match t_2 {
            Type::Float32 => 1,
            Type::Float64 => 2,

            other => panic!("Shouldve been a float, instead its {:?}", other)
    };

    if t_1_score > t_2_score {
        return t_1
    }

    return t_2

}




/// Takes 2 integer types, determines which type can hold more than the other
///
pub fn get_bigger_type_of_two_integers(t_1: Type, t_2: Type) -> Type {
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

