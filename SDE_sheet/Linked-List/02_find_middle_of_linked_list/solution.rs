// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while let (Some(fast_node), Some(next_fast_node)) = (fast, fast.and_then(|node| node.next.as_ref())){
            slow = slow.and_then(|node| node.next.as_ref());
            fast = next_fast_node.next.as_ref();
        }

        slow.cloned()
    }
}
