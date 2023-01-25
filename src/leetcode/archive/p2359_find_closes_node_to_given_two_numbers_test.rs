use super::p2359_find_closes_node_to_given_two_numbers::Solution;

pub fn test() {
    // assert_eq!(call(vec![2,2,3,-1], 0, 1), 2);
    // assert_eq!(call(vec![1,2,-1], 0, 2), 2);
    // assert_eq!(call(vec![2,2,3,0], 0, 1), 2);
    assert_eq!(call(vec![5,4,5,4,3,6,-1], 0, 1), -1);
}

fn call(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    Solution::closest_meeting_node(edges, node1, node2)
}
