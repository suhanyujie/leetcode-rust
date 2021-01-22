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
    /// 查看题解后，采用分治合并：https://leetcode-cn.com/problems/merge-k-sorted-lists/solution/he-bing-kge-pai-xu-lian-biao-by-leetcode-solutio-2/
    /// https://leetcode-cn.com/problems/merge-k-sorted-lists/solution/rust-by-tryfor_-25/
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() < 1 {
            return None;
        }
        let size = lists.len();
        return Self::merge(&mut lists, 0, size - 1);
    }

    pub fn merge(
        lists: &mut Vec<Option<Box<ListNode>>>,
        l: usize,
        r: usize,
    ) -> Option<Box<ListNode>> {
        if l == r {
            return lists[l].take();
        }
        let mid = l + (r - l) / 2;
        let l1 = Self::merge(lists, l, mid);
        let l2 = Self::merge(lists, mid + 1, r);
        return Self::merge_two(l1, l2);
    }

    pub fn merge_two(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut tail = ListNode::new(0);
        let mut cur_node = &mut tail;
        while list1.is_some() && list2.is_some() {
            let (n1, n2) = (list1.as_mut().unwrap(), list2.as_mut().unwrap());
            if n1.val < n2.val {
                let next = n1.next.take();
                cur_node.next = list1.take();
                list1 = next;
            } else {
                let next = n2.next.take();
                cur_node.next = list2.take();
                list2 = next;
            }
            cur_node = cur_node.next.as_mut().unwrap();
        }
        cur_node.next = list1.or(list2);
        tail.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
