use super::GoldError;

use crate::consts;

pub enum QuoteType {
    DoubleQuotes,
    SingleQuotes
}



/// Takes a string and gives back a char, while correctly handling escapes, such as \n, etc.
///
pub fn get_char_from_string(s: &str) -> Result<char, GoldError> {
    let mut chars = s.chars();

    let first_char = chars.next().ok_or_else(|| GoldError::Parse("Expected a UTF-8 character, instead got nothing.".to_string()))?;

    if first_char != '\\' {
        if chars.next().is_some() {
            return Err(GoldError::Parse(format!("Expected a single UTF-8 character, instead got `{}` characters.", s.len() - 1)))
        }

        return Ok(first_char)
    }
    

    let escape_char = chars.next().ok_or_else(|| GoldError::Parse("Expected an escape character after backlash, instead got nothing.".to_string()))?;

    if chars.next().is_some() {
        return Err(GoldError::Parse(format!("Expected a single escape character, instead got `{}` characters.", s.len() - 1)))
    }
    
    match escape_char {
        'n'   => Ok('\n'),
        't'   => Ok('\t'),
        'r'   => Ok('\r'),
        '0'   => Ok('\0'),
        '\\'  => Ok('\\'),
        '\''  => Ok('\''),
        other => Err(GoldError::Parse(format!("Invalid escape character `{other}`")))
    }
    
}



/// Get the closing quote in a string `s`, and returns `s` without quotes. i.e. the strings
/// "content".
/// `quote` is configurable, either a `QuoteType::DoubleQuote` or a `QuoteType::SingleQuote`
///
pub fn get_string_content(s: &str, quote: &QuoteType) -> Result<Option<String>, GoldError> {
    let closing_quote_type = match quote {
        QuoteType::DoubleQuotes => '"',
        QuoteType::SingleQuotes  => '\''
    };

    if !s.starts_with(closing_quote_type) {
        return Err(GoldError::Parse(format!("Expected string opening quotes `{closing_quote_type}`, instead got `{s}`")))
    }

    let mut chars = s.char_indices().skip(1);
    
    let closing = loop {
        match chars.next() {
            Some((_, '\\')) => { chars.next(); }
            Some((i, c)) if c == closing_quote_type => break Some(i),
            None => break None,
            _ => {}
        }
    };

    match closing {
        None =>  Err(GoldError::Parse("String quotes were never closed".to_string())),
        Some(i) if i == s.len() - 1 => Ok(Some(s[1..s.len() - 1].to_string())),

        // Not a string or, a string in binary operation, etc.
        _ => Ok(None)
    }
}



/// Gets matching closing character, and returns its position
///
fn get_matching_close(s: &str, opening_char: char, closing_char: char) -> Option<usize> {
    assert_ne!(opening_char, closing_char, "(Compiler bug) You are misusing `get_matching_close` because `opening_char` and `closing_char` are both the same `{opening_char}`");
    
    assert!( (opening_char == '(' || opening_char == '['),"(Compiler bug) You are misusing `get_matching_close` because `opening_char` is `{opening_char}`");
    assert!( (closing_char == ')' || closing_char == ']'),"(Compiler bug) You are misusing `get_matching_close` because `closing_char` is `{closing_char}`");

    if !s.starts_with(opening_char) {
        return None
    }

    let mut depth = 0usize;
    let mut found = None;
    let mut in_string = false;
    let mut in_escape = false;
    for (i, c) in s[1..].char_indices() {
        if in_escape {
            in_escape = false;
            continue;
        }

        match c {
            '"' => in_string = !in_string,
            '\\' => in_escape = true,
            
            _ if (c == opening_char) && !in_string => depth += 1,
            _ if (c == closing_char) && !in_string => {
                if depth == 0 {
                    found = Some(1 + i);
                    break;
                }
                depth -= 1;
            },

            _ => {}
        }
    }

    found
}

/// Gets parenthesis contents.
/// I.e. ( .. EXPRESSIONS .. ) would give Some(EXPRESSIONS)
///
/// This only gets parenthesis content if the parenthesis extends to the end of the string
/// like, ( .. EXPRESSIONS ..) is ok but ( .. EXPRESSIONS .. ) == EXPRESSION, etc, is not.
///
///
pub fn get_parenthesis_contents(s: &str) -> Option<&str> { 
    let matching_close = get_matching_close(s, '(', ')');

    if let Some(close_pos) = matching_close && close_pos == s.len() - 1 {
        let parenthesis_str = &s[1.. s.len() - 1];

        return Some(parenthesis_str)
    }

    None
}


/// Gets array contents.
/// I.e. [ .. EXPRESSIONS .. ] would give Some(EXPRESSIONS)
///
/// NOTE: This function Doesnt care about splits (i.e. ',') between expressions, 
///       it only cares about valid matching openings and closing.
///
///
///
pub fn get_array_contents(s: &str) -> Option<&str> {
    let matching_close = get_matching_close(s, '[', ']');

    if let Some(close_pos) = matching_close && close_pos == s.len() - 1 {
        let elems_str = &s[1.. s.len() - 1];

        return Some(elems_str)
    }

    None
}


/// Finds binary operators at top level only
/// i.e. "or", "and", "==", "<", ">", etc.
///
pub fn find_top_level_op_any(s: &str) -> Option<(usize, &str)> {
    fn precedence(op: &str) -> u8 {
        match op {
            "or" => 1,
            "and" => 2,
            "==" | "!=" | "&" | "|" | ">" | "<" | ">=" | "<=" | "<<" | ">>" => 3,
            "+" | "-" => 4,
            "*" | "/" => 5,
            _ => panic!("(Compiler bug) If this ever fires, theres a bug in find_top_level_op_any")
        }
    }
    
    const fn is_ident_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut depth = 0usize;
    let mut in_string = false;
    let mut in_escape = false;
    let mut best: Option<(usize, &str)> = None;
    let mut best_prec = u8::MAX;
    let mut i = 0;

    while i < chars.len() {
        let (idx, c) = chars[i];
        
        if in_escape {
            in_escape = false;
        } else {
            match c {
                '"' => in_string = !in_string,
                '\\' => in_escape = true,
                '(' | '[' | '{' if !in_string => {
                    depth += 1;
                },
                ')' | ']' | '}' if (!in_string) && depth > 0 => {
                    depth -= 1;
                },

                _ if (!in_string) && (depth == 0) => {
                    // Peek next char
                    let next = chars.get(i + 1).map(|(_, nc)| *nc);

                    // Determine operator string at this position
                    let op_str: Option<&str> = match c {
                        '=' | '!' | '>' | '<' if next == Some('=') => Some(&s[idx..idx + 2]),
                        '>' if next == Some('>') => Some(&s[idx..idx + 2]),
                        '<' if next == Some('<') => Some(&s[idx..idx + 2]),
                        'a' if s[idx..].starts_with("and") => {
                            let before = if i == 0 { None } else { Some(chars[i - 1].1) };
                            let after = chars.get(i + 3).map(|(_, ch)| *ch);

                            if before.is_none_or(|ch| !is_ident_char(ch))
                                && after.is_none_or(|ch| !is_ident_char(ch))
                            {
                                Some(&s[idx..idx + 3])
                            } else {
                                None
                            }
                        }

                        'o' if s[idx..].starts_with("or") => {
                            let before = if i == 0 { None } else { Some(chars[i - 1].1) };
                            let after = chars.get(i + 2).map(|(_, ch)| *ch);

                            if before.is_none_or(|ch| !is_ident_char(ch))
                                && after.is_none_or(|ch| !is_ident_char(ch))
                            {
                                Some(&s[idx..idx + 2])
                            } else {
                                None
                            }
                        }

                        '+' | '-' | '*' | '/' | '>' | '<' | '|' | '&' => Some(&s[idx..=idx]),
                        _ => None,
                    };

                    if let Some(op) = op_str {
                        // Skip unary (negate, `logical not`, and `bitwise not`)
                        if op == "-" || op == "!" || op == "~" {
                            let prev_non_ws = (0..i).rev()
                                .map(|j| chars[j].1)
                                .find(|ch| !ch.is_whitespace());
                            match prev_non_ws {
                                None => { i += 1; continue; }
                                Some(prev) if "+-*/&|!~=<>(".contains(prev) => { i += 1; continue; }
                                _ => {}
                            }
                        }

                        let prec = precedence(op);
                        if prec <= best_prec {
                            best_prec = prec;
                            best = Some((idx, op));
                        }

                        // Skip both chars for two-char operators so we don't
                        // match the second char again
                        if op.len() == 2 {
                            i += 2;
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    best
}



/// Split "char"-separated args at top-level only (i.e. ignores nested (), [], {})
/// and respects backslash escapes
///
pub fn split_char_top_level(split_char: char, s: &str) -> Result<Vec<&str>, GoldError> {
    assert!(
        !((split_char != ',') && (split_char != ':')), 
        "(Compiler bug) You are most likely misusing split_char_top_level, we expected char to be one of ':', ',', ' ', but instead we got `{split_char}`"
    );

    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut stack: Vec<char> = Vec::new();
    let mut in_string: Option<char> = None; 
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

            continue
        }

        // if we just closed a string, reject any immediate new quote
        if just_closed_string {
            if c == '"' || c == '\'' {
                return Err(GoldError::Parse(format!(
                    "Unexpected adjacent string literal at character index {i}",
                )))
            }
            // clear the flag on the first non-whitespace (so "hi" ) or split_char or bracket clears it)
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
            c if (c == split_char) && stack.is_empty() && in_string.is_none() => {
                parts.push(s[start..i].trim());
                start = i + c.len_utf8();
                just_closed_string = false;
            }
            _ => {}
        }
    }

    if in_string.is_some() {
        return Err(GoldError::Parse("Unclosed string literal".into()));
    }

    // push last part
    parts.push(s[start..].trim());
    Ok(parts)
}



/// Checks if a given name is a valid identifier.
/// Rules:
/// - Can contain letters, digits, and underscore
/// - Must not start with a digit
/// - Must not contain a reserved language keyword (i.e. `own`, etc)
pub fn validate_identifier_name(name: &str) -> Result<(), GoldError> {
    assert!(!name.trim().is_empty(), "(Compiler bug) `validate_identifier_name` got fed an empty string, indicating a bug in the caller's code.");

    // Check first character is not a number
    let first = name.chars().next().unwrap();
    if first.is_ascii_digit() {
        return Err(GoldError::Parse(format!("Binding identifier name `{name}` cannot start with a number!")))
    }

    // Check allowed characters: a-z, A-Z, 0-9, _
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(GoldError::Parse(format!("Binding identifier name `{name}` contains invalid characters (only letters, numbers, and `_` allowed)")))
    }

    // Check against keywords and error even if name is not the 
    // same exact match in terms of being upper or lower case.
    //
    let name_lower = name.to_string();
    let name_lower = name_lower.to_lowercase(); 
    if consts::RESERVED_KEYWORDS.contains(&name_lower.as_ref()) {
        return Err(GoldError::Parse(format!("Binding identifier name `{name}` is a reserved keyword")))
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
            continue
        } 

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
            continue
        } 

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

    (opens, closes)
}



pub fn parse_format_string(s: &str) -> Result<(String, Vec<String>), GoldError> {
    let mut chars = s.chars().peekable();
    let mut buffer = String::new();
    let mut expressions_str: Vec<String> = vec![];

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                // literal {{
                if matches!(chars.peek(), Some('{')) {
                    chars.next();
                    buffer.push('{');
                    buffer.push('{');
                    continue;
                }

                // placeholder start: { ... }
                let mut inner = String::new();
                let mut closed = false;

                for nc in chars.by_ref() {
                    if nc == '}' {
                        closed = true;
                        break
                    }

                    inner.push(nc);
                }

                if !closed {
                    return Err(GoldError::Parse("Unclosed '{' in input".to_string()))
                }

                if inner.is_empty() {
                    return Err(GoldError::Parse("Empty string format {} placeholder is not allowed".to_string()))
                }

                expressions_str.push(inner);

                buffer.push('{');
                buffer.push('}');
            }

            '}' => {
                // literal }}
                if matches!(chars.peek(), Some('}')) {
                    chars.next();
                    buffer.push('}');
                    buffer.push('}');
                } else {
                    return Err(GoldError::Parse("Unmatched '}' in input".to_string()))
                }
            }

            _ => buffer.push(c),
        }
    }

    Ok((buffer, expressions_str))
}

