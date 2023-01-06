pub struct Solution();

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// impl ListNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut p = head.as_mut().unwrap();

        while let Some(cur) = p.next.as_mut() {
            if p.val == cur.val {
                p.next = cur.next.take();
            } else {
                p = p.next.as_mut().unwrap();
            }
        }

        return head;
    }
}

// main.rs
// let head = Some(Box::new(ListNode {
//     next: Some(Box::new(ListNode {
//         next: Some(Box::new(ListNode {
//             next: None,
//             val: 1
//         })),
//         val: 1
//     })),
//     val: 2
// }));

// dbg!(leetcode::p83_remove_dups_from_sorted_list::Solution::delete_duplicates(head));