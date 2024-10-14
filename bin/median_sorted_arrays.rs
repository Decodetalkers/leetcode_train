struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.is_empty() && nums2.is_empty() {
            return 0.;
        }
        if nums1.is_empty() {
            let len2 = nums2.len();
            let medium = len2 / 2;
            if len2 % 2 == 0 {
                return (nums2[medium - 1] + nums2[medium]) as f64 / 2.;
            } else {
                return nums2[medium] as f64;
            }
        }
        if nums2.is_empty() {
            let len2 = nums1.len();
            let medium = len2 / 2;
            if len2 % 2 == 0 {
                return (nums1[medium - 1] + nums1[medium]) as f64 / 2.;
            } else {
                return nums1[medium] as f64;
            }
        }
        let all = nums1.len() + nums2.len();
        if all % 2 != 0 {
            Solution::find_median_sorted_arrays_middle(nums1, nums2, all / 2)
        } else {
            Solution::find_median_sorted_arrays_middle_two(nums1, nums2, all / 2 - 1)
        }
    }

    fn find_median_sorted_arrays_middle(nums1: Vec<i32>, nums2: Vec<i32>, medium: usize) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut float1 = 0;
        let mut float2 = 0;

        let mut current_pos = 0;

        let re;

        loop {
            // To the end
            if float1 == len1 {
                if current_pos == medium {
                    re = nums2[float2] as f64;
                    break;
                }
                float2 += 1;
                current_pos += 1;
                continue;
            }
            if float2 == len2 {
                if current_pos == medium {
                    re = nums1[float1] as f64;
                    break;
                }
                float1 += 1;
                current_pos += 1;
                continue;
            }
            let current_val1 = nums1[float1];
            let current_val2 = nums2[float2];
            if current_val1 < current_val2 {
                if current_pos == medium {
                    re = nums1[float1] as f64;
                    break;
                }
                float1 += 1;
                current_pos += 1;
            } else {
                if current_pos == medium {
                    re = nums2[float2] as f64;
                    break;
                }
                float2 += 1;
                current_pos += 1;
            }
        }
        re
    }

    fn find_median_sorted_arrays_middle_two(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        left_midium: usize,
    ) -> f64 {
        //println!("left_midium: {left_midium}");
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut float1 = 0;
        let mut float2 = 0;

        let mut current_pos = 0;

        let mut re1 = 0;
        let re2;

        loop {
            //println!("float1: {float1}, float2: {float2}, current_pos: {current_pos}");
            // To the end
            if float1 == len1 {
                if current_pos == left_midium {
                    re1 = nums2[float2];
                } else if current_pos == left_midium + 1 {
                    re2 = nums2[float2];
                    break;
                }
                float2 += 1;
                current_pos += 1;
                continue;
            }
            if float2 == len2 {
                if current_pos == left_midium {
                    re1 = nums1[float1];
                } else if current_pos == left_midium + 1 {
                    re2 = nums1[float1];
                    break;
                }
                float1 += 1;
                current_pos += 1;
                continue;
            }
            let current_val1 = nums1[float1];
            let current_val2 = nums2[float2];
            //println!("current_val1: {current_val1}, current_val2: {current_val2}, current_pos: {current_pos}");
            if current_val1 < current_val2 {
                if current_pos == left_midium {
                    re1 = nums1[float1];
                    if float1 + 1 < len1 && nums1[float1 + 1] < current_val2 {
                        re2 = nums1[float1 + 1];
                    } else {
                        re2 = current_val2;
                    }
                    //println!("re1 : {re1}, re2: {re2}");
                    break;
                }
                float1 += 1;
                current_pos += 1;
            } else {
                if current_pos == left_midium {
                    re1 = nums2[float2];
                    if float2 + 1 < len2 && nums2[float2 + 1] < current_val1 {
                        re2 = nums2[float2 + 1];
                    } else {
                        re2 = current_val1;
                    }
                    break;
                }
                float2 += 1;
                current_pos += 1;
            }
        }
        (re1 + re2) as f64 / 2.
    }
}

#[test]
fn solution_test() {
    assert_eq!(2., Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    assert_eq!(
        2.5,
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
    assert_eq!(
        2.,
        Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![1, 2, 2])
    );
    assert_eq!(
        1.5,
        Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3])
    );
    assert_eq!(
        2.5,
        Solution::find_median_sorted_arrays(vec![2, 3, 4], vec![1])
    )
}
