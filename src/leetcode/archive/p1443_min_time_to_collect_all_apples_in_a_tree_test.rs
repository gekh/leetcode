use super::p1443_min_time_to_collect_all_apples_in_a_tree::Solution;

pub fn test() {
    assert_eq!(
        call(
            7,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]],
            vec![false,false,true,false,true,true,false],
        ),
        8
    );

    assert_eq!(
        call(
            7,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]],
            vec![false,false,true,false,false,true,false],
        ),
        6
    );

    assert_eq!(
        call(
            7,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6]],
            vec![false,false,false,false,false,false,false],
        ),
        0
    );

    assert_eq!(
        call(
            12,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6],vec![4,7],vec![5,9],vec![7,8],vec![9,10],vec![10,11]],
            vec![false,false,true,false,true,true,false,false,true,false,false,true],
        ),
        18
    );

    assert_eq!(
        call(
            12,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6],vec![4,7],vec![5,9],vec![7,8],vec![9,10],vec![10,11]],
            vec![false,false,true,false,true,false,false,false,true,false,false,false],
        ),
        10
    );

    assert_eq!(
        call(
            12,
            vec![vec![0,1],vec![0,2],vec![1,4],vec![1,5],vec![2,3],vec![2,6],vec![4,7],vec![5,9],vec![7,8],vec![9,10],vec![10,11]],
            vec![false,false,false,false,false,false,false,false,false,false,false,true],
        ),
        10
    );

    assert_eq!(
        call(
            12,
            vec![vec![0,1],vec![1,2],vec![2,3],vec![1,4],vec![2,5],vec![2,6],vec![4,7]],
            vec![true,true,false,true,false,true,true,false],
        ),
        10
    );

    assert_eq!(
        call(
            4,
            vec![vec![0,2],vec![0,3],vec![1,2]],
            vec![false,true,false,false],
        ),
        4
    );

    assert_eq!(
        call(
            6,
            vec![vec![0,1],vec![1,2],vec![0,3],vec![2,4],vec![0,5]],
            vec![false,true,true,false,false,false],
        ),
        4
    );

    assert_eq!(call( 1, vec![vec![0,1]], vec![false, false], ), 0 );
    assert_eq!(call( 1, vec![vec![0,1]], vec![true, false], ), 0 );
    assert_eq!(call( 1, vec![vec![0,1]], vec![true, true], ), 2 );
    assert_eq!(call( 1, vec![vec![0,1]], vec![false, true], ), 2 );

}

fn call(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
    Solution::min_time(n, edges, has_apple)
}
