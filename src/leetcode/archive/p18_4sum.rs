pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
      nums.sort_unstable();

      let len = nums.len();
      let mut res = vec![];

      if len < 4 {
        return res;
      }

      for i in 0..len - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
          continue;
        }

        for j in i + 1..len - 2 {

          if j > i+1 && nums[j] == nums[j - 1] {
            continue;
          }

          let (mut l, mut r) = (j + 1, len - 1);

          while l < r {
            let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;

            if sum < target as i64 {
              l += 1;
            } else if sum > target as i64 {
              r -= 1;
            } else {
              res.push(vec![nums[i], nums[j], nums[l], nums[r]]);
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
      }

      res
    }
  }
