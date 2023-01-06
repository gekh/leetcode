use super::p19_remove_nth_node_from_end_of_list::{ListNode, Solution};

pub fn test() {
    assert_eq!(
        call(ListNode::create_list(vec![1, 2, 3, 4, 5]), 2),
        ListNode::create_list(vec![1, 2, 3, 5]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1]), 1),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,2]), 1),
        ListNode::create_list(vec![1]),
    );
}

fn call(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    Solution::remove_nth_from_end(head, n)
}
