//! 题目名称：25. K 个一组翻转链表
//! 题目地址：https://leetcode-cn.com/problems/reverse-nodes-in-k-group/

/*
给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。

k 是一个正整数，它的值小于或等于链表的长度。

如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。

示例：

给你这个链表：1->2->3->4->5

当 k = 2 时，应当返回: 2->1->4->3->5

当 k = 3 时，应当返回: 3->2->1->4->5

节点的数量范围 sz: 1 <= sz <= 5000
反转系数 1 <= k <= sz
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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 {
            return head;
        }
        if head.is_none() {
            return head;
        }
        if head.as_ref().is_some() && head.as_ref().as_ref().unwrap().next.is_none() {
            return head;
        }
        let total = Solution::get_node_num(&head);
        // 用一个新的节点放置反转后的链表 head
        let mut new_head = Some(Box::new(ListNode::new(0)));
        let mut new_tail = new_head.as_mut().unwrap();
        for _ in 0..(total / k) {
            let mut p = head.as_mut().unwrap();
            // 步进 k 次，然后 take
            for _ in 0..(k - 1) {
                p = p.next.as_mut().unwrap();
            }
            let tail = p.next.take();
            let reversed = Solution::reverse_one(head, k);
            new_tail = Solution::merge_one(new_tail, reversed);
            head = tail;
        }
        // 处理最后剩余不足 k 个的部分节点
        new_tail.next = head;

        new_head.unwrap().next
    }

    /// 将反转后的一组合并
    fn merge_one(
        mut tail: &mut Box<ListNode>,
        new_list: Option<Box<ListNode>>,
    ) -> &mut Box<ListNode> {
        tail.next = new_list;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail
    }

    /// 反转一组
    fn reverse_one(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        for _ in 0..k {
            if let Some(mut tmp_node) = head {
                head = tmp_node.next.take();
                tmp_node.next = res;
                res = Some(tmp_node);
            }
        }
        res
    }

    /// 从头部开始，遍历计算出链表的长度
    pub fn get_node_num(head: &Option<Box<ListNode>>) -> i32 {
        let mut num = 0;
        if head.is_none() {
            return num;
        }
        let mut cur_node = head;
        while cur_node.is_some() {
            num += 1;
            cur_node = &cur_node.as_ref().unwrap().next;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node1 = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let mut node5 = Some(Box::new(ListNode::new(5)));
        node2.as_mut().unwrap().next = node1;
        node3.as_mut().unwrap().next = node2;
        node4.as_mut().unwrap().next = node3;
        node5.as_mut().unwrap().next = node4;
        let res = Solution::reverse_k_group(node5, 2);
        let expected_res = vec![4, 5, 2, 3, 1];
        let mut cur_node_op = res;
        for expected_one in expected_res {
            let cur_node = cur_node_op.as_mut().unwrap();
            assert!(cur_node.val == expected_one);
            cur_node_op = cur_node.next.take();
        }
    }
}
