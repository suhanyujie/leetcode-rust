//! 21. 合并两个有序链表
//! 题目地址 https://leetcode-cn.com/problems/merge-two-sorted-lists/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
//
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl<'a> Solution {
    /// tips：取节点，到一个数组中，对数组进行排序，然后重新建立链表。
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

      return l1;
    }

    /// 获取一个链表的所有节点
    pub fn get_node_list(l1: &'a Option<Box<ListNode>>) ->Vec<&'a Option<Box<ListNode>>> {
        let mut result: Vec<_> = vec![];
        let mut cur_node = l1;
        while cur_node.as_ref().is_some() {
          result.push(cur_node);
          cur_node = &cur_node.as_ref().unwrap().next;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn test_get_node_list() {
        assert_eq!(Solution::get_node_list(&None).len(), 0);
    }
}
