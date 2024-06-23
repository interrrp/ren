use wordlist::load_all_wordlists;

mod wordlist;

fn main() {
    let wordlists = load_all_wordlists();

    for wordlist in wordlists {
        println!("{}: {} words", wordlist.lang, wordlist.words.len());
    }
}
