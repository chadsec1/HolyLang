use std::collections::HashMap;

use crate::error::HolyError;
use crate::parser::{
    AST, Expr, Function, Param, Stmt, Type, Variable, Span, IntLiteralValue, FloatLiteralValue, UnaryOpKind, BinOpKind,

    validate_identifier_name
};

#[cfg(test)]
mod tests;

mod helpers;
 

#[derive(Clone, Debug)]
struct VarInfo {
    ty: Type,
    moved: bool,
    locked: bool,
    value: Option<Expr>,
    
    len: Option<usize> // NOTE: This field purpose is only for partial, simple compile-time safety
                       // for out of bounds array/string indexing/slicing. It is not reliable but
                       // it will catch most simple out-of-bounds (except if the most upstream
                       // source is a function call). 
                       // This is fine, because Rust automatically inserts bounds checking before
                       // array / string access/slicing anyway!
                       // Rust also will handle literals in expressions smartly so it would also
                       // act as an additional protection for our implementation
                       // but it's something to keep in mind.
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
    let mut upstream_var_names: Vec<String> = vec![];

    for p in &func.params {
        locals.insert(
            p.name.clone(), 
            VarInfo {
                ty: p.type_name.clone(),
                moved: false,
                
                // By default, arguments are locked to help reduce logic bugs.
                locked: true,
                
                // We do not know a parameter value.
                value: None,
                // Nor its length
                len: None
            });

        upstream_var_names.push(p.name.clone());
    }
    
    // Ensure that no code exists after return
    if let Some(last_ret_pos) = func.body.iter().rposition(|s| matches!(s, Stmt::Return(_))) {
        if last_ret_pos + 1 < func.body.len() {
            
            let offending_span = helpers::stmt_span(&func.body[last_ret_pos + 1]);
            return Err(HolyError::Semantic(format!(
                "Code after `return` is not allowed (line {} column {})",
                offending_span.line, offending_span.column
            )));
        }
    }

    check_stmts(func.clone(), &mut func.body, &mut locals, upstream_var_names, fun_sigs)


}

// Parse stmts in a block
fn check_stmts(func: Function, block: &mut Vec<Stmt>, locals: &mut HashMap<String, VarInfo>, upstream_var_names: Vec<String>, fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>) -> Result<(), HolyError> {
    
    // Walk statements in order. 
    for stmt in block {
        let stmt_span = helpers::stmt_span(&stmt);

        match stmt {
            Stmt::VarDecl(var) => {
                // If var has explicit type: keep it. If Infer: try infer from initializer.
                if var.type_name == Type::Infer {
                    if let Some(expr) = &mut var.value {
                        let ty = infer_expr_type(expr, locals, fun_sigs, None)?;
                        // assign inferred type to variable and propagate to literal nodes if needed
                        var.type_name = ty.clone();
                    } else {
                        return Err(HolyError::Semantic(format!(
                            "Variable `{}` requires explicit type when no initializer is provided (line {} column {})",
                            var.name, var.span.line, var.span.column
                        )));
                    }
                } else {
                    // explicit type: if initializer present, ensure initializer is compatible
                    
                    if let Some(expr) = &mut var.value {
                        assign_infer_type_to_expr_value(expr, var.type_name.clone())?;

                        // Now infer/check the expression type as usual.
                        let expr_ty = infer_expr_type(expr, locals, fun_sigs, Some(var.type_name.clone()))?;
                        if !type_compatible(&expr_ty, &var.type_name) {
                            return Err(HolyError::Semantic(format!(
                                "Type mismatch assigning to `{}`: got `{}`, expected `{}` (line {} column {})",
                                var.name, expr_ty, var.type_name, var.span.line, var.span.column
                            )));
                        }

                    } else {
                        assign_default_value_for_type(&mut var.value, &var.type_name, var.span)?;
                    }
                }

            
                // Check if variable exists in locals and if its locked.
                if let Some(info) = locals.get(&var.name) {
                    if info.locked {
                        return Err(HolyError::Semantic(format!(
                                "Variable `{}` is locked, therefore you cannot overshadow it (line {} column {})", 
                                &var.name, var.span.line, var.span.column
                            )));
                    }
                }


                if upstream_var_names.contains(&var.name) {
                    return Err(HolyError::Semantic(format!(
                                "Variable `{}` is already defined upstream, you cannot overshadow upstream variables (line {} column {})", 
                                &var.name, var.span.line, var.span.column
                            )));
                }


                let mut value_len: Option<usize> = None;
                if var.value.is_none() {
                    panic!("(Compiler bug) Variable value is none even after we attempted to assign default value! {:?}", var);
                }

                // Check if source value is a variable and if its locked or moved, and moves it
                if let Some(Expr::Var { name: src_name, span }) = &var.value {
                    let src = locals.get_mut(src_name).ok_or_else(|| {
                        HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", src_name, span.line, span.column))
                    })?;

                    if src.moved {
                        return Err(HolyError::Semantic(format!("Use of moved variable `{}` (line {} column {})", src_name, span.line, span.column)));
                    }


                    // mark source as moved because ownership was transferred
                    src.moved = true;

                    // We copy its length
                    value_len = src.len
                }

                match var.value.clone().unwrap() {
                    Expr::ArrayLiteral{elements: elements, array_ty: _, span: _} => {
                        value_len = Some(elements.len())
                    }

                    Expr::StringLiteral{value: v, span: _} => {
                        value_len = Some(v.len())
                    }
                    // Other experessions we can't / don't need to store their length
                    _ => {}
                }


                // register variable in locals (now with concrete type)
                locals.insert(
                    var.name.clone(),
                    VarInfo {
                        ty: var.type_name.clone(),
                        value: var.value.clone(),
                        moved: false,
                        locked: false,
                        len: value_len
                    }
                );
            }

            Stmt::VarDeclMulti(var_list, call_expr) => {
                // Expect the rhs to be a Call
                if let Expr::Call { name, args, span } = call_expr {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_opt = check_call(name, args, locals, fun_sigs, true, *span)?;
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

                        // Check if variable exists in locals and if its locked.
                        if let Some(info) = locals.get(&var.name) {
                            if info.locked {
                                return Err(HolyError::Semantic(format!(
                                        "Variable `{}` is locked, therefore you cannot overshadow it (line {} column {})", 
                                        &var.name, var.span.line, var.span.column
                                    )));
                            }
                        }

                        if upstream_var_names.contains(&var.name) {
                            return Err(HolyError::Semantic(format!(
                                        "Variable `{}` is already defined upstream, you cannot overshadow upstream variables (line {} column {})", 
                                        &var.name, var.span.line, var.span.column
                                    )));
                        }



                        // insert into locals
                        //
                        locals.insert(
                            var.name.clone(),
                            VarInfo {
                                ty: var.type_name.clone(), 
                                value: var.value.clone(), 
                                moved: false, 
                                locked: false,
                                len: None
                            }
                        );
                    }
                } else {
                    return Err(HolyError::Semantic(format!(
                        "Multi-declarement requires only a single function call on the right-hand side",
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

                // Its fine to clone and pass it to infer_expr_type in this specific scenario
                // because we don't give it any infer type hint and therefore no need for it to be
                // reflected back in locals. I think.

                let expr_ty = infer_expr_type(&mut assign.value.clone(), locals, fun_sigs, Some(varinfo.ty.clone()))?;
                if !type_compatible(&expr_ty, &varinfo.ty) {
                    return Err(HolyError::Semantic(format!(
                        "Cannot assign `{}` to `{}` of type `{}` (line {} column {})",
                        expr_ty, assign.name, varinfo.ty, assign.span.line, assign.span.column
                    )));
                }

                // Check if source value is a variable and if it is moved. 
                // And moves it
                if varinfo.moved {
                    return Err(HolyError::Semantic(format!(
                        "Value assignment to moved variable `{}` (line {} column {})",
                        assign.name, assign.span.line, assign.span.column
                    ))); 
                }

                // Check if variable is locked.
                if varinfo.locked {
                    return Err(HolyError::Semantic(format!(
                            "Variable `{}` is locked, therefore you cannot assign to it (line {} column {})", 
                            &assign.name, assign.span.line, assign.span.column
                        )));
                }

                
                let mut value_len: Option<usize> = None;
              
                if let Expr::Var { name: src_name, span } = &assign.value {
                    let src = locals.get_mut(src_name).ok_or_else(|| {
                        HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", src_name, span.line, span.column))
                    })?;

                    if src.moved {
                        return Err(HolyError::Semantic(format!("Use of moved variable `{}` (line {} column {})", src_name, span.line, span.column)));
                    }



                    // if source name is same as our variable name,
                    // then we don't move. It's re-claiming ownership.
                    if src_name != &assign.name {
                        // mark source as moved because ownership was transferred
                        src.moved = true;
                    }

                    // We copy its length
                    value_len = src.len

                }

                match assign.value.clone() {
                    Expr::ArrayLiteral{elements: elements, array_ty: _, span: _} => {
                        value_len = Some(elements.len());
                    }

                    Expr::StringLiteral{value: v, span: _} => {
                        value_len = Some(v.len());
                    }
                    // Other experessions we can't / don't need to store their length
                    _ => {}
                }

                // Let us get a mutable varinfo to update length
                let varinfo = locals.get_mut(&assign.name).unwrap();
                varinfo.len = value_len;

            }


            Stmt::VarAssignMulti(expr) => {
                
                if let Expr::Call { name, args, span } = &mut expr.value {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_opt = check_call(name, args, locals, fun_sigs, true, *span)?;
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


                        // Check if variable is locked.
                        if varinfo.locked {
                            return Err(HolyError::Semantic(format!(
                                    "Variable `{}` is locked, therefore you cannot assign to it (line {} column {})", 
                                    &var_name, expr.span.line, expr.span.column
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
                        "Multi-assignment requires only a single function call on the right-hand side (line {} column {})",
                        expr.span.line, expr.span.column
                    )));
                }
            }

            Stmt::Expr(expr) => {
                if let Expr::Call { name, args, span } = expr {
                    // allow void calls as statements (require_ret = false)
                    let _ = check_call(name, args, locals, fun_sigs, false, *span)?;
                    // no returned type expected; ok to ignore
                } else {
                    // other expressions-as-statements: fully type-check
                    let _ = infer_expr_type(expr, locals, fun_sigs, None)?;
                }
            }

            Stmt::Lock(expr_vec) => {
                let mut var_names_to_lock: Vec<String> = vec![];

                for expr in expr_vec.iter_mut() {
                    match expr {
                        Expr::Var { name: name, span: span} => {
                            if var_names_to_lock.contains(name) {
                                return Err(HolyError::Semantic(format!(
                                    "Lock arguments have duplicated variable `{}` (line {} column {})",
                                    name, span.line, span.column
                                )))
                            }

                            var_names_to_lock.push(name.to_string());


                            // We dont care about its type, we just checking if it exists or not,
                            // and its contents are valid, etc.
                            infer_expr_type(expr, locals, fun_sigs, None)?;

                        },


                        _ => {
                            return Err(HolyError::Semantic(format!(
                                "Expected variable name, instead got `{}` (line {} column {})",
                                expr, stmt_span.line, stmt_span.column
                            )))
                        }
                    }
                }


                for var_name in var_names_to_lock {
                    let var = locals.get_mut(&var_name).ok_or_else(|| {
                        panic!("(Compiler bug) Variable doesnt exist in locals despite our earlier call to infer_expr_type shouldve checked the variable thourghly, including its existence, but apparently it didnt. expr_vec: `{:?}`, var_name: {:?}", 
                            expr_vec, var_name);
                    })?;

                    if var.locked == true {
                        return Err(HolyError::Semantic(format!(
                                "Variable `{}` is already locked (line {} column {})",
                                var_name, stmt_span.line, stmt_span.column
                            )))

                    }

                    var.locked = true;
                }

            }


            Stmt::Unlock(expr_vec) => {
                let mut var_names_to_unlock: Vec<String> = vec![];

                for expr in expr_vec.iter_mut() {
                    match expr {
                        Expr::Var { name: name, span: span} => {
                            if var_names_to_unlock.contains(name) {
                                return Err(HolyError::Semantic(format!(
                                    "Unlock arguments have duplicated variable `{}` (line {} column {})",
                                    name, span.line, span.column
                                )))
                            }

                            var_names_to_unlock.push(name.to_string());


                            // We dont care about its type, we just checking if it exists or not,
                            // and its contents are valid, etc.
                            infer_expr_type(expr, locals, fun_sigs, None)?;

                        },


                        _ => {
                            return Err(HolyError::Semantic(format!(
                                "Expected variable name, instead got `{}` (line {} column {})",
                                expr, stmt_span.line, stmt_span.column
                            )))
                        }
                    }
                }


                for var_name in var_names_to_unlock {
                    let var = locals.get_mut(&var_name).ok_or_else(|| {
                        panic!("(Compiler bug) Variable doesnt exist in locals despite our earlier call to infer_expr_type shouldve checked the variable thourghly, including its existence, but apparently it didnt. expr_vec: `{:?}`, var_name: {:?}", 
                            expr_vec, var_name);
                    })?;

                    if var.locked == false {
                        return Err(HolyError::Semantic(format!(
                                "Variable `{}` is already unlocked (line {} column {})",
                                var_name, stmt_span.line, stmt_span.column
                            )))

                    }

                    var.locked = false;
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
                            let declared_ty = declared_ty_vec[i].clone();
                            let expr_ty = infer_expr_type(expr, locals, fun_sigs, Some(declared_ty.clone()))?;

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


            Stmt::If(ifStmt) => {
                let main_expr_ty = infer_expr_type(&mut ifStmt.condition, locals, fun_sigs, Some(Type::Bool))?;
                
                if main_expr_ty != Type::Bool {
                    return Err(HolyError::Semantic(format!(
                        "If statement require an expression to be evaulatable to type `bool`, instead we got `{}` (line {} column {})",
                        main_expr_ty, stmt_span.line, stmt_span.column,
                    )));
                }

                // This gets all upstream variable names, and passes it to check stmts to ensure
                // you cannot overshadow them (because it makes reading code confusing).
                let mut upstream = upstream_var_names.clone();
                for var_name in locals.keys() {
                    upstream.push(var_name.to_string());
                }


                    
                let mut main_locals_clone = locals.clone();
                check_stmts(func.clone(), &mut ifStmt.if_branch, &mut main_locals_clone, upstream.clone(), fun_sigs)?;
                update_local_assignments_from_clone(locals, main_locals_clone);
                

                for s in &mut ifStmt.elif_branches {
                    let elif_expr_ty = infer_expr_type(&mut s.0, locals, fun_sigs, Some(Type::Bool))?; 

                    if elif_expr_ty != Type::Bool {
                        return Err(HolyError::Semantic(format!(
                            "Elif statements require an expression to be evaulatable to type `bool`, instead we got `{}` (line {} column {})",
                            elif_expr_ty, stmt_span.line, stmt_span.column,
                        )));
                    }

                
                    let mut elif_locals_clone = locals.clone();
                    check_stmts(func.clone(), &mut s.1, &mut elif_locals_clone, upstream.clone(), fun_sigs)?;
                    update_local_assignments_from_clone(locals, elif_locals_clone);
                }

                if let Some(else_stmts) = ifStmt.else_branch.as_mut() {
                    let mut else_locals_clone = locals.clone();
                    check_stmts(func.clone(), else_stmts, &mut else_locals_clone, upstream, fun_sigs)?;
                    update_local_assignments_from_clone(locals, else_locals_clone);
                }
                
            }

            Stmt::Func(_) => {}
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

fn update_local_assignments_from_clone(upstream: &mut HashMap<String, VarInfo>, downstream: HashMap<String, VarInfo> ) {
    // We loop over the locals, to update our corresponding locals
    // like variable assignments, length change, ownership change, etc
    for (n, vi) in downstream {
        if let Some(info) = upstream.get_mut(&n) {
            *info = vi.clone();
        }

    }
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
    for (i, (arg_expr, param_ty)) in args.iter_mut().zip(param_tys.iter()).enumerate() {
        let arg_ty = infer_expr_type(arg_expr, locals, fun_sigs, Some(param_ty.clone()))?;
        if arg_ty == Type::Infer {
            // assign literal inference to param type when possible
            assign_infer_type_to_expr_value(arg_expr, param_ty.clone())?;
        } else if !type_compatible(&arg_ty, param_ty) {
            return Err(HolyError::Semantic(format!(
                "Argument number {} type mismatch in call to `{}`: expected `{}`, got `{}` (line {} column {})",
                i + 1, name, param_ty, arg_ty, span.line, span.column,
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

        // Note: If infer hint is set, we alter the value to fit the hint, if we can.
        Expr::IntLiteral { value: value, span: span } => {
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
        Expr::FloatLiteral { value: value, span: span } => {
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
                    if !type_compatible(&ety, &Type::Usize) {
                        return Err(HolyError::Semantic(format!("Expected array index to be of type `usize`, instead we got `{}` (line {} column {})", ety, span.line, span.column)));
                    }


                    if !matches!(&info.ty, Type::Array(_)) {
                        return Err(HolyError::Semantic(format!("Array access on non-array variable `{}` (line {} column {})", name, span.line, span.column)));
                    }

                    // If info length is none, then this must not be an array.
                    if info.len.is_none() {
                        panic!("(Compiler bug) this shouldnt ever happen, but it happened. We expected an info.len to be a usize, but we got None. This shouldve been handled by variable declaration/assignment, but apparently not: {:?}", locals);
                    }

                    check_usize_literal_to_src(&**index, info.len.unwrap(), span.clone(), locals.clone())?;
                   
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

                    // If info length is none, then this must not be an array.
                    if info.len.is_none() {
                        panic!("(Compiler bug) this shouldnt ever happen, but it happened. We expected an info.len to be a usize, but we got None. This shouldve been handled by variable declaration/assignment, but apparently not: {:?}", locals);
                    }


                    if let Some(s) = &mut *start {
                        // Ensure that the type of the start index expression is usize, and try to
                        // convert it if possible.
                        let start_ety = infer_expr_type(s, locals, fun_sigs, Some(Type::Usize))?;
                        if !type_compatible(&start_ety, &Type::Usize) {
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
                        if !type_compatible(&end_ety, &Type::Usize) {
                            return Err(HolyError::Semantic(format!(
                                        "Expected end index to be of type `usize` for array `{}`, instead we got `{}` (line {} column {})", 
                                        end_ety, name, span.line, span.column
                                    )));
                        }


                        check_usize_literal_to_src(&e, info.len.unwrap(), span.clone(), locals.clone())?;
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
                
                validate_identifier_name(name)
                    .map_err(|_| 
                        HolyError::Semantic(format!("Invalid syntax `{}` (line {} column {})", name, span.line, span.column)))?;
                
                Err(HolyError::Semantic(format!("Use of undeclared variable `{}` (line {} column {})", name, span.line, span.column)))
            }
        }


        Expr::UnaryOp{ op: op, expr: e, span: span } => {
            let ety = infer_expr_type(e, locals, fun_sigs, infer_hint)?;
            
            // Ensure that no negate unary operation is allowed on an unsigned integer.
            if *op == UnaryOpKind::Negate {
                if !matches!(ety, Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                    return Err(HolyError::Semantic(format!("{} cannot have negate unary operation. (line {} column {})", ety, span.line, span.column)))
                }
            }

            Ok(ety)
        
        }

        Expr::BinOp { left: left, op: op, right: right, span: span } => {
            // infer both sides
            let lty = infer_expr_type(left, locals, fun_sigs, infer_hint.clone())?;
            let rty = infer_expr_type(right, locals, fun_sigs, infer_hint.clone())?;

                
            if matches!(**left, Expr::CopyCall { .. }) || matches!(**right, Expr::CopyCall { .. }) {
                return Err(HolyError::Semantic(format!(
                        "Copying is not needed for variables in binary operations, because they're always copied. Remove the copy call. (line {} column {})", 
                        span.line, span.column)))
            }

            if lty == Type::Infer {
                panic!("(Compiler bug) lty and rty are of type Infer even after we tried to infer: Left: {:?} Right: {:?}", **left, **right);
            }

            // Ensure binary operations resolve to same type.
            if lty != rty {
                return Err(HolyError::Semantic(format!("Type mismatch in binary operation: `{}` vs `{}` (line {} column {})", lty, rty, span.line, span.column)));
            }

            
            // Since we already know both lty and rty equal same type, we can do our operations on
            // lty, and is safe to assume both are equal.
            if matches!(lty, Type::String | Type::Bool | Type::Array(_) ) {
                if matches!(op, BinOpKind::Add | BinOpKind::Subtract | BinOpKind::Multiply | BinOpKind::Divide | BinOpKind::Greater | BinOpKind::GreaterEqual | BinOpKind::Less | BinOpKind::LessEqual) {
                    return Err(HolyError::Semantic(format!("You cannot perform arithmetic on type `{}`. (line {} column {})", lty, span.line, span.column)));
                } 
            }

            // arthmetic
            if matches!(op, BinOpKind::Add | BinOpKind::Subtract | BinOpKind::Multiply | BinOpKind::Divide ) {
                Ok(lty)

            // boolean comparison
            } else if matches!(op, BinOpKind::Equal | BinOpKind::NotEqual | BinOpKind::Greater | BinOpKind::GreaterEqual | BinOpKind::Less | BinOpKind::LessEqual ) {
                Ok(Type::Bool)
            } else {
                panic!("(Compiler bug) We got an unexpected BinOpKind: {:?}", op)
            }
        }

        Expr::CopyCall { expr: e, span: span } => {

            // Catch the "makes no sense" calls (like nested copying, or copying of a literal,  or
            // array access, or a binary op where left and right are both literals)
            // Basically, copy call only works on variables.
            match &mut **e {
                Expr::CopyCall {span: inner_span, ..} => {
                    return Err(HolyError::Semantic(format!("Double copying is not needed, Remove the extra copy call. (line {} column {})", inner_span.line, inner_span.column)))
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

        Expr::FormatCall { template: template, expressions: exprs_vec, span: span} => {

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


// helper: check an expression that's allowed to be an IntLiteral::Usize
fn check_usize_literal_to_src(expr: &Expr, len: usize, span: Span, locals: HashMap<String, VarInfo>) -> Result<(), HolyError> {
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
        other => Ok(())
            
    }
}


fn infer_integer_literal_helper(infer_ty: Type, value: IntLiteralValue, span: Span) -> Result<IntLiteralValue, HolyError> {

    if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128 | Type::Usize | Type::Byte | Type::Uint16 | Type::Uint32 | Type::Uint64 | Type::Uint128) {
        panic!("(Compiler bug) Value {} has unknown type", value);
    }

    match infer_ty {
        Type::Int8 => {
            if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot be inferred to type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();
            if val_raw < i8::MIN as i128 || val_raw > i8::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int8(val_raw as i8))
        }

        Type::Int16 => {
            if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot be inferred to type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();

            if val_raw < i16::MIN as i128 || val_raw > i16::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int16(val_raw as i16))
        }
        Type::Int32 => {
            if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot be inferred to type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();
            if val_raw < i32::MIN as i128 || val_raw > i32::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int32(val_raw as i32))
        }
        Type::Int64 => {
            if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot be inferred to type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
            }

            let val_raw: i128 = value.as_i128();

            if val_raw < i64::MIN as i128 || val_raw > i64::MAX as i128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Int64(val_raw as i64))
        }

        Type::Int128 => {
            if !matches!(value.get_type(), Type::Int8 | Type::Int16 | Type::Int32 | Type::Int64 | Type::Int128) {
                return Err(HolyError::Semantic(format!("Integer literal `{}` is of type `{}`, which cannot be inferred to type `{}` (line {} column {})", value, value.get_type(), infer_ty, span.line, span.column)));
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
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > usize::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Usize(val_raw as usize))
        }

        Type::Byte => {
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > u8::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Byte(val_raw as u8))
        }

        Type::Uint16 => {
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > u16::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint16(val_raw as u16))
        }

        Type::Uint32 => {
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > u32::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint32(val_raw as u32))
        }

        Type::Uint64 => {
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > u64::MAX as u128 {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint64(val_raw as u64))
        }
        Type::Uint128 => {
            let val_raw: u128 = value.as_u128_UNSAFE();
            if val_raw > u128::MAX {
                return Err(HolyError::Semantic(format!("Integer literal {} out of range for type {} (line {} column {})", value, infer_ty, span.line, span.column)));
            }
            
            Ok(IntLiteralValue::Uint128(val_raw))
        }

        other => {
            panic!("(Compiler bug) You must ensure type is an integer literal before passing it to this function");
        }
    }
}

// Function to attempt to change an expression's literal to match `ty`. If fails, it errors.
// ty is the expression holder type (i.e. variable type)
// expr is the value literal its self
fn assign_infer_type_to_expr_value(expr: &mut Expr, ty: Type) -> Result<(), HolyError> {
    match expr {
        Expr::IntLiteral { value: value, span: span } => {
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
        Expr::FloatLiteral { value: value, span: span } => {
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
/// For now: exact match only (no implicit widening/narrowing).
fn type_compatible(a: &Type, b: &Type) -> bool {
    a == b
}




/*
#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn test_default_for_nested_arrays() {
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
*/
