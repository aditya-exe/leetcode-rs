use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32);

pub fn is_path_crossing(path: String) -> bool {
    let mut curr = Point(0, 0);
    let mut set = HashSet::<Point>::new();
    let mut ans = false;

    set.insert(curr.clone());

    path.chars().for_each(|direction| {
        match direction {
            'N' => {
                curr.1 += 1;
            }
            'S' => {
                curr.1 -= 1;
            }
            'E' => {
                curr.0 += 1;
            }
            'W' => {
                curr.0 -= 1;
            }
            _ => unreachable!(),
        }

        if set.contains(&curr) {
            ans = true;
            return;
        }
        set.insert(curr.clone());
    });

    ans
}
