struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // NOTE: edge case
        if num_rows == 1 {
            return s;
        }
        let mut dictronary: Vec<[char; 1000]> = vec![];
        for _ in 0..num_rows {
            dictronary.push(['0'; 1000]);
        }
        let mut backing = false;
        let mut current_list = 0;
        let mut current_row = 0;
        for c in s.chars() {
            if current_row == 0 {
                backing = false;
            }

            dictronary[current_row][current_list] = c;
            if (current_row + 1) % num_rows as usize == 0 {
                backing = true;
            }
            // NOTE: need backing
            if backing {
                current_list += 1;
                current_row -= 1;
                continue;
            }
            current_row += 1;
        }
        let mut output = String::new();
        for str in dictronary {
            let output_tmp: String = str.iter().filter(|c| **c != '0').collect();
            output.push_str(&output_tmp);
        }
        output
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
