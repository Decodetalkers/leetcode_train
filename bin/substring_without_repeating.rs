use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash: HashMap<char, i32> = HashMap::new();
        let mut ans = 0;
        let mut lo = -1;
        for (hi, ch) in s.chars().enumerate() {
            // loop through chars with loop index being hi.
            if let Some(i) = hash.insert(ch, hi as i32) {
                // inserts ch and hi into map and it returns old index.
                lo = lo.max(i); // Move the lo pointer to that old index.
            }
            ans = ans.max(hi as i32 - lo); // updates ans if hi - lo is greater than previous answer.
        }
        ans
    }
}

#[test]
fn solution_test() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
    assert_eq!(
        1,
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    assert_eq!(3, Solution::length_of_longest_substring("dvdf".to_string()));
    assert_eq!(
        5,
        Solution::length_of_longest_substring("tmmzuxt".to_string())
    );
    assert_eq!(
        3,
        Solution::length_of_longest_substring("aabaab!bb".to_string())
    );
}
