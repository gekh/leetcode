use super::p21_merge_two_sorted_lists::{Solution, ListNode};

pub fn test() {
    assert_eq!(
        call(
            ListNode::create_list(vec![1, 3, 5]),
            ListNode::create_list(vec![2, 4, 6]),
        ),
        ListNode::create_list(vec![1, 2, 3, 4, 5, 6]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![2, 4, 6]),
            ListNode::create_list(vec![1, 3, 5]),
        ),
        ListNode::create_list(vec![1, 2, 3, 4, 5, 6]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![1, 2, 4]),
            ListNode::create_list(vec![1, 3, 4]),
        ),
        ListNode::create_list(vec![1, 1, 2, 3, 4, 4]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![]),
            ListNode::create_list(vec![]),
        ),
        ListNode::create_list(vec![]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![0]),
            ListNode::create_list(vec![]),
        ),
        ListNode::create_list(vec![0]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![]),
            ListNode::create_list(vec![0]),
        ),
        ListNode::create_list(vec![0]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![4, 5, 6]),
            ListNode::create_list(vec![1, 2, 3]),
        ),
        ListNode::create_list(vec![1, 2, 3, 4, 5, 6]),
    );

    assert_eq!(
        call(
            ListNode::create_list(vec![-1, 2, 4]),
            ListNode::create_list(vec![-3, -1, -1, 4]),
        ),
        ListNode::create_list(vec![-3, -1, -1, -1, 2, 4, 4]),
    );
}

fn call(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Solution::merge_two_lists(list1, list2)
}
