pub struct Solution;

impl Solution {
    pub fn insert(all: Vec<Vec<i32>>, new: Vec<i32>) -> Vec<Vec<i32>> {
        let mut less = vec![];
        let mut more = vec![];

        let (mut new_x, mut new_y) = (new[0], new[1]);

        for el in all {
            let (x, y) = (el[0], el[1]);
            if y < new_x {
                less.push(el);
            } else if x > new_y {
                more.push(el);
            } else {
                if x < new_x {
                    new_x = x;
                }
                if y > new_y {
                    new_y = y;
                }
            }
        }

        vec![less, vec![vec![new_x, new_y]], more].concat()
    }
}
