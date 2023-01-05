use super::p452_min_number_of_arrow_to_burst_baloons::Solution;

pub fn test() {
    assert!(call(vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]]) == 2);
    assert!(call(vec![vec![0,4],vec![3,9],vec![6,7],vec![5,11],vec![10,11]]) == 3);
    assert!(call(vec![vec![3,9],vec![7,12],vec![3,8],vec![6,8],vec![9,10],vec![2,9],vec![0,9],vec![3,9],vec![0,6],vec![2,8]]) == 2);
    assert!(call(vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]]) == 4);
    assert!(call(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]) == 2);
    assert!(call(vec![vec![-1,1],vec![0,1],vec![2,3],vec![1,2]]) == 2);
}

fn call(points: Vec<Vec<i32>>) -> i32 {
    Solution::find_min_arrow_shots(points)
}
