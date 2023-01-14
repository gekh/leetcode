pub struct Solution;

impl Solution {
    fn union_find(s1: String, s2: String) {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes(), base_str.as_bytes());
        let mut uf = [usize::MAX; 26];

        for i in 0..s1.len() {
            Self::union(&mut uf, Self::c2u(s1[i]), Self::c2u(s2[i]));
        }

        Self::print_uf(&uf);
        String::from("aba")
    }

    fn c2u(c: u8) -> usize {
        (c - b'a') as _
    }

    fn u2c(u: usize) -> char {
        (u as u8 + b'a') as _
    }

    fn print_uf(uf: &[usize; 26]) {
        for i in 0..uf.len() {
            if uf[i] != usize::MAX {
                println!("{} => {}", Self::u2c(i), Self::u2c(uf[i]));
            }
        }
    }

    fn union(mut uf: &mut [usize; 26], x: usize, y: usize) {
        let (x, y) = (Self::find(&mut uf, x), Self::find(&mut uf, y));

        if x < y {
            uf[y] = x;
        } else {
            uf[x] = y;
        }
    }

    fn find(uf: &mut [usize; 26], x: usize) -> usize {
        if uf[x] == x {
            return x;
        }

        if uf[x] == usize::MAX {
            uf[x] = x;
            return x;
        }

        let mut y = x;
        while uf[y] != y {
            y = uf[y]
        }
        uf[x] = y;
        y
    }
}
