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
        let mut old_head = head;
        if old_head.is_none() || old_head.as_ref().unwrap().next.is_none() {
            return old_head;
        }
        let mut old_tail = old_head.as_mut().unwrap().next.take();
        let mut next_head = old_tail.as_mut().unwrap().next.take();
        let mut next = None;
        if next_head.is_some() {
            next = Self::swap_pairs(next_head);
        }
        old_head.as_mut().unwrap().next = next;
        old_tail.as_mut().unwrap().next = old_head;

        old_tail
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
