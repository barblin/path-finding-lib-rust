pub(crate) struct UnionFind {
    sizes: Vec<usize>,
    ids: Vec<usize>,
    components: usize,
}

impl UnionFind {
    pub fn from(node_count: usize) -> UnionFind {
        let sizes = vec![1; node_count];
        let mut ids = Vec::with_capacity(node_count);
        let components = node_count;

        for i in 0..node_count {
            ids.push(i);
        }

        return UnionFind {
            sizes,
            ids,
            components,
        };
    }

    fn find(&mut self, mut p: usize) -> usize {
        let mut root = p;

        while root != self.ids[root] {
            root = self.ids[root];
        }

        while p != root {
            let next = self.ids[p];
            self.ids[p] = root;
            p = next;
        }

        return root;
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        return self.find(p) == self.find(q);
    }

    pub fn unify(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root {
            return;
        }

        if self.sizes[p_root] < self.sizes[q_root] {
            self.sizes[q_root] += self.sizes[p_root];
            self.ids[p_root] = self.ids[q_root];
        } else {
            self.sizes[p_root] += self.sizes[q_root];
            self.ids[q_root] = self.ids[p_root];
        }

        self.components -= 1;
    }

    pub fn size(&self, id: usize) -> usize {
        return self.sizes[id];
    }

    pub fn parent(&self, id: usize) -> usize {
        return self.ids[id];
    }
}

#[test]
fn union_find_with_zero_edges_should_succeed() {
    let union_find = UnionFind::from(0);

    assert_eq!(0, union_find.components)
}

#[test]
fn unify_should_decrease_components() {
    let mut union_find = UnionFind::from(2);

    assert_eq!(2, union_find.components);

    union_find.unify(0, 1);
    assert_eq!(1, union_find.components);
    assert_eq!(2, union_find.size(0));
    assert_eq!(0, union_find.parent(1));
}