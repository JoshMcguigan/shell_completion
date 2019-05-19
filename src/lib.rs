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
    
    /// Given a list of subcommands, print any that match the current word
    fn complete_subcommand<'a, T>(&self, subcommands: T)
    where
        T: IntoIterator<Item = &'a str>,
        T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
    {
        private_complete_subcommand(self, subcommands)
            .into_iter()
            .for_each(|subcommand| println!("{}", subcommand));
    }

    /// Print directory completions based on the current word
    fn complete_directory(&self) {
        private_complete_directory(self, false)
            .into_iter()
            .for_each(|x| println!("{}", x));
    }

    /// Print file completions based on the current word
    /// Also returns directories because the user may be entering a file within that directory
    fn complete_file(&self) {
        private_complete_directory(self, true)
            .into_iter()
            .for_each(|x| println!("{}", x));
    }
}

fn private_complete_subcommand<'a, C, T>(completion: &C, subcommands: T) -> T
where
    C: CompletionInput,
    T: IntoIterator<Item = &'a str>,
    T: std::iter::FromIterator<<T as std::iter::IntoIterator>::Item>,
{
    subcommands
        .into_iter()
        .filter(|&subcommand| subcommand.starts_with(completion.current_word()))
        .collect()
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

    struct TestCompletionInput {
        args: Vec<&'static str>,
        arg_index: usize,
        char_index: usize,
    }

    impl CompletionInput for TestCompletionInput {
        fn args(&self) -> Vec<&str> {
            self.args.clone()
        }
        fn arg_index(&self) -> usize {
            self.arg_index
        }
        fn char_index(&self) -> usize {
            self.char_index
        }
    }

    #[test]
    fn test_subcommand_completions() {
        let input = TestCompletionInput {
            args: vec!["democli", "st"],
            arg_index: 1,
            char_index: 2,
        };

        let completions = private_complete_subcommand(&input, vec!["add", "start", "stop", "delete"]);

        assert_eq!(vec!["start", "stop"], completions);
    }

    #[test]
    fn test_directory_completions() {
        let input = TestCompletionInput {
            args: vec!["democli", "sr"],
            arg_index: 1,
            char_index: 2,
        };

        let completions = private_complete_directory(&input, false);

        assert_eq!(vec!["src"], completions);
    }

    #[test]
    fn test_file_completions() {
        let input = TestCompletionInput {
            args: vec!["democli", "src/li"],
            arg_index: 1,
            char_index: 6,
        };

        let completions = private_complete_directory(&input, true);

        assert_eq!(vec!["src/lib.rs"], completions);
    }

    #[test]
    fn test_directory_completions_project_root() {
        let input = TestCompletionInput {
            args: vec!["democli", "./"],
            arg_index: 1,
            char_index: 2,
        };

        let completions = private_complete_directory(&input, false);

        assert!(completions.contains(&String::from("./src")));
        assert!(completions.contains(&String::from("./target")));
        assert!(completions.contains(&String::from("./.git")));
        assert_eq!(3, completions.len());
    }
}
