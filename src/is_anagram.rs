use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut ok = true;

    if s.len() != t.len() {
        return false;
    }

    let mut map = HashMap::new();

    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

    t.chars().for_each(|c| {
        map.entry(c).and_modify(|f| *f -= 1);
    });

    map.iter().for_each(|it| {
        if it.1 != &0 {
            ok = false;
        }
    });

    ok
}
