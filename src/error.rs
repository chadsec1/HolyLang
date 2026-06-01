use std::fmt;

/// The main holylang compiler error type
///
#[derive(Debug, Clone)]
pub enum HolyError {
    /// Parsing errors (invalid syntax, unknown token, etc.)
    Parse(String),

    /// Semantic errors (type mismatchs, ownership violation, etc.)
    Semantic(String),
}

impl fmt::Display for HolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::Semantic(msg) => write!(f, "Semantic error: {msg}"),
        }
    }
}

impl std::error::Error for HolyError {}
