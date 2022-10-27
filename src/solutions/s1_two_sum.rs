// https://leetcode.com/problems/two-sum/

// submission starts
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mapping = HashMap::with_capacity(nums.len());
        for (num, index) in nums.iter().zip(0..) {
            let remaining = target - num;
            if mapping.contains_key(&remaining) {
                return vec![mapping[&remaining], index];
            }
            mapping.insert(*num, index);
        }
        return vec![];
    }
}
// submission ends

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
