use clap::Parser;
use glob::glob;
use wordlist::load_wordlists;

mod cli;
mod wordlist;

fn main() {
    let args = cli::Args::parse();

    let wordlists = load_wordlists(args.langs);
    for wordlist in wordlists {
        println!("{}: {} words", wordlist.lang, wordlist.words.len());
    }

    let files = glob(&args.pattern).expect("Invalid glob pattern");
    for file in files {
        let file = file.expect("Invalid file path");
        println!("Spell checking file: {}", file.display());
    }
}
