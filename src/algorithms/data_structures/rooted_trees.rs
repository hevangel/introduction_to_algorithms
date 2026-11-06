//! @title Representing Rooted Trees
//! @chapter 10.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Left-child/right-sibling trees, stack-based and O(1)-space tree traversals.
//! @order 40
use crate::outln;

/// Rooted tree with unbounded branching using the left-child, right-sibling representation.
#[derive(Debug, Clone)]
pub struct RootedTree {
    pub key: Vec<i64>,
    pub left_child: Vec<Option<usize>>,
    pub right_sibling: Vec<Option<usize>>,
}

impl RootedTree {
    /// Build from per-node child lists; node 0 is the root.
    pub fn from_children(keys: &[i64], children: &[Vec<usize>]) -> Self {
        let n = keys.len();
        let mut t = RootedTree { key: keys.to_vec(), left_child: vec![None; n], right_sibling: vec![None; n] };
        for (p, ch) in children.iter().enumerate() {
            for w in ch.windows(2) { t.right_sibling[w[0]] = Some(w[1]); }
            t.left_child[p] = ch.first().copied();
        }
        t
    }
    pub fn children(&self, x: usize) -> Vec<usize> {
        let mut v = vec![];
        let mut c = self.left_child[x];
        while let Some(i) = c { v.push(i); c = self.right_sibling[i]; }
        v
    }
    /// Exercise 10.4-2: print every key recursively.
    pub fn preorder_recursive(&self, x: usize, out: &mut Vec<i64>) {
        out.push(self.key[x]);
        for c in self.children(x) { self.preorder_recursive(c, out); }
    }
    /// Exercise 10.4-3/4: the same traversal with an explicit stack.
    pub fn preorder_stack(&self, root: usize) -> Vec<i64> {
        let (mut out, mut st) = (vec![], vec![root]);
        while let Some(x) = st.pop() {
            out.push(self.key[x]);
            for c in self.children(x).into_iter().rev() { st.push(c); }
        }
        out
    }
    pub fn depth(&self, x: usize) -> usize {
        1 + self.children(x).into_iter().map(|c| self.depth(c)).max().unwrap_or(0)
    }
}

pub const NIL: usize = usize::MAX;

/// Binary tree with parent pointers.
#[derive(Debug, Clone, Default)]
pub struct BinTree {
    pub key: Vec<i64>,
    pub left: Vec<usize>,
    pub right: Vec<usize>,
    pub parent: Vec<usize>,
}

impl BinTree {
    /// Insert as in a (non-balancing) binary search tree; returns the root (0).
    pub fn insert_bst(&mut self, k: i64) {
        let z = self.key.len();
        self.key.push(k);
        self.left.push(NIL);
        self.right.push(NIL);
        self.parent.push(NIL);
        if z == 0 { return; }
        let mut x = 0;
        loop {
            let side = if k < self.key[x] { &mut self.left } else { &mut self.right };
            if side[x] == NIL { side[x] = z; self.parent[z] = x; return; }
            x = side[x];
        }
    }
    pub fn inorder_recursive(&self, x: usize, out: &mut Vec<i64>) {
        if x == NIL { return; }
        self.inorder_recursive(self.left[x], out);
        out.push(self.key[x]);
        self.inorder_recursive(self.right[x], out);
    }
    /// Exercise 10.4-5: inorder traversal with O(1) extra space using parent pointers and a `prev` marker.
    pub fn inorder_constant_space(&self) -> Vec<i64> {
        let mut out = vec![];
        if self.key.is_empty() { return out; }
        let (mut cur, mut prev) = (0usize, NIL);
        while cur != NIL {
            if prev == self.parent[cur] {
                // arrived from the parent: go left if possible
                if self.left[cur] != NIL { prev = cur; cur = self.left[cur]; continue; }
                out.push(self.key[cur]);
                let next = if self.right[cur] != NIL { self.right[cur] } else { self.parent[cur] };
                prev = cur; cur = next;
            } else if prev == self.left[cur] {
                // returned from the left child: visit, then right
                out.push(self.key[cur]);
                let next = if self.right[cur] != NIL { self.right[cur] } else { self.parent[cur] };
                prev = cur; cur = next;
            } else {
                // returned from the right child: go up
                prev = cur; cur = self.parent[cur];
            }
        }
        out
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    //        10
    //      / | \
    //     20 30 40
    //    / \     \
    //   50 60    70
    let t = RootedTree::from_children(&[10, 20, 30, 40, 50, 60, 70], &[vec![1, 2, 3], vec![4, 5], vec![], vec![6], vec![], vec![], vec![]]);
    let mut v = vec![];
    t.preorder_recursive(0, &mut v);
    outln!(o, "preorder (recursive): {:?}", v);
    outln!(o, "preorder (stack)    : {:?}", t.preorder_stack(0));
    outln!(o, "children of root    : {:?}, depth = {}", t.children(0), t.depth(0));
    let mut b = BinTree::default();
    for k in [8, 3, 10, 1, 6, 14, 4, 7, 13] { b.insert_bst(k); }
    outln!(o, "inorder, O(1) extra space: {:?}", b.inorder_constant_space());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn lcrs_traversals_agree() {
        let t = RootedTree::from_children(&[1, 2, 3, 4, 5], &[vec![1, 2], vec![3, 4], vec![], vec![], vec![]]);
        let mut a = vec![];
        t.preorder_recursive(0, &mut a);
        assert_eq!(a, vec![1, 2, 4, 5, 3]);
        assert_eq!(t.preorder_stack(0), a);
        assert_eq!(t.depth(0), 3);
    }
    #[test]
    fn constant_space_inorder_matches_recursive() {
        let mut r = Rng::new(51);
        for n in [0, 1, 2, 5, 30, 200] {
            let mut b = BinTree::default();
            for k in r.vec(n, 0, 1000) { b.insert_bst(k); }
            let mut rec = vec![];
            if n > 0 { b.inorder_recursive(0, &mut rec); }
            assert_eq!(b.inorder_constant_space(), rec);
        }
    }
}
