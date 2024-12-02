use crate::ast::*;
use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGen<'a> {
    stmts: &'a Vec<Span<DeclarationStatement>>,
}

impl<'a> CodeGen<'a> {
    pub fn new(stmts: &'a Vec<Span<DeclarationStatement>>) -> Self {
        Self {
            stmts: stmts,
        }
    }

    pub fn run_opt(&mut self, _level: &crate::driver::FluxCompilerOpt) {
        todo!()
    }

    pub fn run(&mut self, _mode: &crate::driver::FluxCompilerOut) {
        todo!()
    }
}