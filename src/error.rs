//! Error printing infrastructure for flux

use ygen::Support::Error as ErrorMaker;

/// Pretty prints errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorPrettyPrinter {
    file: String,
    lines: Vec<String>,
}

impl ErrorPrettyPrinter {
    /// Creates a new `ErrorPrettyPrinter`
    pub fn new(file: String, code: String) -> Self {
        Self {
            file: file,
            lines: code.split("\n").map(|x| x.to_owned()).collect::<Vec<String>>(),
        }
    }

    /// Prints the error
    pub fn print<T: Into<PrettyError> + Clone>(&self, error: &T) {
        // At first we turn the error into a PrettyError
        let error: PrettyError = error.to_owned().into();

        // we create the "maker" which makes our error so that we can print it out
        let mut maker = ErrorMaker::new(
            error.title, 
            &self.file, 
            error.line.to_string(), 
            error.coloumn.to_string
            ()
        );

        let line = self.lines.get((error.line -1 )as usize);

        maker.setCodeLine(
            line.expect("error printer ran out of lines").to_string()
        );

        if let Some(where_string) = error.where_string {
            maker.addWhere(where_string, error.coloumn, error.length);
        }

        // finally we print out the error
        maker.print();
    }
}

/// A error which can be "pretty" printed
pub struct PrettyError {
    line: u64,
    coloumn: u64,

    title: String,

    where_string: Option<String>,
    length: u64,
}

impl Into<PrettyError> for crate::lexer::LexingError {
    fn into(self) -> PrettyError {
        match self {
            crate::lexer::LexingError::UnexpectedCharacter(span) => PrettyError {
                line: span.line.start + 1,
                coloumn: span.coloumn.start + 1,
                title: format!("unexpected character"),
                where_string: Some(format!("unexpected character: `{span}`")),
                length: 1,
            },
        }
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