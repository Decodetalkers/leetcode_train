struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_min = x < 0;
        let mut x = x.abs();
        let mut output: i32 = 0;

        while x != 0 {
            let Some(output_temp) = output
                .checked_mul(10)
                .and_then(|output| output.checked_add(x % 10))
            else {
                return 0;
            };
            output = output_temp;
            x /= 10;
        }

        if is_min {
            output = -output
        }

        output
    }
}

#[test]
fn check() {
    assert_eq!(Solution::reverse(100), 1);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(1534236469), 0);
}
