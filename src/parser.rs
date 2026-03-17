use crate::error::HolyError;
use std::fmt;
use std::num::IntErrorKind;


const KEYWORDS: &[&str] = &[
    "func", "own", "return", "for", "forever", "if", "else", "true", "false",
    "int8", "int16", "int32", "int64", "int128", "byte", "uint16", "uint32", "uint64",
    "uint128", "float32", "float64", "bool", "string", "copy"
];

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
    
    Float32,
    Float64,
    Bool,
    String,
    Array(Box<Type>),
    /// Indicates this needs to be inferred during semantic analysis
    Infer,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Type::Int8 => "int8",
            Type::Int16 => "int16",
            Type::Int32 => "int32",
            Type::Int64 => "int64",
            Type::Int128 => "int128",

            Type::Byte => "byte",
            Type::Uint16 => "uint16",
            Type::Uint32 => "uint32",
            Type::Uint64 => "uint64",
            Type::Uint128 => "uint128",

            Type::Float32 => "float32",
            Type::Float64 => "float64",
            Type::Bool => "bool",
            Type::String => "string",
            Type::Array(inner) => &format!("{}[]", inner),
            Type::Infer => "infer",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for IntLiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntLiteralValue::Int8(v) => write!(f, "{}", v),
            IntLiteralValue::Int16(v) => write!(f, "{}", v),
            IntLiteralValue::Int32(v) => write!(f, "{}", v),
            IntLiteralValue::Int64(v) => write!(f, "{}", v),
            IntLiteralValue::Int128(v) => write!(f, "{}", v),

            IntLiteralValue::Byte(v) => write!(f, "{}", v),
            IntLiteralValue::Uint16(v) => write!(f, "{}", v),
            IntLiteralValue::Uint32(v) => write!(f, "{}", v),
            IntLiteralValue::Uint64(v) => write!(f, "{}", v),
            IntLiteralValue::Uint128(v) => write!(f, "{}", v),
        }
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
    Uint128(u128)
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
    CopyCall {
        expr: Box<Expr>,
        span: Span,
    },
    ArraySingleAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    ArrayMultipleAccess {
        array: Box<Expr>,
        start: Box<Expr>,
        end: Box<Expr>,
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
    Divide
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
pub enum Stmt {
    VarDecl(Variable),
    VarDeclMulti(Vec<Variable>, Expr),
    VarAssign(VariableAssignment),
    VarAssignMulti(MultiAssignment),
    Expr(Expr),
    Return(Vec<Expr>),
    Func(Function), // is this even needed? 
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

    validate_identifier_name(&name, start_i + 1)?;

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
                for part in split_comma_top_level(inner) {
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
            validate_identifier_name(&pname, start_i + 1)?;

            let ptype = parse_type(parts[1], &span)?;
            params.push(Param { name: pname, type_name: ptype, span: span });
        }
    }

    // parse body: everything until matching closing brace
    let mut body: Vec<Stmt> = vec![];
    let mut idx = start_i + 1;
    let mut brace_balance = 1; // we saw the opening brace in header
    while idx < lines.len() {
        let raw = lines[idx];
        let t = raw.trim();

        // track braces to support nested blocks if needed
        if t.contains('{') {
            brace_balance += t.matches('{').count();
        }
        if t.contains('}') {
            brace_balance -= t.matches('}').count();
            if brace_balance <= 0 {
                // function end
                return Ok((
                    Function {
                        name,
                        params,
                        return_type,
                        body,
                        span,
                    },
                    idx + 1,
                ));
            }
        }

        // otherwise parse statements inside function
        if !t.is_empty() && !t.starts_with('#') {
            let stmt = parse_stmt(t, idx + 1)?;
            body.push(stmt);
        }

        idx += 1;
    }

    Err(HolyError::Parse(format!(
        "Unterminated function starting at line {}: `{}`",
        start_i + 1,
        lines[start_i]
    )))
}

/// Parse a single statement from a trimmed line. `line_no` used for error messages.
fn parse_stmt(line: &str, line_no: usize) -> Result<Stmt, HolyError> {
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
                "Return requires a value (line {} column {})",
                span.line, span.column
            )));
        }
            
        // top-level comma -> tuple-like expression (e.g. "a, b")
        //
        // Check if return is like: return a, b, ...
        // then split, parse each element, and return the vec.
        // Otherwise create new vec of single parsed element.
        let top_parts = split_comma_top_level(expr_str);
        if top_parts.len() > 1 {
            let mut elems = vec![];
            for p in top_parts {
                elems.push(parse_expr(p.trim(), span)?);
            }
            return Ok(Stmt::Return(elems));
        } else {
            let expr = parse_expr(expr_str, span)?;
            return Ok(Stmt::Return(vec![expr]));
        }

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
                    validate_identifier_name(n, line_no)?;
                    names.push(n.to_string());
                }

                let value = parse_expr(right, span)?;
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
            validate_identifier_name(&name, line_no)?;


            let value = parse_expr(right, span)?;
            return Ok(Stmt::VarDecl(Variable { name, type_name: var_type, value: Some(value), span: span }));



        } else {
            // no '=', expect "own name type" (explicit type, no initializer)
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(HolyError::Parse(format!("Invalid variable declaration `{}` at line {} column {}", line, span.line, span.column)));
            }
            let name = parts[0].to_string();
            validate_identifier_name(&name, line_no)?;

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
                    validate_identifier_name(n, line_no)?;
                    names.push(n.to_string());
                }
                let value = parse_expr(right, span)?;
                return Ok(Stmt::VarAssignMulti(MultiAssignment { names, value, span }));
            }
        }
    }

    if let Some(eq_pos) = line.find('=') {
        let name = line[..eq_pos].trim();
        let right = line[eq_pos + 1..].trim();

        // validate left is a valid identifier
        validate_identifier_name(name, line_no)?;

        let value = parse_expr(right, span)?;
        return Ok(Stmt::VarAssign(VariableAssignment {
            name: name.to_string(),
            value,
            span,
        }));
    }

    // Expression statement (function call, assignment not supported here yet)
    let expr = parse_expr(line, span)?;
    Ok(Stmt::Expr(expr))
}

/// Checks if a given name is a valid HolyLang identifier.
/// Rules:
/// - Can contain letters, digits, and underscore
/// - Must not start with a digit
/// - Must not contain a reserved language keyword (i.e. `own`, etc)
pub fn validate_identifier_name(name: &str, line_no: usize) -> Result<(), HolyError> {
    if name.is_empty() {
        return Err(HolyError::Parse(format!("Empty variable name at line {}", line_no)));
    }

    // Check first character is not a number
    let first = name.chars().next().unwrap();
    if first.is_ascii_digit() {
        return Err(HolyError::Parse(format!(
            "Variable name `{}` cannot start with a number (line {})",
            name, line_no
        )));
    }

    // Check allowed characters: a-z, A-Z, 0-9, _
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(HolyError::Parse(format!(
            "Variable name `{}` contains invalid characters (only letters, numbers, and `_` allowed) at line {}",
            name, line_no
        )));
    }

    // Check against keywords and error even if name is not the 
    // same exact match in terms of being upper or lower case.
    //
    let name_lower = name.to_string();
    let name_lower = name_lower.to_lowercase(); 
    if KEYWORDS.contains(&name_lower.as_ref()) {
        return Err(HolyError::Parse(format!(
            "Variable name `{}` is a reserved keyword at line {}",
            name, line_no
        )));
    }

    Ok(())
}

/// Minimal expression parser:
/// - handles binary operations (left-associative),
/// - function calls like add(x, y),
/// - integer literals,
/// - variable names
fn parse_expr(s: &str, span: Span) -> Result<Expr, HolyError> {
    let s = s.trim();

    if s.is_empty() {
        return Err(HolyError::Parse(format!(
                    "Empty expression  at line {}, column {}",
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

        let str_nq = &s[1..s.len() - 1];

        let value = Expr::StringLiteral { value: str_nq.to_string(), span};

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
   
    if let Some(first_bracket) = find_constructor_bracket(&s) {
        if s.ends_with(']') {
            let constructor_type_str = s[..first_bracket].trim();
            let elems_str = &s[first_bracket + 1..s.len() - 1];

            if !constructor_type_str.is_empty() {
                match parse_type(constructor_type_str, &span) {
                    Ok(inner_ty) => {
                        // wrap into array type for the variable
                        let rhs_var_type = Type::Array(Box::new(inner_ty.clone()));

                        let mut elems: Vec<Expr> = Vec::new();
                        if !elems_str.trim().is_empty() {
                            for part in split_comma_top_level(elems_str) {
                                let part = part.trim();
                                if find_constructor_bracket(part).is_some() {
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
                        if is_array_type(&rhs_var_type) {
                            if let Type::Array(inner_array_ty) = rhs_var_type.clone() {
                                value = Expr::ArrayLiteral { elements: elems, span, array_ty: *inner_array_ty };
                            }
                        }

                        return Ok(value);
                }
                // Not an array literal, but an array access
                Err(e) => {
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
                    // TODO: I dont think this is reliable approach tbh but im bit lazy. 
                    //       Might be worth stress testing in unit testing..
                    } else if indx_parts.len() >= 2 {
                        let start = parse_expr(indx_parts[0], span)?;
                        let end = parse_expr(indx_parts[indx_parts.len() - 1], span)?;
                        
                        let value = Expr::ArrayMultipleAccess { array: Box::new(array), start: Box::new(start), end: Box::new(end), span };

                        return Ok(value);
                    }
                }
            }
        }
        // handle empty typed-array literal like "int32[]"
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

                if is_array_type(&rhs_var_type) {
                    if let Type::Array(inner_array_ty) = rhs_var_type.clone() {
                        value = Expr::ArrayLiteral { elements: Vec::new(), span, array_ty: *inner_array_ty };
                    }
                }
            }
        }
    }

    
    
    // Binary plus handling: split on the first operator
    if let Some((pos, op)) = find_top_level_op_any(s, &['+', '-', '*', '/']) {
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

            // Check to see if it copy(), which is called like a normal function
            // but its not.
            if name == "copy" {
                let arg_expr = parse_expr(args_str, span)?;

                return Ok(Expr::CopyCall{ expr: Box::new(arg_expr), span: span });
            }


            let mut args = vec![];
            if !args_str.trim().is_empty() {
                for a in split_comma_top_level(args_str) {
                    args.push(parse_expr(a.trim(), span)?);
                }
            }
            return Ok(Expr::Call { name, args, span });
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

    validate_identifier_name(s, span.line)?;
    Ok(Expr::Var { name: s.to_string(), span: span})
}



fn parse_typed_array_literal(s: &str, span: Span) -> Result<Expr, HolyError> {
    let s = s.trim();
    // find the constructor '[' that starts the element list
    let ctor_pos = find_constructor_bracket(s).ok_or_else(|| {
        HolyError::Parse(format!("Malformed typed array literal `{}` (line {} column {})", s, span.line, span.column))
    })?;

    if !s.ends_with(']') {
        return Err(HolyError::Parse(format!("Typed array literal missing trailing ']' (line {} column {})", span.line, span.column)));
    }

    let type_str = s[..ctor_pos].trim();
    let elems_str = &s[ctor_pos + 1..s.len() - 1]; // between constructor '[' and final ']'

    // parse the base/inner type (may be nested literal like "int32[]" -> parse_type handles it)
    match parse_type(type_str, &span) {
        Ok(inner_ty) => {
            let mut elems: Vec<Expr> = Vec::new();
            if !elems_str.trim().is_empty() {
                for part in split_comma_top_level(elems_str) {
                    let part = part.trim();
                    // If the part itself looks like a typed-array-literal (i.e. has a constructor bracket),
                    // parse it recursively; otherwise use parse_expr for general expressions.
                    if find_constructor_bracket(part).is_some() {
                        let nested = parse_typed_array_literal(part, span)?;
                        elems.push(nested);
                    } else {
                        let expr = parse_expr(part, span)?;
                        elems.push(expr);
                    }
                }
            }

            Ok(Expr::ArrayLiteral { elements: elems, array_ty: inner_ty, span })
                
        }

        // If its not a type constructor, we gonna assume it's an expression (like an array access)
        Err(e) => {     
            let expr = parse_expr(s, span)?;

            Ok(expr)
        }
    }
}

/// Returns the index of the first `[` that does NOT immediately form a `[]` pair.
/// Useful to distinguish type-suffix `[]` from the constructor `...[ ... ]`.
fn find_constructor_bracket(s: &str) -> Option<usize> {
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
fn find_top_level_op_any(s: &str, ops: &[char]) -> Option<(usize, char)> {
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

/// Split comma-separated args at top-level only (ignore commas in nested calls)
fn split_comma_top_level(s: &str) -> Vec<&str> {
    let mut parts = vec![];
    let mut start = 0usize;
    let mut depth = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => if depth > 0 { depth -= 1 },
            ',' => {
                if depth == 0 {
                    parts.push(s[start..i].trim());
                    start = i + 1;
                }
            }
            _ => {}
        }
    }
    parts.push(s[start..].trim());
    parts
}



fn is_array_type(t: &Type) -> bool {
    matches!(t, Type::Array(_))
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


#[cfg(test)]
mod tests {
    use super::*; 
    
    fn internal_integer_literal_parsing(src: &str, name: &str, ty: Type, LiteralValue: IntLiteralValue) -> Result<(), String> {
        let result = parse(&src).map_err(|e| {
            format!("Failed to parse {:?}", e).to_string()
        })?;

        dbg!(&result);


        if result.functions.len() != 1 {
            return Err(format!("{} functions are defined, but our src only has 1.", result.functions.len()));
        }
        
        let func = &result.functions[0];

        if func.name != "main" {
            return Err(format!( "Function name is `{}` when it should've been `main` instead!", func.name));
        }

        if func.params.len() != 0 {
            return Err(format!( "Function has parameters when it shouldn't"));
        }

        if func.return_type != None {
            return Err(format!("Function has a {:?} return_type when it shouldn't!", func.return_type));
        }


        if let Stmt::VarDecl(var) = &func.body[0] {

            if var.name != name {
                return Err(format!("Expected variable name to be `{}`, instead we got {}", name, var.name));
            }

            if var.type_name != ty {
                return Err(format!("Expected variable `{}` type to be `{}`, instead we got `{}`", name, ty, var.type_name));
            }

            if let Some(Expr::IntLiteral { value, .. }) = &var.value {
                if *value != LiteralValue {
                    return Err(format!("Value of variable `{}` is not {:?}, instead we got {:?}", name, LiteralValue, *value));
                }
            } else {
                return Err(format!("Expected an IntLiteral, instead we got {:?}", var.value));
            }


        } else {
            return Err(format!("Expected first statement in function to be a variable declaration, instead we got {:?}", func.body[0]));
        }

        Ok(())
    }

    #[test]
    fn test_integer_literal_parsing() {
        let src = String::from("
func main() {
    own x  = 1
}
        ");
        let result = internal_integer_literal_parsing(&src, &"x", Type::Infer, IntLiteralValue::Int8(1));
        assert!(!result.is_err(), "Error: {:?}", result.err());

        let src = String::from("
func main() {
    own x = 1000
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Infer, IntLiteralValue::Int16(1000));
        assert!(!result.is_err(), "Error: {:?}", result.err());


        let src = String::from("
func main() {
    own x int32 = 100000
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Int32, IntLiteralValue::Int32(100000));
        assert!(!result.is_err(), "Error: {:?}", result.err());


        // At parser stage, the literal has its own type. So even though `x` is uint32, the literal its
        // self is int32.
        // In semantics enforcement phase, if the literal type can be safely converted to `x`'s type,
        // it is converted, otherwise, it would error.
        // So the parser does not care if literal and the variable types are not the same, that's
        // the semantics phase job.
        // 
        let src = String::from("
func main() {
    own x uint32 = 100000
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Uint32, IntLiteralValue::Int32(100000));
        assert!(!result.is_err(), "Error: {:?}", result.err());



        let src = String::from("
func main() {
    own x uint32 = 10000000000
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Uint32, IntLiteralValue::Int64(10000000000));
        assert!(!result.is_err(), "Error: {:?}", result.err());



        let src = String::from("
func main() {
    own x byte = 1
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Byte, IntLiteralValue::Int8(1));
        assert!(!result.is_err(), "Error: {:?}", result.err());



        
        // Test invalid syntax

        let src = String::from("
func main() {
    own x intt32 = 100
}       
        ");

        let result = internal_integer_literal_parsing(&src, &"x", Type::Uint32, IntLiteralValue::Int64(10000000000));
        assert!(result.is_err(), "The parser did not error on invalid syntax: {:?}", result);

        





    }
}
