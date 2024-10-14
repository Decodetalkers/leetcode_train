use std::{collections::HashMap, usize};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut box_vals: HashMap<i32, Vec<usize>> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            box_vals
                .entry(*num)
                .and_modify(|list| list.push(index))
                .or_insert(vec![index]);
        }

        let mut start_index = 0;
        let mut end_index = 0;

        'out: for (index, num) in nums.iter().enumerate() {
            let ops = target - *num;
            if let Some(another_indexs) = box_vals.get(&ops) {
                for another_index in another_indexs {
                    if *another_index == index {
                        continue;
                    }
                    start_index = index as i32;
                    end_index = *another_index as i32;
                    break 'out;
                }
            }
        }

        vec![start_index, end_index]
    }
}

#[test]
fn test_solution() {
    let sol1 = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(sol1, vec![0, 1]);
    let sol1 = Solution::two_sum(vec![3, 2, 4], 6);
    assert_eq!(sol1, vec![1, 2]);
    let sol1 = Solution::two_sum(vec![0, 4, 3, 0], 0);
    assert_eq!(sol1, vec![0, 3])
}
