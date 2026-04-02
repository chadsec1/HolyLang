use super::*;


/// This takes 2 expressions, and tries to infer types, and convert each other types to same if
/// possible (only for integer literlas and float literals)
///
pub fn advanced_infer_2_types(
    left: &mut Expr, 
    right: &mut Expr,
    locals: &mut HashMap<String, VarInfo>,
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
    
    // Float literal inferrence
    } else if matches!(*left, Expr::FloatLiteral {..}) && !matches!(*right, Expr::FloatLiteral {..}) {
        lty = infer_expr_type(left, locals, fun_sigs, Some(rty.clone()))?;

    } else if matches!(*right, Expr::FloatLiteral {..}) && !matches!(*left, Expr::FloatLiteral {..}) {
        rty = infer_expr_type(right, locals, fun_sigs, Some(lty.clone()))?;
    


    } else if lty.is_integer_type() && rty.is_integer_type() {
        // If lty and rty are both integer types, we get the bigger type of them, and try force it
        // upon both lty and rty.
        let bigger_type = helpers::get_bigger_type_of_two_integers(lty.clone(), rty.clone());

        rty = infer_expr_type(right, locals, fun_sigs, Some(bigger_type.clone()))?;
        lty = infer_expr_type(left, locals, fun_sigs, Some(bigger_type.clone()))?;


    } else if lty.is_floating_type() && rty.is_floating_type() {
        // Same thing as above, except its for floating points.
        let bigger_type = helpers::get_bigger_type_of_two_floatings(lty.clone(), rty.clone());

        rty = infer_expr_type(right, locals, fun_sigs, Some(bigger_type.clone()))?;
        lty = infer_expr_type(left, locals, fun_sigs, Some(bigger_type.clone()))?;

    }

    Ok((lty, rty))


}


/// Infer the type of an expression, and update literal nodes (and nested nodes) where possible.
/// Returns the deduced Type for the expression.
pub fn infer_expr_type(
    expr: &mut Expr,
    locals: &mut HashMap<String, VarInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    infer_hint: Option<Type>
) -> Result<Type, HolyError> {
    match expr {

        // Note: If infer hint is set, we alter the value to fit the hint, if we can.
        Expr::IntLiteral { value, span } => {
            if infer_hint.is_some() {
                let infer_hint = infer_hint.unwrap();
                match infer_hint {
                    Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Usize | Type::Byte | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128 => {
                        *value = infer_integer_literal_helper(infer_hint, *value, *span)?;
                    }

                    _ => {}
                    
                }
            }
            Ok(value.get_type())
        }
        Expr::FloatLiteral { value, span } => {
            if infer_hint.is_some() {
                match infer_hint.unwrap() {
                    Type::Float32 => {
                        if let FloatLiteralValue::Float64(_) = value {
                            return Err(HolyError::Semantic(format!("Float literal has a float64 value but we expected a float32 value (line {} column {})", span.line, span.column)));
                        }
                    }
                    Type::Float64 => {
                        if let FloatLiteralValue::Float32(f) = value {
                            *value = FloatLiteralValue::Float64(f.clone() as f64);
                        }
                    }
                    _ => {}
             
                }
            }

            Ok(value.get_type())
        }

        Expr::BoolLiteral { value: _, span: _ } => {
            Ok(Type::Bool)
        }

        Expr::StringLiteral { value: _, span: _ } => {
            Ok(Type::String)
        }
        
        Expr::ArrayLiteral { elements, array_ty,  span } => {
            // all elements must have same type as the array type
            for e in elements.iter_mut() {
                let ety = infer_expr_type(e, locals, fun_sigs, Some(array_ty.clone()))?;
                if !helpers::type_compatible(&ety, &array_ty) {

                    return Err(HolyError::Semantic(format!(
                        "Array element type mismatch: expected `{}` got `{}` (line {} column {})",
                        array_ty, ety, span.line, span.column
                    )));
                }
            }

            Ok(Type::Array(Box::new(array_ty.clone())))
        }

        Expr::ArraySingleAccess { array, index,  span } => {
            if let Expr::Var { name, span: inner_span } = &**array {
                if let Some(info) = locals.get(name).cloned() {
                    if info.moved {
                        return Err(HolyError::Semantic(format!(
                                    "Array access on moved variable `{}` (line {} column {})", 
                                    name, inner_span.line, inner_span.column
                                )));
                    }

                
                    // Ensure that the type of the index expression is usize.
                    let ety = infer_expr_type(index, locals, fun_sigs, Some(Type::Usize))?;
                    if !helpers::type_compatible(&ety, &Type::Usize) {
                        return Err(HolyError::Semantic(format!("Expected array index to be of type `usize`, instead we got `{}` (line {} column {})", ety, span.line, span.column)));
                    }


                    if !matches!(&info.ty, Type::Array(_)) {
                        return Err(HolyError::Semantic(format!("Array access on non-array variable `{}` of type `{}` (line {} column {})", name, info.ty, span.line, span.column)));
                    }

                    // We only do the basic out-of-bounds checks if possible
                    // This is fine, because Rust is the one handling the actual safety down hood
                    //
                    // TODO: Though it'd still be nice if we improve upon this 
                    //
                    if info.len.is_some() {
                        check_usize_literal_to_src(&**index, info.len.unwrap(), span.clone(), locals.clone())?;
                    }

                    // Because we are accessing (or shall I say copying) a single element of an array
                    // we only care about the inner type, not the outer array type.
                    if let Type::Array(unarrayed_ty) = &info.ty {
                        Ok(*unarrayed_ty.clone())
                    } else {
                        panic!("(Compiler bug) Expected array type, instead we got: {:?}", info.ty);
                    }

                } else {
                    Err(HolyError::Semantic(format!("Array access on undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
                }
            } else {
                return Err(HolyError::Semantic(format!(
                        "You can only access declared array variables  (line {} column {})", 
                        span.line, span.column
                    )));
            }

        }

        Expr::ArrayMultipleAccess { array, start, end,  span } => {
            if let Expr::Var { name, span: inner_span } = &**array {
                if let Some(info) = locals.get(name).cloned() {
                    if info.moved {
                        return Err(HolyError::Semantic(format!(
                                    "Array access on moved variable `{}` (line {} column {})", 
                                    name, inner_span.line, inner_span.column
                                )));
                    }


                    if start.is_none() && end.is_none() {
                        panic!("(Compiler bug) We expected the parser to not allow such invalid syntax of no start and no end indexes in array multiple access");
                    }

                    if !matches!(&info.ty, Type::Array(_)) {
                        return Err(HolyError::Semantic(format!("Array access on non-array variable `{}` (line {} column {})", name, span.line, span.column)));
                    }


                    // We only do the basic out-of-bounds checks if possible
                    // This is fine, because Rust is the one handling the actual safety down hood
                    //
                    // TODO: Though it'd still be nice if we improve upon this 
                    //
                    if info.len.is_some() {
                        if let Some(s) = &mut *start {
                            // Ensure that the type of the start index expression is usize, and try to
                            // convert it if possible.
                            let start_ety = infer_expr_type(s, locals, fun_sigs, Some(Type::Usize))?;
                            if !helpers::type_compatible(&start_ety, &Type::Usize) {
                                return Err(HolyError::Semantic(format!(
                                            "Expected start index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                            start_ety, name, span.line, span.column
                                        )));
                            }

                            check_usize_literal_to_src(&s, info.len.unwrap(), span.clone(), locals.clone())?;
                        }
     
                        if let Some(e) = &mut *end {
                            // Same as above, for end index.
                            let end_ety = infer_expr_type(e, locals, fun_sigs, Some(Type::Usize))?;
                            if !helpers::type_compatible(&end_ety, &Type::Usize) {
                                return Err(HolyError::Semantic(format!(
                                            "Expected end index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                            end_ety, name, span.line, span.column
                                        )));
                            }


                            check_usize_literal_to_src(&e, info.len.unwrap(), span.clone(), locals.clone())?;
                        }
                    }


                    // If both start and end are present, ensure that start is not larger than end,
                    // and end not smaller than start.
                    // This is **basic** out-of-bounds safety check against int literals.
                    // The real out-of-bounds safety guarantees is inserted in the binary machine code that'd panic if index is
                    // larger than array, thanks to rust.
                    if start.is_some() && end.is_some() {
                        if let Expr::IntLiteral { value: IntLiteralValue::Usize(start_num), .. } = start.as_deref().unwrap() {

                            if let Expr::IntLiteral { value: IntLiteralValue::Usize(end_num), .. } = end.as_deref().unwrap() {
                                if start_num > end_num {
                                    return Err(HolyError::Semantic(format!(
                                                "Start index `{}` cannot be larger than end index `{}` (line {} column {})", 
                                                start_num, end_num, span.line, span.column
                                            )));
                                }
                                // just to be defensive:
                                if end_num < start_num {
                                    return Err(HolyError::Semantic(format!(
                                                "End index `{}` cannot be larger than start index `{}` (line {} column {})", 
                                                end_num, start_num, span.line, span.column
                                            )));
                                }
                            }
                        }
                    }


                    if let Type::Array(_) = info.ty.clone() {
                        // We are fine returning Type wrapping in Aray, because thats what the
                        // caller should expect anyway. x[s:e] always returns an array.
                        Ok(info.ty.clone())
                    }  else {
                        panic!("(Compiler bug) Expected array type, instead we got: {:?}", info.ty);
                    }
                } else {
                    Err(HolyError::Semantic(format!("Array access on undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
                }
            } else {
                return Err(HolyError::Semantic(format!(
                        "You can only access arrays via variables  (line {} column {})", 
                        span.line, span.column
                    )));
            }
        }
        Expr::Var{name, span} => {
            if let Some(info) = locals.get(name) {
                if info.moved {
                    return Err(HolyError::Semantic(format!(
                                "Use of moved variable `{}` (line {} column {})", 
                                name, span.line, span.column
                            )));
                }

                // TODO: Maybe also recursively check value type ?
                // not sure.
                
                Ok(info.ty.clone())
            } else {
                
                validate_identifier_name(name)
                    .map_err(|_| 
                        HolyError::Semantic(format!("Invalid syntax `{}` (line {} column {})", name, span.line, span.column)))?;
                
                Err(HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
            }
        }


        Expr::UnaryOp{ op, expr, span } => {
            let ety = infer_expr_type(expr, locals, fun_sigs, infer_hint)?;
            
            // Ensure that no negate unary operation is allowed on an unsigned integer.
            if *op == UnaryOpKind::Negate {
                if !matches!(ety, Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Float32 | Type::Float64) {
                    return Err(HolyError::Semantic(format!("{} cannot have negate unary operation. (line {} column {})", ety, span.line, span.column)))
                }
            }

            Ok(ety)
        
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

            if lty == Type::Infer || rty == Type::Infer {
                panic!("(Compiler bug) lty and or rty are of type Infer even after we tried to infer: Left: {:?} Right: {:?}", **left, **right);
            }

            
            // Check if lty or rty are of types that cannot have arithmetic performed on.
            if matches!(lty, Type::String | Type::Bool | Type::Array(_) ) || matches!(rty, Type::String | Type::Bool | Type::Array(_) ) {
                if matches!(op, BinOpKind::Add | BinOpKind::Subtract | BinOpKind::Multiply | BinOpKind::Divide | BinOpKind::Greater | BinOpKind::GreaterEqual | BinOpKind::Less | BinOpKind::LessEqual) {
                    return Err(HolyError::Semantic(format!("You cannot perform arithmetic on types: `{}` vs `{}`. (line {} column {})", lty, rty, span.line, span.column)));
                }
            }

            // arthmetic
            if matches!(op, BinOpKind::Add | BinOpKind::Subtract | BinOpKind::Multiply | BinOpKind::Divide ) {
                // This checks type equality, etc.
                let t = helpers::resolve_binary_op_types_numeric(&lty, &rty, span)?;
                Ok(t)

            // boolean comparison
            } else if matches!(op, BinOpKind::Equal | BinOpKind::NotEqual | BinOpKind::Greater | BinOpKind::GreaterEqual | BinOpKind::Less | BinOpKind::LessEqual ) {
                if lty != rty {
                    return Err(HolyError::Semantic(format!("Type mismatch in binary comparison operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
                }
                Ok(Type::Bool)
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
                Expr::FloatLiteral{span: inner_span, ..} | 
                Expr::BoolLiteral{span: inner_span, ..} | 
                Expr::StringLiteral{span: inner_span, ..} | 
                Expr::ArrayLiteral{span: inner_span, ..} => {
                    return Err(HolyError::Semantic(format!("Copying a literal is not needed. Remove the copy call and use the literal directly. (line {} column {})", inner_span.line, inner_span.column)))
                }
                Expr::ArraySingleAccess{span: inner_span, ..} | Expr::ArrayMultipleAccess{span: inner_span, ..} => {
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
                panic!("(Compielr bug) We got a FormatCall Without any template placeholders, the parser should've not allowed this. template: `{:?}`, expressions: `{:?}`", template, exprs_vec);
            }

            for e in exprs_vec {
                // Catch the "makes no sense" calls, like only passing a literal in {..<expr>..} formating
                // placeholders
                match e {
                    Expr::CopyCall {span: inner_span, ..} => {
                        return Err(HolyError::Semantic(format!("Format calls copy by default, Remove the extra copy call. (line {} column {})", inner_span.line, inner_span.column)))
                    }
                    Expr::IntLiteral{span: inner_span, ..} | Expr::FloatLiteral{span: inner_span, ..} | Expr::BoolLiteral{span: inner_span, ..} | Expr::StringLiteral{span: inner_span, ..} | Expr::ArrayLiteral{span: inner_span, ..}   => {
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
pub fn check_usize_literal_to_src(expr: &Expr, len: usize, span: Span, locals: HashMap<String, VarInfo>) -> Result<(), HolyError> {
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
                if inner_info.value.is_none() {
                    // This could happen if the most upstream source is a function call. We just
                    // return Ok.
                    return Ok(());
                }
                check_usize_literal_to_src(&inner_info.value.unwrap(), len, span, locals)?;

                Ok(())
            } else {
                panic!("(Compiler bug) We could not find variable `{}` in in `locals`. This should've been caught by other semantic checks, but that didnt happen..", name);
            }
        },

        Expr::CopyCall{expr: e, ..} => {
            check_usize_literal_to_src(e, len, span, locals)?;
            Ok(())
        }


        // If it's not a literal, like, a function call, etc. We just assume it's within range
        // Rust will insert checks in the compiled binary that'd panic if you try to go
        // out-of-bounds.
        _ => Ok(())
            
    }
}


pub fn infer_integer_literal_helper(infer_ty: Type, value: IntLiteralValue, span: Span) -> Result<IntLiteralValue, HolyError> {

    if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Usize | Type::Byte | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128) {
        panic!("(Compiler bug) Value {} has unknown type", value);
    }

    match infer_ty {
        Type::Int8 => {
            if !value.is_signed() {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();
            if val_raw < i8::MIN as i128 || val_raw > i8::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int8(val_raw as i8))
        }

        Type::Int16 => {
            if !value.is_signed() {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();

            if val_raw < i16::MIN as i128 || val_raw > i16::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int16(val_raw as i16))
        }
        Type::Int32 => {
            if !value.is_signed() {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();
            if val_raw < i32::MIN as i128 || val_raw > i32::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int32(val_raw as i32))
        }
        Type::Int64 => {
            if !value.is_signed() {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();

            if val_raw < i64::MIN as i128 || val_raw > i64::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int64(val_raw as i64))
        }

        Type::Int128 => {
            if !value.is_signed() {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();

            if val_raw < i128::MIN || val_raw > i128::MAX {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int128(val_raw))
        }


        // Since we dont store numbers with negative sign, only wrapped in a negate node, we can
        // actually skip type check and happily infer signed numbers as unsigned if need be.
        //
        // And since u128 can represent all signed numbers assuming no -, that's handled by upper
        // negate node, it should be safe to cast as u128 regardless.
        //


        Type::Usize => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > usize::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Usize(val_raw as usize))
        }

        Type::Byte => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > u8::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Byte(val_raw as u8))
        }

        Type::Uint16 => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > u16::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint16(val_raw as u16))
        }

        Type::Uint32 => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > u32::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint32(val_raw as u32))
        }

        Type::Uint64 => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > u64::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint64(val_raw as u64))
        }
        Type::Uint128 => {
            let val_raw: u128;

            if value.is_signed() {
                if value.as_i128() < 0 {
                    return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}` and is negative, which cannot become type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
                }

                val_raw = value.as_i128() as u128;
            } else {
                val_raw = value.as_u128();
            }

            if val_raw > u128::MAX {
                return Err(HolyError::Semantic(format!("Integer literal `{}` out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint128(val_raw))
        }

        other => {
            panic!("(Compiler bug) You must ensure type is an integer literal before passing it to this function. we got: {:?}", other);
        }
    }
}

// Function to attempt to change an expression's literal to match `ty`. If fails, it errors.
// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
pub fn assign_infer_type_to_expr_value(expr: &mut Expr, ty: Type) -> Result<(), HolyError> {
    match expr {
        Expr::IntLiteral { value, span } => {
            match ty {
                Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Byte | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128 | Type::Usize => {
                    *value = infer_integer_literal_helper(ty, *value, *span)?;
                }

                _ => {
                    return Err(HolyError::Semantic(format!("Cannot assign integer literal to non-integer type `{}` (line {} column {})", ty, span.line, span.column)));
                }
            }

            return Ok(());
        }
        Expr::FloatLiteral { value, span } => {
            match ty {
                Type::Float32 => {
                    if let FloatLiteralValue::Float64(_) = value {
                        return Err(HolyError::Semantic(format!("Float literal has a float64 value but we expected a float32 value (line {} column {})", span.line, span.column)));
                    }
                }
                Type::Float64 => {
                    if let FloatLiteralValue::Float32(f) = value {
                        *value = FloatLiteralValue::Float64(f.clone() as f64);
                    }
                }
                _ => {
                        return Err(HolyError::Semantic(format!("Cannot assign float literal to non-float type `{}` (line {} column {})", ty, span.line, span.column)));
                    }

            }

        }
        // For non-literals, nothing to assign, and that's OK.
        _ => {}
    }

    Ok(())
}


