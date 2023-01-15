use std::collections::{BTreeMap, HashMap};

pub struct Solution;

/** Algo: Union Find */
impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        if edges.len() == 0 {
            return 1;
        }

        let mut btm = BTreeMap::new();

        for (i, el) in vals.iter().enumerate() {
            btm.entry(el).or_insert(vec![]).push(i);
        }

        let mut map = HashMap::new();

        for edge in edges {
            map.entry(edge[0] as usize)
                .or_insert(vec![])
                .push(edge[1] as usize);
            map.entry(edge[1] as usize)
                .or_insert(vec![])
                .push(edge[0] as usize);
        }

        let mut uf = UnionFind::new(vals.len());
        let mut result = 0;

        for (&v, nodes) in btm {
            for &n in &nodes {
                for &nn in &map[&n] {
                    if vals[nn] <= v {
                        uf.union(n, nn)
                    }
                }
            }

            let mut sizes = HashMap::new();
            for &n in &nodes {
                let e = sizes.entry(uf.find(n)).or_insert(0);
                *e += 1;
                result += *e;
            }
        }

        result
    }
}

pub struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            par: (0..size).collect::<Vec<_>>(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        let mut y = x;
        while self.par[y] != y {
            y = self.par[y];
        }

        self.par[x] = y;
        y
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let (x, y) = (self.find(x), self.find(y));

        if x < y {
            self.par[y] = x;
        } else {
            self.par[x] = y;
        }
    }
}
