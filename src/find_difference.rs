use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let s1: HashSet<i32> = HashSet::from_iter(nums1);
    let s2: HashSet<i32> = HashSet::from_iter(nums2);

    let a = s1.difference(&s2).copied().collect::<Vec<i32>>();
    let b = s2.difference(&s1).copied().collect::<Vec<i32>>();

    return vec![a, b];
}