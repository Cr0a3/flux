/// A token returned by the lexer
#[derive(Debug, Clone)]
pub enum Token {
    // Literals

    /// identifiers
    Ident(String),
    /// constant intenger
    Int(i64),
    /// constant float
    Float(f64),

    // Keywords

    /// fn
    Fn,
    /// let
    Let,

    // Tokens

    /// (
    LParan,
    /// )
    RParan,
    /// {
    LCurly,
    /// }
    RCurly,
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// ^
    Xor,
    /// |
    BinaryOr,
    /// &
    BinaryAnd,
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// !
    Not,
    /// =
    Equal,

    // Comments (maybe used in the future by fluxdoc?)
    /// `// ...` or `/* ... */`
    Comment(String),
    /// `/// ...`
    DocComment(String),

    /// ;
    Semicolon

}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ident(l0), Self::Ident(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Token {}