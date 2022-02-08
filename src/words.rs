use std::fs;

pub fn daily_words() -> Vec<[char; 5]> {
    let raw_words = fs::read_to_string("./src/daily_words.txt").unwrap();

    return raw_words
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("Something went wrong when creating words.")
        })
        .collect();
}

pub fn all_words() -> Vec<[char; 5]> {
    let raw_words = fs::read_to_string("./src/allowed_guesses.txt").unwrap();

    return raw_words
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("Something went wrong when creating words.")
        })
        .chain(daily_words().into_iter())
        .collect();
}
