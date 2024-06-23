use check::spellcheck;
use clap::Parser;
use glob::glob;
use wordlist::load_wordlists;

mod check;
mod cli;
mod iter;
mod wordlist;

fn main() {
    let args = cli::Args::parse();

    let wordlists = load_wordlists(args.langs);

    let files = glob(&args.pattern).expect("Invalid glob pattern");
    for file in files {
        let file = file.expect("Invalid file path");
        spellcheck(&file, &wordlists);
    }
}
