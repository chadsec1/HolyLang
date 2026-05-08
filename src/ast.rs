mod fmt_display;
mod span;
mod types;
mod stmts;
mod exprs;
mod int_literal_value;
pub use span::Span;
pub use types::*;
pub use stmts::*;
pub use exprs::*;
pub use int_literal_value::*;

#[cfg(test)]
mod fmt_display_tests;

#[cfg(test)]
mod types_tests;


/// The main parent Abstract Syntax Tree node.
/// 
/// `globals` contain a list of global constant defintions
///
/// `functions` contains a list of function defintions
///
#[derive(Debug)]
pub struct AST {
    pub globals: Vec<Stmt>,
    pub functions: Vec<Function>,
}

