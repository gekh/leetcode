pub struct Solution;

/** Algo: Uninon Find */
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        fn find(map: &[usize], start: usize) -> usize {
            std::iter::successors(Some(start), |&cur| if map[cur] == cur { None } else {
                Some(map[cur])
            }).last().unwrap()
        }

        fn union(map: &mut [usize], x: usize, y: usize) {
            let (x,y) = (find(map, x), find(map, y));
            if x != y {
                let (from, to) = (x.max(y), x.min(y));
                map[from] = to;
            }
        }

        fn c2u(c:char) -> usize {(c as u8 - b'a') as _}
        fn u2c(u:usize) -> char {(u as u8 + b'a') as _}

        fn _print_map(uf: &[usize]) {
            println!();
            for i in 0..uf.len() {
                if uf[i] != i {
                    println!("{} => {}", u2c(i), u2c(uf[i]));
                }
            }
            println!();
        }

        let mut map = [0;26];
        for i in 0..26 {
            map[i] = i;
        }

        map = s1.chars().zip(s2.chars()).fold(map, |mut map, (x, y)| {
            union(&mut map, c2u(x), c2u(y));
            map
        });

        base_str.chars().map(|x| u2c(
            find(&map,c2u(x))
        )).collect::<String>()
    }
}
