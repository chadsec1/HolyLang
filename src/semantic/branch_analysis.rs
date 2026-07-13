use super::{
    Stmt, 
    GoldError, 
    helpers, 
    Function
};

pub fn dead_code_analysis(block: &Vec<Stmt>, in_loop: bool) -> Result<bool, GoldError> {
    // Instead of returning error here, we panic, because if we returned an error here
    // we would not have ability to pinpoint to the empty branch line. leaving responsiblity to
    // caller is best.
    //
    assert!(!block.is_empty(), "(Compiler bug) we got called with an empty block. Always check block size before calling `dead_code_analysis`");

    let mut end_detected = false;
    
    for stmt in block {
        if end_detected {
            let stmt_span = helpers::stmt_span(stmt);

            return Err(GoldError::Semantic(format!(
                        "Dead code detected starting from line `{}` up to the end of the scope",
                        stmt_span.line,
                    )));

        }

        match stmt {
            Stmt::Return(_) => end_detected = true,
            Stmt::Break(_) if in_loop => {
                end_detected = true; 
            },
            Stmt::Infinite(infinite_stmt) => {
                let body = &infinite_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "Infinite loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            infinite_stmt.span.line, infinite_stmt.span.column,
                        )));

                }

                let inner_terminates = dead_code_analysis(body, true)?;
                
                if inner_terminates { 
                    end_detected = true;
                }
            }


            Stmt::While(while_stmt) => {
                let body = &while_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "While loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            while_stmt.span.line, while_stmt.span.column,
                        )));

                }


                dead_code_analysis(body, in_loop)?;
            },

            Stmt::For(for_stmt) => {
                let body = &for_stmt.branch;
                if body.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "For loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            for_stmt.span.line, for_stmt.span.column,
                        )));

                }


                dead_code_analysis(body, in_loop)?;
            },

            Stmt::If(if_stmt) => {
                if if_stmt.if_branch.is_empty() {
                    return Err(GoldError::Semantic(format!(
                            "If statement main branch has no statements. Empty branches are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column,
                        )));
                }

                let if_term: bool = dead_code_analysis(&if_stmt.if_branch, in_loop)?;

                let mut elifs_term = true;

                for s_vec in &if_stmt.elif_branches {
                    let expr_span = helpers::expr_span(&s_vec.0);

                    if s_vec.1.is_empty() {
                        return Err(GoldError::Semantic(format!(
                            "If statement `elif` branch has no statements. Empty branches are not allowed (line {} column {})",
                            expr_span.line, expr_span.column,
                        )));
                    }

                    if !dead_code_analysis(&s_vec.1, in_loop)? {
                        elifs_term = false;
                    }
                }

                // Check if statements branches all terminates
                //
                if let Some(else_branch) = &if_stmt.else_branch {
                    if else_branch.is_empty() {
                        return Err(GoldError::Semantic(format!(
                            "If statement `else` branch has no statements. Empty branches are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column,
                        )));
                    } 
                
                    let else_term = dead_code_analysis(else_branch, in_loop)?;

                    if if_term && else_term && elifs_term {
                        end_detected = true;
                    }
                }
            },

            _ => {}
        }
        
    }

    Ok(end_detected)
}


#[expect(clippy::too_many_lines)]
pub fn return_branch_analysis(
    func: &Function,
    last_stmt: &Stmt,
    is_loop: bool,
    forbid_break: bool
) -> Result<(), GoldError> {
    let ret_ty = func.return_type.as_ref().unwrap_or_else(|| panic!("(Compiler bug) Dont call return_branch_analysis on functions that dont have declared return type(s)!"));

    assert!(!func.body.is_empty(), "(Compiler bug) do not call return_branch_analysis on functions with empty bodies! Always check body size");

    match last_stmt {
        Stmt::Break(break_stmt) => {
            // Just a compiler bug guard.
            assert!(is_loop, "(Compiler bug) check_stmts shouldve errored before we even got called. We got a break statement when we arent even in a loop!");

            if forbid_break {
                return Err(GoldError::Semantic(format!(
                        "You cannot `break` out of a infinite loop if its the last statement in a function that returns. Use a return statement instead. (line {} column {})",
                        break_stmt.span.line, break_stmt.span.column,
                    )));
            }

        },
        Stmt::Return(_) => {},
        Stmt::Infinite(infinite_stmt) => {
            // This is weak check, but I will keep it. It can catch (some) bugs.
            assert!(
                !infinite_stmt.branch.is_empty(), 
                    "(Compiler bug) infinite loop branch is empty! this shouldve been caught by dead_code_analyse before calling us:\nFunc: {func:?}\ninfinite_stmt: {infinite_stmt:?}");

            // If we are in a nested loop(s), we dont care about breaks or whatever.
            // We only care about upper most level infinite loop.
            //
            // Otherwise, we execute this block which ensures you can't break out of the infinite
            // loop because it's last statement in a function that returns
            //
            if !is_loop {
                // So, why do we error on break? can't programmer like break then return outside for
                // loop?
                // Answer is that return_branch_analysis is only called on last statemet, and if
                // infinite loop is last statement, you can't break out of it. You can only return, or
                // you dont return but you don't break.
                //
                for s in &infinite_stmt.branch {
                    match s {
                        Stmt::Break(break_stmt) => {
                            return Err(GoldError::Semantic(format!(
                                "You cannot `break` out of a infinite loop if its the last statement in a function that returns. Use a return statement instead. (line {} column {})",
                                break_stmt.span.line, break_stmt.span.column,
                            )));
                        }

                        Stmt::If(_) => {
                            return_branch_analysis(func, s, true, true)?;
                        }


                        Stmt::While(_) | Stmt::For(_) | Stmt::Infinite(_) => {
                            return_branch_analysis(func, s, true, false)?;
                        }



                        // Skip all other statements
                        _ => {}
                    }
                }
            }
        }

        Stmt::While(while_stmt) => {
            // If this is a nested loop, like a while loop inside a `infinite` loop, we let you do
            // that. if in_loop is true, it might not be last statement after all.
            //

            assert!(
                !while_stmt.branch.is_empty(),
                "(Compiler bug) all branches must contain at least one statement, this shouldve been caught by dead_code_analyse before calling us:\nFunc: {func:?}\nwhile_stmt: {while_stmt:?}");

            if !is_loop {
                return Err(GoldError::Semantic(format!(
                        "While loops may or may not execute at all, therefore you need a return statement outside the loop scope, or consider using `infinite` loops instead. (line {} column {})",
                        while_stmt.span.line, while_stmt.span.column,
                    )))
            
            }
        },
        
        Stmt::For(for_stmt) => {
            assert!(!for_stmt.branch.is_empty(), "(Compiler bug) all branches must contain at least one statement, this shouldve been caught by dead_code_analyse before calling us:\nFunc: {func:?}\nfor_stmt: {for_stmt:?}");

            if !is_loop {
                return Err(GoldError::Semantic(format!(
                        "For loops may or may not execute at all, therefore you need a return statement outside the loop scope. (line {} column {})",
                        for_stmt.span.line, for_stmt.span.column,
                    )))
            }
        },

        Stmt::If(if_stmt) => {
            // If we are not in a loop, then we only care about last statement of if branches
            // bodies
            if is_loop {
                for stmt in &if_stmt.if_branch {
                    return_branch_analysis(func, stmt, is_loop, forbid_break)?;
                }
                
                // We dont care if else branch is none, we in a loop. 
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        return_branch_analysis(func, stmt, is_loop, forbid_break)?;
                    }
                }

                for s_vec in &if_stmt.elif_branches {
                    let body = &s_vec.1;

                    for stmt in body {
                        return_branch_analysis(func, stmt, is_loop, forbid_break)?;
                    }
                }

            } else {
                let main_branch_last_stmt = if_stmt.if_branch.last().unwrap_or_else(|| { panic!(
                        "(Compiler bug) if statement main branch is empty! this shouldve been caught by dead_code_analyse before calling us:\nFunc: {func:?}\nif_stmt: {if_stmt:?}"
                    )});

                return_branch_analysis(func, main_branch_last_stmt, is_loop, forbid_break)?;

                if let Some(else_branch) = &if_stmt.else_branch {
                    let else_branch_last_stmt = else_branch.last().unwrap_or_else(|| { panic!(
                            "(Compiler bug) if statement else branch is empty! this shouldve been caught by dead_code_analyse before calling us:\nFunc: {func:?}\nif_stmt: {if_stmt:?}"
                        )});

                    return_branch_analysis(func, else_branch_last_stmt, is_loop, forbid_break)?;
                } else {
                    return Err(GoldError::Semantic(format!(
                        "Function `{}` only returns in if statement branches, which might not always execute. Add an `else` branch (line {} column {})",
                        func.name, if_stmt.span.line, if_stmt.span.column,
                    )));
                }

                for s_vec in &if_stmt.elif_branches {
                    let body = &s_vec.1;

                    let elif_branch_last_stmt = body.last().unwrap();
                    return_branch_analysis(func, elif_branch_last_stmt, is_loop, forbid_break)?;
                }
            }
        },
        other => {
            if !is_loop {
                let branch_span = helpers::stmt_span(other);

                return Err(GoldError::Semantic(format!(
                    "Function `{}` declares return type(s) `{:?}`, but statement branch body does not end with a return statement (line {} column {})",
                    func.name, ret_ty, branch_span.line, branch_span.column,
                ))) 
            }
        },
    }

    Ok(())
}

