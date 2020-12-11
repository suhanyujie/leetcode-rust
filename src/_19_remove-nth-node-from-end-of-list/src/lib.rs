//! 19. 删除链表的倒数第N个节点
//! 题目地址 https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/



#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

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

impl Solution {
    // 反转链表。这样就能定位到倒数第n个的节点了。然后予以删除。
    // 时间复杂度 O(n)，空间复杂度 O(n)
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        todo!();
        Some(Box::new(ListNode::new(1)))
    }
}

#[cfg(test)]
mod tests{

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
