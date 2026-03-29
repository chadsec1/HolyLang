use crate::parser::{
    BinOpKind, Type
};

pub const AllBinOpKindArth: [BinOpKind; 4] = [
    BinOpKind::Add,
    BinOpKind::Subtract,
    BinOpKind::Multiply,
    BinOpKind::Divide,
];

pub const AllBinOpKindComp: [BinOpKind; 6] = [
    BinOpKind::Equal,
    BinOpKind::NotEqual,
    BinOpKind::Greater,
    BinOpKind::GreaterEqual,
    BinOpKind::Less,
    BinOpKind::LessEqual, 
];

pub const BinOpKindCompSymbols: [&str; 6] = [
    "==",
    "!=",
    ">",
    ">=",
    "<",
    "<="
];

pub const BinOpKindArthSymbols: [&str; 4] = [
    "+",
    "-",
    "*",
    "/"
];



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


