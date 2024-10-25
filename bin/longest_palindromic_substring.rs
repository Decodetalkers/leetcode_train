struct Solution;

// Manacher's algorithm
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let mut start_pos = 0;
        let mut end_pos = 0;
        // eg 1
        for index in 0..len {
            // case 1
            let width_size = index.min(len - index - 1);

            let mut tmp_start = index;
            let mut tmp_end = index;
            for bindex in 1..=width_size {
                if chars[index - bindex] != chars[index + bindex] {
                    break;
                }
                tmp_start = index - bindex;
                tmp_end = index + bindex;
            }
            if tmp_end - tmp_start > end_pos - start_pos {
                start_pos = tmp_start;
                end_pos = tmp_end
            }

            // case 2
            if index + 1 > len - 1 || chars[index] != chars[index + 1] {
                continue;
            }
            let width_size = index.min(len - index - 2);

            let mut tmp_start = index;
            let mut tmp_end = index + 1;
            for bindex in 1..=width_size {
                if chars[index - bindex] != chars[index + 1 + bindex] {
                    break;
                }
                tmp_start = index - bindex;
                tmp_end = index + bindex + 1;
            }
            if tmp_end - tmp_start > end_pos - start_pos {
                start_pos = tmp_start;
                end_pos = tmp_end
            }
        }
        s[start_pos..=end_pos].to_string()
    }
}

#[test]
fn test_clip() {
    assert_eq!(
        "bab".to_string(),
        Solution::longest_palindrome("babad".to_string())
    );
    assert_eq!(
        "bb".to_string(),
        Solution::longest_palindrome("cbbd".to_string())
    );
    assert_eq!(
        "bb".to_string(),
        Solution::longest_palindrome("bb".to_string())
    );
    assert_eq!(
        "aca".to_string(),
        Solution::longest_palindrome("aacabdkacaa".to_string())
    );
}
