use super::*;

#[derive(Debug, Clone)]
pub enum Expr {
    FnCall(FnCall),
    Literal(Literal),
    StaticAccess(StaticAccess),
    IdentLookup(String),
    SharedRef(Box<Expr>),
    MutRef(Box<Expr>),
    Deref(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct StaticAccess {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

