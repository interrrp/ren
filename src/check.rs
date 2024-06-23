use crate::{iter::get_words_from_file, wordlist::Wordlist};
use levenshtein::levenshtein as edit_distance;
use std::path::Path;

#[derive(Debug)]
pub struct Suggestion {
    word: String,
    lang: String,
}

/// Check a file for misspelled words. This is meant to be called from the
/// command line interface.
///
/// This function reads the file at the given path and compares the words in the
/// file against the words in the wordlists. If a word is not found in any of
/// the wordlists, the function prints an error message with the line number,
/// column number, and the best suggestion for the misspelled word.
///
/// # Arguments
///
/// * `path` - A `std::path::Path` that holds the path to the file to check.
/// * `wordlists` - A vector of `Wordlist`s that hold the words to compare against.
pub fn spellcheck(path: &Path, wordlists: &Vec<Wordlist>) {
    for word_with_pos in get_words_from_file(path) {
        let word = word_with_pos.word.to_lowercase();

        if wordlists.iter().all(|wordlist| !wordlist.contains(&word)) {
            let suggestions = get_suggestions(&word, wordlists);
            let best_suggestion = suggestions.first();

            eprintln!(
                "{}:{}:{} - error: '{}' is misspelled. {}",
                path.display(),
                word_with_pos.line + 1,
                word_with_pos.start_column + 1,
                word_with_pos.word,
                match best_suggestion {
                    Some(suggestion) =>
                        format!("Did you mean '{}' ({})?", suggestion.word, suggestion.lang),
                    None => "No suggestions found.".to_string(),
                }
            );
        }
    }
}

/// Get suggestions for a misspelled word.
///
/// The suggestions are based on the edit distance between the misspelled word
/// and the words in the wordlists, meaning the best suggestion is the first
/// element in the returned vector.
///
/// # Arguments
///
/// * `misspelled_word` - A string slice that holds the misspelled word.
/// * `wordlists` - A vector of `Wordlist`s that hold the words to compare against.
///
/// # Returns
///
/// A vector of `Suggestions` that hold the suggestions for the misspelled word
/// sorted by edit distance.
fn get_suggestions(misspelled_word: &str, wordlists: &Vec<Wordlist>) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    for wordlist in wordlists {
        for word in &wordlist.words {
            // If the difference in length is greater than 5, we can assume
            // the words are not similar enough to be suggestions.
            let len_diff = (misspelled_word.len() as i32 - word.len() as i32).abs();
            if len_diff > 5 {
                continue;
            }

            suggestions.push(Suggestion {
                word: word.clone(),
                lang: wordlist.lang.clone(),
            });
        }
    }

    suggestions.sort_by(|a, b| {
        let a_distance = edit_distance(misspelled_word, &a.word);
        let b_distance = edit_distance(misspelled_word, &b.word);

        b_distance.partial_cmp(&a_distance).unwrap()
    });

    suggestions
}
