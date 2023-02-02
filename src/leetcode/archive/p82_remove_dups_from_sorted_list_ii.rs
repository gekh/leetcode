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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(i32::MAX)));
        dummy.as_mut().unwrap().next = head;

        Self::req(dummy).0.unwrap().next
    }

    fn req(list: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        match list {
            None => (None, i32::MAX),
            Some(x) => {
                let (mut r, rep_val) = Self::req(x.next);

                if !r.is_none() {
                    if x.val == r.as_ref().unwrap().val {
                        r = r.as_ref().unwrap().next.clone();
                        return (r, x.val);
                    }
                }

                if rep_val != i32::MAX && rep_val == x.val {
                    return (r, rep_val);
                }

                (Some(Box::new(ListNode {val: x.val, next: r})), i32::MAX)
            }
        }
    }
}
