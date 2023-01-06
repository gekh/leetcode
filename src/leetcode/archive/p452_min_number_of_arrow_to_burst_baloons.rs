pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_cached_key(|b| b[1]);

        Self::print(&points);

        let result = points
            .iter()
            .skip(1)
            .fold((1, points[0][1]), |(result, right), p| {
                if right < p[0] {
                    (result + 1, p[1])
                } else {
                    (result, right)
                }
            })
            .0;

        dbg!(result);
        println!();
        println!();

        result
    }

    fn print(points: &Vec<Vec<i32>>) {
        let len = points[points.len() - 1][1] + 10;
        println!();
        println!();
        for p in points {
            for _ in 0..p[0] {
                print!("_");
            }
            for _ in p[0]..=p[1] {
                print!("0");
            }
            for _ in (p[1] + 1)..=len {
                print!("_");
            }
            println!();
        }
        println!();
        println!();
    }
}
