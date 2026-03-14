use std::fmt;

/// The main HolyLang compiler error type
#[derive(Debug)]
pub enum HolyError {
    /// Parsing errors (invalid syntax, unknown token, etc.)
    Parse(String),

    /// Semantic errors (type mismatch, ownership violation, missing docs)
    Semantic(String),

    /// Transpiler errors (Rust code generation issues)
    Transpile(String),
}

impl fmt::Display for HolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HolyError::Parse(msg) => write!(f, "Parse error: {}", msg),
            HolyError::Semantic(msg) => write!(f, "Semantic error: {}", msg),
            HolyError::Transpile(msg) => write!(f, "Transpile error: {}", msg),
        }
    }
}

impl std::error::Error for HolyError {}
