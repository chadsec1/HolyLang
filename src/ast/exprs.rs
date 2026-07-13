use super::{
    IntLiteralValue, Type, Span
};

/// Abstract syntax tree expressions nodes
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal value, the type is the `IntLiteralValue` Enum wrapper
    IntLiteral {
        value: IntLiteralValue,
        span: Span,
    },
    /// Float64 literal value
    /// goldlang only supports float64.
    Float64Literal {
        value: f64,
        span: Span,
    },
    BoolLiteral {
        value: bool,
        span: Span,
    },
    ArrayLiteral {
        elements: Vec<Self>,
        type_name: Option<Type>, // This is just for the transpiler layer.
        span: Span
    },
    CharLiteral {
        value: char,
        span: Span
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
        expr: Box<Self>,
        span: Span,
    },
    BinOp {
        left: Box<Self>,
        op: BinOpKind,
        right: Box<Self>,
        span: Span,
    },
    Call {
        name: String,
        args: Vec<Self>,
        span: Span,
    },
    ArrayAccess {
        array: Box<Self>,
        index: Box<Self>,
        span: Span,
    },
    ArraySlicing {
        array: Box<Self>,
        range: ArraySliceRange,
        span: Span,
    },

    // internal language functions / expressions hard-coded into the language.
    CopyCall {
        expr: Box<Self>,
        span: Span,
    },
    FormatCall {
        template: String,
        expressions: Vec<Self>,
        span: Span,
    },
    RangeCall {
        start: Box<Self>,
        end: Box<Self>,
        span: Span,
    }

}


/// Array slice range type:
/// `From`, e.g. "x[ EXPRESSION : ]"
/// `To`, e.g. "x[ : EXPRESSION ]"
/// `FromTo`, e.g. "x[ EXPRESSION : EXPRESSION ]"
///
#[derive(Debug, Clone, PartialEq)]
pub enum ArraySliceRange {
    From(Box<Expr>),
    To(Box<Expr>),
    FromTo(Box<Expr>, Box<Expr>)
}



/// Unary operations types:
/// Logical `Negate`, such as "-EXPRESSION"
/// Bitwise `Not`, e.g. "~EXPRESSION"
/// Logical `Not`, e.g. "!EXPRESSION"
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Negate,
    BitwiseNot,
    Not,
}

/// Binary operations types:
/// Add, e.g. "EXPRESSION + EXPRESSION"
/// Subtract, e.g. "EXPRESSION - EXPRESSION"
/// Multiply, e.g. "EXPRESSION * EXPRESSION"
/// Divide, e.g. "EXPRESSION / EXPRESSION"
///
/// etc, etc.
///
#[derive(Debug, Clone, PartialEq, Eq)]
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



