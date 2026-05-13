use super::*;
use crate::ast::{
    IntLiteralValue, UnaryOpKind, BinOpKind, FixedArraySize, ArraySliceRange
};


/// This takes 2 expressions, and tries to infer types, and convert each other types to same if
/// possible (only for integer literlas and float literals)
///
pub fn advanced_infer_2_types(
    left: &mut Expr, 
    right: &mut Expr,
    locals: &mut HashMap<String, BindingInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    infer_hint: Option<Type>
) -> Result<(Type, Type), HolyError> {
    let mut lty = infer_expr_type(left, locals, fun_sigs, infer_hint.clone())?;

    let mut rty = infer_expr_type(right, locals, fun_sigs, infer_hint.clone())?;
    
    

    // Integer literal inferrence
    if matches!(*left, Expr::IntLiteral {..}) && !matches!(*right, Expr::IntLiteral {..}) {
        lty = infer_expr_type(left, locals, fun_sigs, Some(rty.clone()))?;

    } else if matches!(*right, Expr::IntLiteral {..}) && !matches!(*left, Expr::IntLiteral {..}) {
        rty = infer_expr_type(right, locals, fun_sigs, Some(lty.clone()))?;
    

    } else if lty.is_integer_type() && rty.is_integer_type() {
        // If lty and rty are both integer types, we get the bigger type of them, and try force it
        // upon both lty and rty.
        let bigger_type = helpers::get_bigger_type_of_two_integers(lty.clone(), rty.clone());

        rty = infer_expr_type(right, locals, fun_sigs, Some(bigger_type.clone()))?;
        lty = infer_expr_type(left, locals, fun_sigs, Some(bigger_type.clone()))?;

    }

    Ok((lty, rty))


}


/// Infer the type of an expression, and update literal nodes (and nested nodes) where possible.
/// Returns the deduced Type for the expression.
///
/// NOTE: Unfortuntely I couldn't decouple the coerion logic, from getting type from expression,
/// from enforcing the expression's legality. This is not a design limitation, but a limitations on
/// all compilers pretty much. We have to both get type and try to coerce, AND enforce legality in
/// the same function.
///
pub fn infer_expr_type(
    expr: &mut Expr,
    locals: &mut HashMap<String, BindingInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    infer_hint: Option<Type> // For type coercion
) -> Result<Type, HolyError> {
    match expr {
        // NOTE: If `infer_hint` is set, we **TRY** to coerce the expression value into infer_hint (if
        // possible), and error if we can't (BUT we do NOT error if we didnt attempt coercion (i.e.
        // if its impossible to coerce, like its impossible to coerce a string to a different type., so infer_hint is ignored there, etc). ).
        //
        // This function is NOT meant to type validating, it's a function that does 3 things:
        // 1. Resolve expression type(s) and ensure they are compatiable in complex expressions
        //    (like `range`, etc.)
        //
        // 2. Get expression "final" type. 
        // 3. **Attempt** to "soft" coerce an expression type to a hint. (only possible SOMETIME,
        //                                                                  and this is NOT full type validating)
        //
        //  It's still completely the caller's responsiblity to check if the returned type matches
        //  what he expects.
        //
        //  This function only does type validating on complex expressions like BinOp, Range, etc.
        //  to ensure Left and Right are same type, both integer/same types, etc. and error if not.
        //


        Expr::IntLiteral { value, span } => {
            if let Some(infer_hint) = infer_hint {

                // If the hint is an integer, we try to coerce and error if we can't. if hint is not integer, we simply return type of value
                if infer_hint.is_integer_type() {
                    *value = helpers::coerce_integer_literal_to_type_helper(infer_hint, *value, *span)?;
                }
            }
            Ok(value.get_type())
        }
        Expr::Float64Literal { .. } => Ok(Type::Float64),

        Expr::BoolLiteral { .. } => Ok(Type::Bool),

        Expr::StringLiteral { .. } => Ok(Type::String),
        
        Expr::ArrayLiteral { elements, span } => {
            // Array literals e.g. [1,2,3]
            // Array literals type determining method differs based on array elements:
            // - If the array is not empty, the first expression element type becomes the array type,
            //      and all other elements types are checked against it.
            //
            // - If the array is empty, then it is "ceorcied" into the same type as infer_hint.
            //   NOTE: If infer_hint is empty in this case, it would trigger a compiler bug guard
            //          panic. So be careful how you call this.
            //
            //

            // TODO: Add semantics blackbox tests to cover nested arrays.
            if !elements.is_empty() {
                let elem_ih = match infer_hint.clone() {
                    Some(Type::FixedArray(t, _)) => {
                        let fixed_inner = if t.is_array_type() {
                            // NOTE:: I think this is a bug... 
                            t.get_array_inner_most_type()
                        } else {
                            &*t
                        };
                        
                        let array_ty = infer_expr_type(&mut elements[0], locals, fun_sigs, Some(*t.clone()))?;

                        let arr_inner = if array_ty.is_array_type() {
                            array_ty.get_array_inner_most_type()
                        } else {
                            &array_ty
                        };

                        if fixed_inner != arr_inner {
                               return Err(HolyError::Semantic(format!(
                                    "Array literal is of `{}` type, but we expected `{}` type (line {} column {})",
                                    array_ty, t, span.line, span.column
                                )))
                        }

                        *t
                    }
                    Some(Type::Array(t)) => *t,
                    _ => {
                        // First element type
                        let t = infer_expr_type(&mut elements[0], locals, fun_sigs, None)?;
                        t
                    }
                };

                let elem_expected_ty = infer_expr_type(&mut elements[0], locals, fun_sigs, Some(elem_ih))?;


                for e in elements.iter_mut() {
                    let ety = infer_expr_type(e, locals, fun_sigs, Some(elem_expected_ty.clone()))?;
                    if ety != elem_expected_ty {
                        return Err(HolyError::Semantic(format!(
                            "Array element type mismatch: expected `{}` got `{}` (line {} column {})",
                            elem_expected_ty, ety, span.line, span.column
                        )));
                    }
                }

                match infer_hint.clone() {
                    Some(Type::FixedArray(t, size)) => {
                        let size_usize = match size {
                            FixedArraySize::Literal(n) => n,
                            FixedArraySize::Const(_) => panic!("(WORK-IN-PROGRESS) Consts are still unimplemented"),
                        };

                        if elements.len() != size_usize {
                            return Err(HolyError::Semantic(format!(
                                "Expected array of `{}` size, instead found with `{}` size (line {} column {})",
                                size_usize, elements.len(), span.line, span.column
                            )));
                        }

                
                        return Ok(Type::FixedArray(Box::new(*t.clone()), size));

                    },

                    _ => {}
                }



                return Ok(Type::Array(Box::new(elem_expected_ty.clone())))
            }

            if infer_hint.is_none() {
                return Ok(Type::Array(Box::new(Type::Int8)))
            }

            Ok(infer_hint.unwrap())

        }

        Expr::ArrayAccess { array, index,  span } => {
            if let Expr::Var { name, span: inner_span } = &**array {
                if let Some(info) = locals.get(name).cloned() {
                    if !info.ty.is_array_type() {
                        return Err(HolyError::Semantic(format!(
                                    "Array access on non-array variable `{}` of type `{}` (line {} column {})",
                                    name, info.ty, span.line, span.column)));
                    }

                    // Ensure that the type of the index expression is usize.
                    let ety = infer_expr_type(index, locals, fun_sigs, Some(Type::Usize))?;
                    if ety != Type::Usize {
                        return Err(HolyError::Semantic(format!(
                                    "Expected array index to be of type `usize`, instead we got `{}` (line {} column {})", 
                                    ety, span.line, span.column)));
                    }


                    match info.kind {
                        BindingKind::Var { moved, len, .. } => {
                            if moved {
                                return Err(HolyError::Semantic(format!(
                                            "Array access on moved variable `{}` (line {} column {})", 
                                            name, inner_span.line, inner_span.column
                                        )));
                            }

                                                        
                            // We only do the basic out-of-bounds checks if possible
                            // This is fine, because Rust is the one handling the actual safety down hood
                            //
                            // TODO: Though it'd still be nice if we improve upon this 
                            //
                            if len.is_some() {
                                check_usize_literal_to_src(&**index, len.unwrap(), span.clone(), locals.clone())?;
                            }
                        },
                        // Ownership rules dont apply to constants, so its fine to skip.
                        // Also, in constant evaluation, it will catch out of bounds access.
                        BindingKind::Const { .. } => {}
                    }


                    // Because we are accessing (or shall I say copying) a single element of an array
                    // we only care about the inner type, not the outer array type.
                    //
                    if let Type::Array(unarrayed_ty) = &info.ty {
                        Ok(*unarrayed_ty.clone())

                    } else if let Type::FixedArray(unarrayed_ty, _) = &info.ty {
                        Ok(*unarrayed_ty.clone())
                  
                    } else {
                        panic!("(Compiler bug) Expected array type, instead we got: {:?}", info.ty);
                    }

                } else {
                    Err(HolyError::Semantic(format!("Array access on undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
                }
            } else {
                return Err(HolyError::Semantic(format!(
                        "Expected variable of any `array` type, instead got an `{}` (line {} column {})", 
                        array, span.line, span.column
                    )));
            }

        }

        Expr::ArraySlicing { array, range,  span } => {
            if let Expr::Var { name, span: inner_span } = &**array {
                if let Some(info) = locals.get(name).cloned() {
                    if !info.ty.is_array_type() {
                        return Err(HolyError::Semantic(format!("Array access on non-array variable `{}` (line {} column {})", name, span.line, span.column)));
                    }

                    // If the range is FromTo (e.g. x[1:10])
                    // ensure that `start` index is not larger than `end`,
                    //
                    // This is **basic** out-of-bounds safety check against int literals.
                    // The real out-of-bounds safety guarantees is inserted in the binary machine code that'd panic if index is
                    // larger than array, thanks to rust.
                    //
                    match range {
                        ArraySliceRange::FromTo(start, end) => {
                            if let Expr::IntLiteral { value: IntLiteralValue::Usize(start_num), .. } = &**start {
                                if let Expr::IntLiteral { value: IntLiteralValue::Usize(end_num), .. } = &**end {
                                    if start_num > end_num {
                                        return Err(HolyError::Semantic(format!(
                                            "Start index `{}` cannot be larger than end index `{}` (line {} column {})", 
                                            start_num, end_num, span.line, span.column
                                        )));
                                    }
                                }
                            }

                            // Ensure that the type of the start index expression is usize, and try to
                            // convert it if possible.
                            let start_ety = infer_expr_type(start, locals, fun_sigs, Some(Type::Usize))?;
                            if start_ety != Type::Usize { 
                                return Err(HolyError::Semantic(format!(
                                                "Expected start index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                                name, start_ety, span.line, span.column
                                            )));
                            }

                            // Same as above, for end index.
                            let end_ety = infer_expr_type(end, locals, fun_sigs, Some(Type::Usize))?;
                            if end_ety != Type::Usize { 
                                return Err(HolyError::Semantic(format!(
                                            "Expected end index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                            name, end_ety, span.line, span.column
                                        )));
                            }
                        },

                        ArraySliceRange::From(start) => {
                            let start_ety = infer_expr_type(start, locals, fun_sigs, Some(Type::Usize))?;
                            if start_ety != Type::Usize { 
                                return Err(HolyError::Semantic(format!(
                                                "Expected start index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                                name, start_ety, span.line, span.column
                                            )));
                            }
                        },

                        ArraySliceRange::To(end) => {
                            // Same as above, for end index.
                            let end_ety = infer_expr_type(end, locals, fun_sigs, Some(Type::Usize))?;
                            if end_ety != Type::Usize { 
                                return Err(HolyError::Semantic(format!(
                                            "Expected end index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                            name, end_ety, span.line, span.column
                                        )));
                            }
                        }
                    }


                    match info.kind {
                        BindingKind::Var { moved, len, .. } => {
                            if moved {
                                return Err(HolyError::Semantic(format!(
                                            "Array access on moved variable `{}` (line {} column {})", 
                                            name, inner_span.line, inner_span.column
                                        )));
                            }

                            // We only do the basic out-of-bounds checks if possible
                            // This is fine, because Rust is the one handling the actual safety down hood
                            //
                            // TODO: Though it'd still be nice if we improve upon this 
                            //
                            if len.is_some() {
                                match range {
                                    ArraySliceRange::From(start) => {
                                        check_usize_literal_to_src(&start, len.unwrap(), span.clone(), locals.clone())?;
                                    },

                                    ArraySliceRange::To(end) => {
                                        check_usize_literal_to_src(&end, len.unwrap(), span.clone(), locals.clone())?;
                                    },

                                    ArraySliceRange::FromTo(start, end) => {
                                        check_usize_literal_to_src(&start, len.unwrap(), span.clone(), locals.clone())?;
                                        check_usize_literal_to_src(&end, len.unwrap(), span.clone(), locals.clone())?;
                                    }

                                }
                            }
                        },

                        // Ownership rules dont apply to constants, so its fine to skip.
                        BindingKind::Const { .. } => {}
                    }

                    if let Type::Array(_) = info.ty.clone() {
                        // We are fine returning Type wrapping in Array, because thats what the
                        // caller should expect anyway. x[s:e] always returns an array.
                        Ok(info.ty.clone())

                    } else if let Type::FixedArray(_, _) = &info.ty {
                        // Recursively converts fixed array and its inner type to dynamic arrays 
                        // if they're fixed arrays. 
                        //
                        let new_ty = info.ty.fixed_array_to_dynamic_array_type_full();
                        Ok(new_ty)

                    }  else {
                        panic!("(Compiler bug) Expected array type, instead we got: {:?}", info.ty);
                    }


                 } else {
                    Err(HolyError::Semantic(format!("Array access on undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
                }
            } else {
                return Err(HolyError::Semantic(format!(
                        "Expected variable of `array` or `fixed array` type, instead got `{}` (line {} column {})", 
                        array, span.line, span.column
                    )));
            }
        }
        Expr::Var{name, span} => {
            if let Some(info) = locals.get(name) {
                match info.kind {
                    BindingKind::Var { moved, .. } => {
                        if moved {
                            return Err(HolyError::Semantic(format!(
                                        "Use of moved variable `{}` (line {} column {})", 
                                        name, span.line, span.column
                                    )));
                        }
                    },
                    // Ownership rules don't apply to constants.
                    BindingKind::Const { .. } => {}
                }

                // TODO: Maybe also recursively check value type ?
                // not sure.
                //

                
                Ok(info.ty.clone())
            } else {
                Err(HolyError::Semantic(format!("Use of undeclared binding `{}` (line {} column {})", name, span.line, span.column)))
            }
        }


        Expr::UnaryOp{ op, expr, span } => {
            let expr_ty = infer_expr_type(expr, locals, fun_sigs, infer_hint.clone())?;
            
            // Ensure that negate unary operations is only allowed on floating points, and signed integers.
            if *op == UnaryOpKind::Negate {
                if !matches!(expr_ty, Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Float64) {
                    return Err(HolyError::Semantic(format!("type `{}` cannot have negate unary operation. (line {} column {})", expr_ty, span.line, span.column)))
                }
            }

            Ok(expr_ty)
        
        }

        Expr::BinOp { left, op, right, span } => {
            // infer both sides and try to convert to each other if possible (for integers and
            // floats literals only)
            //
            let (lty, rty) = advanced_infer_2_types(left, right, locals, fun_sigs, infer_hint.clone())?;
                
            if matches!(**left, Expr::CopyCall { .. }) || matches!(**right, Expr::CopyCall { .. }) {
                return Err(HolyError::Semantic(format!(
                        "Copying is not needed for variables in binary operations, because they're always copied. Remove the copy call. (line {} column {})", 
                        span.line, span.column)))
            }

            
            
            // Arthmetic binary operations
            //
            if matches!(op, BinOpKind::Add | BinOpKind::Subtract | BinOpKind::Multiply | BinOpKind::Divide) { 
                if lty != rty { 
                    return Err(HolyError::Semantic(format!("Type mismatch in binary arithmetic operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }

                if !lty.is_numeric_type() {
                    return Err(HolyError::Semantic(format!("Expected numeric types in binary arithmetic operation, instead we got: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }


                Ok(lty)

            // Logical 'OR' and 'AND'
            } else if matches!(op, BinOpKind::Or | BinOpKind::And) {
                if !( (lty == Type::Bool) && (rty == Type::Bool) ) {
                    return Err(HolyError::Semantic(format!(
                                "Logical binary operation require both expressions to be evalutable to type `bool`, but we got: `{}` vs `{}` (line {} column {})", 
                                lty, rty, span.line, span.column
                            )));
                }

                Ok(Type::Bool)

            // comparison
            } else if matches!(op, BinOpKind::Equal | BinOpKind::NotEqual ) {
                if lty != rty {
                    return Err(HolyError::Semantic(format!("Type mismatch in binary comparison operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }
                Ok(Type::Bool)
            
            // arthemtic comparison (greater than, less than, etc)
            } else if matches!(op, BinOpKind::Greater | BinOpKind::GreaterEqual | BinOpKind::Less | BinOpKind::LessEqual) {
                if lty != rty {
                    return Err(HolyError::Semantic(format!("Type mismatch in binary comparison operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }
                if !lty.is_numeric_type() {
                    return Err(HolyError::Semantic(format!("You cannot perform arithmetic comparison on non-numeric types: `{}` vs `{}`. (line {} column {})", lty, rty, span.line, span.column)));
                }

                Ok(Type::Bool)



            } else if matches!(op, BinOpKind::BitwiseShiftLeft | BinOpKind::BitwiseShiftRight | BinOpKind::BitwiseAnd | BinOpKind::BitwiseOr) {
                if lty != rty {
                    return Err(HolyError::Semantic(format!("Type mismatch in binary bitwise operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }

                // You can only perform bitwise operations on integer types
                if !lty.is_integer_type() {
                    return Err(HolyError::Semantic(format!("You cannot perform bitwise operations on non-integer types: `{}` vs `{}`. (line {} column {})", lty, rty, span.line, span.column)));
                }

                Ok(lty)


            } else {
                panic!("(Compiler bug) We got an unexpected BinOpKind: {:?}", op)
            }
        }

        Expr::RangeCall { start: start_expr, end: end_expr, span } => {
            
            let end_ty = infer_expr_type(end_expr, locals, fun_sigs, None)?;
            let start_ty = infer_expr_type(start_expr, locals, fun_sigs, Some(end_ty.clone()))?;


            if start_ty != end_ty {
                return Err(HolyError::Semantic(format!(
                        "Expected range arguments to be of the same type, instead we got: `{}` and `{}` (line {} column {})", 
                        start_ty, end_ty, span.line, span.column)))
            }


            if (!start_ty.is_integer_type()) || (!end_ty.is_integer_type()) {
                return Err(HolyError::Semantic(format!(
                        "Expected range arguments to be any Integer type, instead we got: `{}` and `{}` (line {} column {})", 
                        start_ty, end_ty, span.line, span.column)))
            }



            
            // start_ty is same as end_ty
            Ok(start_ty)
        }

        Expr::CopyCall { expr: e, span } => {

            // Catch the "makes no sense" calls (like nested copying, or copying of a literal,  or
            // array access, or a binary op where left and right are both literals)
            // and print helpful error messages
            // Basically, copy call only works on variables.
            match &mut **e {
                Expr::CopyCall {span: inner_span, ..} => {
                    return Err(HolyError::Semantic(format!("Double copying is not needed. Remove the extra copy call. (line {} column {})", inner_span.line, inner_span.column)))
                }
                Expr::IntLiteral{span: inner_span, ..} | 
                Expr::Float64Literal{span: inner_span, ..} | 
                Expr::BoolLiteral{span: inner_span, ..} | 
                Expr::StringLiteral{span: inner_span, ..} | 
                Expr::ArrayLiteral{span: inner_span, ..} => {
                    return Err(HolyError::Semantic(format!("Copying a literal is not needed. Remove the copy call and use the literal directly. (line {} column {})", inner_span.line, inner_span.column)))
                }
                Expr::ArrayAccess{span: inner_span, ..} | Expr::ArraySlicing{span: inner_span, ..} => {
                    return Err(HolyError::Semantic(format!(
                        "Copying is not needed for array access, when you access or slice an array or a string, a new copy is made. Remove the copy call and use operation directly. (line {} column {})", 
                        inner_span.line, inner_span.column)))
                }
                Expr::Var {..} => {
                    let e_ty = infer_expr_type(e, locals, fun_sigs, infer_hint.clone())?;
                    Ok(e_ty)
                }

                other => {
                    return Err(HolyError::Semantic(format!("Copy call expects a variable, instead we got `{}` (line {} column {})", other, span.line, span.column)))

                }
            }
            

        }

        Expr::FormatCall { template, expressions: exprs_vec, span: _} => {

            if !template.contains("{}") {
                panic!("(Compiler bug) We got a FormatCall Without any template placeholders, the parser should've not allowed this. template: `{:?}`, expressions: `{:?}`", template, exprs_vec);
            }

            for e in exprs_vec {
                // Catch the "makes no sense" calls, like only passing a literal in {..<expr>..} formating
                // placeholders
                match e {
                    Expr::CopyCall {span: inner_span, ..} => {
                        return Err(HolyError::Semantic(format!("Format calls copy by default, Remove the extra copy call. (line {} column {})", inner_span.line, inner_span.column)))
                    }
                    Expr::IntLiteral{span: inner_span, ..} | Expr::Float64Literal{span: inner_span, ..} | Expr::BoolLiteral{span: inner_span, ..} | Expr::StringLiteral{span: inner_span, ..} | Expr::ArrayLiteral{span: inner_span, ..}   => {
                        return Err(HolyError::Semantic(format!(
                                    "Plain literals are not allowed in formating! Remove the format placeholders and use the literal directly! (line {} column {})", 
                                    inner_span.line, inner_span.column
                                )))
                    }

                    Expr::FormatCall{span: inner_span, ..} => {
                        return Err(HolyError::Semantic(format!("Nested FormatCalls are not allowed. (line {} column {})", inner_span.line, inner_span.column)))
                    }



                    _ => {
                        // We call infer expr type here for it to validate the expression up to most
                        // upstream expression, and to see if types are compatiable if its a binop, and to
                        // see if variable exists in scope, etc etc.
                        // but we don't return inferred type obviously, the formatcall parent experession is
                        // always of type string.
                        //
                        // TODO: Maybe check if returned type can be converted to a string? or
                        // should I make everything printable? idk yet..
                        infer_expr_type(e, locals, fun_sigs, None)?;
                    }
                }
            }

            Ok(Type::String)

        }
        Expr::Call { name, args, span } => {
            let ret_opt = check_call(name, args, locals, fun_sigs, true, *span)?;
            match ret_opt {
                Some(ret_vec) => {
                    if ret_vec.len() == 1 {
                        Ok(ret_vec[0].clone())
                    } else {
                        Err(HolyError::Semantic(format!(
                            "Call to function `{}` returns {} values but is used in a single-value expression (line {} column {})",
                            name, ret_vec.len(), span.line, span.column
                        )))
                    }
                }
                None => {
                    // check_call should already error when require_ret == true,
                    // but to be defensive:
                    Err(HolyError::Semantic(format!(
                        "Call to function `{}` has no declared return type but is used in an expression (line {} column {})",
                        name, span.line, span.column
                    )))
                }
            }
        }
    }
}



// helper: check an expression that's allowed to be an IntLiteral::Usize
pub fn check_usize_literal_to_src(expr: &Expr, len: usize, span: Span, locals: HashMap<String, BindingInfo>) -> Result<(), HolyError> {
    match expr {
        Expr::IntLiteral { value, .. } => match value {
            IntLiteralValue::Usize(n) => {
                if *n >= len {
                    return Err(HolyError::Semantic(format!(
                        "Index `{}` is out-of-bounds for array length `{}`! Out-of-bounds access will cause a forced panic at runtime! Always check your array length before accessing it! (line {} column {})",
                        n, len, span.line, span.column
                    )));
                }
                Ok(())
            }
            other => panic!(
                "(Compiler bug) expected IntLiteral::Usize, got {:?}. This should've been caught by other semantic checks.",
                other
            ),
        },

        // TODO IMPORTANT NOTE: If later in transcompile stage, even with all rust's optimization
        // disabled, rust errors at compile-time because a BinOp expression makes a literal go out
        // of bounds and rust catches it,
        // then you're gonna have to uncomment this and parse left and right expressions to ensure
        // they do n ot go over len.
        // Expr::BinOp { .. } => Ok(()), // allow expressions evaluated at runtime
        // 
        // Or, just, you know, propgate errors back to user nicely, so we dont have to re-implement
        // safety checks already guaranteed by rust in generated binary
                                      
        Expr::Var {name, ..} => {
            if let Some(inner_info) = locals.get(name).cloned() {
                match inner_info.kind {
                    BindingKind::Var { value, .. } => {
                        if value.is_none() {
                            // This could happen if the most upstream source is a function call. We just
                            // return Ok.
                            return Ok(());
                        }
                        check_usize_literal_to_src(&value.unwrap(), len, span, locals)?;

                        Ok(())
                    },
                    BindingKind::Const { .. } => panic!("Still unimplemented for consts")
                }
            } else {
                panic!("(Compiler bug) We could not find variable `{}` in in `locals`. This should've been caught by other semantic checks, but that didnt happen..", name);
            }
        },

        Expr::CopyCall{..} => Err(HolyError::Semantic(format!(
                        "You do not need to Copy an index when you are accessing an array, it is always copied. Remove the copy call. (line {} column {})"
                        ,span.line, span.column
                    ))),


        // If it's not a literal, like, a function call, etc. We just assume it's within range
        // Rust will insert checks in the compiled binary that'd panic if you try to go
        // out-of-bounds.
        _ => Ok(())
            
    }
}





