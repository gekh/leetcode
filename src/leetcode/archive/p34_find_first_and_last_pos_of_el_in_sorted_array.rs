pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut count = 0;
        while l <= r {
            count += 1;
            if count > 10 {
                break;
            }
            let cur = (l + r) / 2;
            if nums[cur] < target {
                l = cur + 1
            } else if nums[cur] > target {
                if cur == 0 {
                    break;
                }
                r = cur - 1
            } else {
                l = cur;
                while l > 0 && nums[l - 1] == target {
                    l -= 1;
                }

                r = cur;
                while r < nums.len() - 1 && nums[r + 1] == target {
                    r += 1;
                }

                return vec![l as i32, r as i32];
            }
        }

        vec![-1, -1]
    }
}
