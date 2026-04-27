/// All statements, expressions, and even literals contain a span which carries information of the line and column
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}
