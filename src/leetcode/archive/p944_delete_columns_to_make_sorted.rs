pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        (0..strs[0].len())
        .filter(|i|
            strs.iter()
            .map(|s| s.as_bytes()[*i])
            .collect::<Vec<u8>>()
            .windows(2)
            .all(|pair| pair[0] <= pair[1]) == false
        )
        .count() as _
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn min_deletion_size_CLASSIC(strs: Vec<String>) -> i32 {
        let mut r = 0;
        for j in 0..strs[0].len() {
            for i in 1.. strs.len() {
                if strs[i].as_bytes()[j] < strs[i-1].as_bytes()[j] {
                    r += 1;
                    break;
                }
            }
        }

        return r;
    }
}