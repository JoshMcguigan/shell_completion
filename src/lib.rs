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

    /// Given a list of subcommands, print any that match the current word
    pub fn print_subcommand_completions<'a, T>(&self, subcommands: T)
        where T: IntoIterator<Item = &'a str>,
              T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
    {
        self.subcommand_completions(subcommands)
            .into_iter()
            .for_each(|subcommand| println!("{}", subcommand));
    }

    fn subcommand_completions<'a, T>(&self, subcommands: T) -> T
        where T: IntoIterator<Item = &'a str>,
              T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
    {
        subcommands
            .into_iter()
            .filter(|&subcommand| subcommand.starts_with(&self.current_word))
            .collect()
    }

    /// Print directory completions based on the current word
    pub fn print_directory_completions(&self) {
        self.directory_completions()
            .into_iter()
            .for_each(|x| println!("{}", x));
    }

    fn directory_completions(&self) -> Vec<String> {
        match std::fs::read_dir("./") {
            Ok(iter) => {
                iter
                    .filter_map(|r| r.ok()).map(|dir| dir.path().to_string_lossy().into_owned())
                    .map(|dir| dir.trim_start_matches("./").to_owned())
                    .filter(|dir| dir.starts_with(&self.current_word))
                    .collect()
            },
            Err(_) => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_subcommand_completions() {
        let input = CompletionInput {
            command: "democli".to_string(),
            current_word: "st".to_string(),
            preceding_word: "democli".to_string(),
            line: "democli st".to_string(),
            cursor_position: 10,
        };

        let completions = input.subcommand_completions(vec!["add", "start", "stop", "delete"]);

        assert_eq!(vec!["start", "stop"], completions);
    }

    #[test]
    fn test_directory_completions() {
        let input = CompletionInput {
            command: "democli".to_string(),
            current_word: "sr".to_string(),
            preceding_word: "democli".to_string(),
            line: "democli sr".to_string(),
            cursor_position: 10,
        };

        let completions = input.directory_completions();

        assert_eq!(vec!["src"], completions);
    }
}
