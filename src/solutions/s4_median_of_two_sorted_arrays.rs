// https://leetcode.com/problems/median-of-two-sorted-arrays/

// submission starts
impl Solution {
    // 1. Simple one
    // Use rust std method
    // pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let mut v = [nums1, nums2].concat();
    //     v.sort();
    //     let len = v.len();
    //     if len % 2 == 0 {
    //         (v[len / 2] + v[len / 2 - 1]) as f64 / 2.0
    //     } else {
    //         v[len / 2] as f64
    //     }
    // }

    // 2. more time efficient
    // 兩個有序數組合併後的有序數組的中位數，一定是總長度的中間值。
    // 我們無須合併數組，仍然可以找到中位數的位置。
    // 可以從小值往後遍歷，兩數組中較小的一個往後遍歷，直到中位數的位置
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        todo!();
    }
}
// submission ends

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
