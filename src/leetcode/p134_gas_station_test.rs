use super::p134_gas_station::{Solution};

pub fn test() {
    assert_eq!( call( vec![1,2,3,4,5], vec![3,4,5,1,2] ), 3 );
    assert_eq!( call( vec![2,3,4], vec![3,4,3] ), -1 );
    assert_eq!( call( vec![5,8,2,8], vec![6,5,6,6] ), 3 );
}

fn call(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    Solution::can_complete_circuit(gas, cost)
}

/*
1   2   3   4   5
3   4   5   1   2
-2  -2  -2  +3  +3
-4  -4  +1  +6  +1
*/

/*
5   8   2   8
6   5   6   6
-1  +3  -4  +2
+2  -1  -2  +1
*/

