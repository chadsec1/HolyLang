use crate::error::HolyError;
use crate::ast::*;

#[cfg(test)]
mod blackbox_tests;

#[cfg(test)]
mod parse_expr_tests;

#[cfg(test)]
mod helpers_tests;

mod helpers;
mod parse_expr;

/// Internal to `parse_block` and `parse_if_stmt`.
enum BlockEnd {
    Done, 
    Continuation
}

/// Public parse entry
pub fn parse(source: &str) -> Result<AST, HolyError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0usize;
    let mut ast = AST {
        globals: vec![],
        functions: vec![]
    };

    while i < lines.len() {
        let raw = lines[i];
        let line = raw.trim();

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Either the start of a function
        if line.starts_with("func ") {
            // Parse function header and body
            let (func, new_i) = parse_function(&lines, i)?;
            ast.functions.push(func);
            i = new_i;
            continue;

        // Or its a global statement.
        } else {
            let span = Span { line: i + 1, column: 0 };
            if line.starts_with("const ") {
                let cons = parse_const_stmt(line, span)?;
                ast.globals.push(GlobalStmt::Const(cons));
                i += 1;
                continue;
            }

            return Err(HolyError::Parse(format!("Statement `{}` cannot be in the global scope (line {})", line, span.line)));
        }
    }

    Ok(ast)
}

/// Parse function starting at index `start_i`.
/// Returns (Function, index after function end).
fn parse_function(lines: &Vec<&str>, start_i: usize) -> Result<(Function, usize), HolyError> {
    let span = Span { line: start_i + 1, column: 0 };
    
    let header_raw = lines[start_i].trim();
    // header like: func add(a int32, b int32) int32 {
    let after_func = &header_raw["func ".len()..];


    // find '(' matching for params
    let open_paren = after_func.find('(').ok_or_else(|| {
        HolyError::Parse(format!("Invalid function header (no '(') at line {}: `{}`", start_i + 1, header_raw))
    })?;
    
    let name = after_func[..open_paren].trim().to_string();
    if name.ends_with(")")  {
        return Err(HolyError::Parse(format!("Invalid function header: there is an extra closing parenthesis `)` in the function declaration header `{}` at line {}", header_raw, start_i + 1)));
    }



    helpers::validate_identifier_name(&name)
        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

    let rest = &after_func[open_paren..]; // starts with '('
    let close_paren = rest.find(')').ok_or_else(|| {
        HolyError::Parse(format!("Invalid function header (no ')') at line {}: `{}`", start_i + 1, header_raw))
    })?;

    let params_str = &rest[1..close_paren]; // contents inside ()
    let after_params = rest[close_paren + 1..].trim();

    let brace_pos = after_params.find('{').ok_or_else(|| {
        HolyError::Parse(format!("Missing '{{' after function header at line {}: `{}`", start_i + 1, header_raw))
    })?;

    let return_type_str = after_params[..brace_pos].trim();
    let after_brace = after_params[brace_pos+1..].trim();

    if !after_brace.is_empty() {
        return Err(HolyError::Parse(format!("Function body statements must start on the next line (line {})", start_i + 1)));
    }

    
    let return_type = if return_type_str.is_empty() {
        None
    } else {
        if return_type_str.starts_with('(') {
            if !return_type_str.ends_with(')') {
                return Err(HolyError::Parse(format!("Missing closing parentheses for return type in function `{}` at line {}", name, start_i + 1)));
            }

            let inner = &return_type_str[1..return_type_str.len()-1].trim();
            let mut types = Vec::new();
            if inner.is_empty() {
                return Err(HolyError::Parse(format!("Missing types in the `()` return types function `{}` headers at line {}", name, start_i + 1)));
            }
            let split_parts = helpers::split_char_top_level(',', inner)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;


            for part in split_parts {
                let t = parse_type(part.trim(), &span)?;
                types.push(t);
            }

            Some(types)
        } else if return_type_str.ends_with(')') {
            return Err(HolyError::Parse(format!("Missing opening parentheses for return type in function `{}` at line {}", name, start_i + 1)));
        } else {
            Some(vec![parse_type(return_type_str, &span)?])
        }
    };

    // parse params
    let mut params = vec![];
    if !params_str.trim().is_empty() {
        for p in params_str.split(',') {
            let p = p.trim();
            let parts: Vec<&str> = p.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(HolyError::Parse(format!("Invalid parameter `{}` at line {}", p, start_i + 1)));
            }
            let pname = parts[0].to_string();
            helpers::validate_identifier_name(&pname)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            let ptype = parse_type(parts[1], &span)?;
            params.push(Param { name: pname, type_name: ptype, span: span });
        }
    }

    // parse body
    let mut body: Vec<Stmt> = vec![];
    let mut idx = start_i + 1;

    while idx < lines.len() {
        let raw = lines[idx];
        let t = helpers::strip_inline_comment(raw).trim().to_string();

        if t.is_empty() {
            idx += 1;
            continue;
        }

        if t == "}" {
            return Ok((
                Function { name, params, return_type, body, span },
                idx + 1,
            ));
        }

        let (stmt, next_idx) = parse_stmt_at(lines, idx)?;
        body.push(stmt);
        idx = next_idx;
    }

    Err(HolyError::Parse(format!(
        "Unterminated function starting at line {}: `{}`",
        start_i + 1,
        lines[start_i]
    )))

}


fn parse_const_stmt(line: &str, span: Span) -> Result<Constant, HolyError> {
    // Constant declaration: 
    // CONST CONST_NAME TYPE_NAME = EXPRESSION
    //

    let line = helpers::strip_inline_comment(line);

    let rest = line["const ".len()..].trim(); // slicing is safe in rust, and would panic at
                                              // runtime if violated.

    if let Some(eq_pos) = rest.find('=') {
        let left = rest[..eq_pos].trim();
        let right = rest[eq_pos + 1..].trim() ;

        let left_parts: Vec<&str> = left.split_whitespace().collect();
        if left_parts.len() != 2 {
            return Err(HolyError::Parse(format!("Invalid constant declaration: `{}` (line {} column {})", line, span.line, span.column)));
        }

        let name = left_parts[0].to_string();
        let var_type = parse_type(left_parts[1], &span)?;
       
        // Ensure the constant name doesnt have special characters, except _, and doesnt start with a
        // number.
        helpers::validate_identifier_name(&name)
            .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        let value = parse_expr::parse_expr(right, span)?;

        return Ok(Constant { name, type_name: var_type, value: value, span });
    } else {
        return Err(HolyError::Parse(format!("Invalid constant declaration, it's missing the `=' sign: `{}` (line {} column {})", line, span.line, span.column)));
    }
}



fn parse_block(lines: &Vec<&str>, mut idx: usize) -> Result<(Vec<Stmt>, usize, BlockEnd), HolyError> {
    let mut body = Vec::new();

    while idx < lines.len() {
        let raw = lines[idx];
        let t = helpers::strip_inline_comment(raw).trim().to_string();

        if t.is_empty() {
            idx += 1;
            continue;
        }

        // Lines starting with `}` close the current block level.
        // They may have a trailing `else {` or `elif <cond> {`.
        if t.starts_with('}') {
            let after_close = t[1..].trim();

            // Reject anything that isn't a known continuation
            if !after_close.is_empty()
                && after_close != "else {"
                && !(after_close.starts_with("elif ") && after_close.ends_with('{'))
            {
                return Err(HolyError::Parse(format!(
                    "Unexpected expression `{}` after '}}' (line {})",
                    after_close,
                    idx + 1,
                )));
            }

            return if after_close.is_empty() {
                Ok((body, idx + 1, BlockEnd::Done)) // past the lone `}`
            } else {
                Ok((body, idx, BlockEnd::Continuation))     // AT the `} else {` / `} elif {` line
            };
        }

        // Let block-opening statements through before the brace guard.
        // NOTE to self: any statement that legitimately ends with `{` must be listed here.
        // But not elif/else, those are handled above.
        //
        let is_block_opener = t.starts_with("infinite ")
            || t.starts_with("if ")
            || t.starts_with("for ")
            || t.starts_with("while ");

        if !is_block_opener {
            // Reject stray braces in the middle of a line (standalone `{` is still allowed)
            let (opens, closes) = helpers::count_braces_outside_strings(&t);
            if (opens > 0 || closes > 0) && t != "{" {
                return Err(HolyError::Parse(format!(
                    "Brace must appear on its own line at line {}: {}",
                    idx + 1,
                    raw
                )));
            }
        }

        let (stmt, next_idx) = parse_stmt_at(lines, idx)?;
        body.push(stmt);
        idx = next_idx;
    }

    Err(HolyError::Parse("Unterminated block".to_string()))
}


fn parse_if_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    if !line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "If statement must end with `{{`, instead we got `{}` (line {} column {})",
            line, span.line, span.column
        )));
    }

    let cond_str = line["if ".len()..].trim_end_matches('{').trim();
    if cond_str.is_empty() {
        return Err(HolyError::Parse(format!(
            "Missing if condition at line {}",
            span.line
        )));
    }

    let condition = parse_expr::parse_expr(cond_str, span)?;
    let (if_branch, mut next_i, mut end) = parse_block(lines, start_i + 1)?;

    let mut elif_branches: Vec<(Expr, Vec<Stmt>)> = Vec::new();
    let mut else_branch = None;

    // Consume any number of elif chains, then an optional else.
    // Accepts both:
    //   `} elif cond {`  (same line as closing brace)
    // and:
    //   `} else {`
    loop {
        if matches!(end, BlockEnd::Done) {
            break;
        }

        let cur_raw = lines[next_i];
        let cur_line = helpers::strip_inline_comment(cur_raw).trim().to_string();

        // This is an else branch
        if cur_line == "} else {" {
            // else is always last, so its fine to ignore the continutation enum here.
            let (body, after, _) = parse_block(lines, next_i + 1)?;
            else_branch = Some(body);
            next_i = after;
            break; // else is always last arm in an if statement
        

        // This is an elif (else if) branch
        } else if cur_line.starts_with("} elif ") && cur_line.ends_with(" {") {
            let elif_cond_str = cur_line.trim_start_matches("} elif ").trim_end_matches(" {");
            if elif_cond_str.is_empty() {
                return Err(HolyError::Parse(format!(
                    "Missing elif condition at line {}",
                    next_i + 1
                )));
            }
            let elif_span = Span { line: next_i + 1, column: 0 };
            let cond = parse_expr::parse_expr(elif_cond_str, elif_span)?;
            let (body, after, new_end) = parse_block(lines, next_i + 1)?;
            elif_branches.push((cond, body));
            next_i = after;
            end = new_end;
        } else {
            panic!("(Compiler bug) We encountered a line that does not start with elif nor else, this shouldve been caught by `parse_block` but it didnt. cur_line: {:?}", cur_line);
        }
    }

    Ok((
        Stmt::If(IfStmt {
            condition,
            if_branch,
            elif_branches,
            else_branch,
            span,
        }),
        next_i,
    ))
}

fn parse_for_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    if !line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "For loop statement must end with `{{`, instead we got `{}` (line {} column {})",
            line, span.line, span.column
        )));
    }

    let for_str = line["for ".len()..].trim_end_matches('{').trim();


    if for_str.is_empty() {
        return Err(HolyError::Parse(format!(
            "For loop statement construction cannot be empty! (line {} column {})",
            span.line, span.column
        )));
    }

    if let Some(index) = for_str.find(" in ") {
        let holder_name = for_str[..index].to_string();
        let expr_str = &for_str[index + " in ".len() .. ];
        
        helpers::validate_identifier_name(&holder_name)
            .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        let expr: Expr;
        
        // A hack, to only allow "RangeCall" expression to be used within for loop statements.
        // I would love to shove this in parse_expr, but, if I do, programmer would be able to assign
        // `rangecall` to any variable. 
        // I could allow that and catch it in semantic phase, but, rangecall can only be used within
        // for loops, so it's part of the syntax structure, not just semantics.
        //
        //
        if expr_str.starts_with("range(") && expr_str.ends_with(")") {
            let range_str = expr_str["range(".len()..].strip_suffix(")").unwrap();

            let split_args = helpers::split_char_top_level(',', range_str)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            if split_args.len() != 2 {
                return Err(HolyError::Parse(format!(
                    "For loop `range` statement takes exactly 2 arguments, instead we got `{}` arguments. (line {} column {})",
                    split_args.len(), span.line, span.column
                )));

            }

            let start_expr = parse_expr::parse_expr(split_args[0], span)?;
            let end_expr = parse_expr::parse_expr(split_args[1], span)?;

            expr = Expr::RangeCall{ start: Box::new(start_expr), end: Box::new(end_expr), span: span };
        } else {
            
            expr = parse_expr::parse_expr(expr_str, span)?;
        }

        let (branch, next_i, _) = parse_block(lines, start_i + 1)?;

        return Ok((
            Stmt::For(ForStmt {
                holder_name,
                value: expr,
                branch,
                span,
            }),
            next_i,
        ))
    } else {
        return Err(HolyError::Parse(format!(
            "For loop statement is not constructed properly. You are missing `in` keyword. (line {} column {})",
            span.line, span.column
        )));
    }
}

fn parse_infinite_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    
    if line != "infinite {" {
        return Err(HolyError::Parse(format!(
            "Invalid infinite loop statement construction `{}` (line {} column {})",
            line, span.line, span.column
        )));
    }

    let (branch, next_i, _) = parse_block(lines, start_i + 1)?;

    Ok((
        Stmt::Infinite(InfiniteStmt {
            branch,
            span,
        }),
        next_i,
    ))
}



fn parse_while_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    if !line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "While loop statement must end with `{{`, instead we got `{}` (line {} column {})",
            line, span.line, span.column
        )));
    }

    let cond_str = line["while ".len()..].trim_end_matches('{').trim();
    if cond_str.is_empty() {
        return Err(HolyError::Parse(format!(
            "Missing while loop condition at line {}",
            span.line
        )));
    }

    let condition = parse_expr::parse_expr(cond_str, span)?;
    let (branch, next_i, _) = parse_block(lines, start_i + 1)?;

    Ok((
        Stmt::While(WhileStmt {
            condition,
            branch,
            span,
        }),
        next_i,
    ))
}



fn parse_stmt_at(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw).trim().to_string();

    if line.starts_with("infinite ") {
        return parse_infinite_stmt(lines, start_i);

    } else if line.starts_with("if ") {
        return parse_if_stmt(lines, start_i);

    } else if line.starts_with("while ") {
        return parse_while_stmt(lines, start_i);

    } else if line.starts_with("for ") {
        return parse_for_stmt(lines, start_i);
    }

    if line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "Invalid statement syntax `{}` at line {}",
            line, start_i + 1
        )));

    }
    
    let stmt = parse_stmt_line(&line, start_i + 1)?;
    Ok((stmt, start_i + 1))
}

/// Parse a single statement from a comments-removed trimmed line. `line_no` used for error messages.
fn parse_stmt_line(line: &str, line_no: usize) -> Result<Stmt, HolyError> {
    let span = Span { line: line_no, column: 0 };

    // Return statement
    if line == "return" {
        return Err(HolyError::Parse(format!(
            "Return requires (at least) one expression (line {} column {})",
            span.line, span.column
        )));
    }

    if line.starts_with("return ") {
        let expr_str = line["return ".len()..].trim();
        if expr_str.is_empty() {
            panic!("(Compiler bug) We expected to `line` to be a trimmed, comments-removed line. Instead we got {:?}", expr_str);
        }
            
        // Check if return is like: return a, b, ...
        // then split, parse each element, and return the vec.
        // Otherwise create new vec of single parsed element.
        let top_parts = helpers::split_char_top_level(',', expr_str)
            .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        if top_parts.len() > 1 {
            let mut elems = vec![];
            for p in top_parts {
                elems.push(parse_expr::parse_expr(p.trim(), span)?);
            }
            return Ok(Stmt::Return(elems));
        } else {
            let expr = parse_expr::parse_expr(expr_str, span)?;
            return Ok(Stmt::Return(vec![expr]));
        }
    }


    if line == "break" {
        return Ok(Stmt::Break(BreakStmt{ span: span }));
    }

    if line == "continue" {
        return Ok(Stmt::Continue(ContinueStmt{ span: span }));
    }


    // Variable locking, e.g.
    // lock var_name
    // lock var_name, var_name, ...
    //
    if line.starts_with("lock ") {
        let rest = line["lock ".len()..].trim();
        if rest.is_empty() {
            panic!("(Compiler bug) We expected to `line` to be a trimmed, comments-removed line. Instead we got {:?}", rest);
        }

        let split_parts = helpers::split_char_top_level(',', rest)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        let mut expr_vec = vec![];

        for e in split_parts {
            let expr = parse_expr::parse_expr(e, span)?;
            expr_vec.push(expr);
        }

        return Ok(Stmt::Lock(expr_vec));
    }


    // Variable unlocking, e.g.
    // lock var_name
    // lock var_name, var_name, ...
    //
    if line.starts_with("unlock ") {
        let rest = line["unlock ".len()..].trim();
        if rest.is_empty() {
            panic!("(Compiler bug) We expected to `line` to be a trimmed, comments-removed line. Instead we got {:?}", rest);
        }

        let split_parts = helpers::split_char_top_level(',', rest)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        let mut expr_vec = vec![];

        for e in split_parts { 
            let expr = parse_expr::parse_expr(e, span)?;
            expr_vec.push(expr);
        }

        return Ok(Stmt::Unlock(expr_vec));
    }




    // Variable declaration: own VAR_NAME ...
    if line.starts_with("own ") {
        // possibilities:
        // own VAR_NAME TYPE_NAME = EXPRESSION
        // own VAR_NAME TYPE_NAME (all types have default values.)
        // 
        // special-case multi-declaration: own x TYPE_1, y TYPE_2 = call() 
        // (just example, declared can be as many as you want, but RHS can only be a single expression) 
        //
        let rest = line["own ".len()..].trim();
        // check for assignment '='
        if let Some(eq_pos) = rest.find('=') {
            let left = rest[..eq_pos].trim();
            let right = rest[eq_pos + 1..].trim() ;

            // Multiple variable declarations
            if left.contains(',') {
                let mut var_names: Vec<String> = vec![];
                let mut var_types: Vec<Type> = vec![];
                
                for part in left.split(',') {
                    let name_type_arr: Vec<&str> = part.trim().split_whitespace().collect();

                    if name_type_arr.len() != 2 {
                        return Err(HolyError::Parse(format!("Invalid multi-variable declaration `{}` at line {}", line, line_no)));
                    }

                    let name = name_type_arr[0].to_string();
                    let typ = parse_type(name_type_arr[1], &span)?;

                    helpers::validate_identifier_name(&name)
                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                    var_names.push(name.to_string());
                    var_types.push(typ);
                }

                let value = parse_expr::parse_expr(right, span)?;

                let mut vars = vec![];
                for (n, t) in var_names.iter().zip(var_types.iter()) {
                    vars.push(MultiVariableDeclaration { name: n.clone(), type_name: t.clone(), span });
                }
                return Ok(Stmt::VarDeclMulti(vars, value));
            }

            // Otherwise a single variable declaration.
            //
            let left_parts: Vec<&str> = left.split_whitespace().collect();
            if left_parts.len() != 2 {
                return Err(HolyError::Parse(format!("Invalid variable declaration `{}` at line {}", line, line_no)));
            }

            let name = left_parts[0].to_string();
            let var_type = parse_type(left_parts[1], &span)?;
           
            // ensure variable name doesnt have special characters, except _, and doesnt start with a
            // number.
            helpers::validate_identifier_name(&name)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            let value = parse_expr::parse_expr(right, span)?;

            return Ok(Stmt::VarDecl(VariableDeclaration { name, type_name: var_type, value: value, span: span }));



        } else {
            // no '=', expect "own name type"
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(HolyError::Parse(format!("Invalid variable declaration `{}` at line {} column {}", line, span.line, span.column)));
            }
            let name = parts[0].to_string();
            helpers::validate_identifier_name(&name)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            let ty = parse_type(parts[1], &span)?;
            let default_value = ty.get_default_value(span);

            return Ok(Stmt::VarDecl(VariableDeclaration { name, type_name: ty, value: default_value, span: span }))
        }
    }


    // Constant declaration within function scope: 
    if line.starts_with("const ") {
        let cons = parse_const_stmt(line, span)?;
        return Ok(Stmt::Const(cons))
    }

    // Enforces variable assignment to be clean
    // i.e.
    // own x = 1
    // 
    // instead of
    // own x=1
    // etc.
    // TODO: Could use a better help message... but i can't figure a clean non complicated way to
    // do it. :(
    //
    // assignment and multi-assignment e.g.:
    // `own x TYPE = EXPRESSION`
    // 'own x TYPE, y TYPE = EXPRESSION`
    //
    if let Some(eq_pos) = line.find(" = ") {
        let name = line[..eq_pos].trim();
        let right = line[eq_pos + 3..].trim();

        if name.contains(',') {
            let mut var_names = vec![];
            for n in name.split(',') {
                let n = n.trim();
                helpers::validate_identifier_name(n)
                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                var_names.push(n.to_string());
            }

            let value = parse_expr::parse_expr(right, span)?;
            return Ok(Stmt::VarAssignMulti(MultiAssignment { names: var_names, value, span }));
        }


        // validate left is a valid identifier
        helpers::validate_identifier_name(name)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        let value = parse_expr::parse_expr(right, span)?;
        return Ok(Stmt::VarAssign(VariableAssignment {
            name: name.to_string(),
            value,
            span,
        }));
    }


    // Expression statement (function call, assignment not supported here yet)
    let expr = parse_expr::parse_expr(line, span)?;
    Ok(Stmt::Expr(expr))
}




/// This is just a temporary type to parse array types
///
enum InternalArraySuffix {
    Dynamic,
    Fixed(FixedArraySize),
}

fn parse_type(s: &str, span: &Span) -> Result<Type, HolyError> {
    let token = s.trim();

    if token.is_empty() {
        panic!("(Compiler bug) parse_type got called with an empty type string. Caller must always check type string is not empty");
    }

    // Split into base name and bracket suffixes
    // e.g. []int32 becomes "int32" base, with "[]" suffix
    if let Some(last_bracket) = token.rfind(']') {
        let base_str = token[last_bracket + 1..].trim();
        let suffix_str = &token[..last_bracket + 1];

        let base_ty = parse_base_type(base_str, span)?;
        
        // Collect all suffixes left-to-right: e.g. "[1][]" becomes [Fixed(1), Dynamic]
        let suffixes = parse_array_suffixes(suffix_str, span)?;

        // Apply them in reverse so the rightmost suffix wraps the base first (innermost),
        // and the leftmost suffix becomes the outermost type.
        //
        // int32[1][]  suffixes=[Fixed(1), Dynamic]
        //   reverse: 
        //        Dynamic becomes Array(Int32)
        //        Fixed(1) becomes FixedArray(Array(Int32), 1)
        let mut ty = base_ty;
        for suffix in suffixes.iter().rev() {
            ty = match suffix {
                InternalArraySuffix::Dynamic       => Type::Array(Box::new(ty)),
                InternalArraySuffix::Fixed(size)   => Type::FixedArray(Box::new(ty), size.clone()),
            };
        }
        return Ok(ty);
    }

    parse_base_type(token, span)
}

/// Parses a suffix string like "[][1][]" into an ordered Vec of InternalArraySuffix.
fn parse_array_suffixes(s: &str, span: &Span) -> Result<Vec<InternalArraySuffix>, HolyError> {
    let mut suffixes = Vec::new();
    let mut rest = s;

    while !rest.is_empty() {
        if rest.starts_with("[]") {
            suffixes.push(InternalArraySuffix::Dynamic);
            rest = &rest[2..];
        } else if rest.starts_with('[') {
            let close = rest.find(']').
                expect("(Compiler bug) Unclosed ], shouldnt be possible. but yeah its possible lol");

            let size_str = rest[1..close].trim();
            // Empty brackets are handled by the "[]" branch above; reaching here
            // with an empty size_str means something like "int32[ ]" which is invalid.
            //
            // NOTE: Also since caller must always check type against white spaces
            // its not our responsiblity to give user error here
            // if we reach here, its a guard panic.
            //
            if size_str.is_empty() {
                panic!("(Compiler bug) Always check the type whitespaces, types must not have whitespaces");
            }
            suffixes.push(InternalArraySuffix::Fixed(parse_fixed_array_size(size_str, span)?));
            rest = &rest[close + 1..];
        } else {
            panic!("(Compiler bug) Expected [, instead got {:?} this is impossible because parse type shouldve caught it.", rest);
        }
    }

    Ok(suffixes)
}

/// Pure base-type lookup with no bracket handling.
fn parse_base_type(token: &str, span: &Span) -> Result<Type, HolyError> {
    match token {
        "int8"    => Ok(Type::Int8),
        "int16"   => Ok(Type::Int16),
        "int32"   => Ok(Type::Int32),
        "int64"   => Ok(Type::Int64),
        "int128"  => Ok(Type::Int128),
        "byte"    => Ok(Type::Byte),
        "uint16"  => Ok(Type::Uint16),
        "uint32"  => Ok(Type::Uint32),
        "uint64"  => Ok(Type::Uint64),
        "uint128" => Ok(Type::Uint128),
        "usize"   => Ok(Type::Usize),
        "float64" => Ok(Type::Float64),
        "bool"    => Ok(Type::Bool),
        "string"  => Ok(Type::String),
        other     => Err(HolyError::Parse(format!(
            "Unknown type `{}` (line {} column {})",
            other, span.line, span.column
        ))),
    }
}


fn parse_fixed_array_size(s: &str, span: &Span) -> Result<FixedArraySize, HolyError> {
    if let Ok(n) = s.parse::<usize>() {
        return Ok(FixedArraySize::Literal(n));
    }

    helpers::validate_identifier_name(&s)
        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

    Ok(FixedArraySize::Const(s.to_string()))
}

