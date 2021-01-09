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
      let mut list1 = Self::get_node_list(l1);
      let mut list2 = Self::get_node_list(l2);
      list1.append(&mut list2);
      list1.sort_by(|a, b| a.as_ref().unwrap().val.cmp(&b.as_ref().unwrap().val));
      // 重新组装链表

      dbg!(&list1);
      return None;
    }

    pub fn gen_list(arr: Vec<Option<Box<ListNode>>>) {
      
      let mut cur_node = None;
      for i in 0..arr.len() {
        if i > 0 {
          // cur_node.unwrap().next = (arr[i]).take();
        } else {
          cur_node = arr[0];
        }
      }
    }

    /// 获取一个链表的所有节点
    pub fn get_node_list(l1: Option<Box<ListNode>>) ->Vec<Option<Box<ListNode>>> {
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
      let v_n5 = Box::new(ListNode{
        val: 6,
        next: n1,
      });
      let n4 = Some(v_n5);
      Solution::merge_two_lists(n4, None);
      assert!(false);
    }
}
