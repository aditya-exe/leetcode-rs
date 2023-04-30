pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mp = std::collections::HashMap::<i32, i32>::new();

    nums.iter().enumerate().for_each(|(i, x)| {
        mp.insert(x.to_owned(), i as i32);
    });

    let mut ans = vec![-1; 2];

    (0..nums.len()).for_each(|i| {
        let i = i as i32;
       let x = target - nums[i as usize];
        if let Some(num) = mp.get(&x) {
            if num != &i {
                ans[0] = i;
                ans[1] = num.to_owned();
                return;
            }
        }
    });

    return ans;
}