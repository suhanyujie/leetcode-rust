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
    // 遍历一遍，取得节点的数量，计算需要进行多少次反转。
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k <= 1 {
            return head;
        }
        if head.is_none() {
            return head;
        }
        if head.as_ref().is_some() && head.as_ref().as_ref().unwrap().next.is_none() {
            return head;
        }
        // 统计链表数量
        let node_num = Self::get_node_num(&head);
        if k > node_num {
            return head;
        }
        let mut list: Vec<Option<Box<ListNode>>> = vec![];
        let mut cur_node = head;
        let mut index = 0;
        while cur_node.is_some() {
            if index > k {
                break;
            }
            let next = cur_node.as_mut().unwrap().next.take();
            list.push(cur_node.take());
            cur_node = next;
            index += 1;
        }
        let mut new_tail = Some(Box::new(ListNode::new(0)));
        let mut cur_node = &mut new_tail;
        for item in list {
            let next = item.take();
            cur_node.as_mut().unwrap().next = item;
            cur_node = &mut next;
        }
        // cur_node.as_mut().unwrap().next = Self::reverse_k_group();

        new_tail.unwrap().next
    }

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
