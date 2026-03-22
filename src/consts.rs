
/// Reserved language keywords:
/// No variables, or function names, can use them.
/// case-insensitive.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "func", "own", "return", "for", "in", "if", "elif", "else", "true", "false",
    "int8", "int16", "int32", "int64", "int128", "byte", "uint16", "uint32", "uint64",
    "uint128", "float32", "float64", "usize", "bool", "string", "copy", "format",
    "lock", "unlock", "while", "break", "continue"
];
