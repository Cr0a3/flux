use crate::lexer::Token;
use crate::ast::*;
use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'a> {
    tokens: &'a Vec<Span<Token>>,
    errors: Vec<ParserError>,

    out: Vec<Span<DeclarationStatement>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Span<Token>>) -> Self {
        Self {
            tokens: tokens,
            errors: Vec::new(),
            out: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        todo!()
    }

    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    pub fn out(&self) -> &Vec<Span<DeclarationStatement>> {
        &self.out
    }
}