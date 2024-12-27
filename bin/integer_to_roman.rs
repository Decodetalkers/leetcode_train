struct Solution;

impl Solution {
    fn int_to_roman_pos(pos: usize, current_val: i32) -> String {
        if pos >= 3 {
            let mut output = String::new();
            for _ in 0..current_val {
                output.push('M');
            }
            return output;
        }

        let one_char = match pos {
            0 => 'I',
            1 => 'X',
            2 => 'C',
            _ => unreachable!(),
        };
        let five_char = match pos {
            0 => 'V',
            1 => 'L',
            2 => 'D',
            _ => unreachable!(),
        };
        let next_char = match pos {
            0 => 'X',
            1 => 'C',
            2 => 'M',
            _ => unreachable!(),
        };

        if current_val < 4 {
            let mut iii = String::new();
            for _ in 0..current_val {
                iii.push(one_char);
            }
            return iii;
        }
        if current_val == 4 {
            return format!("{one_char}{five_char}");
        }
        if current_val == 5 {
            return five_char.to_string();
        }
        if current_val > 5 && current_val < 9 {
            let mut iii = String::new();
            let distantance = (current_val - 5).abs();
            for _ in 0..distantance {
                iii.push(one_char);
            }
            return format!("{five_char}{iii}");
        }

        format!("{one_char}{next_char}")
    }
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut current_pos = 0;
        let mut output = String::new();
        while num != 0 {
            let current_val = num % 10;
            let mut string_current = Solution::int_to_roman_pos(current_pos, current_val);
            string_current.push_str(&output);
            output = string_current;

            current_pos += 1;
            num /= 10;
        }
        output
    }
}

#[test]
fn check() {
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string());
    assert_eq!(Solution::int_to_roman(1), "I".to_string());
}
