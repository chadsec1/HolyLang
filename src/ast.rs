mod fmt_display;

/// Holy Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Byte,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Usize,
    
    Float64,
    Bool,
    String,
    Array(Box<Type>),
    FixedArray(Box<Type>, FixedArraySize)
}

/// Fixed array size can only be represented as a const, or a literal usize.
#[derive(Debug, Clone, PartialEq)]
pub enum FixedArraySize {
    Literal(usize),
    Const(String)
}


impl Type {
    pub fn is_integer_type(&self) -> bool {
        match self {
            Type::Int8 |
            Type::Int16 |
            Type::Int32 |
            Type::Int64 |
            Type::Int128 |

            Type::Byte |
            Type::Uint16 |
            Type::Uint32 |
            Type::Uint64 |
            Type::Uint128 |
            
            Type::Usize => true,

            _ => false
        }
    }

    pub fn is_floating_type(&self) -> bool {
        match self {
            Type::Float64 => true,

            _ => false
        }
    }


    pub fn is_numeric_type(&self) -> bool {
        return self.is_integer_type() || self.is_floating_type()
    }

    pub fn is_array_type(&self) -> bool {
        let is_dynm_arr = matches!(self, Type::Array(_));
        
        let is_fixed_arr = matches!(self, Type::FixedArray(_, _));


        return is_dynm_arr || is_fixed_arr;
    }


    // TODO: Add tests to cover these functions below and above this comment.
    //
    pub fn is_fully_fixed_array_type(&self) -> bool {
        if !matches!(self, Type::Array(_) | Type::FixedArray(_, _)) {
            panic!("(Compiler bug) Do not call is_fully_fixed_array_type unless you are sure Type is an array. Self: {:?}", self);
        }

        let mut current = self;
        loop {
            match current {
                Type::Array(_) => return false,
                Type::FixedArray(inner, _) => current = inner,
                _ => return true,
            }
        }
    }


    pub fn get_array_inner_most_type(&self) -> &Type {
        if !matches!(self, Type::Array(_) | Type::FixedArray(_, _)) {
            panic!("(Compiler bug) Do not call get_array_inner_most_type unless you are sure Type is an array. Self: {:?}", self);
        }

        let mut current = self;
        loop {
            match current {
                Type::Array(inner) => current = inner,
                Type::FixedArray(inner, _) => current = inner,
                _ => return current,
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntLiteralValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Byte(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
    Usize(usize),
}

impl IntLiteralValue {
    pub fn get_type(self) -> Type {
        match self {
            IntLiteralValue::Int8(_) => Type::Int8,
            IntLiteralValue::Int16(_) => Type::Int16,
            IntLiteralValue::Int32(_) => Type::Int32,
            IntLiteralValue::Int64(_) => Type::Int64,
            IntLiteralValue::Int128(_) => Type::Int128,

            IntLiteralValue::Byte(_) => Type::Byte,

            IntLiteralValue::Uint16(_) => Type::Uint16,
            IntLiteralValue::Uint32(_) => Type::Uint32,
            IntLiteralValue::Uint64(_) => Type::Uint64,
            IntLiteralValue::Uint128(_) => Type::Uint128,
            
            IntLiteralValue::Usize(_) => Type::Usize,
        }
    }

    /// Get the bit_width of an integer literal value
    /// e.g. an  i32 bit-width is 32, etc.
    pub fn bit_width(self) -> u32 {
        match self {
            IntLiteralValue::Int8(_) => i8::BITS,
            IntLiteralValue::Int16(_) => i16::BITS,
            IntLiteralValue::Int32(_) => i32::BITS,
            IntLiteralValue::Int64(_) => i64::BITS,
            IntLiteralValue::Int128(_) => i128::BITS,

            IntLiteralValue::Byte(_) => u8::BITS,

            IntLiteralValue::Uint16(_) => u16::BITS,
            IntLiteralValue::Uint32(_) => u32::BITS,
            IntLiteralValue::Uint64(_) => u64::BITS,
            IntLiteralValue::Uint128(_) => u128::BITS,
            
            IntLiteralValue::Usize(_) => usize::BITS,

        }
    }


    /// Return true if the integer literal value is of signed type
    /// i.e. int8, int16, etc.
    pub fn is_signed(self) -> bool {
        match self {
            IntLiteralValue::Int8(_) |
            IntLiteralValue::Int16(_) |
            IntLiteralValue::Int32(_) |
            IntLiteralValue::Int64(_) |
            IntLiteralValue::Int128(_) => true,

            _ => false
        }
    }

    pub fn as_i128(self) -> i128 {
        match self {
            IntLiteralValue::Int8(v) => v as i128,
            IntLiteralValue::Int16(v) => v as i128,
            IntLiteralValue::Int32(v) => v as i128,
            IntLiteralValue::Int64(v) => v as i128,
            IntLiteralValue::Int128(v) => v,

            other => {
                panic!("(Compiler bug) Safety code to prevent you from casting an unsigned integer as signed i128. {:?}", other);
            }
        }
    }


    pub fn as_u128(self) -> u128 {
        match self {
            IntLiteralValue::Usize(v) => v as u128,
            IntLiteralValue::Byte(v) => v as u128,
            IntLiteralValue::Uint16(v) => v as u128,
            IntLiteralValue::Uint32(v) => v as u128,
            IntLiteralValue::Uint64(v) => v as u128,
            IntLiteralValue::Uint128(v) => v,
            
            other => {
                panic!("(Compiler bug) Safety code prevented you from casting a signed literal as an unsigned u128. {:?}", other);
            }
        }
    }
}



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
    ArraySingleAccess {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    ArrayMultipleAccess {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_name: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub type_name: Type,
    pub value: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub name: String,
    pub type_name: Type,
    pub value: Expr,
    pub span: Span,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Vec<Type>>,
    pub body: Vec<Stmt>,
    pub span: Span,
}


#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiAssignment {
    pub names: Vec<String>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub holder_name: String,
    pub value: Expr,
    pub branch: Vec<Stmt>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub branch: Vec<Stmt>,
    pub span: Span
}


#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub if_branch: Vec<Stmt>,
    pub elif_branches: Vec<(Expr, Vec<Stmt>)>,
    pub else_branch: Option<Vec<Stmt>>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfiniteStmt {
    pub branch: Vec<Stmt>,
    pub span: Span
}


#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl(Variable),
    VarDeclMulti(Vec<Variable>, Expr),
    VarAssign(VariableAssignment),
    VarAssignMulti(MultiAssignment),
    Const(Constant),
    Expr(Expr),
    Lock(Vec<Expr>),
    Unlock(Vec<Expr>),
    Return(Vec<Expr>),
    For(ForStmt),
    While(WhileStmt),
    Break(BreakStmt),
    Continue(ContinueStmt),
    If(IfStmt),
    Infinite(InfiniteStmt),
    Func(Function), 
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}



/// Program AST
#[derive(Debug)]
pub struct AST {
    pub globals: Vec<Stmt>,
    pub functions: Vec<Function>,
}

