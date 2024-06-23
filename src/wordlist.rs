use std::fs;

#[derive(Debug)]
pub struct Wordlist {
    pub lang: String,
    pub words: Vec<String>,
}

impl Wordlist {
    /// Check if a wordlist contains a word.
    ///
    /// # Arguments
    ///
    /// * `word` - A string slice that holds the word to check for.
    ///
    /// # Returns
    ///
    /// A boolean that indicates if the wordlist contains the word.
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }
}

/// Load all wordlists by their language codes.
///
/// # Arguments
///
/// * `langs` - A vector of string slices that hold the language codes of the wordlists to load.
///
/// # Returns
///
/// A vector of `Wordlist`s that hold the words from the wordlists.
pub fn load_wordlists(langs: Vec<String>) -> Vec<Wordlist> {
    langs.iter().map(|lang| load_wordlist(lang)).collect()
}

/// Load a wordlist by its language code.
///
/// The path to the wordlist is determined by the `lang` argument, e.g. `en_us`
/// will load the wordlist from `./wordlists/en_us.txt`. The wordlist is a text
/// file with one word per line, excluding empty lines and comments (lines that
/// start with `//`).
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code of the wordlist to load.
///
/// # Returns
///
/// A `Wordlist` that holds the words from the wordlist.
pub fn load_wordlist(lang: &str) -> Wordlist {
    let path = format!("./wordlists/{}.txt", lang);

    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not read wordlist file"),
    };

    let words = file
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| line.to_string())
        .collect();

    Wordlist {
        lang: lang.to_string(),
        words,
    }
}
