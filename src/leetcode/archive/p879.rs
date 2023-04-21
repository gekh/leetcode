pub struct Solution;

impl Solution {
  pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = n as usize;
    let min_profit = min_profit as usize;

    let mut dp = vec![vec![0;min_profit+1]; n+1];
    dp[0][0] = 1;

    // println!("n:{n}");
    // println!("min_profit:{min_profit}");


    for i in 0..group.len() {
      // println!();
      let g = group[i] as usize;
      let p = profit[i] as usize;
      // println!("p:{p}, g:{g}");

      if n < g {
        continue;
      }

      for nn in (0..=(n-g)).rev() {
        // println!("{nn} {}", nn+g);
        for pp in 0..=min_profit {
          let minpp = min_profit.min(p+pp);
          dp[nn+g][minpp] = (dp[nn+g][minpp] + dp[nn][pp]) % 1_000_007;
        }
      }

      // println!();
      // println!("dp:");
      // for row in &dp {
      //   println!("{row:?}");
      // }
    }

    let mut out = 0;

    for i in 0..=n {
      out = (out + dp[i][min_profit]) % 1_000_007;
    }

    out
  }
}
