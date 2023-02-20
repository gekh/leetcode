pub struct Solution;

impl Solution {
  pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let len = nums.len();
    let mut res = vec![];
    for i in 0..len - 2 {
      if i > 0 && nums[i] == nums[i - 1] {
        continue;
      }
      let (mut l, mut r) = (i + 1, len - 1);
      while l < r {
        let sum = nums[i] + nums[l] + nums[r];

        if sum < 0 {
          l += 1;
        } else if sum > 0 {
          r -= 1;
        } else {
          res.push(vec![nums[i], nums[l], nums[r]]);
          l += 1;
          r -= 1;
          while l < r && nums[l - 1] == nums[l] {
            l += 1;
          }
          while l < r && nums[r + 1] == nums[r] {
            r -= 1;
          }
        }
      }
    }

    res
  }
}
