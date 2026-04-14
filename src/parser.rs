use std::fmt;
use std::num::IntErrorKind;

use crate::error::HolyError;

#[cfg(test)]
mod blackbox_tests;

#[cfg(test)]
mod parse_expr_tests;

#[cfg(test)]
mod helpers_tests;


mod fmt_display;
mod helpers;
mod parse_expr;



/// Holy Types
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
    FixedArray(Box<Type>, FixedArraySize)
}

/// Fixed array size can only be represented as a const, or a literal usize.
#[derive(Debug, Clone, PartialEq)]
pub enum FixedArraySize {
    Literal(usize),
    Const(String)
}


impl Type {
    pub fn is_integer_type(&self) -> bool {
        match self {
            Type::Int8 |
            Type::Int16 |
            Type::Int32 |
            Type::Int64 |
            Type::Int128 |

            Type::Byte |
            Type::Uint16 |
            Type::Uint32 |
            Type::Uint64 |
            Type::Uint128 |
            
            Type::Usize => true,

            _ => false
        }
    }

    pub fn is_floating_type(&self) -> bool {
        match self {
            Type::Float32 |
            Type::Float64 => true,

            _ => false
        }
    }


    pub fn is_numeric_type(&self) -> bool {
        return self.is_integer_type() || self.is_floating_type()
    }

    pub fn is_array_type(&self) -> bool {
        let is_dynm_arr = matches!(self, Type::Array(_));
        
        let is_fixed_arr = matches!(self, Type::FixedArray(_, _));


        return is_dynm_arr || is_fixed_arr;
    }


    pub fn get_array_inner_most_type(&self) -> &Type {
        if matches!(self, Type::Array(_)) {
            let mut current = self;

            while let Type::Array(inner) = current {
                current = inner;
            }

            return current;
        }


        if matches!(self, Type::FixedArray(_, _)) {
            let mut current = self;

            while let Type::FixedArray(inner, _) = current {
                current = inner;
            }

            return current;
        }


        panic!("(Compiler bug) Do not call get_inner_most_type unless you are sure Type is an array. Self: {:?}", self);
    }
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

            IntLiteralValue::Byte(_) => Type::Byte,

            IntLiteralValue::Uint16(_) => Type::Uint16,
            IntLiteralValue::Uint32(_) => Type::Uint32,
            IntLiteralValue::Uint64(_) => Type::Uint64,
            IntLiteralValue::Uint128(_) => Type::Uint128,
            
            IntLiteralValue::Usize(_) => Type::Usize,

        }
    }

    pub fn is_signed(self) -> bool {
        match self {
            IntLiteralValue::Int8(_) |
            IntLiteralValue::Int16(_) |
            IntLiteralValue::Int32(_) |
            IntLiteralValue::Int64(_) |
            IntLiteralValue::Int128(_) => true,

            _ => false
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


    pub fn as_u128(self) -> u128 {
        match self {
            IntLiteralValue::Usize(v) => v as u128,
            IntLiteralValue::Byte(v) => v as u128,
            IntLiteralValue::Uint16(v) => v as u128,
            IntLiteralValue::Uint32(v) => v as u128,
            IntLiteralValue::Uint64(v) => v as u128,
            IntLiteralValue::Uint128(v) => v,
            
            other => {
                panic!("(Compiler bug) Safety code prevented you from casting a signed literal as an unsigned u128. {:?}", other);
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

/* 
 * Basically, this is needed because without it, NaN == NaN, -0.0 == 0.0, inf, and more would
 * produce the wrong boolean comparison result since FloatLiteralValue has
 * PartialEq derived
 *
*/
impl PartialEq for FloatLiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FloatLiteralValue::Float32(a), FloatLiteralValue::Float32(b)) => a.to_bits() == b.to_bits(),
            (FloatLiteralValue::Float64(a), FloatLiteralValue::Float64(b)) => a.to_bits() == b.to_bits(),
            _ => false,
        }
    }
}

impl Eq for FloatLiteralValue {}



/// AST expressions nodes
#[derive(Debug, Clone, PartialEq)]
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
    RangeCall {
        start: Box<Expr>,
        end: Box<Expr>,
        span: Span,
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Negate,
    BitwiseNot,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,

    BitwiseShiftLeft,
    BitwiseShiftRight,
    BitwiseAnd,
    BitwiseOr,

    And,
    Or,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_name: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub type_name: Type,
    pub value: Option<Expr>,
    pub span: Span,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Vec<Type>>,
    pub body: Vec<Stmt>,
    pub span: Span,
}


#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiAssignment {
    pub names: Vec<String>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub holder_name: String,
    pub value: Expr,
    pub branch: Vec<Stmt>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub branch: Vec<Stmt>,
    pub span: Span
}


#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub if_branch: Vec<Stmt>,
    pub elif_branches: Vec<(Expr, Vec<Stmt>)>,
    pub else_branch: Option<Vec<Stmt>>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfiniteStmt {
    pub branch: Vec<Stmt>,
    pub span: Span
}


#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl(Variable),
    VarDeclMulti(Vec<Variable>, Expr),
    VarAssign(VariableAssignment),
    VarAssignMulti(MultiAssignment),
    Expr(Expr),
    Lock(Vec<Expr>),
    Unlock(Vec<Expr>),
    Return(Vec<Expr>),
    For(ForStmt),
    While(WhileStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    If(IfStmt),
    Infinite(InfiniteStmt),
    Func(Function), 
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

            let inner = &return_type_str[1..return_type_str.len()-1];
            let mut types = Vec::new();
            if !inner.trim().is_empty() {
                let split_parts = helpers::split_char_top_level(',', inner)
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
        let is_block_opener = t.starts_with("infinite ")
            || t.starts_with("if ")
            || t.starts_with("elif ")
            || t.starts_with("else ")
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

fn parse_for_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.trim();
    let span = Span { line: start_i + 1, column: 0 };

    if !line.ends_with('{') {
        return Err(HolyError::Parse(format!(
            "For loop statement must end with `{{`, instead we got `{}` (line {} column {})",
            raw, span.line, span.column
        )));
    }

    let for_str = line["for ".len()..].trim_end_matches('{').trim();


    if for_str.is_empty() {
        return Err(HolyError::Parse(format!(
            "For loop statement construction cannot be empty! (line {} column {})",
            span.line, span.column
        )));
    }

    let parts: Vec<&str> = for_str.split(" in ").collect();
    if parts.len() != 2 {
        return Err(HolyError::Parse(format!(
            "For loop statement is not constructed properly. (line {} column {})",
            span.line, span.column
        )));
    }

    let holder_name = parts[0].to_string();

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
    if parts[1].starts_with("range(") && parts[1].ends_with(")") {
        let range_str = parts[1]["range(".len()..].strip_suffix(")").unwrap();

        let split_args = helpers::split_char_top_level(',', range_str)
            .map_err(|e| HolyError::Parse(format!("{} (line {} column {})", e.to_string(), span.line, span.column)))?;

        if split_args.len() != 2 {
            return Err(HolyError::Parse(format!(
                "For loop range statement takes exactly 2 integer arguments. (line {} column {})",
                span.line, span.column
            )));

        }

        let start_expr = parse_expr::parse_expr(split_args[0], span)?;
        let end_expr = parse_expr::parse_expr(split_args[1], span)?;

        expr = Expr::RangeCall{ start: Box::new(start_expr), end: Box::new(end_expr), span: span };
    } else {
        expr = parse_expr::parse_expr(parts[1], span)?;
    }

    let (branch, next_i) = parse_block(lines, start_i + 1)?;

    Ok((
        Stmt::For(ForStmt {
            holder_name,
            value: expr,
            branch,
            span,
        }),
        next_i,
    ))
}

fn parse_infinite_stmt(lines: &Vec<&str>, start_i: usize) -> Result<(Stmt, usize), HolyError> {
    let raw = lines[start_i];
    let line = helpers::strip_inline_comment(raw);
    let line = line.replace(" ", "");
    let span = Span { line: start_i + 1, column: 0 };

    if line != "infinite{" {
        return Err(HolyError::Parse(format!(
            "Invalid infinite loop syntax {{ at line {}: {}",
            span.line, raw
        )));
    }

    let (branch, next_i) = parse_block(lines, start_i + 1)?;

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
    let (branch, next_i) = parse_block(lines, start_i + 1)?;

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
        // own var_name type_name = expression
        // own var_name type_name (all types have default values.)
        // 
        // special-case multi-declaration: own x T2, y T2 = call() (just example, declared can be as many as you want, but RHS can only be a single expression) 
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
                    vars.push(Variable { name: n.clone(), type_name: t.clone(), value: None, span });
                }
                return Ok(Stmt::VarDeclMulti(vars, value));
            }

            // Otherwise a single declaration.
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
                let split_parts = helpers::split_char_top_level(',', elems_str)
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
        Err(_) => {     
            let expr = parse_expr::parse_expr(s, span)?;

            Ok(expr)
        }
    }
}




/// This is NOT meant to be used in any other functions, only within the following functions:
/// parse_type, parse_array_suffix, parse_base_type
enum InternalArraySuffix {
    Dynamic,
    Fixed(FixedArraySize),
}

fn parse_type(s: &str, span: &Span) -> Result<Type, HolyError> {
    let token = s.trim();

    if token.is_empty() {
        return Err(HolyError::Parse(format!(
            "Invalid type construction `{}` (line {} column {})",
            token, span.line, span.column
        )));
    }

    // Split into base name and bracket suffixes at the first '['
    if let Some(bracket_start) = token.find('[') {
        let base_str = token[..bracket_start].trim();
        let suffix_str = &token[bracket_start..];

        let base_ty = parse_base_type(base_str, span)?;

        // Collect all suffixes left-to-right: e.g. "[1][]" becomes [Fixed(1), Dynamic]
        let suffixes = parse_array_suffixes(suffix_str, span)?;

        // Apply them in REVERSE so the rightmost suffix wraps the base first (innermost),
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
            let close = rest.find(']').ok_or_else(|| {
                HolyError::Parse(format!(
                    "Unclosed '[' in type at line {} column {}",
                    span.line, span.column
                ))
            })?;
            let size_str = rest[1..close].trim();
            // Empty brackets are handled by the "[]" branch above; reaching here
            // with an empty size_str means something like "int32[ ]" which is invalid.
            if size_str.is_empty() {
                return Err(HolyError::Parse(format!(
                    "Empty brackets in type at line {} column {}",
                    span.line, span.column
                )));
            }
            suffixes.push(InternalArraySuffix::Fixed(parse_fixed_array_size(size_str, span)?));
            rest = &rest[close + 1..];
        } else {
            return Err(HolyError::Parse(format!(
                "Unexpected characters in type suffix `{}` at line {} column {}",
                rest, span.line, span.column
            )));
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
        "float32" => Ok(Type::Float32),
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

