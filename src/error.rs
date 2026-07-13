use std::fmt;

/// The main goldlang compiler error type
///
#[derive(Debug, Clone)]
pub enum GoldError {
    /// Parsing errors (invalid syntax, unknown token, etc.)
    Parse(String),

    /// Semantic errors (type mismatchs, ownership violation, etc.)
    Semantic(String),
}

impl fmt::Display for GoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::Semantic(msg) => write!(f, "Semantic error: {msg}"),
        }
    }
}

impl std::error::Error for GoldError {}
