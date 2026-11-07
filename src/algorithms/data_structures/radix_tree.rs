//! @title Radix Trees
//! @chapter Problem 12-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary A binary trie over bit strings: insert, search and sort a set of binary strings in Θ(total length).
//! @order 115
use crate::outln;

#[derive(Debug, Default, Clone)]
struct Node { child: [Option<usize>; 2], terminal: bool }

/// Radix tree (binary trie) storing strings of '0'/'1' characters.
#[derive(Debug, Clone)]
pub struct RadixTree { nodes: Vec<Node>, pub len: usize }

impl Default for RadixTree { fn default() -> Self { Self::new() } }

impl RadixTree {
    pub fn new() -> Self { RadixTree { nodes: vec![Node::default()], len: 0 } }
    /// Insert in O(|s|). Returns false if already present.
    pub fn insert(&mut self, s: &str) -> bool {
        let mut x = 0;
        for c in s.bytes() {
            let b = (c - b'0') as usize;
            x = match self.nodes[x].child[b] { Some(n) => n, None => { self.nodes.push(Node::default()); let n = self.nodes.len() - 1; self.nodes[x].child[b] = Some(n); n } };
        }
        let fresh = !self.nodes[x].terminal;
        self.nodes[x].terminal = true;
        if fresh { self.len += 1; }
        fresh
    }
    /// Search in O(|s|).
    pub fn contains(&self, s: &str) -> bool {
        let mut x = 0;
        for c in s.bytes() {
            match self.nodes[x].child[(c - b'0') as usize] { Some(n) => x = n, None => return false }
        }
        self.nodes[x].terminal
    }
    /// All stored strings in lexicographic order: a preorder walk (a string comes before its extensions, the 0-branch before the 1-branch).
    pub fn sorted(&self) -> Vec<String> {
        let mut out = vec![];
        let mut stack = vec![(0usize, String::new())];
        while let Some((x, p)) = stack.pop() {
            if self.nodes[x].terminal { out.push(p.clone()); }
            if let Some(n) = self.nodes[x].child[1] { stack.push((n, format!("{p}1"))); }
            if let Some(n) = self.nodes[x].child[0] { stack.push((n, format!("{p}0"))); }
        }
        out
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
}

/// Problem 12-2: sort distinct binary strings in Θ(Σ|s|) time by inserting them into a radix tree and walking it in preorder.
pub fn radix_sort_strings(strings: &[&str]) -> Vec<String> {
    let mut t = RadixTree::new();
    for s in strings { t.insert(s); }
    t.sorted()
}

pub fn demo() -> String {
    let mut o = String::new();
    let words = ["1011", "10", "011", "100", "0", "1011", "101", "1"];
    let mut t = RadixTree::new();
    for w in words { t.insert(w); }
    outln!(o, "inserted {:?}", words);
    outln!(o, "{} distinct strings in {} trie nodes", t.len, t.node_count());
    outln!(o, "sorted: {:?}", t.sorted());
    outln!(o, "contains '100'? {}   contains '1010'? {}   contains '01'? {}", t.contains("100"), t.contains("1010"), t.contains("01"));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_like_the_standard_library_and_membership() {
        let mut r = Rng::new(320);
        for _ in 0..100 {
            let strs: Vec<String> = (0..r.range(0, 40)).map(|_| (0..r.range(0, 8)).map(|_| if r.range(0, 1) == 1 { '1' } else { '0' }).collect()).collect();
            let refs: Vec<&str> = strs.iter().map(|s| s.as_str()).collect();
            let mut want: Vec<String> = strs.clone();
            want.sort();
            want.dedup();
            assert_eq!(radix_sort_strings(&refs), want);
            let mut t = RadixTree::new();
            refs.iter().for_each(|s| { t.insert(s); });
            assert!(refs.iter().all(|s| t.contains(s)));
            assert_eq!(t.len, want.len());
            assert!(!t.contains("1111111111"));
        }
    }
    #[test]
    fn prefixes_are_not_members_unless_inserted() {
        let mut t = RadixTree::new();
        assert!(t.insert("1010"));
        assert!(!t.insert("1010"));
        assert!(!t.contains("101") && !t.contains("10101"));
        assert!(t.insert("") && t.contains(""));
        assert_eq!(t.sorted(), vec!["", "1010"]);
    }
}
