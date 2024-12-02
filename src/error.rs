//! Error printing infrastructure for flux

use ygen::Support::Error as ErrorMaker;

/// Pretty prints errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPrettyPrinter {
    file: String,
}

impl ErrorPrettyPrinter {
    /// Creates a new `ErrorPrettyPrinter`
    pub fn new(file: String) -> Self {
        Self {
            file: file,
        }
    }

    /// Prints the error
    pub fn print<T: Into<PrettyError> + Clone>(&self, error: &T) {
        // At first we turn the error into a PrettyError
        let error: PrettyError = error.to_owned().into();

        // we create the "maker" which makes our error so that we can print it out
        let maker = ErrorMaker::new(
            error.title, 
            &self.file, 
            error.line.to_string(), 
            error.coloumn.to_string
            ()
        );

        // finally we print out the error
        maker.print();
    }
}

/// A error which can be "pretty" printed
pub struct PrettyError {
    line: u64,
    coloumn: u64,

    title: String,
}

impl Into<PrettyError> for crate::lexer::LexingError {
    fn into(self) -> PrettyError {
        todo!()
    }
}

impl Into<PrettyError> for crate::parser::ParserError {
    fn into(self) -> PrettyError {
        todo!()
    }
}

impl Into<PrettyError> for crate::semnatic::SemaError {
    fn into(self) -> PrettyError {
        todo!()
    }
}