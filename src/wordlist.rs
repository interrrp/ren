use std::fs;

#[derive(Debug)]
pub struct Wordlist {
    pub lang: String,
    pub words: Vec<String>,
}

impl Wordlist {
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }
}

pub fn load_multiple(langs: &[String]) -> Vec<Wordlist> {
    langs.iter().map(|lang| load(lang)).collect()
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
///
/// # Panics
///
/// This function will panic if the wordlist file cannot be read.
pub fn load(lang: &str) -> Wordlist {
    let path = format!("./wordlists/{lang}.txt");

    let file = fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not read wordlist file"));

    let words = file
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(str::to_string)
        .collect();

    Wordlist {
        lang: lang.to_string(),
        words,
    }
}
