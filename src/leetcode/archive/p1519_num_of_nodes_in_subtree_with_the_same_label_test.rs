use super::p1519_num_of_nodes_in_subtree_with_the_same_label::{Solution};

pub fn test() {
    assert_eq!(
        call(7, vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]], "abaedcd"),
        vec![2,1,1,1,1,1,1]
    );

    assert_eq!(
        call(4, vec![vec![0,1],vec![1,2],vec![0,3]], "bbbb"),
        vec![4,2,1,1]
    );

    assert_eq!(
        call(5, vec![vec![0,1],vec![0,2],vec![1,3],vec![0,4]], "aabab"),
        vec![3,2,1,1,1]
    );
}

fn call(n: i32, edges: Vec<Vec<i32>>, labels: &str) -> Vec<i32> {
    Solution::count_sub_trees(n, edges, String::from(labels))
}
