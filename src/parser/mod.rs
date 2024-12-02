//! Flux parsing

use crate::lexer::Token;
use crate::ast::*;
use crate::Span;

/// A error which can occur during parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {

}

/// A parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'a> {
    tokens: &'a Vec<Span<Token>>,
    errors: Vec<ParserError>,

    out: Vec<Span<DeclarationStatement>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    pub fn new(tokens: &'a Vec<Span<Token>>) -> Self {
        Self {
            tokens: tokens,
            errors: Vec::new(),
            out: Vec::new(),
        }
    }

    /// Parses the tokens into statments
    pub fn parse(&mut self) {
        todo!()
    }

    /// Did the parser encounter any errors?
    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    /// All errors of the parser (if there were
    /// no error it simply returns a empty
    /// vector)
    pub fn errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    /// The parsed statements
    pub fn out(&self) -> &Vec<Span<DeclarationStatement>> {
        &self.out
    }
}