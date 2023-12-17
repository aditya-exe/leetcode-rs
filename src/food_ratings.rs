use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type HQ = HashMap<String, BinaryHeap<(i32, Reverse<String>, usize)>>;

struct FoodRatings {
    foods: Vec<String>,
    cuisines: Vec<String>,
    ratings: Vec<i32>,
    ff: HashMap<String, usize>,
    hq: HQ,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut hq: HQ = HashMap::new();

        cuisines.iter().cloned().enumerate().for_each(|(i, c)| {
            hq.entry(c)
                .or_default()
                .push((ratings[i], Reverse(foods[i].clone()), i))
        });

        let ff = foods
            .iter()
            .enumerate()
            .map(|(i, f)| (format!("{}", f), i))
            .collect::<HashMap<String, usize>>();

        Self {
            foods,
            cuisines,
            ratings,
            ff,
            hq,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(&i) = self.ff.get(&food) {
            self.ratings[i] = new_rating;
            self.hq
                .get_mut(&self.cuisines[i])
                .unwrap()
                .push((new_rating, Reverse(food), i));
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        /*
        if let Some(i) = (0..self.foods.len())
                .filter(|&i|self.cuisines[i]==cuisine).max_by_key(|i|(self.ratings[*i], Reverse(self.foods[*i].clone())))  {
            return self.foods[i].clone()    
        };
        panic!("unkown cuisine")
        */
        
        while let Some((r,Reverse(food),i)) = self.hq.get(&cuisine).unwrap().peek() {
            if *r == self.ratings[*i] {
                return food.clone()
            } else {self.hq.get_mut(&cuisine).unwrap().pop();}
        }
        unreachable!();
    }
}
