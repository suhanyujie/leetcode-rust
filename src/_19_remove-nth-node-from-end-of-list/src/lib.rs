//! 19. 删除链表的倒数第N个节点
//! 题目地址 https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list/

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

struct Solution;

impl Solution {
    // 个人思考：反转链表。这样就能定位到倒数第n个的节点了。然后予以删除。
    // 时间复杂度 O(n)，空间复杂度 O(n)
    // 官方题解：利用双指针。例如是删除倒数第2个，则第一个指针快第2个指针2步。当第一个指针到达末尾时，第2个指针刚好到达要删除的位置。
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut before: &Option<Box<ListNode>> = &head;
        let mut after: &Option<Box<ListNode>> = &head;
        // before 先往前走 n 步。此处是从 1 开始，如删除倒数第 2 个，则只需快指针多一步。
        let mut i = 1;
        while i < n {
            before = &before.as_ref()?.next;
            i += 1
        }
        while before.as_ref()?.next.is_some() {
          before = &before.as_ref()?.next;
          after = &after.as_ref()?.next;
        }
        // 至此，已经确定了倒数第 n 个的节点，那就是 after 
        head = Solution::delete_node(head.clone(), &after);
        head
    }

    // 删除一个节点 todo
    fn delete_node(mut head: Option<Box<ListNode>>, target: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut my_head = Some(Box::new(ListNode::new(1)));
      my_head.as_mut()?.next = head;
      let mut root  = &mut my_head;
      while root.as_mut()?.next.is_some() {
        if &root.as_mut()?.next == target {
          let target_next = &target.as_ref()?.next;
          root.as_mut()?.next = target_next.clone();
          break;
        }
        root = &mut root.as_mut()?.next;
      }
      my_head?.next
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
