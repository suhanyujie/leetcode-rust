//! 21. 合并两个有序链表
//! 题目地址 https://leetcode-cn.com/problems/merge-two-sorted-lists/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl<'a> Solution {
    /// tips：取节点，到一个数组中，对数组进行排序，然后重新建立链表。
    /// 通过查阅题解，先以递归的方式实现：
    /// 结果链表的头节点，两个链表位置的临时指针。
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(b1), None) => Some(b1),
            (None, Some(b2)) => Some(b2),
            (Some(mut b1), Some(mut b2)) => {
                if b1.as_ref().val < b2.as_ref().val {
                    b1.next = Self::merge_two_lists(b1.next, Some(b2));
                    Some(b1)
                } else {
                    b2.next = Self::merge_two_lists(Some(b1), b2.next);
                    Some(b2)
                }
            }
        }
    }

    // 非递归方式
    // 参考 https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/rust-fei-di-gui-geng-jian-dan-cao-zuo-wu-mo-shi-pi/
    pub fn merge_two_lists_2(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 声明结果链表的头节点
        let mut tail = ListNode::new(1);
        let mut p_tail = &mut tail;
        while l1.is_some() && l2.is_some() {
            let (p1, p2) = (l1.as_mut().unwrap(), l2.as_mut().unwrap());
            if p1.val < p2.val {
                let next = p1.next.take();
                p_tail.next = l1.take();
                l1 = next;
            } else {
                let next = p2.next.take();
                p_tail.next = l2.take();
                l2 = next;
            }
            p_tail = p_tail.next.as_mut().unwrap();
        }
        p_tail.next = l1.or(l2);
        tail.next
    }

    // todo 通过数组，生成新的链表。
    pub fn gen_list(arr: Vec<Option<Box<ListNode>>>) {
        // let mut cur_node = None;
        // for i in 0..arr.len() {
        //   if i > 0 {
        //     // cur_node.unwrap().next = (arr[i]).take();
        //   } else {
        //     cur_node = arr[0];
        //   }
        // }
    }

    /// 获取一个链表的所有节点
    pub fn get_node_list(l1: Option<Box<ListNode>>) -> Vec<Option<Box<ListNode>>> {
        let mut result: Vec<_> = vec![];
        let mut cur_node = l1;
        while cur_node.as_ref().is_some() {
            // 如果下一个节点有值，则将下一个节点存储到临时变量中。
            if cur_node.as_ref().unwrap().next.as_ref().is_some() {
                let tmp_next = cur_node.as_mut().unwrap().next.take();
                result.push(cur_node.take());
                cur_node = tmp_next;
            } else {
                result.push(cur_node.take());
                break;
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_node_list() {
        assert_eq!(Solution::get_node_list(None).len(), 0);
    }

    #[test]
    fn test_merge_two_lists() {
        let n1 = Some(Box::new(ListNode::new(5)));
        let v_n5 = Box::new(ListNode { val: 6, next: n1 });
        let n4 = Some(v_n5);
        Solution::merge_two_lists(n4, None);
        assert!(false);
    }
}
