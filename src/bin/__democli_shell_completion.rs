use shell_completion::{BashCompletionInput, CompletionInput};

fn main() {
    let completions = BashCompletionInput::from_args()
        .expect("Missing expected arguments and/or environment variables");

    completions.complete_subcommand(vec!["add", "commit"]);
}
