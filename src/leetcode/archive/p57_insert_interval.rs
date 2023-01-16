pub struct Solution;

impl Solution {
    pub fn insert(all: Vec<Vec<i32>>, new: Vec<i32>) -> Vec<Vec<i32>> {
        if all.len() == 0 {
            return vec![new];
        }
        let mut result = vec![];
        let mut x1;
        let mut y1 = -1;
        let (x2, y2) = (new[0], new[1]);
        let mut prev_y1;
        let mut ins_x = -1;
        for (_, el) in all.iter().enumerate() {
            prev_y1 = y1;
            x1 = el[0];
            y1 = el[1];

            if ins_x == -1 {
                if x2 >= x1 && x2 <= y1 {
                    ins_x = x1;
                } else if x2 < x1 {
                    ins_x = x2;
                }
            }

            if ins_x != -1 {
                if y2 > prev_y1 && y2 < x1 {
                    result.push(vec![ins_x, y2]);
                }

                if y2 >= x1 && y2 <= y1 {
                    result.push(vec![ins_x, y1]);
                }
            }

            if y1 < x2 || x1 > y2 {
                result.push(vec![x1, y1]);
            }
        }

        if y2 > y1 {
            if x2 > y1 {
                result.push(vec![x2, y2]);
            } else {
                result.push(vec![ins_x, y2]);
            }
        }

        result
    }
}
