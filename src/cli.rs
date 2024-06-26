use clap::Parser;

use ren::wordlist::{load_wordlists, Wordlist};

/// A tiny spell checker.
#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// The glob pattern to search for files to spell check.
    #[arg(default_value = "**/*.md")]
    pub pattern: String,

    /// The language codes of the wordlists to use for spell checking.
    #[arg(short, long, default_value = "en_us")]
    pub langs: Vec<String>,
}

impl Args {
    /// Get a list of files that match the glob from `pattern`.
    ///
    /// # Returns
    ///
    /// A `Paths` iterator that holds the matched files.
    pub fn get_matched_files(&self) -> glob::Paths {
        glob::glob(&self.pattern).expect("Invalid glob pattern")
    }

    /// Get the wordlists for the languages from `langs`.
    ///
    /// # Returns
    ///
    /// A vector of `Wordlist`s that hold the words for spell checking.
    pub fn get_wordlists(&self) -> Vec<Wordlist> {
        load_wordlists(&self.langs)
    }
}
