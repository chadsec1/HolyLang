use super::*;

use crate::consts;

/// Returns the index of the first `[` that does NOT immediately form a `[]` pair.
/// Useful to distinguish type-suffix `[]` from the constructor `...[ ... ]`.
pub fn find_constructor_bracket(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            // if this '[' is immediately followed by ']' => it's a suffix pair "[]", skip both
            if i + 1 < bytes.len() && bytes[i + 1] == b']' {
                i += 2;
                continue;
            } else {
                return Some(i);
            }
        } else {
            i += 1;
        }
    }
    None
}


/// Find the first top-level operator from `ops` (not inside parentheses).
/// Returns Some((index, operator_char)) if found.
pub fn find_top_level_op_any(s: &str, ops: &[char]) -> Option<(usize, char)> {
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => if depth > 0 { depth -= 1 },
            _ => {}
        }
        if depth == 0 && ops.contains(&c) {
            return Some((i, c));
        }
    }
    None
}




/// Split comma-separated args at top-level only.
/// - respects nested (), [], {}
/// - respects "..." and '...' with backslash escapes
/// - returns slices into `s` (no allocation for substrings beyond the Vec)
pub fn split_comma_top_level(s: &str) -> Result<Vec<&str>, HolyError> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut stack: Vec<char> = Vec::new();
    let mut in_string: Option<char> = None; // Some('"') or Some('\'')
    let mut escape = false;
    let mut just_closed_string = false;

    for (i, c) in s.char_indices() {
        if let Some(q) = in_string {
            // inside quoted string
            if escape {
                escape = false;
                continue;
            }
            if c == '\\' {
                escape = true;
                continue;
            }
            if c == q {
                // closing quote
                in_string = None;
                just_closed_string = true; // remember we just closed a string
            }
            continue;
        } else {
            // if we just closed a string, reject any immediate new quote
            if just_closed_string {
                if c == '"' || c == '\'' {
                    return Err(HolyError::Parse(format!(
                        "Unexpected adjacent string literal at character index {}",
                        i
                    )));
                }
                // clear the flag on the first non-whitespace (so "hi" ) or comma or bracket clears it)
                if !c.is_whitespace() {
                    just_closed_string = false;
                }
            }

            match c {
                '"' | '\'' => {
                    in_string = Some(c);
                }
                '(' | '[' | '{' => {
                    stack.push(c);
                    just_closed_string = false;
                }
                ')' => {
                    if matches!(stack.last(), Some('(')) { stack.pop(); }
                    just_closed_string = false;
                }
                ']' => {
                    if matches!(stack.last(), Some('[')) { stack.pop(); }
                    just_closed_string = false;
                }
                '}' => {
                    if matches!(stack.last(), Some('{')) { stack.pop(); }
                    just_closed_string = false;
                }
                ',' => {
                    if stack.is_empty() && in_string.is_none() {
                        parts.push(s[start..i].trim());
                        start = i + c.len_utf8();
                        just_closed_string = false;
                    }
                }
                _ => {}
            }
        }
    }

    if in_string.is_some() {
        return Err(HolyError::Parse("Unclosed string literal".into()));
    }

    if escape {
        return Err(HolyError::Parse("Invalid trailing escape in string".into()));
    }

    // push last part
    parts.push(s[start..].trim());
    Ok(parts)
}


pub fn strip_outer_quotes_and_unescape(s: &str) -> Result<String, HolyError> {
    // This removes surrounding double quotes if both ends are quotes
    let inner = if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len()-1]
    } else {
        s
    };

    // This unescape of common sequences
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();

    let mut double_quotes_encountered = false;

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some('"') => {
                    double_quotes_encountered = true;
                    out.push('"')
                },
                Some('\'') => out.push('\''),
                Some('0') => out.push('\0'),
                // unknown escape: just emit the escaped char as-is
                Some(other) => out.push(other),
                None => out.push('\\'), // trailing backslash
            }
            

        } else if c == '"' {
            if double_quotes_encountered == false {
                return Err(HolyError::Parse(format!("Unterminated string: `{}`", out).into()));
            }
        } else {
            out.push(c);
        }
    }

    Ok(out)
}



pub fn is_array_type(t: &Type) -> bool {
    matches!(t, Type::Array(_))
}


/// Checks if a given name is a valid HolyLang identifier.
/// Rules:
/// - Can contain letters, digits, and underscore
/// - Must not start with a digit
/// - Must not contain a reserved language keyword (i.e. `own`, etc)
pub fn validate_identifier_name(name: &str) -> Result<(), HolyError> {
    if name.is_empty() {
        return Err(HolyError::Parse("Empty variable name (You most likely have invalid syntax)".to_string()));
    }

    // Check first character is not a number
    let first = name.chars().next().unwrap();
    if first.is_ascii_digit() {
        return Err(HolyError::Parse(format!("Variable name `{}` cannot start with a number!", name)));
    }

    // Check allowed characters: a-z, A-Z, 0-9, _
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(HolyError::Parse(format!(
            "Variable name `{}` contains invalid characters (only letters, numbers, and `_` allowed)",
            name
        )));
    }

    // Check against keywords and error even if name is not the 
    // same exact match in terms of being upper or lower case.
    //
    let name_lower = name.to_string();
    let name_lower = name_lower.to_lowercase(); 
    if consts::RESERVED_KEYWORDS.contains(&name_lower.as_ref()) {
        return Err(HolyError::Parse(format!("Variable name `{}` is a reserved keyword", name)));
    }

    Ok(())
}



/// Remove an inline `#` comment from `s`, but only when the `#` is outside
/// single- or double-quoted string literals. Preserves contents when `#` is inside a string.
pub fn strip_inline_comment(s: &str) -> String {
    let mut in_string: Option<char> = None;
    let mut escape = false;

    for (i, c) in s.char_indices() {
        if let Some(q) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if c == '\\' {
                escape = true;
                continue;
            }
            if c == q {
                in_string = None;
            }
            // while inside string, ignore all other chars
            continue;
        } else {
            // not in string
            if c == '"' || c == '\'' {
                in_string = Some(c);
                continue;
            }
            if c == '#' {
                // found comment start outside of any string, so we should strip from here
                return s[..i].trim_end().to_string();
            }
        }
    }

    // no comment found (or only inside strings)
    s.to_string()
}


/// Count '{' and '}' that are outside string literals.
/// Handles both single-quoted and double-quoted strings and backslash escapes.
pub fn count_braces_outside_strings(line: &str) -> (usize, usize) {
    let mut in_string: Option<char> = None;
    let mut escape = false;
    let mut opens = 0usize;
    let mut closes = 0usize;

    for ch in line.chars() {
        if let Some(q) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == q {
                in_string = None;
            }
            // while inside string, ignore other chars
            continue;
        } else {
            // not inside string
            if ch == '"' || ch == '\'' {
                in_string = Some(ch);
                continue;
            }
            match ch {
                '{' => opens += 1,
                '}' => closes += 1,
                _ => {}
            }
        }
    }

    (opens, closes)
}



pub fn parse_format_string(s: &str) -> Result<(String, Vec<String>), HolyError> {
    let mut chars = s.chars().peekable();
    let mut buffer = String::new();
    let mut expressions_str: Vec<String> = vec![];

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                // literal {{
                if let Some('{') = chars.peek() {
                    chars.next();
                    buffer.push('{');
                    buffer.push('{');
                    continue;
                }

                // placeholder start: { ... }
                let mut inner = String::new();
                let mut closed = false;

                while let Some(nc) = chars.next() {
                    if nc == '}' {
                        // literal }} inside the placeholder text
                        if let Some('}') = chars.peek() {
                            chars.next();
                            inner.push('}');
                            inner.push('}');
                            continue;
                        } else {
                            closed = true;
                            break;
                        }
                    } else {
                        inner.push(nc);
                    }
                }

                if !closed {
                    return Err(HolyError::Parse("Unclosed '{' in input".to_string()));
                }

                if inner.is_empty() {
                    return Err(HolyError::Parse(
                        "Empty string format {} placeholder is not allowed".to_string(),
                    ));
                }

                expressions_str.push(inner);

                buffer.push('{');
                buffer.push('}');
            }

            '}' => {
                // literal }}
                if let Some('}') = chars.peek() {
                    chars.next();
                    buffer.push('}');
                    buffer.push('}');
                } else {
                    return Err(HolyError::Parse("Unmatched '}' in input".to_string()));
                }
            }

            _ => buffer.push(c),
        }
    }

    Ok((buffer, expressions_str))
}

