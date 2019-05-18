# shell_completion

Shell completions, which provide auto-complete for CLI applications, are typically written in Bash. This crate provides low level primitives for writing shell completion scripts in Rust. 

## Usage

Shell completion scripts are written as normal Rust binaries. A minimal example is below:

```rust
use shell_completion::CompletionInput;

fn main() {
    let completions = CompletionInput::from_args()
        .expect("Missing expected arguments and/or environment variables");

    completions.print_subcommand_completions(vec!["add", "commit"]);
}
```

To try it out, run `cargo install --force --path . && complete -C __democli_shell_completion democli` after cloning this repository. Then type `democli <TAB>` in the same shell. Cargo install installed two binaries, one called `democli` and the other `__democli_shell_completion`. The `complete` command registered our shell completion script for `democli`. Note that `complete` commands do not persist (they are only active in the shell where you run `complete`), so if you want to use a completion long term you'll want to add the `complete` command to your `~/.bash_profile`.

## Users

This crate is not quite ready for production use, but if you are an early adopter, feel free to make a PR adding yourself to the list below. 

* N/A

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

All forms of contribution are valued. I've created issues for many high level topics of exploration, and any feedback there will be very helpful in moving this crate in the right direction. Of course, code contributions are appreciated as well. 

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
