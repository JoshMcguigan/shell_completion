/// Unix shell word splitter
/// Derived from
/// https://github.com/AaronErhardt/shell-words/blob/1f0def71072a2be7b1105ee46b989bbb92762372/src/lib.rs#L46-L231
/// https://github.com/tmiasko/shell-words/blob/045e4dccd2478ccc8bfa91bd0fe449dfe5473496/src/lib.rs#L47-L229
use std::{fmt, mem};

/// An error returned when shell parsing fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitError {
    UnfinishedComment,
}

impl fmt::Display for SplitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SplitError::UnfinishedComment => f.write_str("missing closing quote"),
        }
    }
}

impl std::error::Error for SplitError {}

enum State {
    /// Within a delimiter.
    Delimiter,
    /// After backslash, but before starting word.
    Backslash,
    /// Within an unquoted word.
    Unquoted,
    /// After backslash in an unquoted word.
    UnquotedBackslash,
    /// Within a single quoted word.
    SingleQuoted,
    /// Within a double-quoted word.
    DoubleQuoted,
    /// After backslash inside a double-quoted word.
    DoubleQuotedBackslash,
    /// Inside a comment.
    Comment,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Split {
    pub words: Vec<String>,
    /// Index of the word of the cursor position (a numeric index into `words`)
    pub current_word: usize,
    /// Index of the cursor position inside the `current_word`
    pub current_character: usize,
}

#[derive(Default)]
struct SplitBuilder {
    words: Vec<String>,
    word: String,
    /// Index of current word and current character
    location: Option<(usize, usize)>,
}

impl SplitBuilder {
    fn update_indexes(&mut self, c_index: usize, comp_point: usize) {
        if self.location.is_none() && c_index >= comp_point {
            self.location = Some((self.words.len(), self.word.len()))
        }
    }

    fn push_character(&mut self, c_index: usize, comp_point: usize, c: char) {
        self.update_indexes(c_index, comp_point);
        self.word.push(c);
    }

    fn complete_word(&mut self, c_index: usize, comp_point: usize) {
        self.update_indexes(c_index, comp_point);
        self.words.push(mem::take(&mut self.word))
    }

    fn finish(self, final_state: State) -> Result<Split, SplitError> {
        // We can't complete an unfinished comment
        if self.location.is_none() && matches!(final_state, State::Comment) {
            return Err(SplitError::UnfinishedComment);
        }
        Ok(Split {
            words: self.words,
            current_word: self.location.map(|l| l.0).unwrap_or(0),
            current_character: self.location.map(|l| l.1).unwrap_or(0),
        })
    }
}

impl Split {
    pub fn new(s: &str, comp_point: usize) -> Result<Self, SplitError> {
        use State::*;

        let mut state = Delimiter;
        let mut builder = SplitBuilder::default();

        for (idx, c) in s.chars().enumerate() {
            // Process new character
            state = match state {
                Delimiter => match c {
                    '\'' => SingleQuoted,
                    '\"' => DoubleQuoted,
                    '\\' => Backslash,
                    '\t' | ' ' | '\n' => Delimiter,
                    '#' => Comment,
                    c => {
                        builder.push_character(idx, comp_point, c);
                        Unquoted
                    }
                },
                Backslash => match c {
                    '\n' => Delimiter,
                    c => {
                        builder.push_character(idx, comp_point, c);
                        Unquoted
                    }
                },
                Unquoted => match c {
                    '\'' => SingleQuoted,
                    '\"' => DoubleQuoted,
                    '\\' => UnquotedBackslash,
                    '\t' | ' ' | '\n' => {
                        builder.complete_word(idx, comp_point);
                        Delimiter
                    }
                    c => {
                        builder.push_character(idx, comp_point, c);
                        Unquoted
                    }
                },
                UnquotedBackslash => match c {
                    '\n' => Unquoted,
                    c => {
                        builder.push_character(idx, comp_point, c);
                        Unquoted
                    }
                },
                SingleQuoted => match c {
                    '\'' => Unquoted,
                    c => {
                        builder.push_character(idx, comp_point, c);
                        SingleQuoted
                    }
                },
                DoubleQuoted => match c {
                    '\"' => Unquoted,
                    '\\' => DoubleQuotedBackslash,
                    c => {
                        builder.push_character(idx, comp_point, c);
                        DoubleQuoted
                    }
                },
                DoubleQuotedBackslash => match c {
                    '\n' => DoubleQuoted,
                    '$' | '`' | '"' | '\\' => {
                        builder.push_character(idx, comp_point, c);
                        DoubleQuoted
                    }
                    c => {
                        builder.push_character(idx, comp_point, '\\');
                        builder.push_character(idx, comp_point, c);
                        DoubleQuoted
                    }
                },
                Comment => match c {
                    '\n' => Delimiter,
                    _ => Comment,
                },
            }
        }

        // Process end of input
        match state {
            Comment => {}
            Backslash | UnquotedBackslash => {
                builder.push_character(s.len(), comp_point, '\\');
                builder.complete_word(s.len(), comp_point);
            }
            _ => {
                builder.complete_word(s.len(), comp_point);
            }
        }

        builder.finish(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        input: String,
        comp_point: usize,
        expected: Result<Split, SplitError>,
    }

    impl TestCase {
        // Character `|` is used to indicate where the cursor is
        fn get_comp_point(input: &str) -> usize {
            assert_eq!(
                input.chars().filter(|c| *c == '|').count(),
                1,
                "Input must contain one cursor character '|'"
            );
            input.find('|').unwrap()
        }

        fn at_start(input: &str, expected: &[&str]) -> Self {
            Self {
                input: input.to_string(),
                comp_point: 0,
                expected: Ok(Split {
                    words: expected.iter().map(|e| e.to_string()).collect(),
                    current_word: 0,
                    current_character: 0,
                }),
            }
        }

        fn at_cursor(
            input: &str,
            expected: &[&str],
            current_word: usize,
            current_character: usize,
        ) -> Self {
            Self {
                input: input.replace('|', ""),
                comp_point: Self::get_comp_point(input),
                expected: Ok(Split {
                    words: expected.iter().map(|e| e.to_string()).collect(),
                    current_word,
                    current_character,
                }),
            }
        }

        fn error_at_cursor(input: &str, expected: SplitError) -> Self {
            Self {
                input: input.replace('|', ""),
                comp_point: Self::get_comp_point(input),
                expected: Err(expected),
            }
        }
    }

    fn assert_split(cases: &[TestCase]) {
        for case in cases {
            assert_eq!(Split::new(&case.input, case.comp_point), case.expected);
        }
    }

    #[test]
    fn split_empty() {
        assert_split(&[TestCase::at_start("", &[""])]);
    }

    #[test]
    fn split_initial_whitespace_is_removed() {
        assert_split(&[
            TestCase::at_start("     a", &["a"]),
            TestCase::at_start("\t\t\t\tbar", &["bar"]),
            TestCase::at_start("\t \nc", &["c"]),
        ]);
    }

    #[test]
    fn split_trailing_whitespace_is_preserved() {
        // We should indicate that we are trying to start a new word
        assert_split(&[
            TestCase::at_start("a  ", &["a", ""]),
            TestCase::at_start("b\t", &["b", ""]),
            TestCase::at_start("c\t \n \n \n", &["c", ""]),
            TestCase::at_start("d\n\n", &["d", ""]),
        ]);
    }

    #[test]
    fn split_carriage_return_is_not_special() {
        assert_split(&[TestCase::at_start("c\ra\r'\r'\r", &["c\ra\r\r\r"])]);
    }

    #[test]
    fn split_single_quotes() {
        assert_split(&[
            TestCase::at_start(r#"''"#, &[r#""#]),
            TestCase::at_start(r#"'a'"#, &[r#"a"#]),
            TestCase::at_start(r#"'\'"#, &[r#"\"#]),
            TestCase::at_start(r#"' \ '"#, &[r#" \ "#]),
            TestCase::at_start(r#"'#'"#, &[r#"#"#]),
        ]);
    }

    #[test]
    fn split_double_quotes() {
        assert_split(&[
            TestCase::at_start(r#""""#, &[""]),
            TestCase::at_start(r#""""""#, &[""]),
            TestCase::at_start(r#""a b c' d""#, &["a b c' d"]),
            TestCase::at_start(r#""\a""#, &["\\a"]),
            TestCase::at_start(r#""$""#, &["$"]),
            TestCase::at_start(r#""\$""#, &["$"]),
            TestCase::at_start(r#""`""#, &["`"]),
            TestCase::at_start(r#""\`""#, &["`"]),
            TestCase::at_start(r#""\"""#, &["\""]),
            TestCase::at_start(r#""\\""#, &["\\"]),
            TestCase::at_start("\"\n\"", &["\n"]),
            TestCase::at_start("\"\\\n\"", &[""]),
        ]);
    }

    #[test]
    fn split_unquoted() {
        assert_split(&[
            TestCase::at_start(r#"\|\&\;"#, &[r#"|&;"#]),
            TestCase::at_start(r#"\<\>"#, &[r#"<>"#]),
            TestCase::at_start(r#"\(\)"#, &[r#"()"#]),
            TestCase::at_start(r#"\$"#, &[r#"$"#]),
            TestCase::at_start(r#"\`"#, &[r#"`"#]),
            TestCase::at_start(r#"\""#, &[r#"""#]),
            TestCase::at_start(r#"\'"#, &[r#"'"#]),
            TestCase::at_start("\\\n", &[""]),
            TestCase::at_start(" \\\n \n", &[""]),
            TestCase::at_start("a\nb\nc", &["a", "b", "c"]),
            TestCase::at_start("a\\\nb\\\nc", &["abc"]),
            TestCase::at_start("foo bar baz", &["foo", "bar", "baz"]),
            TestCase::at_start(r#"\🦉"#, &[r"🦉"]),
        ]);
    }

    #[test]
    fn split_trailing_backslash() {
        assert_split(&[
            TestCase::at_start("\\", &["\\"]),
            TestCase::at_start(" \\", &["\\"]),
            TestCase::at_start("a\\", &["a\\"]),
        ]);
    }

    #[test]
    fn split_comments() {
        assert_split(&[
            TestCase::at_start(r#" x # comment "#, &["x"]),
            TestCase::at_start(r#" w1#w2 "#, &["w1#w2", ""]),
            TestCase::at_start(r#"'not really a # comment'"#, &["not really a # comment"]),
            TestCase::at_start(" a # very long comment \n b # another comment", &["a", "b"]),
            TestCase::at_cursor("one t|wo # comment", &["one", "two"], 1, 1),
            TestCase::at_cursor("one # comment \n tw|o", &["one", "two"], 1, 2),
            TestCase::error_at_cursor("command # begin comment|", SplitError::UnfinishedComment),
        ]);
    }

    #[test]
    fn split_with_cursor() {
        assert_split(&[
            // First word
            TestCase::at_cursor("|", &[""], 0, 0),
            TestCase::at_cursor("|one two three", &["one", "two", "three"], 0, 0),
            TestCase::at_cursor("o|ne two three", &["one", "two", "three"], 0, 1),
            TestCase::at_cursor("one| two three", &["one", "two", "three"], 0, 3),
            TestCase::at_cursor("'one'| two three", &["one", "two", "three"], 0, 3),
            // Second word
            TestCase::at_cursor("one |two three", &["one", "two", "three"], 1, 0),
            TestCase::at_cursor("one t|wo three", &["one", "two", "three"], 1, 1),
            TestCase::at_cursor("one two| three", &["one", "two", "three"], 1, 3),
            TestCase::at_cursor("one 'two   |' three", &["one", "two   ", "three"], 1, 6),
            // Third word
            TestCase::at_cursor("one two |", &["one", "two", ""], 2, 0),
            TestCase::at_cursor("one two      |", &["one", "two", ""], 2, 0),
            TestCase::at_cursor("one two |three", &["one", "two", "three"], 2, 0),
            TestCase::at_cursor("one two 'three'|", &["one", "two", "three"], 2, 5),
        ]);
    }

    #[test]
    fn split_incomplete() {
        // Should gracefully handle incomplete statements
        assert_split(&[
            // Double quotes
            TestCase::at_cursor("one \"tw|", &["one", "tw"], 1, 2),
            TestCase::at_cursor("one| \"tw", &["one", "tw"], 0, 3),
            // Single quotes
            TestCase::at_cursor("one 'tw|", &["one", "tw"], 1, 2),
            TestCase::at_cursor("one| 'tw", &["one", "tw"], 0, 3),
        ]);
    }
}
