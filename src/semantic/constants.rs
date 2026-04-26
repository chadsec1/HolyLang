use super::*;
use crate::ast::{
    IntLiteralValue, UnaryOpKind, BinOpKind, Constant
};

/// Evaluate a constant expression and fold it into a literal
///
/// This is a safe wrapper around `eval_const_expr_and_fold_it_hazmat`.
///
pub fn eval_const_expr_and_fold_it(
    cons: &mut Constant,
    storage: &mut HashMap<String, BindingInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
) -> Result<(), HolyError> {
    // Validate, and get the type of the value expression.
    let expr_ty = infer::infer_expr_type(&mut cons.value, storage, fun_sigs, Some(cons.type_name.clone()))?;
    if expr_ty != cons.type_name {
        return Err(HolyError::Semantic(format!(
            "Type mismatch assigning to `{}`: got `{}`, expected `{}` (line {} column {})",
            &cons.name, expr_ty, cons.type_name, cons.span.line, cons.span.column
        )));
    }


    if expr_ty.is_array_type() && (!expr_ty.is_fully_fixed_array_type()) {
        return Err(HolyError::Semantic(format!(
            "Dynamic arrays cannot be evaluated at compile time, therefore you cannot assign them to constant `{}` of type `{}` (line {} column {})",
            &cons.name, expr_ty, cons.span.line, cons.span.column
        )));


    }

    // Validate the constant value expression to ensure it is known at compile-time, and
    // evaluate it, and then fold it.
    cons.value = eval_const_expr_and_fold_it_hazmat(&cons.value.clone(), &storage.clone())?;

    Ok(())
}

/// Evaluate a constant expression and fold it into a literal
///
/// NOTE: This is an internal function and assumes you already called infer_expr_type BEFORE
///       calling it on an expression.
/// 
///
fn eval_const_expr_and_fold_it_hazmat(
    expr: &Expr, 
    storage: &HashMap<String, BindingInfo>
) -> Result<Expr, HolyError> {
    let expr_span = helpers::expr_span(expr);

    match expr {
        Expr::IntLiteral{..} |
        Expr::Float64Literal{..} |
        Expr::ArrayLiteral{..} |
        Expr::BoolLiteral{..} => Ok(expr.clone()),
        
        Expr::UnaryOp{ op, expr, ..} => {
            let expr_evaled = eval_const_expr_and_fold_it_hazmat(expr, storage)?;

            match expr_evaled {
                Expr::IntLiteral { value, span} => {
                    if !value.is_signed() {
                        panic!(
                            "(Compiler bug) Unary operation on an unsigned integer.. This should've errored when the wrapper guard called infer_expr_type. Op: {:?}\nExpr: {:?}", 
                            op, expr
                        );
                    }
                    let mut result: i128 = value.as_i128();
                    match op {
                        UnaryOpKind::Negate => {
                            result = result.checked_neg().ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant unary negate result would cause an integer overflow. Integer: {}  (line {} column {})",
                                        result, span.line, span.column
                                    ))
                                })?;
                        },

                        _ => todo!()
                    }
                    

                    // Ensures unary result fits in original value type
                    let folded_result = helpers::coerce_integer_literal_to_type_helper(value.get_type(), IntLiteralValue::Int128(result), span)?;
                    return Ok(Expr::IntLiteral { value: folded_result, span });
                },

                Expr::Float64Literal { value, span} => {
                    let mut result: f64 = value;
                    match op {
                        UnaryOpKind::Negate => {
                            result = -result;
                        },

                        other => panic!("(Compiler bug) Got `{:?}` on a floating point `{:?}, this is illegal and should've been caught by infer_expr_type.\nNON-EVALED EXPR: {:?}", other, expr_evaled, expr)
                    }
                    
                    if !result.is_finite() {
                        return Err(HolyError::Semantic(format!(
                            "Constant floating unary result would cause floating point `{}` to produce non-finite result like `infinite` or `NaN`. (line {} column {})",
                            value, span.line, span.column
                        )));
                    }

                    return Ok(Expr::Float64Literal { value: result, span });
                },



                // TODO: Add NOT support for Bool literals.
                _ => todo!()
            }
        },
        Expr::BinOp{ left, right, op, ..} => {
            let left_lit_expr = eval_const_expr_and_fold_it_hazmat(left, storage)?;
            let right_lit_expr = eval_const_expr_and_fold_it_hazmat(right, storage)?;

            match left_lit_expr {
                Expr::BoolLiteral { value: left_value, span, ..} => {
                    let right_value: bool = match right_lit_expr {
                        Expr::BoolLiteral { value: right_value, .. } => right_value,
                        _ => panic!(
                            "(Compiler bug) eval_const_expr_and_fold_hazmat, infer_expr_type should've been called and verified both left and right binary operations are same type, but apparently not.\nLeft: {:?}\nRight: {:?}",
                            left, right)
                    };


                    match op {
                        BinOpKind::And => {
                            let result: bool = left_value && right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::Or => {
                            let result: bool = left_value || right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },
                        BinOpKind::Equal => {
                            let result: bool = left_value == right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::NotEqual => {
                            let result: bool = left_value != right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        _ => todo!()
                    }

                },

                Expr::Float64Literal { value: left_value, span, ..} => {
                    let right_value = match right_lit_expr {
                        Expr::Float64Literal { value: right_value, .. } => right_value,
                        _ => panic!(
                            "(Compiler bug) eval_const_expr_and_fold_hazmat, infer_expr_type should've been called and verified both left and right binary operations are same type, but apparently not.\nLeft: {:?}\nRight: {:?}",
                            left, right)
                    };

                    let result: f64;

                    match op {
                        BinOpKind::Add => {
                            result = left_value + right_value; 
                        },

                        BinOpKind::Subtract => {
                            result = left_value - right_value; 
                        },

                        BinOpKind::Multiply => {
                            result = left_value * right_value; 
                        },

                        BinOpKind::Divide => {
                            result = left_value / right_value; 
                        },


                        BinOpKind::Equal => {
                            let result: bool = left_value == right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::NotEqual => {
                            let result: bool = left_value != right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::Greater => {
                            let result: bool = left_value > right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::GreaterEqual => {
                            let result: bool = left_value >= right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },


                        BinOpKind::Less => {
                            let result: bool = left_value < right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },

                        BinOpKind::LessEqual => {
                            let result: bool = left_value <= right_value;

                            return Ok(Expr::BoolLiteral { value: result, span });
                        },


                        other => panic!(
                            "(Compiler bug) infer_expr_type should've caught illegal BinOpKind on float.\nLeft: {:?}\nRight: {:?}\nBinOpKind: {:?}", 
                                    left, right, other)
                    }

                    if !result.is_finite() {
                        return Err(HolyError::Semantic(format!(
                            "Constant floating arithemtic result would cause floating point to produce non-finite result like `infinite` or `NaN`. Left: `{}`, Right: `{}`. (line {} column {})",
                            left_value, right_value, span.line, span.column
                        )));
                    }


                    return Ok(Expr::Float64Literal { value: result, span})
                },

                Expr::IntLiteral { value: left_value, span, ..} => {
                    let right_value: IntLiteralValue = match right_lit_expr {
                        Expr::IntLiteral { value: right_value, .. } => {
                            if left_value.get_type() != right_value.get_type() {
                                panic!(
                                    "(Compiler bug) eval_const_expr_and_fold_hazmat, infer_expr_type should've been called and verified both left and right binary operations are same type exact integer literal type, but apparently not.\nLeft: {:?}\nRight: {:?}",
                                    left, right)

                            }
                            right_value
                        },
                        _ => panic!(
                            "(Compiler bug) eval_const_expr_and_fold_hazmat, infer_expr_type should've been called and verified both left and right binary operations are same type, but apparently not.\nLeft: {:?}\nRight: {:?}",
                            left, right)
                    };

                    if left_value.is_signed() {
                        let left_val = left_value.as_i128();
                        let right_val = right_value.as_i128();

                        let result: i128;
                        match op {
                            BinOpKind::Add => {
                                result = left_val.checked_add(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic addition result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Subtract => {
                                result = left_val.checked_sub(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic subtraction result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Multiply => {
                                result = left_val.checked_mul(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic multiplication result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Divide => {
                                result = left_val.checked_div(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic division result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::BitwiseShiftLeft => {
                                let bit_width: u32 = left_value.bit_width();

                                if right_val < 0 {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the left's right-side value cannot be negative. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    )));
                                }

                                if right_val >= (bit_width as i128) {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the left's right-side value cannot exceed `{}`. Left: `{}`, Right: `{}`. (line {} column {})",
                                        bit_width - 1, left_val, right_val, span.line, span.column
                                    )));
                                }

                                // This is not raw bitwise shift left, it is actually checked, because we validated right_val 
                                result = left_val << right_val;


                                // We do conversion here instead of letting it fall down to the
                                // "try into" block, because bit loss in bitwise shifting is
                                // expected.
                                return Ok(truncate_to_int_type(result, left_value.get_type(), span))
                            },
                            BinOpKind::BitwiseShiftRight => {
                                let bit_width: u32 = left_value.bit_width();

                                if right_val < 0 {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the right's right-side value cannot be negative. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    )));
                                }

                                if right_val >= (bit_width as i128) {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the right's right-side value cannot exceed `{}`. Left: `{}`, Right: `{}`. (line {} column {})",
                                        bit_width - 1, left_val, right_val, span.line, span.column
                                    )));
                                }

                                // This is not raw bitwise shift right, it is actually checked, because we validated right_val 
                                result = left_val >> right_val;

                                // We do conversion here instead of letting it fall down to the
                                // "try into" block, because bit loss in bitwise shifting is
                                // expected.
                                return Ok(truncate_to_int_type(result, left_value.get_type(), span))
                            },
                            BinOpKind::BitwiseAnd => {
                                result = left_val & right_val
                            },

                            BinOpKind::BitwiseOr => {
                                result = left_val | right_val
                            },

                            BinOpKind::Equal => {
                                let result: bool = left_val == right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },

                            BinOpKind::NotEqual => {
                                let result: bool = left_val != right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },

                            BinOpKind::Greater => {
                                let result: bool = left_val > right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },

                            BinOpKind::GreaterEqual => {
                                let result: bool = left_val >= right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },


                            BinOpKind::Less => {
                                let result: bool = left_val < right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },

                            BinOpKind::LessEqual => {
                                let result: bool = left_val <= right_val;

                                return Ok(Expr::BoolLiteral { value: result, span });
                            },


                            other => panic!(
                                "(Compiler bug) infer_expr_type should've caught illegal BinOpKind on integer.\nLeft: {:?}\nRight: {:?}\nBinOpKind: {:?}", 
                                        left, right, other)
                        }

                       
                        // Here we operate on left_value type, but it doesn't matter because
                        // left_value type == right_value type, as proven by earlier panic guard
                        // statements.
                        //
                        let folded_result = helpers::coerce_integer_literal_to_type_helper(left_value.get_type(), IntLiteralValue::Int128(result), span)?;
                        return Ok(Expr::IntLiteral { value: folded_result, span });
                    } else {
                        let left_val = left_value.as_u128();
                        let right_val = right_value.as_u128();

                        let result: u128;
                        match op {
                            BinOpKind::Add => {
                                result = left_val.checked_add(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic addition result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Subtract => {
                                result = left_val.checked_sub(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic subtraction result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Multiply => {
                                result = left_val.checked_mul(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic multiplication result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::Divide => {
                                result = left_val.checked_div(right_val).ok_or_else(|| {
                                    HolyError::Semantic(format!(
                                        "Constant arithemtic division result would cause an integer overflow. Left: `{}`, Right: `{}`. (line {} column {})",
                                        left_val, right_val, span.line, span.column
                                    ))
                                })?;
                            },

                            BinOpKind::BitwiseShiftLeft => {
                                let bit_width: u32 = left_value.bit_width();

                                if right_val >= (bit_width as u128) {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the left's right-side value cannot exceed `{}`. Left: `{}`, Right: `{}`. (line {} column {})",
                                        bit_width - 1, left_val, right_val, span.line, span.column
                                    )));
                                }

                                // This is not raw bitwise shift left, it is actually checked, because we validated right_val 
                                result = left_val << right_val;


                                // We do conversion here instead of letting it fall down to the
                                // "try into" block, because bit loss in bitwise shifting is
                                // expected.
                                return Ok(truncate_to_uint_type(result, left_value.get_type(), span))
                            },
                            BinOpKind::BitwiseShiftRight => {
                                let bit_width: u32 = left_value.bit_width();

                                if right_val >= (bit_width as u128) {
                                    return Err(HolyError::Semantic(format!(
                                        "Constant bitwise shift to the right's right-side value cannot exceed `{}`. Left: `{}`, Right: `{}`. (line {} column {})",
                                        bit_width - 1, left_val, right_val, span.line, span.column
                                    )));
                                }

                                // This is not raw bitwise shift right, it is actually checked, because we validated right_val 
                                result = left_val >> right_val;

                                // We do conversion here instead of letting it fall down to the
                                // "try into" block, because bit loss in bitwise shifting is
                                // expected.
                                return Ok(truncate_to_uint_type(result, left_value.get_type(), span))
                            },
                            BinOpKind::BitwiseAnd => {
                                result = left_val & right_val
                            },
                            BinOpKind::BitwiseOr => {
                                result = left_val | right_val
                            },

                            other => panic!(
                                "(Compiler bug) infer_expr_type should've caught illegal BinOpKind on integer.\nLeft: {:?}\nRight: {:?}\nBinOpKind: {:?}", 
                                        left, right, other)
                        }

                        // Here we operate on left_value type, but it doesn't matter because
                        // left_value type == right_value type, as proven by earlier panic guard
                        // statements.
                        // 
                        let folded_result = helpers::coerce_integer_literal_to_type_helper(left_value.get_type(), IntLiteralValue::Uint128(result), span)?;
                        return Ok(Expr::IntLiteral { value: folded_result, span });
                    }
                }
                
                other => panic!( "(Compiler bug) We didn't get literal AND didnt error earlier in binop check. Other: {:?}", other)
            }
        },


        Expr::Var { name, .. } => {
            if let Some(info) = storage.get(name).cloned() {
                match info.kind {
                    BindingKind::Var { .. } => Err(HolyError::Semantic(format!(
                                        "You cannot use variable `{}` in a constant value expression. You can only use literals and or other constants (line {} column {})", 
                                        name, expr_span.line, expr_span.column
                                    ))),
                    BindingKind::Const { ref value, .. } => eval_const_expr_and_fold_it_hazmat(value, storage)
                }            
            } else {
                panic!("(Compiler bug) Binding doesnt exist in scope, which is impossible because infer_expr_type shouldve been called and validated its existence.\nBinding name: {:?}\nscope: {:#?}", name, storage);
            }
        }

        _ => Err(HolyError::Semantic(format!(
                        "{} expression cannot be evaluated at compile-time, therefore it cannot be assigned to a constant. (line {} column {})",
                        expr, expr_span.line, expr_span.column
                    )))
    }

}


/// Takes a `target` which is an uint128, and a type to try to coerce it to.
/// The reason this function exists is purely only for bitwise shift operations
fn truncate_to_uint_type(target: u128, ty: Type, span: Span) -> Expr {
    match ty {
        Type::Byte => Expr::IntLiteral { value: IntLiteralValue::Byte(target as u8), span},
        Type::Uint16 => Expr::IntLiteral { value: IntLiteralValue::Uint16(target as u16), span},
        Type::Uint32 => Expr::IntLiteral { value: IntLiteralValue::Uint32(target as u32), span},
        Type::Uint64 => Expr::IntLiteral { value: IntLiteralValue::Uint64(target as u64), span},
        Type::Uint128 => Expr::IntLiteral { value: IntLiteralValue::Uint128(target as u128), span },
        Type::Usize => Expr::IntLiteral { value: IntLiteralValue::Usize(target as usize), span },

        other => panic!("(Compiler bug) Expected target to be of an unsigned integer type, instead got `{:?}`. Target: {:?}", other, target)
    }
}


/// Takes a `target` which is an int128, and a type to try to coerce it to.
/// The reason this function exists is purely only for bitwise shift operations
fn truncate_to_int_type(target: i128, ty: Type, span: Span) -> Expr {
    match ty {
        Type::Int8 => Expr::IntLiteral { value: IntLiteralValue::Int8(target as i8), span},
        Type::Int16 => Expr::IntLiteral { value: IntLiteralValue::Int16(target as i16), span},
        Type::Int32 => Expr::IntLiteral { value: IntLiteralValue::Int32(target as i32), span},
        Type::Int64 => Expr::IntLiteral { value: IntLiteralValue::Int64(target as i64), span},
        Type::Int128 => Expr::IntLiteral { value: IntLiteralValue::Int128(target as i128), span },

        other => panic!("(Compiler bug) Expected target to be of an signed integer type, instead got `{:?}`. Target: {:?}", other, target)
    }
}



