struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let count = height.len();
        let mut start = 0;
        let mut end = count - 1;
        let mut max = 0;
        while end > start {
            let tmp_max = height[start].min(height[end]) * (end - start) as i32;

            if tmp_max > max {
                max = tmp_max;
            }
            if height[start] > height[end] {
                end -= 1;
            } else {
                start += 1;
            }
        }
        max
    }
}

#[test]
fn check() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
