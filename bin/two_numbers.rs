use std::mem::ManuallyDrop;

// https://leetcode.com/problems/add-two-numbers/
//
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn trans_from_vec_to_listnode(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut res = None;

    for li in list.iter().rev() {
        let mut tmp = ListNode::new(*li);
        let mut tmp_before = ManuallyDrop::new(res);
        tmp.next = tmp_before.take();
        res = Some(Box::new(tmp));
    }

    res
}

struct Solution;
impl Solution {
    pub fn add_two_numbers_addition(
        addition: i32,
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1box), Some(l2box)) => {
                let ListNode {
                    val: val1,
                    next: next1,
                } = *l1box;
                let ListNode {
                    val: val2,
                    next: next2,
                } = *l2box;
                let mut val = val1 + val2 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, next1, next2);
                Some(Box::new(renode))
            }
            (Some(l1box), None) => {
                let ListNode {
                    val: val1,
                    next: next1,
                } = *l1box;
                let mut val = val1 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, next1, None);
                Some(Box::new(renode))
            }
            (None, Some(l2box)) => {
                let ListNode {
                    val: val2,
                    next: next2,
                } = *l2box;
                let mut val = val2 + addition;
                let mut current_addition = 0;
                if val >= 10 {
                    val = val - 10;
                    current_addition = 1;
                }
                let mut renode = ListNode::new(val);
                renode.next = Self::add_two_numbers_addition(current_addition, None, next2);

                Some(Box::new(renode))
            }
            (None, None) => {
                if addition != 0 {
                    return Some(Box::new(ListNode::new(1)));
                }
                None
            }
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_addition(0, l1, l2)
    }
}

#[test]
fn test_two_number() {
    let left = trans_from_vec_to_listnode(vec![2, 4, 3]);
    let right = trans_from_vec_to_listnode(vec![5, 6, 4]);

    assert_eq!(
        Solution::add_two_numbers(left, right),
        trans_from_vec_to_listnode(vec![7, 0, 8])
    )
}
#[test]
fn test_two_number2() {
    let left = trans_from_vec_to_listnode(vec![9, 9, 9, 9, 9, 9, 9]);
    let right = trans_from_vec_to_listnode(vec![9, 9, 9, 9]);

    assert_eq!(
        Solution::add_two_numbers(left, right),
        trans_from_vec_to_listnode(vec![8, 9, 9, 9, 0, 0, 0, 1])
    )
}
