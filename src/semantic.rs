use std::collections::HashMap;

use crate::error::HolyError;
use crate::parser::{
    AST, Expr, Function, Param, Stmt, Type, Variable, Span, validate_identifier_name, IntLiteralValue, FloatLiteralValue, UnaryOpKind
};


#[derive(Clone)]
struct VarInfo {
    ty: Type,
    moved: bool,
}

/// Public entry: check semantics and fill in inferred types where possible.
/// This mutates the AST: Type::Infer are replaced with concrete types where
/// they can be inferred.
pub fn check_semantics(ast: &mut AST) -> Result<(), HolyError> {
    // First pass: collect function signatures (params types and return types (possible multiple))
    let mut fun_sigs: HashMap<String, (Vec<Type>, Option<Vec<Type>>)> = HashMap::new();
    for f in &ast.functions {
        let param_tys = f.params.iter().map(|p| p.type_name.clone()).collect();
        if fun_sigs.insert(f.name.clone(), (param_tys, f.return_type.clone())).is_some() {
            return Err(HolyError::Semantic(format!("Duplicate function declaration: {}", f.name)));
        }
    }

    // Second pass: check each function body
    for func in &mut ast.functions {
        check_function(func, &fun_sigs)?;
    }

    Ok(())
}

/// Check single function: infer local var types where possible, check calls, returns.
fn check_function(func: &mut Function, fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>) -> Result<(), HolyError> {
    // Build local symbol table starting with params
    let mut locals: HashMap<String, VarInfo> = HashMap::new();
    for p in &func.params {
        locals.insert(
            p.name.clone(), 
            VarInfo {
                ty: p.type_name.clone(),
                moved: false,
            });
    }
    
    // Ensure that no code exists after return
    if let Some(last_ret_pos) = func.body.iter().rposition(|s| matches!(s, Stmt::Return(_))) {
        if last_ret_pos + 1 < func.body.len() {
            // helper to get the span of a statement (so we can point to offending code)
            fn stmt_span(s: &Stmt) -> Span {
                match s {
                    Stmt::VarDecl(v) => v.span,
                    Stmt::VarAssign(a) => a.span,
                    Stmt::Expr(e) => expr_span(e),
                    Stmt::Return(e) => expr_span(&e[0]), // First return element is always present
                                                        // if there is a return
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
                    Expr::Var { span, .. } => *span,
                    Expr::BinOp { span, .. } => *span,
                    Expr::UnaryOp { span, .. } => *span,
                    Expr::Call { span, .. } => *span,
                }
            }

            let offending_span = stmt_span(&func.body[last_ret_pos + 1]);
            return Err(HolyError::Semantic(format!(
                "Code after `return` is not allowed (line {} column {})",
                offending_span.line, offending_span.column
            )));
        }
    }


    // Walk statements in order. This is a single-pass inference (simple)
    for stmt in &mut func.body {
        match stmt {
            Stmt::VarDecl(var) => {
                // If var has explicit type: keep it. If Infer: try infer from initializer.
                if var.type_name == Type::Infer {
                    if let Some(expr) = &mut var.value {
                        let ty = infer_expr_type(expr, &mut locals, fun_sigs, None)?;
                        // assign inferred type to variable and propagate to literal nodes if needed
                        var.type_name = ty.clone();
                    } else {
                        return Err(HolyError::Semantic(format!(
                            "Variable `{}` requires explicit type when no initializer is provided in function `{}` (line {} column {})",
                            var.name, func.name, var.span.line, var.span.column
                        )));
                    }
                } else {
                    // explicit type: if initializer present, ensure initializer is compatible
                    
                    if let Some(expr) = &mut var.value {
                        assign_infer_type_to_expr(expr, var.type_name.clone())
                            .map_err(|e| HolyError::Semantic(format!("{} in function `{}`", e, func.name)))?;


                        // Now infer/check the expression type as usual.
                        let expr_ty = infer_expr_type(expr, &mut locals, fun_sigs, Some(var.type_name.clone()))?;
                        if !type_compatible(&expr_ty, &var.type_name) {
                            return Err(HolyError::Semantic(format!(
                                "Type mismatch assigning to `{}`: got `{}`, expected `{}` in function `{}` (line {} column {})",
                                var.name, expr_ty, var.type_name, func.name, var.span.line, var.span.column
                            )));
                        }

                    } else {
                        assign_default_value_for_type(&mut var.value, &var.type_name, var.span)?;
                    }
                }


                // Check if source value is a variable and if it is moved. 
                // And moves it
                if let Some(Expr::Var { name: src_name, span }) = &var.value {
                    let src = locals.get_mut(src_name).ok_or_else(|| {
                        HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", src_name, span.line, span.column))
                    })?;

                    if src.moved {
                        return Err(HolyError::Semantic(format!("Use of moved variable `{}` (line {} column {})", src_name, span.line, span.column)));
                    }

                    // mark source as moved because ownership was transferred
                    src.moved = true;
                }

                // register variable in locals (now with concrete type)
                locals.insert(
                    var.name.clone(),
                    VarInfo {
                        ty: var.type_name.clone(),
                        moved: false,
                    },
                );
            }

            Stmt::VarDeclMulti(var_list, call_expr) => {
                // Expect the rhs to be a Call
                if let Expr::Call { name, args, span } = call_expr {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_opt = check_call(name, args, &mut locals, fun_sigs, true, *span)?;
                    let ret_vec = ret_opt.ok_or_else(|| HolyError::Semantic(format!(
                        "Call to `{}` used in multi-declaration but has no return types (line {} column {})",
                        name, span.line, span.column
                    )))?;

                    if ret_vec.len() != var_list.len() {
                        return Err(HolyError::Semantic(format!(
                            "Return length mismatch: {} variables on the left side, but the call returns {} values (line {} column {})",
                            var_list.len(), ret_vec.len(), span.line, span.column
                        )));
                    }

                    // assign types and register locals
                    for (var, ret_ty) in var_list.iter_mut().zip(ret_vec.iter()) {
                        if var.type_name == Type::Infer {
                            var.type_name = ret_ty.clone();
                        } else if !type_compatible(&var.type_name, ret_ty) {
                            return Err(HolyError::Semantic(format!(
                                "Type mismatch for variable `{}`: declared `{}` but call returns `{}` (line {} column {})",
                                var.name, var.type_name, ret_ty, var.span.line, var.span.column
                            )));
                        }

                        // insert into locals
                        locals.insert(
                            var.name.clone(),
                            VarInfo { ty: var.type_name.clone(), moved: false }
                        );
                    }
                } else {
                    return Err(HolyError::Semantic(format!(
                        "Multi-declarement requires only a single call on the right-hand side",
                        // call_expr.span.line, call_expr.span.column
                    )));
                }
            }

            Stmt::VarAssign(assign) => {
                let varinfo = locals.get(&assign.name).ok_or_else(|| {
                    HolyError::Semantic(format!(
                        "Use of undeclared variable `{}` (line {} column {})",
                        assign.name, assign.span.line, assign.span.column
                    ))
                })?.clone();

                let expr_ty = infer_expr_type(&mut assign.value.clone(), &mut locals, fun_sigs, None)?;
                if !type_compatible(&expr_ty, &varinfo.ty) {
                    return Err(HolyError::Semantic(format!(
                        "Cannot assign `{}` to `{}` of type `{}` (line {} column {})",
                        expr_ty, assign.name, varinfo.ty, assign.span.line, assign.span.column
                    )));
                }


                if varinfo.moved {
                    return Err(HolyError::Semantic(format!(
                        "Value assignment to moved variable `{}` (line {} column {})",
                        assign.name, assign.span.line, assign.span.column
                    )));
                }
                // Check if source value is a variable and if it is moved. 
                // And moves it
                if let Expr::Var { name: src_name, span } = &assign.value {
                    let src = locals.get_mut(src_name).ok_or_else(|| {
                        HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", src_name, span.line, span.column))
                    })?;

                    if src.moved {
                        return Err(HolyError::Semantic(format!("Use of moved variable `{}` (line {} column {})", src_name, span.line, span.column)));
                    }

                    // mark source as moved because ownership was transferred
                    src.moved = true;
                }

            }


            Stmt::VarAssignMulti(expr) => {
                
                if let Expr::Call { name, args, span } = &mut expr.value {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_opt = check_call(name, args, &mut locals, fun_sigs, true, *span)?;
                    let ret_vec = ret_opt.ok_or_else(|| HolyError::Semantic(format!(
                        "Call to `{}` used in multi-assignment but has no return types (line {} column {})",
                        name, span.line, span.column
                    )))?;

                    if ret_vec.len() != expr.names.len() {
                        return Err(HolyError::Semantic(format!(
                            "Return length mismatch: {} variables on the left side, but the call returns {} values (line {} column {})",
                            expr.names.len(), ret_vec.len(), span.line, span.column
                        )));
                    }

                    // assign types and register locals
                    for (var_name, ret_ty) in expr.names.iter_mut().zip(ret_vec.iter()) {
                        let varinfo = locals.get(var_name).ok_or_else(|| {
                            HolyError::Semantic(format!(
                                "Use of undeclared variable `{}` (line {} column {})",
                                var_name, expr.span.line, expr.span.column
                            ))
                        })?.clone();

                        if varinfo.moved {
                            return Err(HolyError::Semantic(format!(
                                "Value assignment to moved variable `{}` (line {} column {})",
                                var_name, expr.span.line, expr.span.column
                            )));
                        }

                        if !type_compatible(&varinfo.ty, ret_ty) {
                            return Err(HolyError::Semantic(format!(
                                "Type mismatch for variable `{}`: declared `{}` but call returns `{}` (line {} column {})",
                                var_name, varinfo.ty, ret_ty, expr.span.line, expr.span.column
                            )));
                        }

                    }


                } else {
                    return Err(HolyError::Semantic(format!(
                        "Multi-assignment requires only a single call on the right-hand side (line {} column {})",
                        expr.span.line, expr.span.column
                    )));
                }
            }

            Stmt::Expr(expr) => {
                if let Expr::Call { name, args, span } = expr {
                    // allow void calls as statements (require_ret = false)
                    let _ = check_call(name, args, &mut locals, fun_sigs, false, *span)?;
                    // no returned type expected; ok to ignore
                } else {
                    // other expressions-as-statements: fully type-check
                    let _ = infer_expr_type(expr, &mut locals, fun_sigs, None)?;
                }
            }

            Stmt::Return(expr_vec) => {
                // If function has no declared return type, we error.
                match &func.return_type {
                    None => {
                        return Err(HolyError::Semantic(format!(
                            "Function `{}` contains return but has no declared return type (line {} column {})",
                            func.name,
                            func.span.line,
                            func.span.column,
                        )));
                    }
                    Some(declared_ty_vec) => {

                        if declared_ty_vec.len() != expr_vec.len() {
                            return Err(HolyError::Semantic(format!(
                                    "Return length mismatch in `{}`: got `{}` expressions, expected `{}` expressions (line {} column {})",
                                    func.name, expr_vec.len(), declared_ty_vec.len(), func.span.line, func.span.column,
                                )));
                        
                        }

                        for (i, expr) in expr_vec.iter_mut().enumerate() {
                            let expr_ty = infer_expr_type(expr, &mut locals, fun_sigs, None)?;

                            let declared_ty = declared_ty_vec[i].clone();

                            if !type_compatible(&expr_ty, &declared_ty) {
                                return Err(HolyError::Semantic(format!(
                                    "Return type mismatch in `{}`: got `{}`, expected `{}` (line {} column {})",
                                    func.name, expr_ty, declared_ty_vec[i], func.span.line, func.span.column,
                                )));
                            }
                            
                        }
                    }
                }
            }

            Stmt::Func(_) => {
                // nested function nodes aren't used in our current parser; ignore or error
                // we'll ignore for now
                // and honestly, that's a good thing. HolyLang shall not have nested
                // functions (ewwww)
            }
        }
    }


    if let Some(ret_ty) = &func.return_type {
        match func.body.last() {
            Some(Stmt::Return(_)) => {},
            _ => {
                return Err(HolyError::Semantic(format!(
                    "Function `{}` declares return type {:?} but does not end with a `return` (line {} column {})",
                    func.name,
                    ret_ty,
                    func.span.line,
                    func.span.column,
                )));
            }
        }
    }


    Ok(())
}

/// Validate a call's arguments, infer literal arg types to parameter types,
/// and apply move semantics for variable args.
///
/// - `require_ret`: if true, the function must declare a return type (otherwise Err).
/// - returns `Ok(Some(return_type))` when function has a return type,
///   `Ok(None)` when no return type (allowed only when `require_ret == false`).
fn check_call(
    name: &str,
    args: &mut [Expr],
    locals: &mut HashMap<String, VarInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    require_ret: bool,
    span: Span,
) -> Result<Option<Vec<Type>>, HolyError> {
    // lookup signature
    let sig = fun_sigs.get(name).ok_or_else(|| {
        HolyError::Semantic(format!("Call to unknown function `{}` (line {} column {})", name, span.line, span.column))
    })?;
    let (param_tys, ret_ty_opt) = sig;

    // arity check
    if args.len() != param_tys.len() {
        return Err(HolyError::Semantic(format!(
            "Function `{}` expects {} arguments, got {} (line {} column {})",
            name,
            param_tys.len(),
            args.len(),
            span.line,
            span.column,
        )));
    }

    // check each arg type and apply moves
    for (arg_expr, param_ty) in args.iter_mut().zip(param_tys.iter()) {
        let arg_ty = infer_expr_type(arg_expr, locals, fun_sigs, None)?;
        if arg_ty == Type::Infer {
            // assign literal inference to param type when possible
            assign_infer_type_to_expr(arg_expr, param_ty.clone())?;
        } else if !type_compatible(&arg_ty, param_ty) {
            return Err(HolyError::Semantic(format!(
                "Argument type mismatch in call to `{}`: expected `{}`, got `{}` (line {} column {})",
                name, param_ty, arg_ty, span.line, span.column,
            )));
        }

        // If this arg is a variable, mark it moved (same semantics as before)
        if let Expr::Var { name: vname, span: vspan } = arg_expr {
            let v = locals.get_mut(vname).ok_or_else(|| {
                HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", vname, vspan.line, vspan.column))
            })?;
            if v.moved {
                return Err(HolyError::Semantic(format!(
                    "Variable `{}` already moved (line {} column {})",
                    vname, vspan.line, vspan.column
                )));
            }
            v.moved = true;
        }
    }

    // return handling
    match ret_ty_opt {
        Some(rt) => Ok(Some(rt.clone())),
        None => {
            if require_ret {
                Err(HolyError::Semantic(format!(
                    "Function `{}` has no return type declared but is used in an expression (must declare return type) (line {} column {})",
                    name, span.line, span.column
                )))
            } else {
                Ok(None)
            }
        }
    }
}

/// Infer the type of an expression, and update literal nodes (and nested nodes) where possible.
/// Returns the deduced Type for the expression.
fn infer_expr_type(
    expr: &mut Expr,
    locals: &mut HashMap<String, VarInfo>,
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    infer_hint: Option<Type>
) -> Result<Type, HolyError> {
    match expr {
        Expr::IntLiteral { value: value, ty: ty, span: span } => {
            // If literal already has concrete type, return it; otherwise default to Int32
            if *ty == Type::Infer {
                // Never duplicate same code twice, but shitty Rust forces us to in this very specific
                // scenario
                // This is needed because of Arrays, to allow type inferrence for hard-coded array
                // elements, depending on the array's exact type.
                // NOTE:: Whenever you change anything here, reflect it in assign_infer_type_to_expr function!

                // 
                // If no hint, this should not get overwritten.
                match value {
                    IntLiteralValue::Signed(v) => {
                        if *v < i128::MIN || *v > i128::MAX {
                            return Err(HolyError::Semantic(format!(
                                        "Integer literal {} out of range for representable int128, your signed integer literal will overflow because it's bigger than int128 capacity (line {} column {})", 
                                        v, span.line, span.column)));
                        }
                        *ty = Type::Int128;
                    }
                    IntLiteralValue::Unsigned(v) => {
                        if *v > u128::MAX {
                            return Err(HolyError::Semantic(format!(
                                        "Integer literal {} out of range for representable int128, your unsigned integer literal will overflow because it's bigger than uint128 capacity (line {} column {})", 
                                        v, span.line, span.column)));
                        }
                        *ty = Type::Uint128;
                    }
                }

                if infer_hint.is_some() {
                    let infer_hint = infer_hint.unwrap();
                    match infer_hint {
                        Type::Int16 => {
                            let v: i128 = match *value {
                                IntLiteralValue::Signed(v) => v,
                                IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                                IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > i16::MAX as i128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Int16;
                        }
                        Type::Int32 => {
                            let v: i128 = match *value {
                                IntLiteralValue::Signed(v) => v,
                                IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                                IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > i32::MAX as i128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Int32;
                        }
                        Type::Int64 => {
                            let v: i128 = match *value {
                                IntLiteralValue::Signed(v) => v,
                                IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                                IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > i64::MAX as i128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Int64;
                        }

                        Type::Int128 => {
                            let v: i128 = match *value {
                                IntLiteralValue::Signed(v) => v,
                                IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                                IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > i128::MAX {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Int128;
                        }


                        Type::Uint16 => {
                            let v: u128 = match *value {
                                IntLiteralValue::Unsigned(v) => v,
                                IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                                IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > u16::MAX as u128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Uint16;
                        }

                        Type::Uint32 => {
                            let v: u128 = match *value {
                                IntLiteralValue::Unsigned(v) => v,
                                IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                                IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > u32::MAX as u128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Uint32;
                        }
                        Type::Uint64 => {
                            let v: u128 = match *value {
                                IntLiteralValue::Unsigned(v) => v,
                                IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                                IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > u64::MAX as u128 {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Uint64;
                        }
                        Type::Uint128 => {
                            let v: u128 = match *value {
                                IntLiteralValue::Unsigned(v) => v,
                                IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                                IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                            "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                            v, infer_hint, span.line, span.column))),
                            };

                            if v > u128::MAX {
                                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, infer_hint, span.line, span.column)));
                            }

                            *ty = Type::Uint128;
                        }

                        _ => {}
                        
                    }
                }
            }
            Ok(ty.clone())
        }

        Expr::FloatLiteral { value: value, ty: ty, span: span } => {
            if *ty == Type::Infer {
                // Never duplicate same code twice, but shitty Rust forces us to in this very specific
                // scenario
                // This is needed because of Arrays, to allow type inferrence for hard-coded array
                // elements, depending on the array's exact type.
                // NOTE:: Whenever you change anything here, reflect it in assign_infer_type_to_expr function!

                // 
                // If no hint, this should not get overwritten.

                if let FloatLiteralValue::Float32(_) = value {
                    *ty = Type::Float32;

                } else if let FloatLiteralValue::Float64(_) = value {
                    *ty = Type::Float64;
                } else {
                    panic!("(Compiler bug) FloatLiteralValue should have only float32 and float64, if you add more float types, be ensure to extend this.");
                }

                if infer_hint.is_some() {
                    match infer_hint.unwrap() {
                        Type::Float32 => {
                            if let FloatLiteralValue::Float64(_) = value {
                                return Err(HolyError::Semantic(format!("Float literal has a float64 value but we expected a float32 value (line {} column {})", span.line, span.column)));
                            }
                            *ty = Type::Float32;
                            
                        }
                        Type::Float64 => {
                            *ty = Type::Float64;
                            if let FloatLiteralValue::Float32(f) = value {
                                *value = FloatLiteralValue::Float64(f.clone() as f64);
                            }
                        }
                        _ => {}
                 
                    }
                }
            }
            Ok(ty.clone())
        }

        Expr::BoolLiteral { value: _, span: _ } => {
            Ok(Type::Bool)
        }

        
        Expr::ArrayLiteral { elements, array_ty,  span } => {
            // all elements must have same type
            for e in elements.iter_mut() {
                let ety = infer_expr_type(e, locals, fun_sigs, Some(array_ty.clone()))?;
                if !type_compatible(&ety, &array_ty) {
                    return Err(HolyError::Semantic(format!(
                        "Array element type mismatch: expected `{}` got `{}` (line {} column {})",
                        array_ty, ety, span.line, span.column
                    )));
                }
            }

            Ok(Type::Array(Box::new(array_ty.clone())))
        }

        Expr::Var{name: name, span: span} => {
            if let Some(info) = locals.get(name) {
                if info.moved {
                    return Err(HolyError::Semantic(format!(
                                "Use of moved variable `{}` (line {} column {})", 
                                name, span.line, span.column
                            )));
                }
                Ok(info.ty.clone())
            } else {
                
                validate_identifier_name(name, span.line)
                    .map_err(|_| 
                        HolyError::Semantic(format!("Invalid syntax `{}` (line {} column {})", name, span.line, span.column)))?;
                
                Err(HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
            }
        }


        Expr::UnaryOp{ op: op, expr: e, span: span } => {
            let ety = infer_expr_type(e, locals, fun_sigs, infer_hint)?;
            
            // Ensure that no negate unary operation is allowed on an unsigned integer.
            if *op == UnaryOpKind::Negate {
                if matches!(ety, Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128) {
                    return Err(HolyError::Semantic(format!("{} cannot have negate unary operation. (line {} column {})", ety, span.line, span.column)))
                }
            }

            Ok(ety)
        
        }

        Expr::BinOp { left: left, op: _, right: right, span: span } => {
            // infer both sides
            let lty = infer_expr_type(left, locals, fun_sigs, None)?;
            let rty = infer_expr_type(right, locals, fun_sigs, None)?;

            // If either side is Infer (shouldn't after recursive call), try to resolve:
            let resolved = resolve_binary_op_types(&lty, &rty, &span)?;
            // update literal nodes inside if they were Infer (not necessary here but ok)
            Ok(resolved)
        }




        Expr::Call { name: name, args: args, span: span } => {
            let ret_opt = check_call(name, args, locals, fun_sigs, true, *span)?;
            match ret_opt {
                Some(ret_vec) => {
                    if ret_vec.len() == 1 {
                        Ok(ret_vec[0].clone())
                    } else {
                        Err(HolyError::Semantic(format!(
                            "Call to `{}` returns {} values but is used in a single-value expression (line {} column {})",
                            name, ret_vec.len(), span.line, span.column
                        )))
                    }
                }
                None => {
                    // check_call should already error when require_ret == true,
                    // but to be defensive:
                    Err(HolyError::Semantic(format!(
                        "Call to `{}` has no return type but is used in an expression (line {} column {})",
                        name, span.line, span.column
                    )))
                }
            }
        }
    }
}



// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
fn assign_infer_type_to_expr(expr: &mut Expr, ty: Type) -> Result<(), HolyError> {
    match expr {
        Expr::IntLiteral { value: value, ty: t, span: span } => {
            if *t == Type::Infer {
                match ty {
                    Type::Int16 => {
                        let v: i128 = match *value {
                            IntLiteralValue::Signed(v) => v,
                            IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                            IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > i16::MAX as i128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Int16;
                        return Ok(());
                    }

                    Type::Int32 => {
                        let v: i128 = match *value {
                            IntLiteralValue::Signed(v) => v,
                            IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                            IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > i32::MAX as i128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Int32;
                        return Ok(());

                    }

                    Type::Int64 => {
                        let v: i128 = match *value {
                            IntLiteralValue::Signed(v) => v,
                            IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                            IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > i64::MAX as i128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Int64;
                        return Ok(());
                    }

                    Type::Int128 => {
                        let v: i128 = match *value {
                            IntLiteralValue::Signed(v) => v,
                            IntLiteralValue::Unsigned(v) if v <= i128::MAX as u128 => v as i128,
                            IntLiteralValue::Unsigned(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is unsigned and is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > i128::MAX {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Int128;
                        return Ok(());
                    }

                    Type::Uint16 => {
                        let v: u128 = match *value {
                            IntLiteralValue::Unsigned(v) => v,
                            IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                            IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > u16::MAX as u128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Uint16;
                        return Ok(());
                    }

                    Type::Uint32 => {
                        let v: u128 = match *value {
                            IntLiteralValue::Unsigned(v) => v,
                            IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                            IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > u32::MAX as u128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Uint32;
                        return Ok(());
                    }

                    Type::Uint64 => {
                        let v: u128 = match *value {
                            IntLiteralValue::Unsigned(v) => v,
                            IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                            IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > u64::MAX as u128 {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Uint64;
                        return Ok(());
                    }

                    Type::Uint128 => {
                        let v: u128 = match *value {
                            IntLiteralValue::Unsigned(v) => v,
                            IntLiteralValue::Signed(v) if v >= 0 => v as u128,
                            IntLiteralValue::Signed(v) => return Err(HolyError::Semantic(format!(
                                        "Integer literal {} is signed and negative, which is out of range for type {} (line {} column {})", 
                                        v, ty, span.line, span.column))),
                        };

                        if v > u128::MAX {
                            return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", v, ty, span.line, span.column)));
                        }

                        *t = Type::Uint128;
                        return Ok(());
                    }

                    _ => {
                        return Err(HolyError::Semantic(format!("Cannot assign integer literal to non-integer type `{}` (line {} column {})", ty, span.line, span.column)));
                    }
                }
            }
        }
        Expr::FloatLiteral { value: value, ty: t, span: span } => {
            if *t == Type::Infer {
                match ty {
                    Type::Float32 => {
                        if let FloatLiteralValue::Float64(_) = value {
                            return Err(HolyError::Semantic(format!("Float literal has a float64 value but we expected a float32 value (line {} column {})", span.line, span.column)));
                        }

                        *t = Type::Float32;
                        return Ok(());
                    }
                    Type::Float64 => {
                        *t = Type::Float64;

                        // Allow automatic conversions of float32 to float64 
                        if let FloatLiteralValue::Float32(f) = value {
                            *value = FloatLiteralValue::Float64(f.clone() as f64);
                        }
                        return Ok(());
                    }
                    _ => {
                        return Err(HolyError::Semantic(format!("Cannot assign float literal to non-float type `{}` (line {} column {})", ty, span.line, span.column)));
                    }
                }
            }

            // Unlike integer literals, here we allow automatic conversion of float32 literals to
            // float64 if the holder type `ty` is float64
            if ty == Type::Float64 {
                if let FloatLiteralValue::Float32(f) = value {
                    *value = FloatLiteralValue::Float64(f.clone() as f64);
                    *t = Type::Float64
                }
            }

        }
        // For non-literals, nothing to assign, and that's OK.
        _ => {}
    }

    Ok(())
}


// When variable is declared like
// own x int32
// its value is None, so we need to set it to a default value based on its type.
// ints default are 0, floats are 0.0, string is "", etc.
// Only primitives listed above though. Everything else needs a value.
//
// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
fn assign_default_value_for_type(expr: &mut Option<Expr>, ty: &Type, span: Span) -> Result<(), HolyError> {
    if *ty == Type::Infer {
        // To ensure that I dont do mistakes and call this function on things I am not supposed to.

        panic!("(Compiler bug) Cannot assign default value for type `{}` because the expression holder type is Infer. Expression: {:?}\nDont call this on things of type Infer.", 
                        ty, expr);
    }
    
    match ty {
        Type::Int16 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Signed(0), ty: Type::Int16, span: span })
        }
        Type::Int32 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Signed(0), ty: Type::Int32, span: span })
        }
        Type::Int64 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Signed(0), ty: Type::Int64, span: span })
        }
        Type::Int128 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Signed(0), ty: Type::Int128, span: span })
        }
        
        Type::Uint16 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Unsigned(0), ty: Type::Uint16, span: span })
        }
        Type::Uint32 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Unsigned(0), ty: Type::Uint32, span: span })
        }
        Type::Uint64 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Unsigned(0), ty: Type::Uint64, span: span })
        }
        Type::Uint128 => {
            *expr = Some(Expr::IntLiteral { value: IntLiteralValue::Unsigned(0), ty: Type::Uint128, span: span })
        }

        Type::Float32 => {
            *expr = Some(Expr::FloatLiteral { value: FloatLiteralValue::Float32(0.0), ty: Type::Float32, span: span })
        }

        Type::Float64 => {
            *expr = Some(Expr::FloatLiteral { value: FloatLiteralValue::Float64(0.0), ty: Type::Float64, span: span })
        }

        Type::Bool => {
            *expr = Some(Expr::BoolLiteral { value: false, span: span })
        }

        Type::Array(inner) => {
            let inner_ty = inner.clone();
            *expr = Some(Expr::ArrayLiteral { elements: Vec::new(), array_ty: *inner_ty, span: span })
        }




        _ => {
            panic!("(Compiler bug) Cannot assign default value for type `{}` because the expression holder type is Non-literal. Expression: {:?}\nDont call this on non-literals.", 
                        ty, expr);
        }
    }

    Ok(())
}


/// Decide whether two types are compatible for assignment / matching.
/// For now: exact match only (no implicit widening/narrowing).
fn type_compatible(a: &Type, b: &Type) -> bool {
    a == b
}

/// Resolve binary operation types (e.g., `+`). Rules:
/// - both must be numeric and same kind (int/int or float/float)
/// - mixing signed and unsigned is an error
/// - return the resulting type
fn resolve_binary_op_types(a: &Type, b: &Type, span: &Span) -> Result<Type, HolyError> {
    use Type::*;
    match (a, b) {
        (Int32, Int32) => Ok(Int32),
        (Uint32, Uint32) => Ok(Uint32),
        (Float32, Float32) => Ok(Float32),
        (Float64, Float64) => Ok(Float64),

        // If one side is Infer, prefer the other side if concrete
        (Infer, t @ _) if *t != Infer => Ok(t.clone()),
        (t @ _, Infer) if *t != Infer => Ok(t.clone()),

        // both infer -> default to Int32
        (Infer, Infer) => Ok(Int32),

        // mixed signed/unsigned or int/float combos -> error
        _ => Err(HolyError::Semantic(format!("Type mismatch in binary operation: `{}` vs `{}` (line {} column {})", a, b, span.line, span.column))),
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn default_for_nested_arrays() {
        let span = Span { line: 1, column: 0 };
        let mut expr: Option<Expr> = None;

        let ty = Type::Array(Box::new(Type::Array(Box::new(Type::Array(Box::new(Type::Int32))))));

        assign_default_value_for_type(&mut expr, &ty, span).unwrap();

        match expr {
            Some(Expr::ArrayLiteral { elements, array_ty, .. }) => {
                assert!(elements.is_empty());
                assert_eq!(array_ty, Type::Array(Box::new(Type::Array(Box::new(Type::Int32)))));
            }
            other => panic!("expected ArrayLiteral, got {:?}", other),
        }
    }

}
