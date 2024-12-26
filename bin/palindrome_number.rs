struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // NOTE: when is min
        if x < 0 {
            return false;
        }
        let mut num_storage = vec![];
        let mut x = x;
        while x != 0 {
            num_storage.push(x % 10);
            x /= 10;
        }
        let num_storage_len = num_storage.len();
        // NOTE: when x is zero
        if num_storage_len == 0 {
            return true;
        }
        let mut start_pos = 0;
        loop {
            let mirror_pos = num_storage_len - 1 - start_pos;

            if num_storage[start_pos] != num_storage[mirror_pos] {
                return false;
            }
            if (start_pos as i32 - mirror_pos as i32).abs() <= 1 {
                break;
            }
            start_pos += 1;
        }
        true
    }
}

#[test]
fn check() {
    assert_eq!(Solution::is_palindrome(121), true);
}
