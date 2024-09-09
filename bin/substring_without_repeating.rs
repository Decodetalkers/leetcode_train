use std::{char, collections::HashMap};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut position_mark: HashMap<char, usize> = HashMap::new();
        let array: Vec<char> = s.chars().collect();

        let mut max_count = 0;
        let mut current_count = 0;

        let mut start_index = 0;
        let end_index = array.len();

        'out: loop {
            for index in start_index..end_index {
                let current_key = array[index];
                if position_mark
                    .get(&current_key)
                    .is_some_and(|oindex| *oindex < index)
                {
                    let renew_pos = position_mark[&current_key];
                    if renew_pos < start_index {
                        current_count += 1;
                    }
                    start_index = renew_pos + 1;
                    position_mark.remove(&current_key);
                    if current_count > max_count {
                        max_count = current_count;
                    }
                    current_count = 0;
                    continue 'out;
                }
                position_mark
                    .entry(current_key)
                    .and_modify(|cindex| *cindex = index)
                    .or_insert(index);
                current_count += 1;
            }
            break;
        }

        if current_count > max_count {
            max_count = current_count;
        }

        max_count
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
