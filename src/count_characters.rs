pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let get_freq = |s: String| {
        let mut freq = vec![0; 27];
        s.bytes().map(|c| (c - b'a') as usize).for_each(|c| {
            freq[c] += 1;
            freq[26] += 1;
        });
        freq
    };

    let available_chars = get_freq(chars);
    let is_good = |s: &Vec<i32>| {
        available_chars
            .iter()
            .zip(s.into_iter())
            .all(|(a, b)| a >= b)
    };
    let get_len = |s: Vec<i32>| s[26];

    words
        .into_iter()
        .map(get_freq)
        .filter(is_good)
        .map(get_len)
        .sum()
}
