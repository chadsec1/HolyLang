use super::{ Type, Expr, FixedArraySize, IntLiteralValue };
use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Int8 => "int8",
            Self::Int16 => "int16",
            Self::Int32 => "int32",
            Self::Int64 => "int64",
            Self::Int128 => "int128",

            Self::Byte => "byte",
            Self::Uint16 => "uint16",
            Self::Uint32 => "uint32",
            Self::Uint64 => "uint64",
            Self::Uint128 => "uint128",
            
            Self::Usize => "usize",

            Self::Float64 => "float64",
            Self::Bool => "bool",
            Self::String => "string",
            Self::Array(inner_ty) => &format!("[]{inner_ty}"),
            Self::FixedArray(inner_ty, size) => &format!("[{size}]{inner_ty}")
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for FixedArraySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{l}"),
            Self::Const(c) => write!(f, "{c}"),
        }
 
    }
}


impl fmt::Display for IntLiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int8(v) => write!(f, "{v}"),
            Self::Int16(v) => write!(f, "{v}"),
            Self::Int32(v) => write!(f, "{v}"),
            Self::Int64(v) => write!(f, "{v}"),
            Self::Int128(v) => write!(f, "{v}"),

            Self::Usize(v) => write!(f, "{v}"),

            Self::Byte(v) => write!(f, "{v}"),
            Self::Uint16(v) => write!(f, "{v}"),
            Self::Uint32(v) => write!(f, "{v}"),
            Self::Uint64(v) => write!(f, "{v}"),
            Self::Uint128(v) => write!(f, "{v}")
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::IntLiteral { .. } => "Int Literal",
            Self::Float64Literal { .. } => "Float64 Literal",
            Self::BoolLiteral { .. } => "Bool Literal",
            Self::ArrayLiteral { .. } => "Array Literal",
            Self::StringLiteral { .. } => "String Literal",
            Self::Var { .. } => "Variable",
            Self::UnaryOp { .. } => "Unary Operation",
            Self::BinOp { .. } => "Binary Operation",
            Self::Call { .. } => "Function Call",
            Self::ArrayAccess { .. } => "Array Access",
            Self::ArraySlicing { .. } => "Array Slicing",
            Self::CopyCall { .. } => "Copy Call",
            Self::FormatCall { .. } => "Format Call",
            Self::RangeCall { .. } => "Range Call",
        };
        write!(f, "{name}")
    }
}


