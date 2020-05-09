use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        // initialize
        let mut v: Vec<usize> = vec![];
        for i in 0..n + 1 {
            v.push(i)
        }
        let mut s: Vec<usize> = vec![1; n + 1];

        UnionFind { parent: v, size: s }
    }

    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        } else {
            return self.root(self.parent[x]);
        }
    }

    fn size(&self, a: usize) -> usize {
        let ra = self.root(a);
        self.size[ra]
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb)
        };
        self.size[ra] += self.size[rb];
        self.parent[rb] = ra;
    }

    fn same(&self, a: usize, b: usize) -> bool {
        let ra = self.root(a);
        let rb = self.root(b);
        self.parent[a] == self.parent[b]
    }
}

fn main() {
    let mut uf = UnionFind::new(12);
    uf.unite(1, 3);
    uf.unite(1, 5);
    uf.unite(1, 7);
    uf.unite(1, 8);
    uf.unite(1, 10);
    uf.unite(1, 12);

    uf.unite(4, 6);
    uf.unite(4, 9);
    uf.unite(4, 11);
    let x: usize = read();
    let y: usize = read();
    if uf.same(x, y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
