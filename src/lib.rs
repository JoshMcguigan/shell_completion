use std::env;

/// CompletionInput is a struct which contains all the input data passed from the shell into a
/// completion script. Data within this struct should be used by a completion script to determine
/// appropriate completion options.
pub struct CompletionInput {
    /// Argument 1 - the name of the command whose arguments are being completed
    pub command: String,
    /// Argument 2 - the word under the users cursor when they pressed tab to ask for completions
    pub current_word: String,
    /// Argument 3 - the word preceding the word under the users cursor
    pub preceding_word: String,
    /// $COMP_LINE - the full text that the user has entered
    pub line: String,
    /// $COMP_POINT - the cursor position (a numeric index into `line`)
    pub cursor_position: u32,
}

pub enum CompletionInputParsingError {
    MissingArg,
    MissingEnvVar,
    CursorPositionNotNumber,
}

impl CompletionInput {
    /// Create a new CompletionInput by reading arguments and environment variables
    pub fn from_args() -> Result<Self, CompletionInputParsingError> { 
        let mut args = env::args().skip(1);

        Ok(CompletionInput {
            command: args.next().ok_or(CompletionInputParsingError::MissingArg)?,
            current_word: args.next().ok_or(CompletionInputParsingError::MissingArg)?,
            preceding_word: args.next().ok_or(CompletionInputParsingError::MissingArg)?,
            line: env::var("COMP_LINE").map_err(|_| CompletionInputParsingError::MissingEnvVar)?,
            cursor_position: env::var("COMP_POINT").map_err(|_| CompletionInputParsingError::MissingEnvVar)?
                .parse::<u32>().map_err(|_| CompletionInputParsingError::CursorPositionNotNumber)?,
        })
    }
}

// todo how to test this
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
