#[derive(Debug, Clone)]
pub enum Token {
    // Literals

    Ident(String),
    Int(i64),
    Float(f64),

    // Keywords

    Fn,
    Let,

    // Tokens

    LParan, // (
    RParan, // )
    LCurly, // {
    RCurly, // }
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Equal,  // =
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