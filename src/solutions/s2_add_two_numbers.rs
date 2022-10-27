// https://leetcode.com/problems/add-two-numbers/

// submission starts
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut result = None;
        let mut dummy_head = &mut result;

        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            dummy_head = &mut (dummy_head.insert(Box::new(ListNode::new(sum % 10))).next);
            carry = sum / 10;
        }
        result
    }
}
// submission ends

pub struct Solution;
use crate::utils::linked_list::{to_list, ListNode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                to_list(vec![9, 9, 9, 9, 9, 9, 9]),
                to_list(vec![9, 9, 9, 9])
            ),
            to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
