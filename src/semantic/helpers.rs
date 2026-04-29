use super::*;

use crate::ast::{
    IntLiteralValue
};


// When variable is declared like
// own x int32
// its value is None, so we need to set it to a default value based on its type.
// ints default are 0, floats are 0.0, string is "", etc.
// Only primitives listed above though. Everything else needs a value.
//
// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
pub fn assign_default_value_for_type(expr: &mut Option<Expr>, ty: &Type, span: Span) -> Result<(), HolyError> {
        

    // Reason why we don't just take a &mut Expr, is because variables values are defined as
    // Option<Expr>.
    //
    // So this guard statement will be fine and will catch most misuse of this function.
    if expr.is_some() {
        panic!(
            "(Compiler bug) Cannot assign default value for an expression that already has a value. Expression: {:?}\nType: {:?}",
            expr, ty
        );
    }


    
    match ty {
        Type::Int8 
        | Type::Int16 
        | Type::Int32
        | Type::Int64 
        | Type::Int128 
        | Type::Byte
        | Type::Uint16
        | Type::Uint32
        | Type::Uint64
        | Type::Uint128
        | Type::Usize
        | Type::Float64
        | Type::Bool
        | Type::String
            => {
            *expr = Some(get_default_expr_for_type_hazmat(ty, span))
        }

        Type::Array(_) => {
            *expr = Some(Expr::ArrayLiteral { elements: Vec::new(), span: span })
        }


        Type::FixedArray(_, _) => {
            // TODO: In future, allow default fixed array types to have default values. up tp its
            // size.
            //
            // for now i will just error
            return Err(HolyError::Semantic(format!(
                    "Default values are not allowed for fixed-size arrays (line {} column {})",
                    span.line, span.column
                )));

        }
    }

    Ok(())
}


fn get_default_expr_for_type_hazmat(ty: &Type, span: Span) -> Expr {
    match ty {
        Type::Int8 => Expr::IntLiteral { value: IntLiteralValue::Int8(0), span },
        Type::Int16 => Expr::IntLiteral { value: IntLiteralValue::Int16(0), span },
        Type::Int32 => Expr::IntLiteral { value: IntLiteralValue::Int32(0), span },

        Type::Int64 => Expr::IntLiteral { value: IntLiteralValue::Int64(0), span },
        Type::Int128 => Expr::IntLiteral { value: IntLiteralValue::Int128(0), span },

        Type::Usize => Expr::IntLiteral { value: IntLiteralValue::Usize(0), span },
        Type::Byte => Expr::IntLiteral { value: IntLiteralValue::Byte(0), span },
        Type::Uint16 => Expr::IntLiteral { value: IntLiteralValue::Uint16(0), span },
        Type::Uint32 => Expr::IntLiteral { value: IntLiteralValue::Uint32(0), span },
        Type::Uint64 => Expr::IntLiteral { value: IntLiteralValue::Uint64(0), span },
        Type::Uint128 => Expr::IntLiteral { value: IntLiteralValue::Uint128(0), span },

        Type::Float64 => Expr::Float64Literal { value: 0.0, span },

        Type::Bool => Expr::BoolLiteral { value: false, span: span },

        Type::String => Expr::StringLiteral { value: "".to_string(), span },

        other => panic!("(Compiler bug) do not call this function on `{:?}` types", other)
    }
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
        Stmt::Const(c) => c.span,
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
        Stmt::Infinite(i) => i.span,
        Stmt::If(i) => i.span,
        Stmt::Func(f) => f.span,
        Stmt::VarDeclMulti(_, v) => expr_span(v), 
        Stmt::VarAssignMulti(ma) => ma.span,
    }
}

// helper to get spanof a expr
pub fn expr_span(e: &Expr) -> Span {
    match e {
        Expr::IntLiteral { span, .. } => *span,
        Expr::Float64Literal { span, .. } => *span,
        Expr::BoolLiteral { span, .. } => *span,
        Expr::ArrayLiteral { span, .. } => *span,
        Expr::StringLiteral { span, .. } => *span,

        Expr::ArrayAccess { span, .. } => *span,
        Expr::ArraySlicing { span, .. } => *span,
        Expr::Var { span, .. } => *span,
        Expr::BinOp { span, .. } => *span,
        Expr::UnaryOp { span, .. } => *span,
        Expr::Call { span, .. } => *span,
        Expr::CopyCall { span, .. } => *span,
        Expr::FormatCall { span, .. } => *span,
        Expr::RangeCall { span, .. } => *span,
    }
}


pub fn coerce_integer_literal_to_type_helper(expected_ty: Type, value: IntLiteralValue, span: Span) -> Result<IntLiteralValue, HolyError> {
    if !value.get_type().is_integer_type() {
        panic!("(Compiler bug) Value `{}` of type `{}` is not an integer type", value, value.get_type());
    }

    let range_err = || HolyError::Semantic(format!(
        "Integer literal `{}` out of range for type `{}` (line {} column {})",
        value, expected_ty, span.line, span.column
    ));

    // Normalize up front. One or both may be None if the value can't be represented that way.
    let (as_signed, as_unsigned) = if value.is_signed() {
        let s = value.as_i128();
        let u = if s >= 0 { Some(s as u128) } else { None };
        (Some(s), u)
    } else {
        let u = value.as_u128();
        let s = if u <= i128::MAX as u128 { Some(u as i128) } else { None };
        (s, Some(u))
    };

    let fits_signed   = |min: i128, max: i128| as_signed.filter(|&v| v >= min && v <= max).ok_or_else(range_err);
    let fits_unsigned = |max: u128|            as_unsigned.filter(|&v| v <= max).ok_or_else(range_err);

    match expected_ty {
        Type::Int8   => Ok(IntLiteralValue::Int8  (fits_signed(i8::MIN   as i128, i8::MAX   as i128)? as i8)),
        Type::Int16  => Ok(IntLiteralValue::Int16 (fits_signed(i16::MIN  as i128, i16::MAX  as i128)? as i16)),
        Type::Int32  => Ok(IntLiteralValue::Int32 (fits_signed(i32::MIN  as i128, i32::MAX  as i128)? as i32)),
        Type::Int64  => Ok(IntLiteralValue::Int64 (fits_signed(i64::MIN  as i128, i64::MAX  as i128)? as i64)),
        Type::Int128 => Ok(IntLiteralValue::Int128(fits_signed(i128::MIN,         i128::MAX)?)),

        Type::Byte   => Ok(IntLiteralValue::Byte  (fits_unsigned(u8::MAX   as u128)? as u8)),
        Type::Uint16 => Ok(IntLiteralValue::Uint16(fits_unsigned(u16::MAX  as u128)? as u16)),
        Type::Uint32 => Ok(IntLiteralValue::Uint32(fits_unsigned(u32::MAX  as u128)? as u32)),
        Type::Uint64 => Ok(IntLiteralValue::Uint64(fits_unsigned(u64::MAX  as u128)? as u64)),
        Type::Uint128 => Ok(IntLiteralValue::Uint128(fits_unsigned(u128::MAX)?)),
        Type::Usize  => Ok(IntLiteralValue::Usize (fits_unsigned(usize::MAX as u128)? as usize)),

        other => panic!("(Compiler bug) Unexpected type in infer_integer_literal_helper: {:?}", other),
    }
}
