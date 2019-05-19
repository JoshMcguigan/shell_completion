use shell_completion::BashCompletionInput;

fn main() {
    let completions = BashCompletionInput::from_args()
        .expect("Missing expected arguments and/or environment variables");

    completions.print_subcommand_completions(vec!["add", "commit"]);
}
