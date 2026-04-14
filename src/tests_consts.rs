use crate::parser::{
    BinOpKind, Type
};

// Order of these consts matters for tests.


// This array order must match BinOpKindSymbols order
pub const ALL_BIN_OP_KIND: [BinOpKind; 16] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 

    BinOpKind::And,
    BinOpKind::Or,


    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,

    BinOpKind::BitwiseShiftLeft,
    BinOpKind::BitwiseShiftRight,
    BinOpKind::BitwiseAnd,
    BinOpKind::BitwiseOr,
];




pub const ALL_BIN_OP_KIND_ARTH: [BinOpKind; 8] = [
    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,

    BinOpKind::BitwiseShiftLeft,
    BinOpKind::BitwiseShiftRight,
    BinOpKind::BitwiseAnd,
    BinOpKind::BitwiseOr,
];

pub const ALL_BIN_OP_KIND_REAL_ARTH: [BinOpKind; 4] = [
    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,
];

pub const ALL_BIN_OP_KIND_BIT_ARTH: [BinOpKind; 4] = [
    BinOpKind::BitwiseShiftLeft,
    BinOpKind::BitwiseShiftRight,
    BinOpKind::BitwiseAnd,
    BinOpKind::BitwiseOr,
];



pub const ALL_BIN_OP_KIND_COMP: [BinOpKind; 6] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 
];



pub const ALL_BIN_OP_KIND_COMP_ARTH: [BinOpKind; 4] = [
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 
];


pub const ALL_BIN_OP_KIND_COMP_EQ: [BinOpKind; 2] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
];


pub const ALL_BIN_OP_KIND_LOGIC: [BinOpKind; 2] = [
    BinOpKind::And,
    BinOpKind::Or,
];



pub const BIN_OP_KIND_SYMBOLS: [&str; 16] = [
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<=",
    
    "and",
    "or",

    "+",
    "-",
    "*",
    "/",

    "<<",
    ">>",
    "&",
    "|"

];


pub const BIN_OP_KIND_COMP_SYMBOLS: [&str; 6] = [
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<="
];

pub const BIN_OP_KIND_ARTH_SYMBOLS: [&str; 8] = [
    "+",
    "-",
    "*",
    "/",

    "<<",
    ">>",
    "&",
    "|"

];




// Only integers, no array, etc.
pub const ALL_INT_TYPES_NO_ARR: &[Type] = &[
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
];


// No integers, no array type
pub const ALL_TYPES_NO_INTS_NO_ARR: &[Type] = &[
    Type::Float32,
    Type::Float64,
    Type::Bool,
    Type::String,
];


pub const ALL_FLOATS_TYPES: &[Type] = &[
    Type::Float32,
    Type::Float64,
];




// No array type, no bool
pub const ALL_TYPES_NO_ARR_NO_BOOL: &[Type] = &[
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
    Type::String,
];


// No array type, no bool, no string
pub const ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING: &[Type] = &[
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
];

pub const ALL_TYPES_NO_ARR_NO_BOOL_NO_STRING_SCATTERED: &[Type] = &[
    Type::Uint32,
    Type::Int8,
    Type::Int64,
    Type::Uint128,
    Type::Float32,
    Type::Uint16,
    Type::Usize,
    Type::Int16,
    Type::Byte,
    Type::Float64,
    Type::Uint64,
    Type::Int128,
    Type::Int32,
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
];

// No array type, no usize type
pub const ALL_TYPES_NO_ARR_NO_USIZE: &[Type] = &[
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
    Type::Float32,
    Type::Float64,
    Type::Bool,
    Type::String,
];




pub const ALL_SIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Int8,
    Type::Int16,
    Type::Int32,
    Type::Int64,
    Type::Int128,
    Type::Float32,
    Type::Float64,
];


pub const ALL_UNSIGNED_TYPES_NO_ARR: &[Type] = &[
    Type::Byte,
    Type::Uint16,
    Type::Uint32,
    Type::Uint64,
    Type::Uint128,
    Type::Usize,
];




