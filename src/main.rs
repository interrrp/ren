use std::path::Path;

use clap::Parser;
use glob::glob;

use ren::{get_suggestions, get_words_from_file, load_wordlists, Wordlist};

mod cli;

fn main() {
    let args = cli::Args::parse();

    let wordlists = load_wordlists(args.langs);

    let files = glob(&args.pattern).expect("Invalid glob pattern");
    for file in files {
        let file = file.expect("Invalid file path");
        spellcheck(&file, &wordlists);
    }
}

fn spellcheck(path: &Path, wordlists: &[Wordlist]) {
    for word_with_pos in get_words_from_file(path) {
        let word = word_with_pos.word.to_lowercase();

        let spelled_correctly = wordlists.iter().any(|wordlist| wordlist.contains(&word));
        if spelled_correctly {
            continue;
        }

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
