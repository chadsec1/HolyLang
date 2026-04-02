use crate::parser::{
    BinOpKind, Type
};

// Order of these consts matters for tests.


// This array order must match BinOpKindSymbols order
pub const ALL_BIN_OP_KIND: [BinOpKind; 10] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 
    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,

];




pub const ALL_BIN_OP_KIND_ARTH: [BinOpKind; 4] = [
    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,
];

pub const ALL_BIN_OP_KIND_COMP: [BinOpKind; 6] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 
];

pub const ALL_BIN_OP_KIND_COMP_EQ: [BinOpKind; 2] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
];


pub const BIN_OP_KIND_SYMBOLS: [&str; 10] = [
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<=",
    "+",
    "-",
    "*",
    "/"

];


pub const BIN_OP_KIND_COMP_SYMBOLS: [&str; 6] = [
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<="
];

pub const BIN_OP_KIND_ARTH_SYMBOLS: [&str; 4] = [
    "+",
    "-",
    "*",
    "/"
];



// No array, no infer types.
pub const ALL_TYPES_NO_ARR_NO_INFER: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Float32,
    Type::Float64,
    Type::Bool,
    Type::String
];

// No array type, and no float types
pub const ALL_TYPES_NO_ARR_NO_FLOAT: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Bool,
    Type::String,
    Type::Infer,
];



// No array type
pub const ALL_TYPES_NO_ARR: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Float32,
    Type::Float64,
    Type::Bool,
    Type::String,
    Type::Infer,
];

// No array, but scattered order.
pub const ALL_TYPES_NO_ARR_SCATTERED: &[Type] = &[
    Type::Int128,
    Type::Int8,
    Type::Uint64,
    Type::Float32,
    Type::Int64,
    Type::Uint16,
    Type::String,
    Type::Uint128,
    Type::Float64,
    Type::Uint32,
    Type::Int16,
    Type::Bool,
    Type::Byte,
    Type::Int32,
    Type::Usize,

    Type::Infer,
];



pub const ALL_SIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Float32,
    Type::Float64,
    Type::Infer,
];


pub const ALL_UNSIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
    Type::Infer,
];




