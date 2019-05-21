use std::env;

use crate::CompletionInput;

/// BashCompletionInput is a struct which contains input data passed from the shell into a
/// completion script. Data within this struct should be used by a completion script to determine
/// appropriate completion options.
pub struct BashCompletionInput {
    /// $COMP_LINE - the full text that the user has entered
    line: String,
    /// $COMP_POINT - the cursor position (a numeric index into `line`)
    cursor_position: usize,
}

#[derive(Debug)]
pub enum BashCompletionInputParsingError {
    MissingEnvVar,
    CursorPositionNotNumber,
}

impl BashCompletionInput {
    /// Create a new BashCompletionInput by reading arguments and environment variables
    pub fn from_args() -> Result<Self, BashCompletionInputParsingError> {
        Ok(BashCompletionInput {
            line: env::var("COMP_LINE").map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?,
            cursor_position: env::var("COMP_POINT")
                .map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?
                .parse::<usize>()
                .map_err(|_| BashCompletionInputParsingError::CursorPositionNotNumber)?,
        })
    }
}

/// Used only for unit testing
impl<T> From<T> for BashCompletionInput
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        let line = s.into();
        let cursor_position = line.len();

        BashCompletionInput {
            line, 
            cursor_position,
        }
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
    fn test_trait_impl_two_parts() {
        let input = BashCompletionInput::from("democli src/li");

        assert_eq!(vec!["democli", "src/li"], input.args());
        assert_eq!(1, input.arg_index());
        assert_eq!(6, input.char_index());
    }

    #[test]
    fn test_trait_impl_one_part() {
        let input = BashCompletionInput::from("democli ");

        assert_eq!(vec!["democli", ""], input.args());
        assert_eq!(1, input.arg_index());
        assert_eq!(0, input.char_index());
    }
}
