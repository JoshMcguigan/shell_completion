use shell_completion::{BashCompletionInput, CompletionInput, CompletionSet};

fn main() {
    let input = BashCompletionInput::from_args()
        .expect("Missing expected arguments and/or environment variables");

    complete(input).suggest();
}

fn complete(input: impl CompletionInput) -> Vec<String> {
    match input.arg_index() {
        0 => unreachable!(),
        1 => input.complete_subcommand(vec!["run", "test"]), // todo also include cargo-subcommands on path
        _ => {
            match input.args()[1] {
                "run" => complete_run(input),
                _ => vec![],
            }
        },
    }
}

fn complete_run(input: impl CompletionInput) -> Vec<String> {
    let run_options = vec![
        "--bin",
        "--example",
        "--package",
        "--jobs",
        "--release",
        "--features",
        "--all-features",
        "--no-default-features",
        "--target",
        "--target-dir",
        "--manifest-path",
        "--message-format",
        "--verbose",
        "--quiet",
        "--color",
        "--frozen",
        "--locked",
        "--help",
    ];
    input.complete_subcommand(run_options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_subcommand_run() {
        let input = BashCompletionInput::from("cargo ru");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("run", completions[0]);
    }

    #[test]
    fn complete_run_option_bin() {
        let input = BashCompletionInput::from("cargo run --bi");
        let completions = complete(input);

        assert_eq!(1, completions.len());
        assert_eq!("--bin", completions[0]);
    }
}
