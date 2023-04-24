
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

// 反转链表
// https://leetcode.cn/problems/UHnkqh/

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> { 
  if head == None {
    return head;
  }
  let mut head = head;
  let mut newHead = None;

  while let Some(mut current) = head {
      head = current.next;
      current.next = newHead;
      newHead = Some(current);
  }
  newHead
}


fn main() {
  println!("Hello, world!");
}
