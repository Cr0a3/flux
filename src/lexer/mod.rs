//! Flux lexing

mod token;
use std::ops::Range;

pub use token::*;
use crate::Span;

/// Error which can occur during lexing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingError {
    /// unexpected character was seen
    UnexpectedCharacter(Span<char>),
}

/// The lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer<'a> {
    input: &'a String,

    start: usize,
    current: usize,
    line: Range<u64>,
    coloumn: Range<u64>,

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
            start: 0,
            current: 0,
            coloumn: 0..0,
            line: 0..0,
        }
    }

    /// Lexes the input string
    pub fn lex(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;

            self.coloumn = self.coloumn.end..self.coloumn.end;
            self.line = self.line.end..self.line.end;

            if let Err(err) = self.lex_tok() {
                self.errors.push(err);
            }
        }
    }

    /// This function lexes tokens
    fn lex_tok(&mut self) -> Result<(), LexingError> {
        let tok = self.advance().expect("lexer ran out of tokens");

        match tok {
            _ if tok.is_whitespace() => {}, // we ignore all whitespaces (e.g: \n)

            '(' => self.tok(Token::LParan),
            ')' => self.tok(Token::RParan),
            '{' => self.tok(Token::LCurly),
            '}' => self.tok(Token::RCurly),
            '+' => self.tok(Token::Add),
            '-' => self.tok(Token::Sub),
            '*' => self.tok(Token::Mul),
            '/' => self.tok(Token::Div),
            '^' => self.tok(Token::Xor),
            '|' => self.tok(Token::BinaryOr),
            '&' => self.tok(Token::BinaryAnd),
            '!' => self.tok(Token::Not),
            '=' => self.tok(Token::Equal),

            ';' => self.tok(Token::Semicolon),

            'a'..='z' | 'A'..='Z' => self.parse_ident()?,
            '0'..='9' => self.parse_number()?,

            unex => {
                return Err(LexingError::UnexpectedCharacter( self.new_span(unex) ));
            }
        }

        Ok(())
    }

    fn new_span<T: std::fmt::Debug + Clone + PartialEq + Eq + ?Sized>(&self, value: T) -> Span<T> {
        Span {
            line: self.line.clone(),
            coloumn: self.coloumn.clone(),
            inner: value,
        }
    } 

    /// Adds a new token to the intern token list
    #[inline]
    fn tok(&mut self, token: Token) {
        self.tokens.push(
            self.new_span(token)
        );
    }

    fn advance(&mut self) -> Option<char> {
        let chars = self.input.chars().collect::<Vec<char>>();
    
        if let Some(char) = chars.get(self.current) {
            if *char == '\n' {
                self.line = self.line.start..self.line.end + 1;
                self.coloumn = 0..0;
            } else {
                self.coloumn = self.coloumn.start..self.coloumn.end + 1;
            }

            self.current += 1;

            Some(*char)
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.input.chars().count() == self.current
    }

    fn parse_ident(&mut self) -> Result<(), LexingError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<(), LexingError> {
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