mod token;
pub use token::*;
use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingError {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer<'a> {
    input: &'a String,

    errors: Vec<LexingError>,

    tokens: Vec<Span<Token>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            input: input,
            errors: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        todo!()
    }

    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn errors(&self) -> &Vec<LexingError> {
        &self.errors
    }

    pub fn tokens(&self) -> &Vec<Span<Token>> {
        &self.tokens
    }
}