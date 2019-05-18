
use shell_completion::CompletionInput;

fn main() {
    let completions = CompletionInput::from_args()
        .expect("Missing expected arguments and/or environment variables");

    completions.print_subcommand_completions(vec!["add", "commit"]);
}
