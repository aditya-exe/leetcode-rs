use std::cmp::Ordering;

pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut neg = 0;
    let mut zero = 0;

    nums.iter().for_each(|&x| match x.cmp(&0) {
        Ordering::Greater => pos += 1,
        Ordering::Less => neg += 1,
        Ordering::Equal => {
            zero += 1;
            return;
        }
    });

    return if zero > 0 {
        0
    } else {
        if neg % 2 == 0 {
            1
        } else {
            -1
        }
    }
}