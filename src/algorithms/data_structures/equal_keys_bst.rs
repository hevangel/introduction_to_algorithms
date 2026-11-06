//! @title Binary Search Trees with Equal Keys
//! @chapter Problem 12-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Four ways to insert duplicates into a BST — always right, alternate sides, keep a list at the node, choose randomly — and what each costs when n identical keys arrive.
//! @order 40
use crate::common::Rng;
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
    /// (a) Equal keys always go to the right subtree.
    AlwaysRight,
    /// (b) Each node carries a flag that alternates left / right for its equal keys.
    Alternate,
    /// (c) A node stores all keys equal to its own as a count (the book's linked list).
    CountAtNode,
    /// (d) Equal keys go left or right with probability ½.
    Random,
}

#[derive(Debug)]
struct Node { key: i64, left: Option<usize>, right: Option<usize>, flag: bool, copies: usize }

#[derive(Debug)]
pub struct Bst { nodes: Vec<Node>, root: Option<usize>, strategy: Strategy, rng: Rng }

impl Bst {
    pub fn new(strategy: Strategy) -> Self { Bst { nodes: vec![], root: None, strategy, rng: Rng::new(12) } }
    /// Insert and return the number of nodes visited (the depth of the new element).
    pub fn insert(&mut self, key: i64) -> usize {
        let new = self.nodes.len();
        self.nodes.push(Node { key, left: None, right: None, flag: false, copies: 1 });
        let Some(mut cur) = self.root else { self.root = Some(new); return 1; };
        let mut depth = 1;
        loop {
            let go_left = if key < self.nodes[cur].key { true } else if key > self.nodes[cur].key { false } else {
                match self.strategy {
                    Strategy::AlwaysRight => false,
                    Strategy::Alternate => { let f = self.nodes[cur].flag; self.nodes[cur].flag = !f; f }
                    Strategy::CountAtNode => { self.nodes[cur].copies += 1; self.nodes.pop(); return depth; }
                    Strategy::Random => self.rng.range(0, 1) == 0,
                }
            };
            let next = if go_left { self.nodes[cur].left } else { self.nodes[cur].right };
            match next {
                Some(n) => { cur = n; depth += 1; }
                None => { if go_left { self.nodes[cur].left = Some(new); } else { self.nodes[cur].right = Some(new); } return depth + 1; }
            }
        }
    }
    /// In-order list of keys (equal keys repeated according to `copies`).
    pub fn in_order(&self) -> Vec<i64> {
        fn walk(b: &Bst, n: Option<usize>, out: &mut Vec<i64>) {
            if let Some(i) = n { walk(b, b.nodes[i].left, out); for _ in 0..b.nodes[i].copies { out.push(b.nodes[i].key); } walk(b, b.nodes[i].right, out); }
        }
        let mut v = vec![];
        walk(self, self.root, &mut v);
        v
    }
}

/// Total number of nodes visited while inserting n identical keys.
pub fn cost_of_identical_keys(strategy: Strategy, n: usize) -> usize { let mut t = Bst::new(strategy); (0..n).map(|_| t.insert(7)).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "total nodes visited to insert n identical keys:");
    outln!(o, "{:>6} | {:>12} | {:>10} | {:>10} | {:>8}", "n", "always right", "alternate", "count", "random");
    for n in [100usize, 1000, 5000] {
        outln!(o, "{n:>6} | {:>12} | {:>10} | {:>10} | {:>8}", cost_of_identical_keys(Strategy::AlwaysRight, n), cost_of_identical_keys(Strategy::Alternate, n), cost_of_identical_keys(Strategy::CountAtNode, n), cost_of_identical_keys(Strategy::Random, n));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identical_key_costs_match_the_analysis() {
        let n = 512usize;
        assert_eq!(cost_of_identical_keys(Strategy::AlwaysRight, n), n * (n + 1) / 2);  // (a) Θ(n²)
        assert_eq!(cost_of_identical_keys(Strategy::CountAtNode, n), n);                // (c) Θ(n)
        let lg = (n as f64).log2();
        let alt = cost_of_identical_keys(Strategy::Alternate, n) as f64;
        assert!(alt <= n as f64 * (lg + 1.0), "alternate {alt}");                       // (b) Θ(n lg n): perfectly balanced
        let rnd = cost_of_identical_keys(Strategy::Random, n) as f64;
        assert!(rnd < n as f64 * 4.0 * lg && rnd > n as f64, "random {rnd}");           // (d) Θ(n lg n) expected
    }
    #[test]
    fn all_strategies_keep_the_inorder_sorted_and_complete() {
        let mut r = Rng::new(445);
        for strategy in [Strategy::AlwaysRight, Strategy::Alternate, Strategy::CountAtNode, Strategy::Random] {
            let mut t = Bst::new(strategy);
            let keys: Vec<i64> = (0..300).map(|_| r.range(0, 20) as i64).collect();
            for &k in &keys { t.insert(k); }
            let mut e = keys.clone();
            e.sort();
            assert_eq!(t.in_order(), e, "{strategy:?}");
        }
    }
}
