use crate::ast::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemnaticAnalysis<'a> {
    stmts: &'a Vec<DeclarationStatement>,

    errors: Vec<SemaError>,
}

impl<'a> SemnaticAnalysis<'a> {
    pub fn new(stmts: &'a Vec<DeclarationStatement>) -> Self {
        Self {
            stmts: stmts,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self) {
        todo!()
    }

    pub fn had_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn errors(&self) -> &Vec<SemaError> {
        &self.errors
    }
}