/// Fixed array size can only be represented as a const, or a literal usize.

use crate::ast::exprs::Expr;
use crate::ast::span::Span;
use crate::ast::int_literal_value::IntLiteralValue;

#[derive(Debug, Clone, PartialEq)]
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
    String,
    Array(Box<Type>),
    FixedArray(Box<Type>, FixedArraySize)
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

    /// Converts a fixed_array into dynamic array, and walks recursively into fixed_array type and
    /// does the same.
    /// 
    pub fn fixed_array_to_dynamic_array_type_full(&self) -> Type {
        if !matches!(self, Type::Array(_) | Type::FixedArray(_, _)) {
            panic!("(Compiler bug) Do not call fixed_array_to_dynamic_array_type_full unless you are sure Type is an array. Self: {:?}", self);
        }

        fn fixed_array_to_dynamic_array_type_full_hazmat(t: &Type) -> Type {
            match t {
                Type::FixedArray(inner, _) => {
                    let new_inner = Box::new(fixed_array_to_dynamic_array_type_full_hazmat(inner));
                    return Type::Array(new_inner)
                },
                Type::Array(inner) => {
                    let new_inner = Box::new(fixed_array_to_dynamic_array_type_full_hazmat(inner));
                    return Type::Array(new_inner)
                }
                _ => t.clone(),
            }
        }

        return fixed_array_to_dynamic_array_type_full_hazmat(self);
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

    // When variable is declared, e.g.
    // own VAR_NAME TYPE_NAME
    //
    // It has no value. So parser has to assign it a value.
    // Parser must use TYPE.get_default_value()
    // to get the default value for a type.
    //
    // Integers are 0, Float64 is 0.0, Strings are "", and dynamic arrays are empty.
    // Any other types produces a panic, because it requires programmer explicit initialization 
    //
    pub fn get_default_value(&self, span: Span) -> Expr {
        match self {
            Type::Int8 => Expr::IntLiteral { value: IntLiteralValue::Int8(0), span },
            Type::Int16 => Expr::IntLiteral { value: IntLiteralValue::Int16(0), span },
            Type::Int32 => Expr::IntLiteral { value: IntLiteralValue::Int32(0), span },

            Type::Int64 => Expr::IntLiteral { value: IntLiteralValue::Int64(0), span },
            Type::Int128 => Expr::IntLiteral { value: IntLiteralValue::Int128(0), span },

            Type::Byte => Expr::IntLiteral { value: IntLiteralValue::Byte(0), span },
            Type::Uint16 => Expr::IntLiteral { value: IntLiteralValue::Uint16(0), span },
            Type::Uint32 => Expr::IntLiteral { value: IntLiteralValue::Uint32(0), span },
            Type::Uint64 => Expr::IntLiteral { value: IntLiteralValue::Uint64(0), span },
            Type::Uint128 => Expr::IntLiteral { value: IntLiteralValue::Uint128(0), span },
            Type::Usize => Expr::IntLiteral { value: IntLiteralValue::Usize(0), span },

            Type::Float64 => Expr::Float64Literal { value: 0.0, span },

            Type::Bool => Expr::BoolLiteral { value: false, span: span },

            Type::String => Expr::StringLiteral { value: "".to_string(), span },
            Type::Array(t) => {
                // This is just to ensure it doesnt have any fixedArrays with in.
                if t.is_array_type() {
                    t.get_default_value(span);
                }

                Expr::ArrayLiteral { elements: Vec::new(), span: span }
            },
            Type::FixedArray(_, _) => panic!(),
        }
    }


    
}


