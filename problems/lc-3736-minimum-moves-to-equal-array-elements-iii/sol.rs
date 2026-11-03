// Naive
impl Solution {
    fn find_max(nums: &[i32]) -> i32 {
        let mut maxi = i32::MIN;
        for &num in nums {
            if num > maxi {
                maxi = num;
            }
        }
        maxi
    }

    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let maxi = Solution::find_max(&nums);
        let mut moves = 0;
        for num in nums {
            moves += maxi - num;
        }
        moves
    }
}

// Optimized
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (sum, max) = nums
            .iter()
            .fold((0, i32::MIN), |(s, m), &n| (s + n, m.max(n)));
        (max * nums.len() as i32) - sum
    }
}
