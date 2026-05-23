use std::collections::HashMap;

use crate::error::HolyError;
use crate::ast::{
    AST, Expr, Function, GlobalStmt, Stmt, Type, Span, Constant 
};

mod branch_analysis;
mod constants;
mod infer;
mod helpers;


#[cfg(test)]
mod branch_analysis_tests; 

#[cfg(test)]
mod helpers_tests;

#[cfg(test)]
mod blackbox_tests;


#[derive(Clone, Debug)]
enum BindingKind {
    Const {
        value: Expr
    },
    Var {
        moved: bool,
        locked: bool,
        value: Option<Expr>,

        len: Option<usize> // NOTE: This field purpose is only for partial, simple compile-time safety
                           //       for out of bounds array/string indexing/slicing. It is not reliable but
                           //       it will catch most simple out-of-bounds (except if the most upstream
                           //       source is a function call). 
                           //       This is fine, because Rust automatically inserts bounds checking before
                           //       array / string access/slicing anyway!
                           //       Rust also will handle literals in expressions smartly so it would also
                           //       act as an additional protection for our implementation
                           //       but it's something to keep in mind.
    }
}

#[derive(Clone, Debug)]
struct BindingInfo {
    ty: Type,
    kind: BindingKind,
}
   
/// Checks semantics and fill in inferred types where possible.
/// This mutates the AST, because int literals can be safely are replaced with their binding types
pub fn check_semantics(ast: &mut AST) -> Result<(), HolyError> {
    // First pass: collect function signatures (params types and return types (possible multiple))
    let mut fun_sigs: HashMap<String, (Vec<Type>, Option<Vec<Type>>)> = HashMap::new();
    for f in &ast.functions {
        let param_tys = f.params.iter().map(|p| p.type_name.clone()).collect();
        if fun_sigs.insert(f.name.clone(), (param_tys, f.return_type.clone())).is_some() {
            return Err(HolyError::Semantic(format!("Duplicate function declaration: {}", f.name)));
        }
    }
    
    // Second pass: check each global statement
    let mut globals: HashMap<String, BindingInfo> = HashMap::new();
    for global_stmt in &mut ast.globals {
        check_global_stmt(global_stmt, &mut globals, &fun_sigs)?;
    }


    // Third pass: check each function body
    for func in &mut ast.functions {
        // Clone on globals here because we don't want global variables to be modified
        // cross-function
        //
        check_function(func, &fun_sigs, &mut globals.clone())?;
    }

    Ok(())
}

/// Check single function, infer local var types where possible, check calls, returns.
fn check_function(
    func: &mut Function, 
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    locals: &mut HashMap<String, BindingInfo>
) -> Result<(), HolyError> {
    // Build local symbol table starting with params
    let mut upstream_var_names: Vec<String> = vec![];

    for p in &func.params {
        if locals.contains_key(&p.name) {
            return Err(HolyError::Semantic(format!(
                        "Cannot use `{}` as function parameter name, because it is already declared globally", // (line {} column {})", 
                        &p.name //, cons.span.line, cons.span.column
                    )));
        }
        locals.insert(
            p.name.clone(), 
            BindingInfo {
                ty: p.type_name.clone(),
                kind: BindingKind::Var {
                    moved: false,
                    
                    // By default, function arguments are locked 
                    locked: true,
                    
                    // We do not know a parameter value.
                    value: None,
                    // Nor its length
                    len: None
                }
            });

        upstream_var_names.push(p.name.clone());
    }

    check_stmts(func.clone(), &mut func.body, locals, upstream_var_names, fun_sigs, false)?;


    // Branch analysis to determine if function returns in all branches
    //

    // This is just to check that function has at least one statement
    // Reason it's here and not in dead_code_snalysis is because so
    // dead code analysis can properly error with lines.
    //
    let last_func_stmt = func.body.last();
    if last_func_stmt.is_none() {
        return Err(HolyError::Semantic(format!(
                        "Function `{}` has no statements, empty functions are not allowed! (line {} column {})",
                        func.name, func.span.line, func.span.column,
                    )));
    }

    // We call dead code analysis here after check_stmts, because we want checked semantics.
    // Semantics take priority more than dead code
    //
    branch_analysis::dead_code_analysis(&func.body, false)?;

    // Return analysis only needs to check last statement which has return statements
    // because dead code analysis should not let dead code pass.
    // last statement should be always be the one actualy always returning.
    //
    // We only do return branch analysis if function has declared return type.
    if func.return_type.is_some() {
        branch_analysis::return_branch_analysis(&func.clone(), last_func_stmt.clone().unwrap(), false, false)?;
    }

    Ok(())

}



/// Checks Stmt to see if its a statement is legally allowed to be global
/// and error if not.
///
fn check_global_stmt(
    globalstmt: &mut GlobalStmt,
    storage: &mut HashMap<String, BindingInfo>, 
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
) -> Result<(), HolyError> { 
    match globalstmt {
        GlobalStmt::Const(cons) => check_const(cons, storage, fun_sigs),

        other => panic!("(Compiler bug) Unimplemented {:?}", other)

    }
}

fn check_const(
    cons: &mut Constant,
    storage: &mut HashMap<String, BindingInfo>, 
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
) -> Result<(), HolyError> {
    if fun_sigs.contains_key(&cons.name) {
        return Err(HolyError::Semantic(format!(
                    "Constant identifier name `{}` is already taken by a function. (line {} column {})", 
                    &cons.name, cons.span.line, cons.span.column
                )));
    }

    if storage.contains_key(&cons.name) {
        return Err(HolyError::Semantic(format!(
                "Cannot use `{}` as the constant identifier name as it is already declared. Overshadowing is not allowed. (line {} column {})", 
                &cons.name, cons.span.line, cons.span.column
            )));
    }


    // Validate the constant type against the expression, AND internally coerce literals if
    // possible (i.e. int8 -> int32, etc), AND validate the constant value expression 
    // to ensure it is known at compile-time, AND evaluate it, and then fold it.
    //
    constants::eval_const_expr_and_fold_it(cons, storage, fun_sigs)?;

    // register the constant in storage (could be locals, or globals. we don't care.) 
    storage.insert(
        cons.name.clone(),
        BindingInfo {
            ty: cons.type_name.clone(),
            kind: BindingKind::Const {
                value: cons.value.clone()
            }
        }
    );

    Ok(())

}


/// Parse holylang statements in a block, it does:
/// Enforce language semantics, and ownership safety model, check function calls, etc.
fn check_stmts(
    func: Function, 
    block: &mut Vec<Stmt>, 
    locals: &mut HashMap<String, BindingInfo>, 
    upstream_var_names: Vec<String>, 
    fun_sigs: &HashMap<String, (Vec<Type>, Option<Vec<Type>>)>,
    in_loop: bool

) -> Result<(), HolyError> {


    // Special rule: Despite fact you cannot lock/unlock variables declared upstream,
    // and function arguments are considered declared upstream, the
    // special rule, if we are not in a nested scope (i.e. the block of
    // stmts is equal to the function block its self.)
    // then you CAN unlock/lock function arguments. Only function
    // arguments though.
    //

    let is_nested_scope = if block.clone() == func.body {
        false
    } else {
        true
    };


    // Walk statements in order. 
    for stmt in block {
        let stmt_span = helpers::stmt_span(&stmt);

        match stmt {
            Stmt::Const(cons) => {
                check_const(cons, locals, fun_sigs)?;
            },
            Stmt::VarDecl(var) => {
                if fun_sigs.contains_key(&var.name) {
                    return Err(HolyError::Semantic(format!(
                                "Variable identifier name `{}` is already taken by a function. (line {} column {})", 
                                &var.name, var.span.line, var.span.column
                            )));
                }

                let expr_ty = infer::infer_expr_type(&mut var.value, locals, fun_sigs, Some(var.type_name.clone()))?;
                if expr_ty != var.type_name {
                    return Err(HolyError::Semantic(format!(
                        "Type mismatch assigning to `{}`: got `{}`, expected `{}` (line {} column {})",
                        var.name, expr_ty, var.type_name, var.span.line, var.span.column
                    )));
                }

                // HolyLang commandment 1. You shall not overshadow variables
                if let Some(_) = locals.get(&var.name) {
                    return Err(HolyError::Semantic(format!(
                            "Variable `{}` is already declared, overshadowing is not allowed. (line {} column {})", 
                            &var.name, var.span.line, var.span.column
                        )));
                }


                let mut value_len: Option<usize> = None;

                // Check if source value is a variable and if its locked or moved, and moves it
                if let Expr::Var { name: src_name, span } = &var.value {
                    let src = locals.get_mut(src_name).unwrap_or_else(|| panic!(
                            "(Compiler bug) infer_expr_type should've already errored if source variable didnt exist, but it didnt. var: {:?}", 
                            var
                        ));

                    match &mut src.kind {
                        BindingKind::Var { moved: src_moved, len: src_len, .. } => {
                            if *src_moved {
                                panic!("(Compiler bug) infer_expr_type should've already errored if source variable is moved, but it didnt. var: {:?}", var)
                            }


                            // Error if we are in loop, and we tried to take ownership of an upstream variable
                            if in_loop && upstream_var_names.contains(&src_name) {
                                return Err(HolyError::Semantic(format!(
                                            "Upstream variable `{}` is potentially moved multiple times, because you are in a loop. Consider using `copy()` (line {} column {})", 
                                            &src_name, span.line, span.column
                                        )));
                            }

                            // mark source as moved because ownership was transferred
                            *src_moved = true;

                            // We copy its length
                            value_len = *src_len

                        },
                        // Ownership rules don't apply to constants.
                        BindingKind::Const { .. } => {}
                    }
                }

                match var.value.clone() {
                    Expr::ArrayLiteral {elements, .. } => {
                        value_len = Some(elements.len())
                    }

                    Expr::StringLiteral {value: v, ..} => {
                        value_len = Some(v.len())
                    }
                    // Other experessions we can't / don't need to store their length
                    _ => {}
                }


                // register variable in locals 
                locals.insert(
                    var.name.clone(),
                    BindingInfo {
                        ty: var.type_name.clone(),
                        kind: BindingKind::Var {
                            value: Some(var.value.clone()),
                            moved: false,
                            locked: false,
                            len: value_len
                        }
                    }
                );
            }

            Stmt::VarDeclMulti(var_list, call_expr) => {
                // Expect the right-hand side to be a function Call expression
                if let Expr::Call { name, args, span } = call_expr {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_vec = check_call(name, args, locals, fun_sigs, true, *span)?.unwrap();

                    if ret_vec.len() != var_list.len() {
                        return Err(HolyError::Semantic(format!(
                            "Return length mismatch: {} variables on the left side, but the call returns {} values (line {} column {})",
                            var_list.len(), ret_vec.len(), span.line, span.column
                        )));
                    }

                    // assign types and register locals
                    for (var, ret_ty) in var_list.iter_mut().zip(ret_vec.iter()) {
                        if var.type_name != *ret_ty {
                            return Err(HolyError::Semantic(format!(
                                "Type mismatch for variable `{}`: declared `{}` but call returns `{}` (line {} column {})",
                                var.name, var.type_name, ret_ty, var.span.line, var.span.column
                            )));
                        }


                        // HolyLang commandment 1. You shall not overshadow variables
                        if let Some(_) = locals.get(&var.name) {
                            return Err(HolyError::Semantic(format!(
                                    "Variable `{}` is already declared, overshadowing is not allowed. (line {} column {})", 
                                    &var.name, var.span.line, var.span.column
                                )));
                        }

                        // insert into locals
                        //
                        locals.insert(
                            var.name.clone(),
                            BindingInfo {
                                ty: var.type_name.clone(), 
                                kind: BindingKind::Var {
                                    value: None,
                                    moved: false, 
                                    locked: false,
                                    len: None
                                }
                            }
                        );
                    }
                } else {
                    return Err(HolyError::Semantic(format!(
                        "Multi-declarement requires only a single function call on the right-hand side (line {} column {})",
                        stmt_span.line, stmt_span.column
                    )));
                }
            }

            Stmt::VarAssign(assign) => {
                // NOTE: If any weird ass bugs arise, its this fucking clone.
                // we need it to satisify rust borrow checker
                //
                let varinfo = locals.get(&assign.name).ok_or_else(|| {
                    HolyError::Semantic(format!(
                        "Use of undeclared variable `{}` (line {} column {})",
                        assign.name, assign.span.line, assign.span.column
                    ))
                })?.clone();


                let expr_ty = infer::infer_expr_type(&mut assign.value, locals, fun_sigs, Some(varinfo.ty.clone()))?;
                if expr_ty != varinfo.ty {
                    return Err(HolyError::Semantic(format!(
                            "Type mismatch assigning to `{}`: got `{}`, expected `{}` (line {} column {})",
                            assign.name, expr_ty, varinfo.ty, assign.span.line, assign.span.column
                    )));
                }


                match varinfo.kind {
                    BindingKind::Var { moved, locked, .. } => {
                        // Check if our variable is moved
                        if moved {
                            return Err(HolyError::Semantic(format!(
                                "Value assignment to moved variable `{}` (line {} column {})",
                                &assign.name, assign.span.line, assign.span.column
                            )));
                        }

                
                        // Check if our variable is locked.
                        if locked {
                            return Err(HolyError::Semantic(format!(
                                    "Variable `{}` is locked, therefore you cannot assign to it (line {} column {})", 
                                    &assign.name, assign.span.line, assign.span.column
                                )));
                        }
                    },
                    BindingKind::Const { .. } => {
                        return Err(HolyError::Semantic(format!(
                            "You cannot assign to constant `{}` (line {} column {})",
                            &assign.name, assign.span.line, assign.span.column
                        )))
                    }
                }

                
                let mut value_len: Option<usize> = None;
              
                if let Expr::Var { name: src_name, span } = &assign.value {
                    let src = locals.get_mut(src_name).unwrap_or_else(|| panic!(
                                "(Compiler bug) infer_expr_type should've already errored if source variable didnt exist, but it didnt. assign: {:?}", 
                                assign
                            ));

                    match &mut src.kind {
                        BindingKind::Var { moved: src_moved, len: src_len, .. } => {
                            // Error if we are in loop, and we tried to take ownership of an upstream variable
                            if in_loop && upstream_var_names.contains(&src_name) {
                                return Err(HolyError::Semantic(format!(
                                            "Upstream variable `{}` is potentially moved multiple times, because you are in a loop. Consider using `copy()` (line {} column {})", 
                                            &src_name, span.line, span.column
                                        )));
                            }
            


                            if *src_moved {
                                panic!("(Compiler bug) infer_expr_type should've already errored if source variable is moved, but it didnt. assign: {:?}", assign)
                            }

                            // if source name is same as our variable name,
                            // then we don't move. It's re-claiming ownership.
                            if *src_name != assign.name { 
                                // mark source as moved because ownership was transferred
                                *src_moved = true;
                            }

                            // We copy its length
                            value_len = *src_len
                        },
                        // Ownership rules don't apply to constants.
                        BindingKind::Const { .. } => {}
                     }

                }

                match assign.value.clone() {
                    Expr::ArrayLiteral { elements, .. } => {
                        value_len = Some(elements.len());
                    }

                    Expr::StringLiteral { value: v, .. } => {
                        value_len = Some(v.len());
                    }
                    // Other experessions we can't / don't need to store their length
                    _ => {}
                }

                // Let us get a mutable varinfo to update length
                let varinfo = locals.get_mut(&assign.name).unwrap();

                match &mut varinfo.kind {
                    BindingKind::Var { len, .. } => {
                        *len = value_len;
                    },
                    BindingKind::Const { .. } => panic!("(Compiler bug) Variable is const, despite our supposed earlier checks that its not. Wtf ?: {:?}", varinfo)
                 }
            },

            Stmt::VarAssignMulti(expr) => {
                if let Expr::Call { name, args, span } = &mut expr.value {
                    // check_call with require_ret = true -> Option<Vec<Type>>
                    let ret_vec = check_call(name, args, locals, fun_sigs, true, *span)?.unwrap();

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

                        match varinfo.kind {
                            BindingKind::Var { moved, locked, .. } => {
                                if moved {
                                    return Err(HolyError::Semantic(format!(
                                        "Value assignment to moved variable `{}` (line {} column {})",
                                        var_name, expr.span.line, expr.span.column
                                    )));
                                }

                                // Check if variable is locked.
                                if locked {
                                    return Err(HolyError::Semantic(format!(
                                            "Variable `{}` is locked, therefore you cannot assign to it (line {} column {})", 
                                            &var_name, expr.span.line, expr.span.column
                                        )));
                                }
                            },
                            BindingKind::Const { .. } => {
                                return Err(HolyError::Semantic(format!(
                                    "You cannot assign to constant `{}` (line {} column {})",
                                    var_name, stmt_span.line, stmt_span.column
                                )))
                            }
                        }
                        
                        if &varinfo.ty != ret_ty {
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
                    let _ = infer::infer_expr_type(expr, locals, fun_sigs, None)?;
                }
            }

            Stmt::Lock(expr_vec) => {
                let mut var_names_to_lock: Vec<String> = vec![];

                for expr in expr_vec.iter_mut() {
                    match expr {
                        Expr::Var { name, span} => {
                            if var_names_to_lock.contains(name) {
                                return Err(HolyError::Semantic(format!(
                                    "Lock arguments have duplicated variable `{}` (line {} column {})",
                                    name, span.line, span.column
                                )))
                            }

                            if upstream_var_names.contains(&name) {
                                // Special rule: Despite fact you cannot lock/unlock variables declared upstream,
                                // and function arguments are considered declared upstream, the
                                // special rule, if we are not in a nested scope (i.e. the block of
                                // stmts is equal to the function block its self.)
                                // then you CAN unlock/lock function arguments. Only function
                                // arguments though.
                                //


                                if is_nested_scope || (!func.params.iter().any(|p| p.name == *name)) {
                                    return Err(HolyError::Semantic(format!(
                                            "You cannot lock variable `{}` because it is declared upstream (line {} column {})", 
                                            name, span.line, span.column
                                        )));
                                }  
                            }

                            var_names_to_lock.push(name.to_string());


                            // We dont care about its type, we just checking if it exists or not,
                            // and its contents are valid, etc.
                            infer::infer_expr_type(expr, locals, fun_sigs, None)?;

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

                    match &mut var.kind {
                        BindingKind::Var { locked, .. } => {
                            if *locked == true {
                                return Err(HolyError::Semantic(format!(
                                        "Variable `{}` is already locked (line {} column {})",
                                        var_name, stmt_span.line, stmt_span.column
                                    )))

                            }
                            *locked = true;
                        },
                        BindingKind::Const { .. } => {
                            return Err(HolyError::Semantic(format!(
                                "Expected variable name, instead got a constant name `{}` (line {} column {})",
                                var_name, stmt_span.line, stmt_span.column
                            )))

                        }
                    }
                }

            }


            Stmt::Unlock(expr_vec) => {
                let mut var_names_to_unlock: Vec<String> = vec![];

                for expr in expr_vec.iter_mut() {
                    match expr {
                        Expr::Var { name, span} => {
                            if var_names_to_unlock.contains(name) {
                                return Err(HolyError::Semantic(format!(
                                    "Unlock arguments have duplicated variable `{}` (line {} column {})",
                                    name, span.line, span.column
                                )))
                            }

                            if upstream_var_names.contains(&name) {
                                // Special rule: Despite fact you cannot lock/unlock variables declared upstream,
                                // and function arguments are considered declared upstream, the
                                // special rule, if we are not in a nested scope (i.e. the block of
                                // stmts is equal to the function block its self.)
                                // then you CAN unlock/lock function arguments. Only function
                                // arguments though.
                                //
                                if is_nested_scope || (!func.params.iter().any(|p| p.name == *name)) {
                                    return Err(HolyError::Semantic(format!(
                                            "You cannot unlock variable `{}` because it is declared upstream (line {} column {})", 
                                            name, span.line, span.column
                                        )));
                                }    
                            }


                            var_names_to_unlock.push(name.to_string());


                            // We dont care about its type, we just checking if it exists or not,
                            // and its contents are valid, etc.
                            infer::infer_expr_type(expr, locals, fun_sigs, None)?;

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

                    match &mut var.kind {
                        BindingKind::Var { locked, .. } => {
                            if *locked == false {
                                return Err(HolyError::Semantic(format!(
                                        "Variable `{}` is already unlocked (line {} column {})",
                                        var_name, stmt_span.line, stmt_span.column
                                    )))

                            }
                            *locked = false;
                        },
                        BindingKind::Const { .. } => {
                            return Err(HolyError::Semantic(format!(
                                "Expected variable name, instead got a constant name `{}` (line {} column {})",
                                var_name, stmt_span.line, stmt_span.column
                            )))

                        }
                    }
                }


            }

            Stmt::Return(expr_vec) => {
                // If function has no declared return type, we error.
                match &func.return_type {
                    None => {
                        return Err(HolyError::Semantic(format!(
                            "Function `{}` contains return but has no declared return type (line {} column {})",
                            func.name,
                            stmt_span.line,
                            stmt_span.column,
                        )));
                    }
                    Some(declared_ty_vec) => {

                        if declared_ty_vec.len() != expr_vec.len() {
                            return Err(HolyError::Semantic(format!(
                                    "Return length mismatch in `{}`: got `{}` expressions, expected `{}` expressions (line {} column {})",
                                    func.name, expr_vec.len(), declared_ty_vec.len(), stmt_span.line, stmt_span.column,
                                )));
                        
                        }

                        for (i, expr) in expr_vec.iter_mut().enumerate() {
                            let declared_ty = declared_ty_vec[i].clone();
                            let expr_ty = infer::infer_expr_type(expr, locals, fun_sigs, Some(declared_ty.clone()))?;

                            if expr_ty != declared_ty {
                                return Err(HolyError::Semantic(format!(
                                    "Return type mismatch in `{}`: got `{}`, expected `{}` (line {} column {})",
                                    func.name, expr_ty, declared_ty_vec[i], stmt_span.line, stmt_span.column,
                                )));
                            }
                            
                        }
                    }
                }
            }


            Stmt::For(for_stmt) => {
                let expr_ty = infer::infer_expr_type(&mut for_stmt.value, locals, fun_sigs, None)?;

                if (!expr_ty.is_array_type()) && (!matches!(for_stmt.value, Expr::RangeCall{ .. })) {
                    return Err(HolyError::Semantic(format!(
                        "For loop statement require an expression to be evaulatable to any `Array` type, or `range(expr1, expr2)`, instead we got `{}` (line {} column {})",
                        expr_ty, stmt_span.line, stmt_span.column,
                    )));
                }


                if let Some(_) = locals.get(&for_stmt.holder_name) {
                    return Err(HolyError::Semantic(format!(
                        "Cannot use variable name `{}` in for loop statement as it is already declared. (line {} column {})",
                        for_stmt.holder_name, stmt_span.line, stmt_span.column,
                    )));

                }

                // If this is a for looping over an array, move the array only if its not an array
                // literal. i.e. its a variable that holds an array.
                if expr_ty.is_array_type()  {
                    if let Expr::Var { name, .. } = &for_stmt.value {
                        let src = locals.get_mut(name).unwrap_or_else(|| panic!(
                            "(Compiler bug) infer_expr_type should've already errored if the array variable didnt exist, but it didnt. for_stmt: {:?}", 
                            for_stmt
                        ));

                        match &mut src.kind {
                            BindingKind::Var { moved: src_moved, .. } => {
                                if *src_moved {
                                    panic!("(Compiler bug) infer_expr_type should've already errored if the array variable is moved, but it didnt. for_stmt: {:?}", for_stmt)
                                }

                                *src_moved = true;
                            },
                            // Ownership rules don't apply to constants.
                            BindingKind::Const { .. } => {}
                        }
                    }
                }


                let mut locals_clone = locals.clone();

                // We inject the holder variable into the locals. It is "fake" variable that does
                // not exist in the AST, but we need it in locals to make analysis work.
                //

                let decided_ty: Type;

                if let Type::Array(inner_ty) = expr_ty {
                    decided_ty = *inner_ty;

                } else if let Type::FixedArray(inner_ty, _) = expr_ty {
                    decided_ty = *inner_ty;
                    
                } else if expr_ty.is_integer_type() {
                    decided_ty = expr_ty;
                } else {
                    panic!(
                        "(Compiler bug) Expected for loop expression to either be an array or an integer (more precisely an integer cuz programmer used range()), instead we got: {:?} {:?}", 
                        for_stmt.value, expr_ty);
                }



                // NOTE only specific if decided_ty is of an array: 
                //      Out-of-bounds access protection here is non-existent, but thats fine because Rust
                //      will catch at transpile layer
                //      However it would be nicer if we can do better job of catching out of bounds
                //      for literals at compile time.
                //
                locals_clone.insert(
                    for_stmt.holder_name.clone(),
                    BindingInfo {
                        ty: decided_ty,
                        kind: BindingKind::Var {
                            value: None,
                            moved: false,
                            locked: true, // the holder variable is locked by default.
                            len: None
                        }
                    }
                );


                
                // This gets all upstream variable names, and passes it to check stmts to ensure
                // you cannot overshadow them.
                let mut upstream = upstream_var_names.clone();
                for var_name in locals_clone.keys() {
                    upstream.push(var_name.to_string());
                }

                // We also add holder_name to the list to prevent programmer overshadowing the
                // variable within the loop.
                upstream.push(for_stmt.holder_name.clone());

                    
                check_stmts(func.clone(), &mut for_stmt.branch, &mut locals_clone, upstream.clone(), fun_sigs, true)?;
                update_local_assignments_from_clone(locals, locals_clone);
            }


            Stmt::While(while_stmt) => {
                let expr_ty = infer::infer_expr_type(&mut while_stmt.condition, locals, fun_sigs, Some(Type::Bool))?;
                
                if expr_ty != Type::Bool {
                    return Err(HolyError::Semantic(format!(
                        "While statement require an expression to be evaulatable to type `bool`, instead we got `{}` (line {} column {})",
                        expr_ty, stmt_span.line, stmt_span.column,
                    )));
                }

                // This gets all upstream variable names, and passes it to check stmts to ensure
                // you cannot overshadow them.
                let mut upstream = upstream_var_names.clone();
                for var_name in locals.keys() {
                    upstream.push(var_name.to_string());
                }
                    
                let mut locals_clone = locals.clone();
                check_stmts(func.clone(), &mut while_stmt.branch, &mut locals_clone, upstream.clone(), fun_sigs, true)?;
                update_local_assignments_from_clone(locals, locals_clone);
                
            }

            Stmt::Infinite(infinite_stmt) => {
                // This gets all upstream variable names, and passes it to check stmts to ensure
                // you cannot overshadow them.
                let mut upstream = upstream_var_names.clone();
                for var_name in locals.keys() {
                    upstream.push(var_name.to_string());
                }
                    
                let mut locals_clone = locals.clone();
                check_stmts(func.clone(), &mut infinite_stmt.branch, &mut locals_clone, upstream.clone(), fun_sigs, true)?;
                update_local_assignments_from_clone(locals, locals_clone);
                
            }



            Stmt::Break(break_stmt) => {
                if !in_loop {
                    return Err(HolyError::Semantic(format!(
                        "Break can only be used in loops! (line {} column {})",
                        break_stmt.span.line, break_stmt.span.column,
                    )));
                }

            }

            Stmt::Continue(continue_stmt) => {
                if !in_loop {
                    return Err(HolyError::Semantic(format!(
                        "Continue can only be used in loops! (line {} column {})",
                        continue_stmt.span.line, continue_stmt.span.column,
                    )));
                }
            }

            Stmt::If(if_stmt) => {
                let main_expr_ty = infer::infer_expr_type(&mut if_stmt.condition, locals, fun_sigs, Some(Type::Bool))?;
                
                if main_expr_ty != Type::Bool {
                    return Err(HolyError::Semantic(format!(
                        "If statement require an expression to be evaulatable to type `bool`, instead we got `{}` (line {} column {})",
                        main_expr_ty, stmt_span.line, stmt_span.column,
                    )));
                }

                // This gets all upstream variable names, and passes it to check stmts to ensure
                // you cannot overshadow them.
                let mut upstream = upstream_var_names.clone();
                for var_name in locals.keys() {
                    upstream.push(var_name.to_string());
                }

                // This is for elif, because elif is run many times we want single copy from clean
                // state.
                let locals_clone = locals.clone();

                    
                let mut main_locals_clone = locals.clone();
                let mut else_locals_clone = locals.clone();

                check_stmts(func.clone(), &mut if_stmt.if_branch, &mut main_locals_clone, upstream.clone(), fun_sigs, in_loop)?;
                update_local_assignments_from_clone(locals, main_locals_clone);
                

                for s in &mut if_stmt.elif_branches {
                    let elif_expr_ty = infer::infer_expr_type(&mut s.0, locals, fun_sigs, Some(Type::Bool))?; 

                    if elif_expr_ty != Type::Bool {
                        return Err(HolyError::Semantic(format!(
                            "Elif statements require an expression to be evaulatable to type `bool`, instead we got `{}` (line {} column {})",
                            elif_expr_ty, stmt_span.line, stmt_span.column,
                        )));
                    }

                
                    let mut elif_locals_clone = locals_clone.clone();
                    check_stmts(func.clone(), &mut s.1, &mut elif_locals_clone, upstream.clone(), fun_sigs, in_loop)?;
                    update_local_assignments_from_clone(locals, elif_locals_clone);
                }

                if let Some(else_stmts) = if_stmt.else_branch.as_mut() {
                    check_stmts(func.clone(), else_stmts, &mut else_locals_clone, upstream, fun_sigs, in_loop)?;
                    update_local_assignments_from_clone(locals, else_locals_clone);
                }
                
            }
        }
    }

    Ok(())
}

fn update_local_assignments_from_clone(upstream: &mut HashMap<String, BindingInfo>, downstream: HashMap<String, BindingInfo> ) {
    // We loop over the downstream locals, to update our 
    // corresponding upstream locals
    // like variable assignments, length change, ownership change, etc
    //
    for (n, vi) in downstream {
        if let Some(info) = upstream.get_mut(&n) {
            match info.kind {
                BindingKind::Var { moved, ..} => {
                    // If variable is already moved, we don't care about its assignments no more, we just
                    // skip.
                    if moved == true {
                        continue
                    }
                    *info = vi.clone();
                },
                BindingKind::Const { .. } => continue
            }
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
    args: &mut Vec<Expr>,
    locals: &mut HashMap<String, BindingInfo>,
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
        let arg_ty = infer::infer_expr_type(arg_expr, locals, fun_sigs, Some(param_ty.clone()))?;
        if arg_ty != *param_ty {
            return Err(HolyError::Semantic(format!(
                "Argument number `{}` type mismatch in call to `{}`: expected `{}`, got `{}` (line {} column {})",
                i + 1, name, param_ty, arg_ty, span.line, span.column,
            )));
        }

        // If this arg is a variable, mark it moved (same semantics as before)
        if let Expr::Var { name: vname, span: _ } = arg_expr {
            let v = locals.get_mut(vname).unwrap_or_else(|| panic!(
                    "(Compiler bug) infer_expr_type should've already errored if source argument variable didnt exist, but it didnt. arg_expr: {:?}", 
                    arg_expr
                ));

            match &mut v.kind {
                BindingKind::Var { moved, ..} => {
                    if *moved {
                        panic!("(Compiler bug) infer_expr_type should've already errored if source variable is moved, but it didnt. arg_expr: {:?}", arg_expr)
                    }

                    *moved = true;
                },
                // Ownership rules dont apply to constants
                BindingKind::Const { .. } => {}
            }
        }
    }

    // return handling
    match ret_ty_opt {
        Some(rt) => Ok(Some(rt.clone())),
        None => {
            if require_ret {
                Err(HolyError::Semantic(format!(
                    "Function `{}` has no return type declared but is used in an expression (line {} column {})",
                    name, span.line, span.column
                )))
            } else {
                Ok(None)
            }
        }
    }
}

