/// 题目名称：23. 合并K个升序链表
/// 题目地址：https://leetcode-cn.com/problems/merge-k-sorted-lists/
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
    /// 使用一个指针作为结果指针，然后挨个对比链表的首节点。
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let tail = Some(ListNode::new(-1 * 10000));
        let mut cur_node = &tail;
        forlink in lists {
            if link.is_none() {
                continue;
            }
            if link.as_ref().unwrap().val < cur_node.as_ref().unwrap().val {
                cur_node.as_mut().unwrap().next = link.as_mut().take();
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
