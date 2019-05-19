use std::env;

use crate::CompletionInput;

/// BashCompletionInput is a struct which contains all the input data passed from the shell into a
/// completion script. Data within this struct should be used by a completion script to determine
/// appropriate completion options.
pub struct BashCompletionInput {
    /// Argument 1 - the name of the command whose arguments are being completed
    pub command: String,
    /// Argument 2 - the word under the users cursor when they pressed tab to ask for completions
    pub current_word: String,
    /// Argument 3 - the word preceding the word under the users cursor
    pub preceding_word: String,
    /// $COMP_LINE - the full text that the user has entered
    pub line: String,
    /// $COMP_POINT - the cursor position (a numeric index into `line`)
    pub cursor_position: usize,
}

#[derive(Debug)]
pub enum BashCompletionInputParsingError {
    MissingArg,
    MissingEnvVar,
    CursorPositionNotNumber,
}

impl BashCompletionInput {
    /// Create a new BashCompletionInput by reading arguments and environment variables
    pub fn from_args() -> Result<Self, BashCompletionInputParsingError> {
        let mut args = env::args().skip(1);

        Ok(BashCompletionInput {
            command: args.next().ok_or(BashCompletionInputParsingError::MissingArg)?,
            current_word: args.next().ok_or(BashCompletionInputParsingError::MissingArg)?,
            preceding_word: args.next().ok_or(BashCompletionInputParsingError::MissingArg)?,
            line: env::var("COMP_LINE").map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?,
            cursor_position: env::var("COMP_POINT")
                .map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?
                .parse::<usize>()
                .map_err(|_| BashCompletionInputParsingError::CursorPositionNotNumber)?,
        })
    }
}

impl CompletionInput for BashCompletionInput {
    fn args(&self) -> Vec<&str> {
        // todo this should perform a more sophisticated bash parsing
        self.line.split(" ").collect()
    }
    fn arg_index(&self) -> usize {
        self.line.split_at(self.cursor_position).0
            .chars()
            .filter(|c| *c == ' ')
            .count()
    }
    fn char_index(&self) -> usize {
        let start = self.line.split_at(self.cursor_position).0;
        let current_word_fraction = start.rsplitn(2, ' ').next();
        match current_word_fraction {
            Some(word) => word.len(),
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_impl() {
        let input = BashCompletionInput {
            command: "democli".to_string(),
            current_word: "src/li".to_string(),
            preceding_word: "democli".to_string(),
            line: "democli src/li".to_string(),
            cursor_position: 14,
        };

        assert_eq!(vec!["democli", "src/li"], input.args());
        assert_eq!(1, input.arg_index());
        assert_eq!(6, input.char_index());
    }
}
