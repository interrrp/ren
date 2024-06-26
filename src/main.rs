use std::path::Path;

use clap::Parser;
use owo_colors::OwoColorize;
use rayon::prelude::*;

use ren::{
    iter::{get_words_from_file, PositionedWord},
    suggestion::{get_best_suggestion, Suggestion},
    wordlist::Wordlist,
};

mod cli;

fn main() {
    let args = cli::Args::parse();

    let wordlists = args.get_wordlists();

    args.get_matched_files().par_bridge().for_each(|file| {
        let file = file.expect("Invalid file path");
        spellcheck(&file, &wordlists);
    });
}

fn spellcheck(path: &Path, wordlists: &[Wordlist]) {
    for word_with_pos in get_words_from_file(path) {
        let word = word_with_pos.word.to_lowercase();

        if spelled_correctly(&word, wordlists) {
            continue;
        }

        let best_suggestion = get_best_suggestion(&word, wordlists);
        notify_misspelled_word(path, &word_with_pos, best_suggestion);
    }
}

fn notify_misspelled_word(
    path: &Path,
    word_with_pos: &PositionedWord,
    suggestion: Option<Suggestion>,
) {
    let location = format!(
        "{}:{}:{}",
        path.display(),
        word_with_pos.line + 1,
        word_with_pos.start_column + 1,
    )
    .bright_black()
    .to_string();

    let misspelled_word = word_with_pos.word.bright_red();

    let suggestion = match suggestion {
        Some(suggestion) => format!(
            "Did you mean {} {}?",
            suggestion.word.bright_green(),
            format!("({})", suggestion.lang).bright_black(),
        ),
        None => "No suggestions found".bright_black().to_string(),
    };

    println!("{location}: {misspelled_word} is misspelled. {suggestion}");
}

fn spelled_correctly(word: &str, wordlists: &[Wordlist]) -> bool {
    wordlists.iter().any(|wordlist| wordlist.contains(word))
}
