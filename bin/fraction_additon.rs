use std::ops::{Add, Deref};

const NUM_LIST: &[i32] = &[2, 3, 5, 7];

struct Solution;

/// something line 1/2, -1/2
#[derive(Debug)]
struct StrFraction<'a>(&'a str);

fn check_number(num: i32) -> Vec<i32> {
    NUM_LIST
        .iter()
        .filter(|num_d| num % *num_d == 0)
        .map(|numd| *numd)
        .collect()
}

impl<'a> Deref for StrFraction<'a> {
    type Target = &'a str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> StrFraction<'a> {
    fn mom(&self) -> Option<&'a str> {
        (&self).split('/').last()
    }
    fn child(&self) -> Option<&'a str> {
        (&self).split('/').next()
    }
}

impl<'a> Add for StrFraction<'a> {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0 == "0" || self.is_empty() {
            return rhs.to_string();
        }
        if rhs.0 == "0" || rhs.is_empty() {
            return self.to_string();
        }
        let self_mum: i32 = self.mom().unwrap().parse().unwrap();
        let self_child: i32 = self.child().unwrap().parse().unwrap();
        let rhs_mum: i32 = rhs.mom().unwrap().parse().unwrap();
        let rhs_child: i32 = rhs.child().unwrap().parse().unwrap();

        let (mut new_child, mut new_mum) = if self_mum == rhs_mum {
            (self_child + rhs_child, self_mum)
        } else {
            (
                self_child * rhs_mum + rhs_child * self_mum,
                self_mum * rhs_mum,
            )
        };

        if new_child == 0 {
            return "0/1".to_string();
        }
        loop {
            println!("{new_child}, {new_mum}");
            let check_list = check_number(new_child);
            if check_list.is_empty() {
                break;
            }
            let mut do_change = false;
            for num_d in check_list {
                if new_mum % num_d == 0 {
                    new_child = new_child / num_d;
                    new_mum = new_mum / num_d;
                    do_change = true;
                }
            }
            if !do_change {
                break;
            }
        }
        format!("{new_child}/{new_mum}")
    }
}

impl Solution {
    fn sub_addition(after: &str, before: &str) -> String {
        if before.is_empty() {
            return after.to_string();
        }
        let val_before = StrFraction(after);

        let mut chars = before.chars();

        let mut next_val = String::new();

        while let Some(c) = chars.next_back() {
            next_val = c.to_string() + &next_val;
            if c == '+' || c == '-' {
                break;
            }
        }

        let val_after = StrFraction(&next_val);

        Self::sub_addition(&(val_after + val_before), chars.as_str())
    }

    pub fn fraction_addition(expression: String) -> String {
        Self::sub_addition("0", &expression)
    }
}

#[test]
fn test_solution() {
    let example1 = "-1/2+1/2".to_string();
    assert_eq!("0/1".to_string(), Solution::fraction_addition(example1));

    let example1 = "-1/2+1/2+1/3".to_string();
    assert_eq!("1/3".to_string(), Solution::fraction_addition(example1));

    let example1 = "5/3+1/3".to_string();
    assert_eq!("2/1".to_string(), Solution::fraction_addition(example1));
}
