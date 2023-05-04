use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    return senate.chars().collect::<DotaSenate>().get_winner();
}

#[derive(Default)]
struct DotaSenate {
    vote_queue: VecDeque<char>,
    ban_queue: VecDeque<char>,
    radiant: usize,
    dire: usize,
}

impl DotaSenate {
    pub fn add(&mut self, voter: char) {
        match voter {
            'R' => {
                self.vote_queue.push_back('R');
                self.radiant += 1;
            }
            'D' => {
                self.vote_queue.push_back('D');
                self.dire += 1;
            }
            _ => unreachable!(),
        }
    }

    pub fn get_winner(&mut self) -> String {
        self.find_map(|maybe_winner| maybe_winner).unwrap()
    }
}

impl FromIterator<char> for DotaSenate {
    fn from_iter<T: IntoIterator<Item=char>>(iter: T) -> Self {
        let mut dota_senate = DotaSenate::default();
        iter.into_iter().for_each(|ch| dota_senate.add(ch));
        dota_senate
    }
}

impl Iterator for DotaSenate {
    type Item = Option<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let voter = self.vote_queue.pop_front()?;

        if Some(&voter) == self.ban_queue.front() {
            self.ban_queue.pop_front();
            match voter {
                'R' => self.radiant -= 1,
                'D' => self.dire -= 1,
                _ => {}
            }
            return Some(None);
        }

        if voter == 'R' && self.dire == 0 {
            Some(Some("Radiant".to_owned()))
        } else if voter == 'D' && self.radiant == 0 {
            Some(Some("Dire".to_owned()))
        } else {
            match voter {
                'R' => {
                    self.vote_queue.push_back('R');
                    self.ban_queue.push_back('D');
                }
                'D' => {
                    self.vote_queue.push_back('D');
                    self.ban_queue.push_back('R');
                }
                _ => {}
            }
            Some(None)
        }
    }
}

// pub fn predict_party_victory(senate: String) -> String {
//     let mut dire = VecDeque::<usize>::new();
//     let mut radiant = VecDeque::<usize>::new();
//
//     senate.chars().enumerate().for_each(|(i, c)| {
//         match c {
//             'D' => dire.push_back(i),
//             'R' => radiant.push_back(i),
//             _ => unreachable!()
//         };
//     });
//
//     while !dire.is_empty() && !radiant.is_empty() {
//         let d = dire.pop_front().unwrap();
//         let r = radiant.pop_front().unwrap();
//
//         if d < r {
//             dire.push_back(d + senate.len());
//         } else {
//             radiant.push_back(r + senate.len());
//         }
//     }
//
//     return if radiant.is_empty() {
//         "Dire".to_string()
//     } else {
//         "Radiant".to_string()
//     }
// }