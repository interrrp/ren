/// Compute the [edit distance](https://en.wikipedia.org/wiki/Edit_distance) between two strings.
///
/// This function implements the
/// [Levenshtein distance algorithm](https://en.wikipedia.org/wiki/Levenshtein_distance).
///
/// # Arguments
///
/// * `a` - The first string to compare.
/// * `b` - The second string to compare.
///
/// # Returns
///
/// A floating point number between 0 and 1 that represents the difference between the two strings.
///
/// # Examples
///
/// ```
/// use ren::edit_distance;
///
/// let distance = edit_distance("kitten", "sitting");
/// assert_eq!(distance, 0.57142854); // 57.14% similarity
/// ```
pub fn edit_distance(a: &str, b: &str) -> f32 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let a_len = a_chars.len();
    let b_len = b_chars.len();

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }

    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for (i, a_char) in a_chars.iter().enumerate() {
        for (j, b_char) in b_chars.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };

            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    let distance = matrix[a_len][b_len] as f32;
    let max_len = a_len.max(b_len) as f32;

    1.0 - distance / max_len
}
