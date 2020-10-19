use strsim::normalized_levenshtein;

pub fn clean_string<S: Into<String>>(s: S) -> String {
    let mut s: String = s.into();

    s = s.to_lowercase();

    s = s.replace("<i>", "");
    s = s.replace("</i>", "");
    s = s.replace("</i>", "");

    s = s.trim_start_matches("a ").to_string();

    // Character removals

    s.chars().filter(|c| c.is_alphanumeric()).collect()
}

/// Returns (bool, f64), bool true if exact match (after string cleaned)
/// f64 is normalized levenshtein distance for close guesses
pub fn check_answer(guess: &str, answer: &str) -> (bool, f64) {
    let guess_cleaned = clean_string(guess);
    let answer_cleaned = clean_string(answer);

    let dist = normalized_levenshtein(&guess_cleaned, &answer_cleaned);

    (guess_cleaned == answer_cleaned, dist)
}
