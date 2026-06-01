use super::Type;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
    #[must_use]
    pub const fn get_type(self) -> Type {
        match self {
            Self::Int8(_)   => Type::Int8,
            Self::Int16(_)  => Type::Int16,
            Self::Int32(_)  => Type::Int32,
            Self::Int64(_)  => Type::Int64,
            Self::Int128(_) => Type::Int128,

            Self::Byte(_)    => Type::Byte,
            Self::Uint16(_)  => Type::Uint16,
            Self::Uint32(_)  => Type::Uint32,
            Self::Uint64(_)  => Type::Uint64,
            Self::Uint128(_) => Type::Uint128,
            Self::Usize(_)   => Type::Usize
        }
    }

    /// Get the `bit_width` of an integer literal value
    /// e.g. an  i32 bit-width is 32, etc.
    ///
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub const fn bit_width(self) -> u32 {
        match self {
            Self::Int8(_)   => i8::BITS,
            Self::Int16(_)  => i16::BITS,
            Self::Int32(_)  => i32::BITS,
            Self::Int64(_)  => i64::BITS,
            Self::Int128(_) => i128::BITS,

            Self::Byte(_)    => u8::BITS,
            Self::Uint16(_)  => u16::BITS,
            Self::Uint32(_)  => u32::BITS,
            Self::Uint64(_)  => u64::BITS,
            Self::Uint128(_) => u128::BITS,
            Self::Usize(_)   => usize::BITS
        }
    }


    /// Return true if the integer literal value is of signed type
    /// i.e. int8, int16, etc.
    pub fn is_signed(self) -> bool {
        matches!(self,
            IntLiteralValue::Int8(_) |
            IntLiteralValue::Int16(_) |
            IntLiteralValue::Int32(_) |
            IntLiteralValue::Int64(_) |
            IntLiteralValue::Int128(_))
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


