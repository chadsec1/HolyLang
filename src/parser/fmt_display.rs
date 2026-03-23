use super::*;

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
            
            Type::Usize => "usize",

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


            IntLiteralValue::Usize(v) => write!(f, "{}", v),

            IntLiteralValue::Byte(v) => write!(f, "{}", v),
            IntLiteralValue::Uint16(v) => write!(f, "{}", v),
            IntLiteralValue::Uint32(v) => write!(f, "{}", v),
            IntLiteralValue::Uint64(v) => write!(f, "{}", v),
            IntLiteralValue::Uint128(v) => write!(f, "{}", v),
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Expr::IntLiteral { .. } => "Int Literal",
            Expr::FloatLiteral { .. } => "Float Literal",
            Expr::BoolLiteral { .. } => "Bool Literal",
            Expr::ArrayLiteral { .. } => "Array Literal",
            Expr::StringLiteral { .. } => "String Literal",
            Expr::Var { .. } => "Variable",
            Expr::UnaryOp { .. } => "Unary Operation",
            Expr::BinOp { .. } => "Binary Operation",
            Expr::Call { .. } => "Function Call",
            Expr::ArraySingleAccess { .. } => "Array Index Access",
            Expr::ArrayMultipleAccess { .. } => "Array Slice Access",
            Expr::CopyCall { .. } => "Copy Call",
            Expr::FormatCall { .. } => "Format Call",
            Expr::RangeCall { .. } => "Range Call",
        };
        write!(f, "{}", name)
    }
}


