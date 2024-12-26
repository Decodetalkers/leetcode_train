struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // NOTE: edge case
        if num_rows == 1 {
            return s;
        }
        let mut dictronary: Vec<String> = vec![String::new(); num_rows as usize];
        let mut backing = false;
        let mut current_row = 0;
        for c in s.chars() {
            if current_row == 0 {
                backing = false;
            }

            dictronary[current_row].push(c);
            if (current_row + 1) % num_rows as usize == 0 {
                backing = true;
            }
            // NOTE: need backing
            if backing {
                current_row -= 1;
                continue;
            }
            current_row += 1;
        }
        dictronary.concat()
    }
}

#[test]
fn test_result() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    )
}
