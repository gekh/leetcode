use super::p206_reverse_linked_list::{Solution, ListNode};

pub fn test() {
    assert_eq!(
        call(ListNode::create_list(vec![1, 2, 3, 4, 5, 6])),
        ListNode::create_list(vec![6, 5, 4, 3, 2, 1]),
    );

    // assert_eq!(
    //     call(ListNode::create_list(vec![1, 2])),
    //     ListNode::create_list(vec![2, 1]),
    // );

    // assert_eq!(
    //     call(ListNode::create_list(vec![1])),
    //     ListNode::create_list(vec![1]),
    // );

    // assert_eq!(
    //     call(ListNode::create_list(vec![])),
    //     ListNode::create_list(vec![]),
    // );
}

fn call(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Solution::reverse_list(head)
}
