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
    
    if input.previous_word() == "run" || unary_options.contains(&input.previous_word()) {
        let all_options = unary_options.into_iter().chain(other_options);
        input.complete_subcommand(all_options)
    } else {
        vec![]
    }
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

    #[test]
    fn complete_run_option_bin_requires_name() {
        let input = BashCompletionInput::from("cargo run --bin ");
        let completions = complete(input);

        // for now, test that this doesn't return the full list of subcommands
        // eventually this could return the list of binary targets in the crate
        assert_eq!(0, completions.len());
    }
}
