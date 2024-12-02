//! Flux lexing

mod token;
pub use token::*;
use crate::Span;

/// Error which can occur during lexing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingError {

}

/// The lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer<'a> {
    input: &'a String,

    errors: Vec<LexingError>,

    tokens: Vec<Span<Token>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(input: &'a String) -> Self {
        Self {
            input: input,
            errors: Vec::new(),
            tokens: Vec::new(),
        }
    }

    /// Lexes the input string
    pub fn lex(&mut self) {
        todo!()
    }

    /// Did the lexer encounter any errors?
    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    /// All errors of the lexer (if there were no errors
    /// it returns a empty vector)
    pub fn errors(&self) -> &Vec<LexingError> {
        &self.errors
    }

    /// The tokens produced by the lexer
    pub fn tokens(&self) -> &Vec<Span<Token>> {
        &self.tokens
    }
}