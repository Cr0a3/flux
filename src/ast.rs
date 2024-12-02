//! Contains the definition for the ast of flux

use crate::Span;

/// A top level statment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationStatement {
    /// A function
    Fn(FuncStmt),
}

/// A function
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    /// The name
    pub name: Span<String>,
    /// The arguments
    pub args: Span<Vec<Span<Var>>>,
    /// The (optional) visibilty
    pub visibilty: Option<Span<Visibilty>>,
    /// The function body
    pub body: Option<Statement>,
}

/// A visibilty
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibilty {
    /// local
    Local,
    /// extern
    Extern,
}

/// A type in flux
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Intenger
    Int,

    /// Named type (e.g: struct)
    Named(String),
}

/// A variable in flux (name + type)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
    /// The variable name
    pub name: Span<String>,
    /// The variable type
    pub ty:   Span<Type>,
}

/// A binary operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperation {

    // Math

    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `^`
    Xor,
    /// `&`
    BinaryAnd,
    /// `|`
    BinaryOr,

    // Comparison

    /// `&&`
    LogicalAnd,
    /// `||`
    LogicalOr
}

/// A binary expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    /// Left side
    pub left: Box<Span<Expr>>,
    /// The operation
    pub op: Span<BinaryOperation>,
    /// Right side
    pub right: Box<Span<Expr>>,
}

/// A call expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpr {
    /// Function to call
    pub name: Span<String>,
    /// The arguments of the call
    pub args: Span<Vec<Span<Expr>>>,
}

/// A expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Binary expression (like `a + b`)
    Binary(Span<BinaryExpr>),
    /// Call expression (like `foo()`)
    Call(Span<CallExpr>),
}

/// A let statement (used for variable declerations)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    /// The variable
    pub var: Span<Var>,
    /// The initializer
    pub init: Option<Span<Expr>>,
}

/// A assignment statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    /// The variable
    pub var: Span<Var>,
    /// The value
    pub value: Span<Expr>,
}

/// A return statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
    /// The (optional) value to return
    pub value: Option<Span<Expr>>,
}

/// A statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// { body }
    Block(Box<Vec<Span<Statement>>>),

    /// let var: type;
    Let(Span<LetStmt>),
    /// var = expr;
    VarAssignment(Span<Assignment>),

    /// return ...;
    Return(Span<ReturnStatement>),
}