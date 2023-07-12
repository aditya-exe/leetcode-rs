use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: i32,
    nums: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut pq = BinaryHeap::with_capacity(k as usize);

        for x in nums {
            pq.push(Reverse(x));
            if pq.len() > k as usize {
                pq.pop();
            }
        }

        Self {
            k,
            nums: pq,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));
        if self.nums.len() > self.k as usize {
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}