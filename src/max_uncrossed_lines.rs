struct Soln {}

impl Soln {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![-1; nums2.len()]; nums1.len()];

        return Self::go(&0, &0, &nums1, &nums2, &mut dp);
    }
    fn go(i: &usize, j: &usize, nums1: &Vec<i32>, nums2: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
        if *i == nums1.len() || *j == nums2.len() {
            return 0;
        }

        if dp[*i][*j] != -1 {
            return dp[*i][*j];
        }

        let mut cnt = 0;

        if nums1[*i] == nums2[*j] {
            cnt = 1 + Self::go(&(i + 1), &(j + 1), nums1, nums2, dp);
        } else {
            cnt += std::cmp::max(Self::go(&(i + 1), &j, nums1, nums2, dp), Self::go(&i, &(j + 1), nums1, nums2, dp));
        }
        dp[*i][*j] = cnt;
        return dp[*i][*j];
    }
}

