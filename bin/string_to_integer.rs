const NINE: i32 = '9' as i32;
const ZERO: i32 = '0' as i32;

struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut is_minus = false;
        let mut output: i32 = 0;
        for (index, c) in s.chars().enumerate() {
            if c == '-' && index == 0 {
                is_minus = true;
                continue;
            }
            if c == '+' && index == 0 {
                continue;
            }
            if (c as i32) > NINE || (c as i32) < ZERO {
                break;
            }
            let step = c as i32 - ZERO;
            if is_minus {
                let Some(tmp) = output
                    .checked_mul(10)
                    .and_then(|output| output.checked_sub(step))
                else {
                    return i32::MIN;
                };
                output = tmp;
            } else {
                let Some(tmp) = output
                    .checked_mul(10)
                    .and_then(|output| output.checked_add(step))
                else {
                    return i32::MAX;
                };
                output = tmp;
            }
        }

        output
    }
}

#[test]
fn check() {
    assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
    assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(
        Solution::my_atoi("9223372036854775808".to_string()),
        2147483647
    );
}
