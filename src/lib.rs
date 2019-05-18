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

#[derive(Debug)]
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
            cursor_position: env::var("COMP_POINT")
                .map_err(|_| CompletionInputParsingError::MissingEnvVar)?
                .parse::<u32>()
                .map_err(|_| CompletionInputParsingError::CursorPositionNotNumber)?,
        })
    }

    /// Given a list of subcommands, print any that match the current word
    pub fn print_subcommand_completions<'a, T>(&self, subcommands: T)
    where
        T: IntoIterator<Item = &'a str>,
        T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
    {
        self.subcommand_completions(subcommands)
            .into_iter()
            .for_each(|subcommand| println!("{}", subcommand));
    }

    fn subcommand_completions<'a, T>(&self, subcommands: T) -> T
    where
        T: IntoIterator<Item = &'a str>,
        T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
    {
        subcommands
            .into_iter()
            .filter(|&subcommand| subcommand.starts_with(&self.current_word))
            .collect()
    }

    /// Print directory completions based on the current word
    pub fn print_directory_completions(&self) {
        self.directory_completions(false)
            .into_iter()
            .for_each(|x| println!("{}", x));
    }

    /// Print file completions based on the current word
    /// Also returns directories because the user may be entering a file within that directory
    pub fn print_file_completions(&self) {
        self.directory_completions(true)
            .into_iter()
            .for_each(|x| println!("{}", x));
    }

    fn directory_completions(&self, include_files: bool) -> Vec<String> {
        let current_word_parts: Vec<&str> = self.current_word.rsplitn(2, "/").collect();
        let (root_path, partial_path) = match current_word_parts.len() {
            2 => (current_word_parts[1], current_word_parts[0]),
            0 | 1 => ("./", current_word_parts[0]),
            _ => unreachable!(),
        };
        match std::fs::read_dir(&root_path) {
            Ok(iter) => {
                let paths = iter
                    .filter_map(|r| r.ok())
                    // include_files returns files and directories
                    //     because the user may be targeting a file which
                    //     is several directories deep
                    .filter(|dir| include_files || match dir.metadata() {
                        Ok(metadata) => metadata.is_dir(),
                        Err(_) => false,
                    })
                    .map(|dir| dir.path().to_string_lossy().into_owned())
                    .filter(|dir| {
                        dir.rsplitn(2, "/")
                            .next()
                            .unwrap()
                            .starts_with(partial_path)
                    });
                if self.current_word.starts_with("./") {
                    paths.collect()
                } else {
                    paths
                        .map(|p| p.trim_start_matches("./").to_string())
                        .collect()
                }
            }
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

        let completions = input.directory_completions(false);

        assert_eq!(vec!["src"], completions);
    }

    #[test]
    fn test_file_completions() {
        let input = CompletionInput {
            command: "democli".to_string(),
            current_word: "src/li".to_string(),
            preceding_word: "democli".to_string(),
            line: "democli src/li".to_string(),
            cursor_position: 14,
        };

        let completions = input.directory_completions(true);

        assert_eq!(vec!["src/lib.rs"], completions);
    }

    #[test]
    fn test_directory_completions_project_root() {
        let input = CompletionInput {
            command: "democli".to_string(),
            current_word: "./".to_string(),
            preceding_word: "democli".to_string(),
            line: "democli ./".to_string(),
            cursor_position: 10,
        };

        let completions = input.directory_completions(false);

        assert!(completions.contains(&String::from("./src")));
        assert!(completions.contains(&String::from("./target")));
        assert!(completions.contains(&String::from("./.git")));
        assert_eq!(3, completions.len());
    }
}
