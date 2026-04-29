use super::{ IntLiteralValue, Span };
/// Abstract syntax tree expressions nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal value, the type is the IntLiteralValue Enum wrapper
    IntLiteral {
        value: IntLiteralValue,
        span: Span,
    },
    /// Float64 literal value
    /// HolyLang only supports float64.
    Float64Literal {
        value: f64,
        span: Span,
    },
    BoolLiteral {
        value: bool,
        span: Span,
    },
    ArrayLiteral {
        elements: Vec<Expr>,
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
    ArrayAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    ArraySlicing {
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

/// Logical NEGATE, such as "-EXPRESSION"
/// Bitwise NOT, e.g. "~EXPRESSION"
/// Logical NOT, e.g. "!EXPRESSION"
///
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
