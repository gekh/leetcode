pub struct Solution;

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

    pub fn create_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for x in list {
            *current = Some(Box::new(ListNode::new(x)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;

        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();

        for _ in 0..n {
            fast = fast.unwrap().next;
        }

        while fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next;
            slow = slow.unwrap().next.as_mut();
        }

        let next = slow.as_ref().unwrap().next.clone();
        slow.as_mut().unwrap().next = next.unwrap().next;

        dummy.unwrap().next
    }
}
