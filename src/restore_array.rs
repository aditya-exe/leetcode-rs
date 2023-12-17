use std::collections::HashMap;

pub fn restore_array(a: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut ans = Vec::new();

    a.iter().for_each(|x| {
        map.entry(x[0]).or_insert(vec![]).push(x[1]);
        map.entry(x[1]).or_insert(vec![]).push(x[0]);
    });

    let mut curr = *map.iter().find(|(_, v)| v.len() == 1).unwrap().0;

    while let Some((_, nexts)) = map.remove_entry(&curr) {
        ans.push(curr);

        if let Some(next) = nexts.iter().find(|x| map.contains_key(&x)) {
            curr = *next;
        }
    }

    return ans;
}
