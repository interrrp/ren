use std::{fs::read_to_string, path::Path};

/// A struct that holds a word, line number, and column number.
#[derive(Debug)]
pub struct PositionedWord {
    pub word: String,
    pub line: usize,
    pub start_column: usize,
}

/// Get a list of words from a string.
///
/// # Arguments
///
/// * `s` - A string slice to read from.
///
/// # Returns
///
/// A vector of `PositionedWord`s that hold the words from the string.
///
/// # Example
///
/// ```
/// use ren::get_words_from_str;
///
/// let words = get_words_from_str("Hello, world!");
/// assert_eq!(words.len(), 2);
///
/// assert_eq!(words[0].word, "Hello");
/// assert_eq!(words[0].line, 0);
/// assert_eq!(words[0].start_column, 0);
/// ```
pub fn get_words_from_str(s: &str) -> Vec<PositionedWord> {
    let mut words = Vec::new();
    let mut column = 0;

    for (i, line_str) in s.lines().enumerate() {
        let line = i;

        for word in line_str.split_whitespace() {
            let word = remove_punctuation(word);

            let start_column = column;
            let end_column = start_column + word.len();

            if word.is_empty() {
                // Skip words that are made up of only punctuation
                column = end_column + 1;
                continue;
            }

            words.push(PositionedWord {
                word: word.to_string(),
                line,
                start_column,
            });

            column = end_column + 1;
        }
    }

    words
}

/// Get a list of words from a file.
///
/// See `get_words_from_str` for more information.
///
/// # Arguments
///
/// * `path` - A `std::path::Path` that holds the path to the file to read from.
///
/// # Returns
///
/// A vector of `PositionedWord`s that hold the words from the file.
pub fn get_words_from_file(path: &Path) -> Vec<PositionedWord> {
    let content = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => panic!("Could not read file"),
    };

    get_words_from_str(&content)
}

fn remove_punctuation(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}
