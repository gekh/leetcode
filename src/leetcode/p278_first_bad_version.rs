pub struct Solution {
    pub v: i32
}

impl Solution {
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.v
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;

        while right != left {
            println!("{} {}", left, right);
            let i = (right / 2 + left / 2 + 1) as i32;

            if self.isBadVersion(i) {
                right = i - 1
            } else {
                left = i
            }
        }

        return left + 1;
    }
}
