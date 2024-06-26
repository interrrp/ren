use levenshtein::levenshtein as edit_distance;
use rayon::prelude::*;

use crate::wordlist::Wordlist;

/// A suggestion for a misspelled word.
///
/// This struct contains the suggested word and the language of the wordlist it
/// was found in.
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub word: String,
    pub lang: String,
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
pub fn get_suggestions(misspelled_word: &str, wordlists: &[Wordlist]) -> Vec<Suggestion> {
    let mut suggestions: Vec<Suggestion> = wordlists
        .par_iter()
        .flat_map(|wordlist| {
            wordlist
                .words
                .par_iter()
                .filter(|word| {
                    // Only consider words with a length difference of 5 or less
                    // to reduce the number of comparisons
                    let len_diff = misspelled_word.len().abs_diff(word.len());
                    len_diff <= 5
                })
                .map(|word| Suggestion {
                    word: word.clone(),
                    lang: wordlist.lang.clone(),
                })
                .collect::<Vec<Suggestion>>()
        })
        .collect();

    suggestions.par_sort_by(|a, b| {
        let a_distance = edit_distance(misspelled_word, &a.word);
        let b_distance = edit_distance(misspelled_word, &b.word);

        a_distance.cmp(&b_distance)
    });

    suggestions
}

/// Get the best suggestion for a misspelled word.
///
/// This function is a convenience wrapper around `get_suggestions` that returns
/// the first suggestion from the list of suggestions.
///
/// # Arguments
///
/// * `misspelled_word` - A string slice that holds the misspelled word.
/// * `wordlists` - A vector of `Wordlist`s that hold the words to compare against.
///
/// # Returns
///
/// An `Option` that holds the best suggestion for the misspelled word.
#[allow(clippy::module_name_repetitions)]
pub fn get_best_suggestion(misspelled_word: &str, wordlists: &[Wordlist]) -> Option<Suggestion> {
    get_suggestions(misspelled_word, wordlists).first().cloned()
}
