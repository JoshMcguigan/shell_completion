use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    let input = BashCompletionInput::from_env()
        .expect("Missing expected environment variables");

    complete(input).suggest();
}

fn complete(input: impl CompletionInput) -> Vec<String> {
    match input.arg_index() {
        0 => unreachable!(),
        1 => complete_cargo_commands(input),
        _ => {
            match input.args()[1] {
                "run" => complete_run(input),
                "test" => complete_test(input),
                _ => vec![],
            }
        },
    }
}

fn complete_cargo_commands(input: impl CompletionInput) -> Vec<String> {
    use std::process::Command;
    let output = Command::new("cargo")
            .arg("--list")
            .output()
            .expect("failed to execute cargo");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let cargo_commands : Vec<&str> = stdout.lines()
        .skip(1) // first line is description
        .map(|line| line.split_whitespace().next().unwrap()) // each line is COMMAND DESCRIPTION
        .collect();
    input.complete_subcommand(cargo_commands) 
}

fn complete_run(input: impl CompletionInput) -> Vec<String> {
    let unary_options = vec![
        "--release",
        "--all-features",
        "--no-default-features",
        "--verbose",
        "--quiet",
        "--frozen",
        "--locked",
        "--help",
    ];
    let other_options = vec![
        "--bin",
        "--example",
        "--package",
        "--jobs",
        "--features",
        "--target",
        "--target-dir",
        "--manifest-path",
        "--message-format",
        "--color",
    ];
    
    if input.previous_word() == "run" 
        || !input.previous_word().starts_with("-")
        || unary_options.contains(&input.previous_word()) 
    {
        let all_options = unary_options.into_iter().chain(other_options);
        input.complete_subcommand(all_options)
    } else {
        match input.previous_word() {
            "--target-dir" => input.complete_directory(),
            "--manifest-path" => input.complete_file(),
            "--message-format" => input.complete_subcommand(vec!["human", "json", "short"]),
            "--color" => input.complete_subcommand(vec!["auto", "always", "never"]),
            _ => vec![],
        }
    }
}

// TODO find an appropriate abstraction to solve duplication between complete_run and complete_test
fn complete_test(input: impl CompletionInput) -> Vec<String> {
    let unary_options = vec![
        "--lib",
        "--bins",
        "--examples",
        "--tests",
        "--benches",
        "--all-targets",
        "--doc",
        "--no-run",
        "--no-fail-fast",
        "--all",
        "--jobs",
        "--release",
        "--all-features",
        "--no-default-features",
        "--verbose",
        "--quiet",
        "--frozen",
        "--locked",
        "--help",
    ];
    let other_options = vec![
        "--bin",
        "--example",
        "--test",
        "--bench",
        "--package",
        "--exclude",
        "--jobs",
        "--features",
        "--target",
        "--target-dir",
        "--manifest-path",
        "--message-format",
        "--color",
    ];
    
    if input.previous_word() == "run" 
        || !input.previous_word().starts_with("-")
        || unary_options.contains(&input.previous_word()) 
    {
        let all_options = unary_options.into_iter().chain(other_options);
        input.complete_subcommand(all_options)
    } else {
        match input.previous_word() {
            "--target-dir" => input.complete_directory(),
            "--manifest-path" => input.complete_file(),
            "--message-format" => input.complete_subcommand(vec!["human", "json", "short"]),
            "--color" => input.complete_subcommand(vec!["auto", "always", "never"]),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_fetch() {
        let input = BashCompletionInput::from("cargo fe");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("fetch", completions[0]);
    }

    #[test]
    fn complete_run_option_bin() {
        let input = BashCompletionInput::from("cargo run --bi");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("--bin", completions[0]);
    }

    #[test]
    fn complete_run_option_bin_requires_name() {
        let input = BashCompletionInput::from("cargo run --bin ");
        let completions = complete(input);

        // for now, test that this doesn't return the full list of subcommands
        // eventually this could return the list of binary targets in the crate
        assert_eq!(0, completions.len());
    }

    #[test]
    fn complete_run_option_target_dir() {
        let input = BashCompletionInput::from("cargo run --target-dir sr");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("src", completions[0]);
    }

    #[test]
    fn complete_run_option_manifest_path() {
        let input = BashCompletionInput::from("cargo run --manifest-path Cargo.to");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("Cargo.toml", completions[0]);
    }

    #[test]
    fn complete_run_option_message_format() {
        let input = BashCompletionInput::from("cargo run --message-format ");
        let completions = complete(input);

        assert_eq!(3, completions.len());
        assert_eq!("human", completions[0]);
        assert_eq!("json", completions[1]);
        assert_eq!("short", completions[2]);
    }

    #[test]
    fn complete_run_option_color() {
        let input = BashCompletionInput::from("cargo run --color ");
        let completions = complete(input);

        assert_eq!(3, completions.len());
        assert_eq!("auto", completions[0]);
        assert_eq!("always", completions[1]);
        assert_eq!("never", completions[2]);
    }

    #[test]
    fn complete_run_option_chaining() {
        let input = BashCompletionInput::from("cargo run --color auto --manif");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("--manifest-path", completions[0]);
    }

    #[test]
    fn complete_subcommand_test() {
        let input = BashCompletionInput::from("cargo tes");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("test", completions[0]);
    }

    #[test]
    fn complete_test_option_lib() {
        let input = BashCompletionInput::from("cargo test --li");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("--lib", completions[0]);
    }
}
