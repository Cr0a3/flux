use std::{ fs::File, io::Read };
use ygen::Target::Triple;

use crate::{lexer, parser, semnatic, codegen, error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluxCompilerOut {
    Object,
    Assembly,
    Ir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluxCompilerOpt {
    O0,
    O3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxCompiler {
    emit: FluxCompilerOut,

    code: String,
    file_path: String,
    opt_level: FluxCompilerOpt,

    target: Triple,

    /// If the `custom_out_file` is some the content is the output file
    /// else we should construct it based on the path
    custom_out_file: Option<String>,
}

impl FluxCompiler {
    pub fn new(file_path: &String) -> Result<Self, std::io::Error> {
        // Here we just read the input file
        
        let mut file = File::open(&file_path)?;

        let mut code = String::new();
        file.read_to_string(&mut code)?;

        // And return a flux compiler with default options

        Ok(Self {
            emit: FluxCompilerOut::Object,
            opt_level: FluxCompilerOpt::O0,
            code: code,
            file_path: file_path.to_owned(),
            target: Triple::host(),
            custom_out_file: None,
        })
    }

    pub fn set_o0(&mut self) {
        self.opt_level = FluxCompilerOpt::O0;
    }

    pub fn set_o3(&mut self) {
        self.opt_level = FluxCompilerOpt::O3;
    }

    pub fn set_passes(&mut self, _passes: &String) {
        todo!("we currently can't set a specific pass order");
    }

    pub fn emit_asm(&mut self) {
        self.emit = FluxCompilerOut::Assembly;
    }

    pub fn emit_ir(&mut self) {
        self.emit = FluxCompilerOut::Ir;
    }

    pub fn set_target_from_string(&mut self, triple: &String) -> Result<(), ygen::Target::TripleError>{
        self.set_target( &Triple::parse(triple)? );
        Ok(())
    }

    pub fn set_target(&mut self, target: &Triple) {
        self.target = *target;
    }

    pub fn set_out(&mut self, file_path: &String) {
        self.custom_out_file = Some(file_path.to_owned());
    }

    pub fn compile(&mut self) {
        let err = error::ErrorPrettyPrinter::new(self.file_path.to_string());

        // 1. We lex the file

        let mut lexer = lexer::Lexer::new(&self.code);
        lexer.lex();

        // we check for errors

        if lexer.had_errors() {
            for error in lexer.errors() {
                err.print(error);
            }

            return;
        }

        // 2. We run the parser

        let mut parser = parser::Parser::new(lexer.tokens());
        parser.parse();

        // and now we check for errors

        if parser.had_errors() {
            for error in parser.errors() {
                err.print(error);
            }

            return;
        }

        // 3. We run the semnatic analysis

        let mut sema = semnatic::SemnaticAnalysis::new(parser.out());
        sema.analyze();

        // we now check for errors in the semnatic analyis

        if sema.had_errors() {
            for error in sema.errors() {
                err.print(error);
            }

            return;
        }
        
        // 4. Run all optimizations and the backend
        
        let mut codegen = codegen::CodeGen::new(parser.out());
        
        codegen.run_opt(&self.opt_level);

        // And now finally we run the backend
        codegen.run(&self.emit);
    }
}