mod bash;
pub use bash::BashCompletionInput;

pub trait CompletionInput : Sized {
    fn args(&self) -> Vec<&str>;
    fn arg_index(&self) -> usize;
    fn char_index(&self) -> usize;

    // Returns the current word under the users cursor 
    // Does not include any characters after the cursor
    fn current_word(&self) -> &str {
        self.args()[self.arg_index()].split_at(self.char_index()).0
    }

    // Returns the word before the word under the users cursor
    fn previous_word(&self) -> &str {
        self.args()[self.arg_index() - 1]
    }
    
    /// Given a list of subcommands, print any that match the current word
    fn complete_subcommand<'a, T>(&self, subcommands: T) -> Vec<String>
    where
        T: IntoIterator<Item = &'a str>,
    {
        subcommands
            .into_iter()
            .filter(|&subcommand| subcommand.starts_with(self.current_word()))
            .map(|s| s.to_string())
            .collect()
    }

    /// Print directory completions based on the current word
    fn complete_directory(&self) -> Vec<String> {
        private_complete_directory(self, false)
    }

    /// Print file completions based on the current word
    /// Also returns directories because the user may be entering a file within that directory
    fn complete_file(&self) -> Vec<String> {
        private_complete_directory(self, true)
    }
}

pub trait CompletionSet {
    fn suggest(self);
}

impl<'a, T, U> CompletionSet for T 
where
    T: IntoIterator<Item = U>,
    U: std::fmt::Display,
{
    fn suggest(self) {
        self
            .into_iter()
            .for_each(|completion| println!("{}", completion));
    }
}


fn private_complete_directory<C>(completion: &C, include_files: bool) -> Vec<String> 
where
    C: CompletionInput,
{
    let current_word_parts: Vec<&str> = completion.current_word().rsplitn(2, "/").collect();
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
            if completion.current_word().starts_with("./") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_previous_word() {
        let input = BashCompletionInput::from("democli run --bi");

        assert_eq!("run", input.previous_word());
    }

    #[test]
    fn test_subcommand_completions() {
        let input = BashCompletionInput::from("democli st");

        let completions =input.complete_subcommand(vec!["add", "start", "stop", "delete"]);

        assert_eq!(vec!["start", "stop"], completions);
    }

    #[test]
    fn test_directory_completions() {
        let input = BashCompletionInput::from("democli sr");

        let completions = input.complete_directory();

        assert_eq!(vec!["src"], completions);
    }

    #[test]
    fn test_file_completions() {
        let input = BashCompletionInput::from("democli src/li");

        let completions = input.complete_file();

        assert_eq!(vec!["src/lib.rs"], completions);
    }

    #[test]
    fn test_directory_completions_project_root() {
        let input = BashCompletionInput::from("democli ./");

        let completions = input.complete_directory();

        assert!(completions.contains(&String::from("./src")));
        assert_eq!(1, completions.len());
    }
}
