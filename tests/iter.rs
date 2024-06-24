use ren::iter::get_words_from_str;

#[test]
fn test_get_words_from_str() {
    let words = get_words_from_str("Hello, world!");

    assert_eq!(words.len(), 2);
    assert_eq!(words[0].word, "hello");
    assert_eq!(words[0].line, 0);
    assert_eq!(words[0].start_column, 0);

    assert_eq!(words[1].word, "world");
    assert_eq!(words[1].line, 0);
    assert_eq!(words[1].start_column, 7);
}

#[test]
fn test_filtering() {
    let words = get_words_from_str("Hello, world! https://google.com");

    assert_eq!(words.len(), 2);
    assert_eq!(words[0].word, "hello");
}
