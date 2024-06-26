use clap::Parser;

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
