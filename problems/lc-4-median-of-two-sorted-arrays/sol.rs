pub enum Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let (m, n) = (a.len(), b.len());
        let half = (m + n + 1) / 2;
        let (mut low, mut high) = (0, m);

        while low <= high {
            let i = (low + high) / 2;
            let j = half - i;

            let max_left_a = if i > 0 { a[i - 1] } else { i32::MIN };
            let min_right_a = if i < m { a[i] } else { i32::MAX };

            let max_left_b = if j > 0 { b[j - 1] } else { i32::MIN };
            let min_right_b = if j < n { b[j] } else { i32::MAX };

            if max_left_a > min_right_b {
                high = i - 1;
            } else if max_left_b > min_right_a {
                low = i + 1;
            } else {
                let max_left = max_left_a.max(max_left_b) as f64;
                return if (m + n) & 1 == 1 {
                    max_left
                } else {
                    (max_left + min_right_a.min(min_right_b) as f64) / 2.0
                };
            }
        }
        unreachable!()
    }
}

fn main() {
    let nums1: Vec<i32> = vec![1, 3];
    let nums2: Vec<i32> = vec![2];
    println!("{}", Solution::find_median_sorted_arrays(nums1, nums2));
    return;
}
