use super::{ Type, Span, Expr };

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



