//! Compiler middle and back end

use crate::ast::*;
use crate::Span;

/// The middle and back end for the compiler
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGen<'a> {
    stmts: &'a Vec<Span<DeclarationStatement>>,
}

impl<'a> CodeGen<'a> {
    /// Creates the new code gen
    pub fn new(stmts: &'a Vec<Span<DeclarationStatement>>) -> Self {
        Self {
            stmts: stmts,
        }
    }

    /// Run optimizations on the statements
    pub fn run_opt(&mut self, _level: &crate::driver::FluxCompilerOpt) {
        todo!()
    }

    /// Run the code generation on the statements
    pub fn run(&mut self, _mode: &crate::driver::FluxCompilerOut) {
        todo!()
    }
}