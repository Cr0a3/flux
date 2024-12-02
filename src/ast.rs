use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationStatement {
    Fn(FuncStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    pub name: Span<String>,
}