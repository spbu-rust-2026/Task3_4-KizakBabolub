use std::collections::HashMap;

pub fn word_frequencies(input: &str) -> std::collections::HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in input.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    counts
}
