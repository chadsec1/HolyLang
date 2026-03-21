use std::fmt;
use std::num::IntErrorKind;

use crate::error::HolyError;

#[cfg(test)]
mod tests;

mod fmt_display;
mod helpers;
mod parse_expr;


pub(crate) use helpers::validate_identifier_name;


/// Types for HolyLang
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Byte,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Usize,
    
    Float32,
    Float64,
    Bool,
    String,
    Array(Box<Type>),
    /// Indicates this needs to be inferred during semantic analysis
    Infer,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntLiteralValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Byte(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Usize(usize),
}

impl IntLiteralValue {
    pub fn get_type(self) -> Type {
        match self {
            IntLiteralValue::Int8(_) => Type::Int8,
            IntLiteralValue::Int16(_) => Type::Int16,
            IntLiteralValue::Int32(_) => Type::Int32,
            IntLiteralValue::Int64(_) => Type::Int64,
            IntLiteralValue::Int128(_) => Type::Int128,

            IntLiteralValue::Byte(v) => Type::Byte,

            IntLiteralValue::Uint16(v) => Type::Uint16,
            IntLiteralValue::Uint32(v) => Type::Uint32,
            IntLiteralValue::Uint64(v) => Type::Uint64,
            IntLiteralValue::Uint128(v) => Type::Uint128,
            
            IntLiteralValue::Usize(v) => Type::Usize,

        }
    }

    pub fn as_i128(self) -> i128 {
        match self {
            IntLiteralValue::Int8(v) => v as i128,
            IntLiteralValue::Int16(v) => v as i128,
            IntLiteralValue::Int32(v) => v as i128,
            IntLiteralValue::Int64(v) => v as i128,
            IntLiteralValue::Int128(v) => v,

            other => {
                panic!("(Compiler bug) Safety code to prevent you from casting an unsigned integer as signed i128. {:?}", other);
            }
        }
    }


    // Since we dont store numbers with negative sign, only wrapped in a negate node, we can
    // actually skip type check and happily infer signed numbers as unsigned if need be.
    //
    // And since u128 can represent all signed numbers assuming no -, that's handled by upper
    // negate node, it should be safe to cast as u128 regardless.
    //
    pub fn as_u128_UNSAFE(self) -> u128 {
        match self {
            // Signed types: check for negative before casting
            IntLiteralValue::Int8(v) => {
                if v < 0 { panic!("Cannot cast negative Int8 ({}) to u128", v); }
                v as u128
            }
            IntLiteralValue::Int16(v) => {
                if v < 0 { panic!("Cannot cast negative Int16 ({}) to u128", v); }
                v as u128
            }
            IntLiteralValue::Int32(v) => {
                if v < 0 { panic!("Cannot cast negative Int32 ({}) to u128", v); }
                v as u128
            }
            IntLiteralValue::Int64(v) => {
                if v < 0 { panic!("Cannot cast negative Int64 ({}) to u128", v); }
                v as u128
            }
            IntLiteralValue::Int128(v) => {
                if v < 0 { panic!("Cannot cast negative Int128 ({}) to u128", v); }
                v as u128
            }

            // Unsigned types are always safe
            IntLiteralValue::Usize(v) => v as u128,
            IntLiteralValue::Byte(v) => v as u128,
            IntLiteralValue::Uint16(v) => v as u128,
            IntLiteralValue::Uint32(v) => v as u128,
            IntLiteralValue::Uint64(v) => v as u128,
            IntLiteralValue::Uint128(v) => v,
            
            other => {
                panic!("(Compiler bug) Safety code prevented you from casting an unspported literal as signed u128. {:?}", other);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FloatLiteralValue {
    Float32(f32),
    Float64(f64),
}

impl FloatLiteralValue {
    pub fn get_type(self) -> Type {
        match self {
            FloatLiteralValue::Float32(_) => Type::Float32,
            FloatLiteralValue::Float64(_) => Type::Float64,

        }
    }
}



/// AST nodes
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal value, the type is the IntLiteralValue Enum wrapper
    IntLiteral {
        value: IntLiteralValue,
        span: Span,
    },
    /// Float literal (value) and type marker (the FloatLiteralValue Enum wrapper)
    FloatLiteral {
        value: FloatLiteralValue,
        span: Span,
    },
    BoolLiteral {
        value: bool,
        span: Span,
    },
    ArrayLiteral {
        elements: Vec<Expr>,
        array_ty: Type,
        span: Span,
    },
    StringLiteral {
        value: String,
        span: Span
    },
    Var { 
        name: String,
        span: Span,
    },
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>,
        span: Span,
    },
    BinOp {
        left: Box<Expr>,
        op: BinOpKind,
        right: Box<Expr>,
        span: Span,
    },
    Call {
        name: String,
        args: Vec<Expr>,
        span: Span,
    },
    ArraySingleAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    ArrayMultipleAccess {
        array: Box<Expr>,
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        span: Span,
    },


    // internal language functions / expressions hard-coded into the language.
    CopyCall {
        expr: Box<Expr>,
        span: Span,
    },
    FormatCall {
        template: String,
        expressions: Vec<Expr>,
        span: Span,
    },

}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Negate,
}

#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_name: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    /// Always present; Type::Infer means "infer in semantic phase"
    pub type_name: Type,
    pub value: Option<Expr>,
    pub span: Span,

}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Vec<Type>>,
    pub body: Vec<Stmt>,
    pub span: Span,
}


#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct MultiAssignment {
    pub names: Vec<String>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr,
    pub branch: Vec<Stmt>,
    pub span: Span
}


#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Expr,
    pub if_branch: Vec<Stmt>,
    pub elif_branches: Vec<(Expr, Vec<Stmt>)>,
    pub else_branch: Option<Vec<Stmt>>,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct BreakStmt {
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct ContinueStmt {
    pub span: Span
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(Variable),
    VarDeclMulti(Vec<Variable>, Expr),
    VarAssign(VariableAssignment),
    VarAssignMulti(MultiAssignment),
    Expr(Expr),
    Lock(Vec<Expr>),
    Unlock(Vec<Expr>),
    Return(Vec<Expr>),
    While(WhileStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    If(IfStmt),
    Func(Function), 
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}



/// Program AST
#[derive(Debug)]
pub struct AST {
    pub functions: Vec<Function>,
}

/// Public parse entry
pub fn parse(source: &str) -> Result<AST, HolyError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0usize;
    let mut ast = AST { functions: vec![] };

    while i < lines.len() {
        let raw = lines[i];
        let line = raw.trim();

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with("func ") {
            // Parse function header and body
            let (func, new_i) = parse_function(&lines, i)?;
            ast.functions.push(func);
            i = new_i;
            continue;
        }

        // unknown top-level line
        return Err(HolyError::Parse(format!(
            "Unexpected statement outside function at line {}: `{}`",
            i + 1,
            raw
        )));
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

            let inner = &return_type_str[1..return_type_str.len()-1];
            let mut types = Vec::new();
            if !inner.trim().is_empty() {
                let split_parts = helpers::split_comma_top_level(inner)
                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;


                for part in split_parts {
                    let t = parse_type(part.trim(), &span)?;
                    types.push(t);
                }
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

        if t.is_empty() || t.starts_with('#') {
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



fn parse_block(lines: &Vec<&str>, mut idx: usize) -> Result<(Vec<Stmt>, usize), HolyError> {
    let mut body = Vec::new();
    let mut brace_balance = 1usize;

    while idx < lines.len() {
        let raw = lines[idx];
        let t = helpers::strip_inline_comment(raw).trim().to_string();

        if t.is_empty() || t.starts_with('#') {
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
                    "Unexpected content after '}}' at line {}: {}",
                    idx + 1,
                    raw
                )));
            }

            brace_balance -= 1;
            if brace_balance == 0 {
                return if after_close.is_empty() {
                    Ok((body, idx + 1)) // past the lone `}`
                } else {
                    Ok((body, idx))     // AT the `} else {` / `} elif {` line
                };
            }
            idx += 1;
            continue;
        }

        // Let block-opening statements through before the brace guard.
        // NOTE to self: any statement that legitimately ends with `{` must be listed here.
        let is_block_opener = t.starts_with("if ")
            || t.starts_with("elif ")
            || t.starts_with("else ")
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

            if t == "{" {
                brace_balance += 1;
                idx += 1;
                continue;
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
            "If statement must end with {{ at line {}: {}",
            span.line, raw
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
    let (if_branch, mut next_i) = parse_block(lines, start_i + 1)?;

    let mut elif_branches: Vec<(Expr, Vec<Stmt>)> = Vec::new();
    let mut else_branch = None;

    // Consume any number of elif chains, then an optional else.
    // Accepts both:
    //   `} elif cond {`  (same line as closing brace)
    //   `elif cond {`    (own line, for when you keep old style)
    // and both:
    //   `} else {`
    //   `else {`
    loop {
        if next_i >= lines.len() {
            break;
        }

        let cur_raw = lines[next_i];
        let cur_line = helpers::strip_inline_comment(cur_raw).trim().to_string();

        // This is else branch
        if cur_line == "} else {" {
            let (body, after) = parse_block(lines, next_i + 1)?;
            else_branch = Some(body);
            next_i = after;
            break; // else is always last
        }

        // This is elif (else if) branch
        let elif_tail: Option<&str> = if cur_line.starts_with("} elif ") {
            Some(&cur_line["} elif ".len()..])
        } else {
            None
        };

        if let Some(tail) = elif_tail {
            if !tail.ends_with('{') {
                return Err(HolyError::Parse(format!(
                    "elif must end with {{ at line {}: {}",
                    next_i + 1,
                    cur_raw
                )));
            }
            let elif_cond_str = tail.trim_end_matches('{').trim();
            if elif_cond_str.is_empty() {
                return Err(HolyError::Parse(format!(
                    "Missing elif condition at line {}",
                    next_i + 1
                )));
            }
            let elif_span = Span { line: next_i + 1, column: 0 };
            let cond = parse_expr::parse_expr(elif_cond_str, elif_span)?;
            let (body, after) = parse_block(lines, next_i + 1)?;
            elif_branches.push((cond, body));
            next_i = after;
        } else {
            break; // not an elif/else continuation — done
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



fn parse_while_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    if !line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "While statement must end with {{ at line {}: {}",
            span.line, raw
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
    let (branch, mut next_i) = parse_block(lines, start_i + 1)?;

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
    let span = Span { line: start_i + 1, column: 0 };

    if line.starts_with("if ") {
        return parse_if_stmt(lines, start_i);
    } else if line.starts_with("while ") {
        return parse_while_stmt(lines, start_i);
    }

    let stmt = parse_stmt_line(&line, start_i + 1)?;
    Ok((stmt, start_i + 1))
}

/// Parse a single statement from a trimmed line. `line_no` used for error messages.
fn parse_stmt_line(line: &str, line_no: usize) -> Result<Stmt, HolyError> {
    let span = Span { line: line_no, column: 0 };

    // Return statement
    if line == "return" {
        return Err(HolyError::Parse(format!(
            "Return requires a value (line {} column {})",
            span.line, span.column
        )));
    }

    if line.starts_with("return ") {
        let expr_str = line["return ".len()..].trim();
        if expr_str.is_empty() {
            return Err(HolyError::Parse(format!(
                "Return requires (at least) one expression (line {} column {})",
                span.line, span.column
            )));
        }
            
        // Check if return is like: return a, b, ...
        // then split, parse each element, and return the vec.
        // Otherwise create new vec of single parsed element.
        let top_parts = helpers::split_comma_top_level(expr_str)
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

    

    // Variable locking: lock ...
    if line.starts_with("lock ") {
        // possibilities:
        // lock x
        // lock x, y
        //
        
        let rest = line["lock ".len()..].trim();

        // Not needed, but I like defensive-coding style
        if rest.is_empty() {
            return Err(HolyError::Parse(format!(
                    "Lock requires at least one variable name (line {} column {})",
                    span.line, span.column
                )));
        }

        let mut expr_vec = vec![];

        for e in rest.split(',') {
            let expr = parse_expr::parse_expr(e, span)?;
            expr_vec.push(expr);
        }

        return Ok(Stmt::Lock(expr_vec));
    }


    // Variable unlocking: unlock ...
    if line.starts_with("unlock ") {
        // possibilities:
        // unlock x
        // unlock x, y
        //
        
        let rest = line["unlock ".len()..].trim();
        // Not needed, but I like defensive-coding style
        if rest.is_empty() {
            return Err(HolyError::Parse(format!(
                "Unlock requires at least one variable name (line {} column {})",
                span.line, span.column
            )));

        }

        let mut expr_vec = vec![];

        for e in rest.split(',') {
            let expr = parse_expr::parse_expr(e, span)?;
            expr_vec.push(expr);
        }

        return Ok(Stmt::Unlock(expr_vec));
    }

    // Variable declaration: own ...
    if line.starts_with("own ") {
        // possibilities:
        // own name = expr
        // own name type = expr
        // own name type
        // special-case: own name = int32[1,2,3]  (typed array literal on RHS)
        // special-case: own x, y = call() (just example, declared can be as many as you want.)
        //
        let rest = line["own ".len()..].trim();
        // check for assignment '='
        if let Some(eq_pos) = rest.find('=') {
            let left = rest[..eq_pos].trim();
            let right = rest[eq_pos + 1..].trim() ;


            // Multiple variable declarations
            if left.contains(',') {
                let mut names = vec![];
                for part in left.split(',') {
                    let n = part.trim();
                    // disallow typed multi-declaration for now (keep parser simple)
                    if n.split_whitespace().count() != 1 {
                        return Err(HolyError::Parse(format!("Invalid multi-variable declaration `{}` at line {}", left, line_no)));
                    }

                    helpers::validate_identifier_name(n)
                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                    names.push(n.to_string());
                }

                let value = parse_expr::parse_expr(right, span)?;
                // create multiple variables with Infer type; inference happens in semantic phase
                let mut vars = vec![];
                for n in &names {
                    vars.push(Variable { name: n.clone(), type_name: Type::Infer, value: None, span });
                }
                return Ok(Stmt::VarDeclMulti(vars, value));
            }




            // left can be "name" or "name type"
            let left_parts: Vec<&str> = left.split_whitespace().collect();
            let (name, var_type) = match left_parts.len() {
                1 => (left_parts[0].to_string(), Type::Infer),
                2 => {
                    let tp = parse_type(left_parts[1], &span)?;
                    (left_parts[0].to_string(), tp)
                }
                _ => return Err(HolyError::Parse(format!("Invalid variable declaration `{}` at line {}", line, line_no))),
            };

           
            // ensure name doesnt have special characters, except _, and doesnt start with a
            // number.
            helpers::validate_identifier_name(&name)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            let value = parse_expr::parse_expr(right, span)?;

            return Ok(Stmt::VarDecl(Variable { name, type_name: var_type, value: Some(value), span: span }));



        } else {
            // no '=', expect "own name type" (explicit type, no initializer)
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(HolyError::Parse(format!("Invalid variable declaration `{}` at line {} column {}", line, span.line, span.column)));
            }
            let name = parts[0].to_string();
            helpers::validate_identifier_name(&name)
                .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

            let tp = parse_type(parts[1], &span)?;
            return Ok(Stmt::VarDecl(Variable { name, type_name: tp, value: None, span: span }));
        }
    }

    // multi-assignment outside 'own': "x, y = expr"
    if line.contains(',') && line.contains('=') {
        if let Some(eq_pos) = line.find('=') {
            let left = line[..eq_pos].trim();
            let right = line[eq_pos + 1..].trim();

            if left.contains(',') {
                let mut names = vec![];
                for part in left.split(',') {
                    let n = part.trim();
                    helpers::validate_identifier_name(n)
                        .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                    names.push(n.to_string());
                }
                let value = parse_expr::parse_expr(right, span)?;
                return Ok(Stmt::VarAssignMulti(MultiAssignment { names, value, span }));
            }
        }
    }

    if let Some(eq_pos) = line.find('=') {
        let name = line[..eq_pos].trim();
        let right = line[eq_pos + 1..].trim();

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


fn parse_typed_array_literal(s: &str, span: Span) -> Result<Expr, HolyError> {
    let s = s.trim();
    // find the constructor '[' that starts the element list
    let ctor_pos = helpers::find_constructor_bracket(s).ok_or_else(|| {
        HolyError::Parse(format!("Malformed typed array literal `{}` (line {} column {})", s, span.line, span.column))
    })?;

    if !s.ends_with(']') {
        return Err(HolyError::Parse(format!("Typed array literal missing trailing ']' (line {} column {})", span.line, span.column)));
    }

    let type_str = s[..ctor_pos].trim();
    let elems_str = &s[ctor_pos + 1..s.len() - 1]; // between constructor '[' and final ']'

    // parse the base/inner type (may be nested literal like "int32[]") we let  parse_type handle it
    match parse_type(type_str, &span) {
        Ok(inner_ty) => {
            let mut elems: Vec<Expr> = Vec::new();
            if !elems_str.trim().is_empty() {
                let split_parts = helpers::split_comma_top_level(elems_str)
                                    .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

                for part in split_parts {
                    let part = part.trim();
                    // If the part itself looks like a typed-array-literal (i.e. has a constructor bracket),
                    // parse it recursively; otherwise use parse_expr for general expressions.
                    if helpers::find_constructor_bracket(part).is_some() {
                        let nested = parse_typed_array_literal(part, span)?;
                        elems.push(nested);
                    } else {
                        let expr = parse_expr::parse_expr(part, span)?;
                        elems.push(expr);
                    }
                }
            }

            Ok(Expr::ArrayLiteral { elements: elems, array_ty: inner_ty, span })
                
        }

        // If its not a type constructor, we gonna assume it's an expression (like an array access)
        Err(e) => {     
            let expr = parse_expr::parse_expr(s, span)?;

            Ok(expr)
        }
    }
}



/// Parse type token like "int32" into `Type`
fn parse_type(s: &str, span: &Span) -> Result<Type, HolyError> {
    let mut token = s.trim();

    if token.is_empty() {
        panic!("(Compiler bug) Empty string to parse_type, ensure token is not empty before passing it.");
    }


    let mut depth = 0usize;
    while token.ends_with("[]") {
        depth += 1;
        token = token[..token.len() - 2].trim_end();
    }

    let mut base = match token {
        "int8" => Type::Int8,
        "int16" => Type::Int16,
        "int32" => Type::Int32,
        "int64" => Type::Int64,
        "int128" => Type::Int128,

        "byte" => Type::Byte,
        "uint16" => Type::Uint16,
        "uint32" => Type::Uint32,
        "uint64" => Type::Uint64,
        "uint128" => Type::Uint128,

        "usize" => Type::Usize,
        
        "float32" => Type::Float32,
        "float64" => Type::Float64,
        "bool" => Type::Bool,
        "string" => Type::String,
        other => return Err(HolyError::Parse(format!(
                    "Unknown type `{}` (line {} column {})",
                    other, span.line, span.column
                )))
    };

    for _ in 0..depth {
        base = Type::Array(Box::new(base));
    }

    Ok(base)
}

