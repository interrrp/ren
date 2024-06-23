use std::fs;

pub struct Wordlist {
    pub lang: String,
    pub words: Vec<String>,
}

/// Load all wordlists from the `./wordlists` directory.
///
/// See `load_wordlist` for more information on how wordlists are loaded.
pub fn load_all_wordlists() -> Vec<Wordlist> {
    fs::read_dir("./wordlists")
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let lang = path.file_stem().unwrap().to_str().unwrap();
            load_wordlist(lang)
        })
        .collect()
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
