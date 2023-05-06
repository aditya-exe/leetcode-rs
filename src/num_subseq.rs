pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();

    let m = 1_000_000_007;
    let mut ret = 0;

    let fast_pow = |mut n: usize| -> usize {
        let mut ret = 1;
        let mut base = 2;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * base % m;
            }
            n >>= 1;
            base = base * base % m;
        }
        ret
    };


    for i in 0..nums.len() {
        if nums[i] > target / 2 {
            break;
        }

        let target = target - nums[i];
        let mut l = i;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        ret = (ret + fast_pow(l - i - 1)) % m;
    }
    ret as i32
}