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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (x, None) | (None, x) => x,
            (Some(a), Some(b)) => {
                if a.val < b.val {
                    Some(Box::new(ListNode {
                        val: a.val,
                        next: Self::merge_two_lists(a.next, Some(b)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: b.val,
                        next: Self::merge_two_lists(Some(a), b.next),
                    }))
                }
            }
        }
    }
}
