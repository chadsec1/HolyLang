use super::*;


/// Minimal expression parser:
/// - handles binary operations (left-associative),
/// - function calls like add(x, y),
/// - integer literals,
/// - variable names
pub fn parse_expr(s: &str, span: Span) -> Result<Expr, HolyError> {
    let s = s.trim();

    if s.is_empty() {
        return Err(HolyError::Parse(format!(
                    "Empty expression at line {}, column {}",
                    span.line, span.column
            )));
    }

    
    if s.starts_with('[') {
        return Err(HolyError::Parse(format!(
                "Array literal requires an explicit type on right-hand side, e.g. `own x = int32[1,2,3]` (line {} column {})",
                span.line, span.column
            )));
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

    

    // String Literal ?
    if s.starts_with('"') {
        if !s.ends_with('"') {
            return Err(HolyError::Parse(format!(
                "String double quotes were never closed (line {} column {})",
                span.line, span.column
            )));
        }

        let str_unescaped = helpers::strip_outer_quotes_and_unescape(s);

        let value = Expr::StringLiteral { value: str_unescaped.to_string(), span};

        return Ok(value);
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
                    if depth > 0 {
                        depth -= 1;
                        if depth == 0 && i == s.len() - 1 {
                            matched_at_end = true;
                        }
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




    // special-case: typed array literal on RHS: e.g. "int32[1, 2, 3]" 
    // detect pattern: "<type_without_brackets>[ ... ]"
   
    if let Some(first_bracket) = helpers::find_constructor_bracket(&s) && s.ends_with(']') {
        let constructor_type_str = s[..first_bracket].trim();
        let elems_str = &s[first_bracket + 1..s.len() - 1];

        if !constructor_type_str.is_empty() {
            match parse_type(constructor_type_str, &span) {
                Ok(inner_ty) => {
                    // wrap into array type for the variable
                    let rhs_var_type = Type::Array(Box::new(inner_ty.clone()));

                    let mut elems: Vec<Expr> = Vec::new();
                    if !elems_str.trim().is_empty() {
                        let split_parts = helpers::split_comma_top_level(elems_str)
                                            .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                        for part in split_parts {
                            let part = part.trim();
                            if helpers::find_constructor_bracket(part).is_some() {
                                let nested = parse_typed_array_literal(part, span )?;
                                elems.push(nested);

                            } else {
                                let expr = parse_expr(part.trim(), span)?;
                                // I could override expression's type here because we already
                                // know array's type, but I leave it up to semantic analysis 
                                // to determine types and error according.
                                elems.push(expr);
                            }
                        }
                    }


                    // This is so it allows programmer to optionally explicitly set type of
                    // array on left hand side. 
                    // we still require rhs var type though, the optional left hand side
                    // type of array is useful when you calling a function and want to lock
                    // your code to expect a specific type and error otherwise.
                    // Example:
                    // own x int32[] = int32[1, 2, 3] # This is valid
                    // own x = int32[1, 2, 3] # This is also valid
                    // own x uint32[] = int32[1, 2, 3] # This is invalid.
                    //
                    let mut value = Expr::ArrayLiteral { elements: elems.clone(), span, array_ty: inner_ty.clone() };
                    if helpers::is_array_type(&rhs_var_type) {
                        if let Type::Array(inner_array_ty) = rhs_var_type.clone() {
                            value = Expr::ArrayLiteral { elements: elems, span, array_ty: *inner_array_ty };
                        }
                    }

                    return Ok(value);
            }
            // Not an array literal, but an array access
            Err(_) => {
                let array = parse_expr(constructor_type_str, span)?;
                let indx_parts: Vec<&str> = elems_str.split(':').collect();

                // Treat as access to a single element. 
                if indx_parts.len() == 1 {
                    let index = parse_expr(indx_parts[0], span)?;
                    
                    let value = Expr::ArraySingleAccess { array: Box::new(array), index: Box::new(index), span };

                    return Ok(value);
                
                // We do >= here because indx_parts could themselves contain
                // expressions of array access. 
                // We only care about first, and last indx_parts.
                } else if indx_parts.len() >= 2 {
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

                    // i.e. x[:10]
                    if start.is_empty() {
                        end_expr = Some(Box::new(parse_expr(end, span)?));
                    }

                    // i.e. x[1:]
                    if end.is_empty() {
                        start_expr = Some(Box::new(parse_expr(start, span)?));
                    }

                    // i.e. x[1:10]
                    if !start.is_empty() && !end.is_empty() {
                        start_expr = Some(Box::new(parse_expr(start, span)?));
                        end_expr = Some(Box::new(parse_expr(end, span)?));
                    }

                    
                    let value = Expr::ArrayMultipleAccess { array: Box::new(array), start: start_expr, end: end_expr, span };

                    return Ok(value);
                }
            }
        }
    }
       
    // handle empty typed-array literal like:
    // own x = int32[]
    } else if s.ends_with("[]") {
        let type_str = s[..s.len() - 2].trim();
        if !type_str.is_empty() {
            // parse the inner element type (may be nested like "int32[]", parse_type handles nesting)
            let inner_ty = parse_type(type_str, &span)?;

            let rhs_var_type = Type::Array(Box::new(inner_ty.clone()));

            // create empty array literal (no elements)
            let mut value = Expr::ArrayLiteral {
                elements: Vec::new(),
                array_ty: inner_ty.clone(),
                span,
            };

            if helpers::is_array_type(&rhs_var_type) {
                if let Type::Array(inner_array_ty) = rhs_var_type.clone() {
                    value = Expr::ArrayLiteral { elements: Vec::new(), span, array_ty: *inner_array_ty };
                }
            }
            return Ok(value);
        }
    }


    
    
    // Binary plus handling: split on the first operator
    if let Some((pos, op)) = helpers::find_top_level_op_any(s, &['+', '-', '*', '/']) {
        let left = &s[..pos].trim();
        let right = &s[pos + 1..].trim();
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

        let op_enum = match &op {
            '+' => BinOpKind::Add,
            '-' => BinOpKind::Subtract,
            '*' => BinOpKind::Multiply,
            '/' => BinOpKind::Divide,
            o => {
                return Err(HolyError::Parse(format!(
                    "Unknown operand {} (line {} column {})",
                    o,
                    span.line, span.column
                )));
            },
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

    // Function call: name(arg1, arg2)
    if let Some(open) = s.find('(') {
        if s.ends_with(')') {
            let name = s[..open].trim().to_string();
            let args_str = &s[open + 1..s.len() - 1];

            
            // Argument parsing function
            let mut args = vec![];
            if !args_str.trim().is_empty() {
                let split_args = helpers::split_comma_top_level(args_str)
                                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                for a in split_args {
                    args.push(parse_expr(a.trim(), span)?);
                }
            }


            // Check for language-defined functions, otherwise, treat this 
            // expression as a normal programmer-defined function call.
            match name.as_ref() {
                "copy" => {
                    if args.len() != 1 {
                        return Err(HolyError::Parse(format!(
                            "copy() takes exactly 1 argument, {} arguments provided (line {} column {})",
                            args.len(), span.line, span.column
                        )));
                    }

                    return Ok(Expr::CopyCall{ expr: Box::new(args[0].clone()), span: span });
                }

                "format" => {
                    if args.len() != 1 {
                        return Err(HolyError::Parse(format!(
                            "format() takes exactly 1 argument, {} arguments provided (line {} column {})",
                            args.len(), span.line, span.column
                        )));
                    }
                    return Ok(Expr::FormatCall{ expr: Box::new(args[0].clone()), span: span });

                }

                _ => return Ok(Expr::Call { name, args, span })   
            }
        }
    }

    // integer literal (int8) ?
    if let Ok(i) = s.parse::<i8>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int8(i), span: span });
    }

    // integer literal (int16) ?
    if let Ok(i) = s.parse::<i16>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int16(i), span: span });
    }

    // integer literal (int32) ?
    if let Ok(i) = s.parse::<i32>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int32(i), span: span });
    }

    // integer literal (int64) ?
    if let Ok(i) = s.parse::<i64>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int64(i), span: span });
    }

    // integer literal (int128) ?
    if let Ok(i) = s.parse::<i128>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Int128(i), span: span });
    }


    // integer literal (byte, aka uint8) ?
    if let Ok(i) = s.parse::<u8>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Byte(i), span: span });
    }

    if let Ok(i) = s.parse::<u16>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Uint16(i), span: span });
    }

    if let Ok(i) = s.parse::<u32>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Uint32(i), span: span });
    }

    if let Ok(i) = s.parse::<u64>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Uint64(i), span: span });
    }

    if let Ok(i) = s.parse::<u128>() {
        return Ok(Expr::IntLiteral { value: IntLiteralValue::Uint128(i), span: span });

    } else if let Err(e) = s.parse::<u128>() {
        if matches!(e.kind(), IntErrorKind::PosOverflow) {
            // Return error only if we sure expression is not meant as a float
            if !s.contains('.') {
                return Err(HolyError::Parse(format!(
                    "Literal is an integer but is too big to fit even as an uint128, consider using a float literal (line {} column {})",
                    span.line, span.column
                )));
                
            }
        }
    }

    

    // float literal?
    if let Ok(f64_val) = s.parse::<f64>() {
        if f64_val.is_nan() {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` is Nan (line {} column {})",
                s, span.line, span.column
            )));
        }

        if f64_val.is_infinite() {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` is Infinite (line {} column {})",
                s, span.line, span.column
            )));
        }

        if s.chars().any(|c| !c.is_ascii_digit() && c != '.') {
            return Err(HolyError::Parse(format!(
                "Floating point literal `{}` is invalid (line {} column {})",
                s, span.line, span.column
            )));

        }

        let sig_trimmed = s.trim_start_matches('0');
        let sig_count = sig_trimmed.len();

        
        // f32 has about 7 decimal digits of precision (log10(2^24) = 7.22).
        // Use 1 for the dot, that makes 8 a conservative threshold.
        // It's reasonable for us to check inprecision and just use float64 if sig_count is higher
        // than 8.
        //
        if sig_count <= 8 {
            let f32_val = f64_val as f32;

            if (!f32_val.is_infinite()) && (!f32_val.is_nan()) {
                let roundtrip = f32_val as f64;
                let diff = (f64_val - roundtrip).abs();

                // compute next representable f32 (neighbor) by bit-twiddling
                let bits = f32_val.to_bits();
                // increment/decrement to get the neighbor toward +∞
                let next_bits = if f32_val >= 0.0 { bits.wrapping_add(1) } else { bits.wrapping_sub(1) };
                let next_up = f32::from_bits(next_bits);
                let ulp = (next_up as f64 - roundtrip).abs();

                // fallback: if ulp is zero (shouldn't happen for normals/subnormals), use EPSILON heuristic
                let ok = if ulp > 0.0 {
                    diff <= (ulp / 2.0)
                } else {
                    diff <= (f32::EPSILON as f64) * roundtrip.abs().max(1.0)
                };


                if ok {
                    return Ok(Expr::FloatLiteral { value: FloatLiteralValue::Float32(f32_val), span: span });
                }
            }
        }

        return Ok(Expr::FloatLiteral { value: FloatLiteralValue::Float64(f64_val), span: span });


    } else {
        // Check to see if parsing as float failed due to it having more than one dot
        let cleaned_s = s.replace(".", "");
        if let Ok(f) = cleaned_s.parse::<f64>() {
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

    // otherwise a variable name

    helpers::validate_identifier_name(s, span.line)?;
    Ok(Expr::Var { name: s.to_string(), span: span})
}


