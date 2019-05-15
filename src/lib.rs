/// CompletionInput is a struct which contains all the input data passed from the shell into a
/// completion script. Data within this struct should be used by a completion script to determine
/// appropriate completion options.
pub struct CompletionInput<'a> {
    /// Argument 1 - the name of the command whose arguments are being completed
    pub command: &'a str,
    /// Argument 2 - the word under the users cursor when they pressed tab to ask for completions
    pub current_word: &'a str,
    /// Argument 3 - the word preceding the word under the users cursor
    pub preceding_word: &'a str,
    /// $COMP_LINE - the full text that the user has entered
    pub line: &'a str,
    /// $COMP_POINT - the cursor position (a numeric index into `line`)
    pub cursor_position: u32,
}
// todo write impl for creating an instance of this struct from the env
// what should we call this method? should it return an error or simply fail?

// todo how to test this

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
