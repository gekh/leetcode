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
