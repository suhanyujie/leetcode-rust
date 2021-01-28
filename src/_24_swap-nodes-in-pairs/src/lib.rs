//! 题目名称：24. 两两交换链表中的节点
//! 题目地址：https://leetcode-cn.com/problems/swap-nodes-in-pairs/

/*
给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
输入：head = [1,2,3,4]
输出：[2,1,4,3]

输入：head = []
输出：[]

输入：head = [1]
输出：[1]
*/

struct Solution;

// Definition for singly-linked list.
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

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tail = head;
        let mut new_tail = tail.take();
        Self::swap_pairs(tail.as_mut().unwrap().next);
        None
    }

    pub fn swap_two(one: Option<Box<ListNode>>, two: &Option<Box<ListNode>>) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
