use std::env;

use crate::CompletionInput;
use crate::split::{Split, SplitError};

/// BashCompletionInput is a struct which contains input data passed from the shell into a
/// completion script. Data within this struct should be used by a completion script to determine
/// appropriate completion options.
pub struct BashCompletionInput {
    split: Split
}

#[derive(Debug)]
pub enum BashCompletionInputParsingError {
    MissingEnvVar,
    CursorPositionNotNumber,
    SplitError(SplitError),
}

impl BashCompletionInput {
    /// Create a new BashCompletionInput by reading environment variables
    pub fn from_env() -> Result<Self, BashCompletionInputParsingError> {
        let line = env::var("COMP_LINE").map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?;
        let cursor_position = env::var("COMP_POINT")
            .map_err(|_| BashCompletionInputParsingError::MissingEnvVar)?
            .parse::<usize>()
            .map_err(|_| BashCompletionInputParsingError::CursorPositionNotNumber)?;
        Ok(BashCompletionInput {
            split: Split::new(&line, cursor_position)
                .map_err(BashCompletionInputParsingError::SplitError)?,
        })
    }

    /// Create a new BashCompletionInput manually, useful for testing
    pub fn new(line: &str, cursor_position: usize) -> Result<Self, BashCompletionInputParsingError> {
        Ok(BashCompletionInput {
            split: Split::new(line, cursor_position)
                .map_err(BashCompletionInputParsingError::SplitError)?,
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
            split: Split::new(&line, cursor_position).unwrap(),
        }
    }
}

impl CompletionInput for BashCompletionInput {
    fn args(&self) -> Vec<&str> {
        self.split.words.iter().map(|w| w.as_str()).collect()
    }
    fn arg_index(&self) -> usize {
        self.split.current_word
    }
    fn char_index(&self) -> usize {
        self.split.current_character
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
