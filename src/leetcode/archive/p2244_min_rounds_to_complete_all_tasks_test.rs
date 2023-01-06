use super::p2244_min_rounds_to_complete_all_tasks::Solution;

pub fn test() {
    assert!(call(vec![2,2,3,3,2,4,4,4,4,4]) == 4);
    assert!(call(vec![2,3,3]) == -1);
    assert!(call(vec![2,3]) == -1);
    assert!(call(vec![2]) == -1);
    assert!(call(vec![2;1]) == -1);
    assert!(call(vec![2;2]) == 1);
    assert!(call(vec![2;3]) == 1);
    assert!(call(vec![2;4]) == 2);
    assert!(call(vec![2;5]) == 2);
    assert!(call(vec![2;6]) == 2);
    assert!(call(vec![2;7]) == 3);
    assert!(call(vec![2;8]) == 3);
    assert!(call(vec![2;9]) == 3);
    assert!(call(vec![2;10]) == 4);
    assert!(call(vec![2;11]) == 4);
    assert!(call(vec![2;12]) == 4);
    assert!(call(vec![2;13]) == 5);
    assert!(call(vec![2;14]) == 5);
    assert!(call(vec![2;15]) == 5);
}

fn call(tasks: Vec<i32>) -> i32 {
    Solution::minimum_rounds(tasks)
}
