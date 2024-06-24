use ren::{suggestion::get_suggestions, wordlist::Wordlist};

#[test]
fn best_suggestion() {
    let wordlists = vec![
        Wordlist {
            lang: "en".to_string(),
            words: vec!["hello".to_string(), "world".to_string()],
        },
        Wordlist {
            lang: "es".to_string(),
            words: vec!["hola".to_string(), "mundo".to_string()],
        },
    ];

    let suggestions = get_suggestions("hewwo", &wordlists);
    let best_suggestion = suggestions.first().unwrap();
    assert_eq!(best_suggestion.word, "hello");
    assert_eq!(best_suggestion.lang, "en");
}
