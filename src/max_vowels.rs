use std::collections::VecDeque;

pub fn max_vowels(s: String, k: i32) -> i32 {
    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }

    let mut substring = VecDeque::<char>::new();
    s.chars().take(k as usize).for_each(|c| substring.push_back(c));
    let mut mx = substring.iter().filter(|&c| is_vowel(*c)).count();

    let mut cur_count = mx;

    s.chars().skip(k as usize).for_each(|c| {
        let front = substring.pop_front().unwrap();
        if is_vowel(front) {
            cur_count -= 1;
        }
        substring.push_back(c);
        if is_vowel(c) {
            cur_count += 1;
        }
        mx = std::cmp::max(mx, cur_count);
    });

    return mx as i32;
}