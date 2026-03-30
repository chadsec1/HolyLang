use super::*;


pub fn dead_code_analysis(block: &Vec<Stmt>) -> Result<(), HolyError> {

    // Instead of returning error here, we panic, because if we returned an error here
    // we would not have ability to pinpoint to the empty branch line. leaving responsiblity to
    // caller is best.
    if block.len() == 0 {
        panic!("(Compiler bug) we got called with an empty block. Always check block size before calling dead_code_analysis");
    }

    let mut end_detected = false;
    
    for stmt in block {
        if end_detected {
            let stmt_span = helpers::stmt_span(&stmt);

            return Err(HolyError::Semantic(format!(
                        "Dead code detected starting from line `{}` up to the end of the scope",
                        stmt_span.line,
                    )));

        }

        match stmt {
            Stmt::Break(_) | Stmt::Return(_) => {
                end_detected = true; 
            }

            Stmt::Infinite(infinite_stmt) => {
                let body = &infinite_stmt.branch;
                if body.len() == 0 {
                    return Err(HolyError::Semantic(format!(
                            "Infinite loop branch has no statements. Empty branches are not allowed (line {} column {})",
                            infinite_stmt.span.line, infinite_stmt.span.column,
                        )));

                }


                dead_code_analysis(body)?;
                
                if block_always_terminates(&infinite_stmt.branch) {
                    end_detected = true;
                }
            }


            Stmt::If(if_stmt) => {
                if if_stmt.if_branch.len() == 0 {
                    return Err(HolyError::Semantic(format!(
                            "Empty `if` branch are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column,
                        )));
                }

                dead_code_analysis(&if_stmt.if_branch)?;

                if if_stmt.else_branch.is_some() {
                    if if_stmt.else_branch.as_ref().unwrap().len() == 0 {
                        return Err(HolyError::Semantic(format!(
                            "Empty `else` branch detected for if statement. Empty branches are not allowed (line {} column {})",
                            if_stmt.span.line, if_stmt.span.column,
                        )));
                    } 
                
                    dead_code_analysis(&if_stmt.else_branch.as_ref().unwrap())?;
                }

                for s_vec in &if_stmt.elif_branches {
                    let body = &s_vec.1;

                    let expr_span = helpers::expr_span(&s_vec.0);

                    if body.len() == 0 {
                        return Err(HolyError::Semantic(format!(
                            "Empty `elif` branches are not allowed (line {} column {})",
                            expr_span.line, expr_span.column,
                        )));
                    }

                    dead_code_analysis(body)?;
                }



                // Check if statements branches all terminates
                if if_stmt.else_branch.is_some() {
                    let if_term = block_always_terminates(&if_stmt.if_branch);
                    let else_term = block_always_terminates(if_stmt.else_branch.as_ref().unwrap());

                    // Apparently this is fine because `.all` returns true if elif_branches are
                    // empty.
                    let elifs_term = if_stmt.elif_branches
                        .iter()
                        .all(|s_vec| block_always_terminates(&s_vec.1));

                    if if_term && else_term && elifs_term {
                        end_detected = true;
                    }
                }




            }
            

            _ => {}
        }
        
    }

    Ok(())
}


/// Recursive helper that tells us if a block of code terminates or not
/// Like if it returns or breaks, then it terminates. 
pub fn block_always_terminates(block: &Vec<Stmt>) -> bool {
    for stmt in block {
        match stmt {
            Stmt::Return(_) | Stmt::Break(_) => return true,
            Stmt::If(if_stmt) => {
                // Without an else, we can't guarantee termination
                // because the if might not execute at all
                if if_stmt.else_branch.is_none() {
                    continue;
                }
                let if_terminates = block_always_terminates(&if_stmt.if_branch);
                let else_terminates = block_always_terminates(
                    if_stmt.else_branch.as_ref().unwrap()
                );
                let elifs_terminate = if_stmt.elif_branches
                    .iter()
                    .all(|s_vec| block_always_terminates(&s_vec.1));

                if if_terminates && else_terminates && elifs_terminate {
                    return true;
                }
            }

            Stmt::Infinite(infinite_stmt) => {
                return block_always_terminates(&infinite_stmt.branch);
            } 


            _ => {}
        }
    }
    false
}



pub fn return_branch_analysis(
    func: &Function,
    last_stmt: Option<Stmt>,
    is_loop: bool,
    forbid_break: bool
) -> Result<(), HolyError> {
    let ret_ty = func.return_type.clone().unwrap();

    match last_stmt {

        Some(Stmt::Break(break_stmt)) => {
            if forbid_break {
                return Err(HolyError::Semantic(format!(
                        "You cannot `break` out of a forever loop if its the last statement in a function that returns. Use a return statement instead. (line {} column {})",
                        break_stmt.span.line, break_stmt.span.column,
                    )));
            }

        }

        Some(Stmt::Return(_)) => {}

        Some(Stmt::Infinite(infinite_stmt)) => {

            // If we are in a nested loop(s), we dont care about breaks or whatever.
            // We only care about upper most level infinite loop.
            //
            //
            if !is_loop {
                // So, why do we error on break? can't programmer like break then return outside for
                // loop?
                // Answer is that return_branch_analysis is only called on last statemet, and if
                // forever loop is last statement, you can't break out of it. You can only return, or
                // you dont return but you don't break.
                //
                for s in &infinite_stmt.branch {
                    match s {
                        Stmt::Break(break_stmt) => {

                            // If this is a nested loop, like a forever inside another forever, you can
                            // break out of it fine.
                            if !is_loop {
                                return Err(HolyError::Semantic(format!(
                                    "You cannot `break` out of a forever loop if its the last statement in a function that returns. Use a return statement instead. (line {} column {})",
                                    break_stmt.span.line, break_stmt.span.column,
                                )));
                                
                            }
                        }

                        Stmt::If(_) => {
                            return_branch_analysis(func, Some(s.clone()), true, true)?;
                        }


                        Stmt::While(_) => {
                            return_branch_analysis(func, Some(s.clone()), true, false)?;
                        }

                        Stmt::Infinite(_) => {
                            return_branch_analysis(func, Some(s.clone()), true, false)?;
                        }



                        // Skip all other statements
                        _ => {}
                    }
                }
            }
        }

        Some(Stmt::While(while_stmt)) => {
            // If this is a nested loop, like a while loop inside a `forever` loop, we let you do
            // that. if in_loop is true, it might not be last statement after all.
            //
            if !is_loop {
                return Err(HolyError::Semantic(format!(
                        "While loops may or may not execute at all, therefore you need a return statement outside the loop scope, or consider using `forever` infinite loops instead. (line {} column {})",
                        while_stmt.span.line, while_stmt.span.column,
                    )));
            
            }
        }
        
        Some(Stmt::For(for_stmt)) => return Err(HolyError::Semantic(format!(
                    "For loops may or may not execute at all, therefore you need a return statement outside the loop scope. (line {} column {})",
                    for_stmt.span.line, for_stmt.span.column,
                ))),
        

        Some(Stmt::If(if_stmt)) => {

            // If we are not in a loop, then we only care about last statement of if branches
            // bodies
            if !is_loop {
                let stmt = if_stmt.if_branch.last();
                return_branch_analysis(func, stmt.cloned(), is_loop, forbid_break)?;

                if if_stmt.else_branch.is_none() {
                    return Err(HolyError::Semantic(format!(
                        "Function `{}` only returns in if statement branches, which might not always execute. Add an `else` branch (line {} column {})",
                        func.name, if_stmt.span.line, if_stmt.span.column,
                    )));
                }

                let stmt = if_stmt.else_branch.as_ref().unwrap().last();

                return_branch_analysis(func, stmt.cloned(), is_loop, forbid_break)?;


                for s_vec in &if_stmt.elif_branches {
                    let body = &s_vec.1;

                    let stmt = body.last();
                    return_branch_analysis(func, stmt.cloned(), is_loop, forbid_break)?;
                }

            } else {
                for stmt in &if_stmt.if_branch {
                    return_branch_analysis(func, Some(stmt).cloned(), is_loop, forbid_break)?;
                }
                
                // We dont care if else branch is none, we in a loop. 
                if if_stmt.else_branch.is_some() {
                    for stmt in &if_stmt.else_branch.unwrap() {
                        return_branch_analysis(func, Some(stmt).cloned(), is_loop, forbid_break)?;
                    }
                }

                for s_vec in &if_stmt.elif_branches {
                    let body = &s_vec.1;


                    for stmt in body {
                        return_branch_analysis(func, Some(stmt).cloned(), is_loop, forbid_break)?;
                    }
                }


            }
        },
        Some(other) => {
            if !is_loop {
                let branch_span = helpers::stmt_span(&other);

                return Err(HolyError::Semantic(format!(
                    "Function `{}` declares return type(s) `{:?}`, but statement branch body does not end with a return statement (line {} column {})",
                    func.name, ret_ty, branch_span.line, branch_span.column,
                ))) 
            }
        },


        _ => panic!("(Compiler bug) dead code analysis should've errored when it encounterd an empty block, but it didn't:\nFunc: {:?}\nlast_stmt: {:?}", func, last_stmt)
    }


    Ok(())
}


