struct UnionFind {
    parent_indexs: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent_indexs = vec![0; size];
        for (i, idx) in parent_indexs.iter_mut().enumerate().take(size) {
            *idx = i;
        }

        Self { parent_indexs }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent_indexs[x] == x {
            x
        } else {
            self.parent_indexs[x] = self.root(self.parent_indexs[x]);
            self.parent_indexs[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);

        if rx != ry {
            self.parent_indexs[rx] = ry;
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);

        rx == ry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unite_and_same() {
        let mut uf = UnionFind::new(5);

        assert!(!uf.same(0, 1));
        assert!(!uf.same(0, 2));
        assert!(!uf.same(3, 2));

        uf.unite(0, 1);
        uf.unite(3, 2);
        assert!(uf.same(0, 1));
        assert!(uf.same(3, 2));

        uf.unite(1, 2);
        assert!(uf.same(0, 3));
    }
}
