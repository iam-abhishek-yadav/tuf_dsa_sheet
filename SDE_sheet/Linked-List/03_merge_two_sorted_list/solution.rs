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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut temp = ListNode::new(0);
        let mut tail = &mut temp;

        while list1.is_some() && list2.is_some() {
            let l1 = list1.as_mut().unwrap();
            let l2 = list2.as_mut().unwrap();

            if l1.val < l2.val {
                tail.next = list1;
                list1 = tail.next.as_mut().unwrap().next.take();
            } else {
                tail.next = list2;
                list2 = tail.next.as_mut().unwrap().next.take();
            }
            tail = tail.next.as_mut().unwrap();
        }

        if list1.is_some() {
            tail.next = list1;
        } else {
            tail.next = list2;
        }

        temp.next
    }
}
