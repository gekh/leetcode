use super::p278_first_bad_version::Solution;

pub fn test() {
    // assert!(call(500, 123) == 123);
    // assert!(call(213_324_245, 123) == 123);
    assert!(call(2126753390, 1702766719) == 1702766719);
}

fn call(n: i32, v: i32) -> i32 {
    Solution{v}.first_bad_version(n)
}
