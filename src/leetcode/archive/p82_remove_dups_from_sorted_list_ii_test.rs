use super::p82_remove_dups_from_sorted_list_ii::{ListNode, Solution};

pub fn test() {
    assert_eq!(
        call(ListNode::create_list(vec![1,2,3,3,4,4,5])),
        ListNode::create_list(vec![1,2,5]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,2,3,3,3,3,4,4,5])),
        ListNode::create_list(vec![1,2,5]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,2,3,3,3,4,4,5])),
        ListNode::create_list(vec![1,2,5]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,1,1,2,3])),
        ListNode::create_list(vec![2,3]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,2,3,3,3])),
        ListNode::create_list(vec![1,2]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1])),
        ListNode::create_list(vec![1]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,1])),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,1,1])),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,1,1,1])),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![0,0,0,0])),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(ListNode::create_list(vec![1,1,1,1,2,2,2,2,2])),
        ListNode::create_list(vec![]),
    );
}

fn call(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Solution::delete_duplicates(head)
}
