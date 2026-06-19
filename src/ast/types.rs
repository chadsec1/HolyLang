/// Fixed array size can only be represented as a const, or a literal usize.
///
use crate::ast::exprs::Expr;
use crate::ast::span::Span;
use crate::ast::int_literal_value::IntLiteralValue;
use crate::error::HolyError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixedArraySize {
    Literal(usize),
    Const(String)
}

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
    Char,
    String,
    Array(Box<Self>),
    FixedArray(Box<Self>, FixedArraySize)
}



impl Type {
    #[must_use]
    pub const fn is_integer_type(&self) -> bool {
        matches!(self, Self::Int8 | Self::Int16 | Self::Int32 | Self::Int64 | Self::Int128 | Self::Byte | Self::Uint16 | Self::Uint32 | Self::Uint64 | Self::Uint128 | Self::Usize)
    }

    #[must_use]
    pub const fn is_floating_type(&self) -> bool {
        matches!(self, Self::Float64)
    }

    #[must_use]
    pub const fn is_numeric_type(&self) -> bool {
        self.is_integer_type() || self.is_floating_type()
    }

    #[must_use]
    pub const fn is_array_type(&self) -> bool {
        let is_dynm_arr = matches!(self, Self::Array(_));
        
        let is_fixed_arr = matches!(self, Self::FixedArray(_, _));

        is_dynm_arr || is_fixed_arr
    }


    ///
    /// # Panics
    /// If callled on non-array types
    ///
    #[must_use]
    pub fn is_fully_fixed_array_type(&self) -> bool {
        assert!(
            matches!(self, Self::Array(_) | Self::FixedArray(_, _)), 
            "(Compiler bug) Do not call is_fully_fixed_array_type unless you are sure Type is an array. Self: {self:?}"
        );

        let mut current = self;
        loop {
            match current {
                Self::Array(_) => return false,
                Self::FixedArray(inner, _) => current = inner,
                _ => return true,
            }
        }
    }

    /// Converts a `FixedArray` into a dynamic `Array`, and walks recursively into `FixedArray` type and
    /// does the same.
    ///
    /// # Panics
    /// If called on non-array types
    /// 
    #[must_use]
    pub fn fixed_array_to_dynamic_array_type_full(&self) -> Self {
        fn fixed_array_to_dynamic_array_type_full_hazmat(t: &Type) -> Type {
            match t {
                Type::FixedArray(inner, _) | Type::Array(inner) => Type::Array(Box::new(fixed_array_to_dynamic_array_type_full_hazmat(inner))),
                _ => t.clone(),
            }
        }

        assert!(matches!(self, Self::Array(_) | Self::FixedArray(_, _)), "(Compiler bug) Do not call `fixed_array_to_dynamic_array_type_full` unless you are sure Type is an array. Self: {self:?}");


        fixed_array_to_dynamic_array_type_full_hazmat(self)
    }

    ///
    /// # Panics
    /// If called on non-array types
    ///
    #[must_use]
    pub fn get_array_inner_most_type(&self) -> &Self {
        assert!(matches!(self, Self::Array(_) | Self::FixedArray(_, _)), "(Compiler bug) Do not call `get_array_inner_most_type` unless you are sure Type is an array. Self: {self:?}");

        let mut current = self;
        loop {
            match current {
                Self::FixedArray(inner, _) | Self::Array(inner) => current = inner,
                _ => return current
            }
        }
    }

    /// When variable is declared, e.g.
    /// `own VAR_NAME TYPE_NAME`
    ///
    /// It has no value. So parser has to assign it a value.
    /// Parser must use `TYPE.get_default_value()`
    /// to get the default value for a type.
    ///
    /// Integers are 0, Float64 is 0.0, Strings are "", and dynamic arrays are empty.
    /// Any other types produces a panic, because it requires programmer explicit initialization 
    ///
    /// # Errors
    /// Calling this on fixed-array types, or char types, will cause a `HolyError` to be returned.
    ///
    pub fn get_default_value(&self, span: Span) -> Result<Expr, HolyError> {
        match self {
            Self::Int8  => Ok(Expr::IntLiteral { value: IntLiteralValue::Int8(0), span }),
            Self::Int16 => Ok(Expr::IntLiteral { value: IntLiteralValue::Int16(0), span }),
            Self::Int32 => Ok(Expr::IntLiteral { value: IntLiteralValue::Int32(0), span }),

            Self::Int64  => Ok(Expr::IntLiteral { value: IntLiteralValue::Int64(0), span }),
            Self::Int128 => Ok(Expr::IntLiteral { value: IntLiteralValue::Int128(0), span }),

            Self::Byte    => Ok(Expr::IntLiteral { value: IntLiteralValue::Byte(0), span }),
            Self::Uint16  => Ok(Expr::IntLiteral { value: IntLiteralValue::Uint16(0), span }),
            Self::Uint32  => Ok(Expr::IntLiteral { value: IntLiteralValue::Uint32(0), span }),
            Self::Uint64  => Ok(Expr::IntLiteral { value: IntLiteralValue::Uint64(0), span }),
            Self::Uint128 => Ok(Expr::IntLiteral { value: IntLiteralValue::Uint128(0), span }),
            Self::Usize   => Ok(Expr::IntLiteral { value: IntLiteralValue::Usize(0), span }),

            Self::Float64 => Ok(Expr::Float64Literal { value: 0.0, span }),
            Self::Bool => Ok(Expr::BoolLiteral { value: false, span }),
            Self::String => Ok(Expr::StringLiteral { value: String::new(), span }),

            Self::Array(t) => Ok({
                // This is just to ensure it doesnt have any fixedArrays with in.
                if t.is_array_type() {
                    let _ = t.get_default_value(span);
                }

                // NOTE: If any weird bugs arise that trigger panicing guard statements, its this
                // "type_name: None". I think no bugs should arise, but I am keeping this comment
                // here just in case :).
                //
                Expr::ArrayLiteral { elements: Vec::new(), type_name: None, span }
            }),
            Self::FixedArray(_, _) => Err(HolyError::Parse(format!("Fixed-size arrays must have explicit values (line {} column {})", span.line, span.column))),

            Self::Char => Err(HolyError::Parse(format!("chars must have explicit values (line {} column {})",  span.line, span.column)))
        }
    }


    
}


