use super::{HolyError, helpers};
use crate::ast::*;

/// Expression parser:
/// - handles binary operations (left-associative),
/// - handles unary operations (like negate, logical not, bitwise not )
/// - function calls like add(x, y),
/// - Internal "function" calls (like CopyCall, FormatCall, etc),
/// - integer literals,
/// - float literals
/// - string literals
/// - array literals
/// - variables
pub fn parse_expr(s: &str, span: Span) -> Result<Expr, HolyError> {
    let s = s.trim();

    if s.is_empty() {
        return Err(HolyError::Parse(format!(
                "Empty expression at line {}, column {}",
                span.line, span.column
        )));
    }

    // String Literal ?
    if s.starts_with('"') {
        // Find the matching closing quote
        //
        // The reason we do this here, instead of just letting up to
        // string_strip_outer_quotes_and_unescape due to fact it errors on invalid strings thinking
        // its a string, but it could also be an expression concerning strings. so we have to still
        // manually basic match here.
        //
        let mut chars = s.char_indices().skip(1);
        let closing = loop {
            match chars.next() {
                Some((_, '\\')) => { chars.next(); }
                Some((i, '"')) => break Some(i),
                None => break None,
                _ => {}
            }
        };

        match closing {
            None => {
                return Err(HolyError::Parse(format!(
                    "String double quotes were never closed (line {} column {})",
                    span.line, span.column
                )));
            }
            Some(i) if i == s.len() - 1 => {
                // Escapes the string content (like \n, etc), and removes the outer double quotes
                //
                let str_unescaped = helpers::string_strip_outer_quotes_and_unescape(s)
                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})",
                        e, span.line, span.column)))?;

                return Ok(Expr::StringLiteral { value: str_unescaped, span });
            }
            // Fall through
            _ => {}
        }

    }


    // Parentheses grouping: if the whole expression is wrapped in top-level parentheses, parse inner
    if s.starts_with('(') && s.ends_with(')') {
        // ensure the closing paren matches the opening at position 0 (top-level wrap)
        let mut depth = 0usize;
        let mut matched_at_end = false;
        for (i, c) in s.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    // NOTE: We dont check depth > 0 here because, we already checkk starts_with
                    // and ends_with, so ) is guaranteed to be > 0
                    //
                    depth -= 1;
                    if depth == 0 && i == s.len() - 1 {
                        matched_at_end = true;
                    }
                }
                _ => {}
            }
            if depth == 0 && i < s.len() - 1 {
                // top-level closed before end means its not a full wrap
                matched_at_end = false;
                break;
            }
        }
        if matched_at_end {
            let inner = &s[1..s.len() - 1];
            return parse_expr(inner, span);
        }
    }


    // integer literal (int8) ?
    if let Ok(i) = s.parse::<i8>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int8(i), span });
    }

    // integer literal (int16) ?
    if let Ok(i) = s.parse::<i16>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int16(i), span });
    }

    // integer literal (int32) ?
    if let Ok(i) = s.parse::<i32>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int32(i), span });
    }

    // integer literal (int64) ?
    if let Ok(i) = s.parse::<i64>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int64(i), span });
    }

    // integer literal (int128) ?
    if let Ok(i) = s.parse::<i128>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int128(i), span });
    }


    // We only check for u128 here, because anything less should've been caught 
    // by the earlier checks
    //
    if let Ok(i) = s.parse::<u128>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Uint128(i), span });
    } 

    // float literal?
    if let Ok(f64_val) = s.parse::<f64>() {
        if f64_val.is_nan() {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` is NaN (line {} column {})",
                s, span.line, span.column
            )));
        }

        if f64_val.is_infinite() {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` is Infinite (line {} column {})",
                s, span.line, span.column
            )));
        }

        return Ok(Expr::Float64Literal { value: f64_val, span: span });


    } else {
        // Check to see if parsing as float failed due to it having more than one dot
        let cleaned_s = s.replace(".", "");
        if let Ok(_) = cleaned_s.parse::<f64>() {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` must have only one `.` (line {} column {})",
                s, span.line, span.column
            )));
         
        }
    }

    // bool literal ? (true / false) 
    if let Ok(b) = s.parse::<bool>() {
        return Ok(Expr::BoolLiteral { value: b, span: span });
    }


    // Binary operations handling: split on the first operator
    if let Some((pos, op)) = helpers::find_top_level_op_any(s) {
        let left = s[..pos].trim();
        let right = s[pos + op.len()..].trim();
        if left.is_empty() {
            return Err(HolyError::Parse(format!(
                "Expected expression before '{}' at line {} column {}",
                op, span.line, span.column
            )));
        }
        if right.is_empty() {
            return Err(HolyError::Parse(format!(
                "Expected expression after '{}' at line {} column {}",
                op, span.line, span.column
            )));
        }

        let op_enum = match op {
            "+"  => BinOpKind::Add,
            "-"  => BinOpKind::Subtract,
            "*"  => BinOpKind::Multiply,
            "/"  => BinOpKind::Divide,
            "==" => BinOpKind::Equal,
            "!=" => BinOpKind::NotEqual,
            ">"  => BinOpKind::Greater,
            ">=" => BinOpKind::GreaterEqual,
            "<"  => BinOpKind::Less,
            "<=" => BinOpKind::LessEqual,
            ">>" => BinOpKind::BitwiseShiftRight,
            "<<" => BinOpKind::BitwiseShiftLeft,
            "&" => BinOpKind::BitwiseAnd,
            "|" => BinOpKind::BitwiseOr,
            "and" => BinOpKind::And,
            "or" => BinOpKind::Or,
            o => panic!("(Compiler bug) Unknown operand {:?} indicating a bug is in `find_top_level_op_any` func.", o)
        };

        let left_expr = parse_expr(left, span)?;
        let right_expr = parse_expr(right, span)?;
        return Ok(Expr::BinOp {
            left: Box::new(left_expr),
            op: op_enum,
            right: Box::new(right_expr),
            span: span,
        });
    }

    // Unary negate support.
    if s.starts_with('-') {
        let rest = s[1..].trim();

        if rest.is_empty() {
            return Err(HolyError::Parse(format!(
                "Expected expression before '-' at line {} column {}",
                span.line, span.column
            )));
        }

        // Parse inner expression
        let inner = parse_expr(rest, span)?;

        // Return the expression wrapped in Unary of operation negate.
        return Ok(Expr::UnaryOp {
            op: UnaryOpKind::Negate, 
            expr: Box::new(inner), 
            span: span
        });
    }

    // Unary logical NOT support
    if s.starts_with('!') {
        let rest = s[1..].trim();

        if rest.is_empty() {
            return Err(HolyError::Parse(format!(
                "Expected expression before '!' at line {} column {}",
                span.line, span.column
            )));
        }

        // Parse inner expression
        let inner = parse_expr(rest, span)?;

        // Return the expression wrapped in Unary of operation NOT.
        return Ok(Expr::UnaryOp {
            op: UnaryOpKind::Not, 
            expr: Box::new(inner), 
            span: span
        });
    }

    // Unary bitwise NOT support.
    if s.starts_with('~') {
        let rest = s[1..].trim();

        if rest.is_empty() {
            return Err(HolyError::Parse(format!(
                "Expected expression before '~' at line {} column {}",
                span.line, span.column
            )));
        }

        // Parse inner expression
        let inner = parse_expr(rest, span)?;

        // Return the expression wrapped in Unary of operation bitwise not.
        return Ok(Expr::UnaryOp {
            op: UnaryOpKind::BitwiseNot, 
            expr: Box::new(inner), 
            span: span
        });
    }




    // special-case: array literal: 
    // e.g. "[1, 2, 3]", "[]", "[1, [2, 3], 4, 5]"
    // detect pattern: "[ ... ]"
    //

    if s.starts_with("[") {
        // Find the bracket that actually closes this opening '[', not just the last ']'
        let matching_close = {
            let mut depth = 0usize;
            let mut found = None;
            for (i, c) in s[1..].char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => {
                        if depth == 0 {
                            found = Some(1 + i);
                            break;
                        }
                        depth -= 1;
                    }
                    _ => {}
                }
            }
            found
        };

        // Only take the array path if the matching ']' is the very last character.
        // If it isn't (e.g. `[1, 2, 3] == [1, 2, 3]`, etc), then we just let it fall through to other expression detections.
        //

        if let Some(close_pos) = matching_close && close_pos == s.len() - 1 {
            let elems_str = &s[1..s.len() - 1];

            let mut elems: Vec<Expr> = Vec::new();
            if !elems_str.trim().is_empty() {
                let split_parts = helpers::split_char_top_level(',', elems_str)
                                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                for part in split_parts {
                    let part = part.trim();
                    let expr = parse_expr(part.trim(), span)?;
                    elems.push(expr);
                }
            }

            return Ok(
                Expr::ArrayLiteral { 
                    elements: elems, 
                    span,
                });
        }
    }


    // Array access
    // e.g. "x[0]", "x[0:1]", etc.
    if let Some(first_bracket) = s.find("[") {
        // Find the bracket that actually closes this opening '[', not just the last ']'
        let matching_close = {
            let mut depth = 0usize;
            let mut found = None;
            for (i, c) in s[first_bracket..].char_indices() {
                match c {
                    '[' => depth += 1,
                    ']' => {
                        depth -= 1;
                        if depth == 0 {
                            found = Some(first_bracket + i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            found
        };

        if let Some(close_pos) = matching_close && close_pos == s.len() - 1 {
            let arr_expr = parse_expr(&s[..first_bracket], span)?;

            // like whats inside the a[...]
            let inner_str = &s[first_bracket + 1 .. s.len() - 1];

            let indx_parts = helpers::split_char_top_level(':', inner_str)
                                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;
            
            // If only one part, treat as access to a single element. 
            if indx_parts.len() == 1 {
                let index = parse_expr(indx_parts[0], span)?;
                
                let value = Expr::ArrayAccess { array: Box::new(arr_expr), index: Box::new(index), span };

                return Ok(value);

            // Otherwise this is a slicing operation
            // We do >= here to print helpful error messages
            //
            // NOTE TODO: Ensure this doesnt mess with nested expressions within array
            // access/slicing. 
            } else {
                if indx_parts.len() != 2 {
                    return Err(HolyError::Parse(format!(
                                "Invalid array slicing syntax `{}` ! (line {} column {})",
                                s, span.line, span.column
                            )));
                }

                let start = indx_parts[0].trim();
                let end = indx_parts[indx_parts.len() - 1].trim();

                let mut start_expr: Option<Box<Expr>> = None;
                let mut end_expr: Option<Box<Expr>> = None;

                if start.is_empty() && end.is_empty() {
                    return Err(HolyError::Parse(format!(
                                "Start and or end index are empty! (line {} column {})",
                                span.line, span.column
                            )));
                }

                // i.e. x[:EXPRESSION]
                if start.is_empty() {
                    end_expr = Some(Box::new(parse_expr(end, span)?));
                }

                // i.e. x[EXPRESSION:]
                if end.is_empty() {
                    start_expr = Some(Box::new(parse_expr(start, span)?));
                }

                // i.e. x[EXPRESSION:EXPRESSION]
                if !start.is_empty() && !end.is_empty() {
                    start_expr = Some(Box::new(parse_expr(start, span)?));
                    end_expr = Some(Box::new(parse_expr(end, span)?));
                }

                
                return Ok(
                    Expr::ArraySlicing { 
                        array: Box::new(arr_expr), 
                        start: start_expr,
                        end: end_expr,
                        span 
                    });
            }
        }
    }


    // Function call: name(arg1, arg2)
    if let Some(open) = s.find('(') {
        if s.ends_with(')') {
            let name = s[..open].trim().to_string();
            let args_str = &s[open + 1..s.len() - 1];

            
            // Argument parsing function
            let mut args = vec![];
            if !args_str.trim().is_empty() {
                let split_args = helpers::split_char_top_level(',', args_str)
                                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                for a in split_args {
                    args.push(parse_expr(a.trim(), span)?);
                }
            }

            
            // Check for language-defined functions, otherwise, treat this 
            // expression as a normal programmer-defined function call.
            //
            // Even though the parser in general is pretty dumb, we have to check it a bit more strictly for internal functions whose argument sizes
            // and argument type are part of the language syntax its self.
            //
            match name.as_ref() {
                "range" => return Err(HolyError::Parse(format!(
                            "range() can only be used in for loop statements! (line {} column {})",
                            span.line, span.column
                        ))),

                "copy" => {
                    if args.len() != 1 {
                        return Err(HolyError::Parse(format!(
                            "copy() takes exactly 1 argument, instead found {} arguments provided (line {} column {})",
                            args.len(), span.line, span.column
                        )));
                    }

                    return Ok(Expr::CopyCall{ expr: Box::new(args[0].clone()), span: span });
                }

                "format" => {
                    if args.len() != 1 {
                        return Err(HolyError::Parse(format!(
                            "format() takes exactly 1 string argument, instead found `{}` arguments provided (line {} column {})",
                            args.len(), span.line, span.column
                        )));
                    }

                    let format_arg_raw = args[0].clone();


                    let raw_str: String = if let Expr::StringLiteral { value: s, .. } = format_arg_raw {
                            s.to_string()
                        } else {
                            return Err(HolyError::Parse(format!(
                                "Expected a string literal, instead found `{}` (line {} column {})",
                                format_arg_raw, span.line, span.column
                            )));
                        };



                    let (template_str, expr_str_vec) = helpers::parse_format_string(&raw_str)
                                                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                    // Parse expressions from string to Expr
                    let mut expr_vec: Vec<Expr> = vec![];
                    for e in expr_str_vec {
                        expr_vec.push(parse_expr(e.trim(), span)?);
                    }

                    if expr_vec.len() == 0 {
                        return Err(HolyError::Parse(format!(
                                "format() must have at least one embedded expression! (line {} column {})",
                                span.line, span.column
                            )));

                    }

                    return Ok(Expr::FormatCall{ template: template_str, expressions: expr_vec, span: span});

                }

                _ => {
                    helpers::validate_identifier_name(&name)
                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;


                    return Ok(Expr::Call { name, args, span })   
                }
            }
        }
    }
    

    // otherwise a variable name

    helpers::validate_identifier_name(s)
        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

    Ok(Expr::Var { name: s.to_string(), span: span})
}


