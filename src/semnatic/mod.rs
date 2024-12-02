//! Flux semnatic analysis

use crate::ast::*;
use crate::Span;

/// Error which can occur during the semnatic analysis proccess
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaError {}

/// The semnatic analysis (checks for errors)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemnaticAnalysis<'a> {
    stmts: &'a Vec<Span<DeclarationStatement>>,

    errors: Vec<SemaError>,
}

impl<'a> SemnaticAnalysis<'a> {
    /// Creates a new semnatic analysis
    pub fn new(stmts: &'a Vec<Span<DeclarationStatement>>) -> Self {
        Self {
            stmts: stmts,
            errors: Vec::new(),
        }
    }

    /// Analyzes the statments
    pub fn analyze(&mut self) {
        todo!()
    }

    /// Returns if the sema analysis had errors
    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    /// Returns all errors of the semnatic analysis (if
    /// there were no errors it simply returns a 
    /// empty vector)
    pub fn errors(&self) -> &Vec<SemaError> {
        &self.errors
    }
}