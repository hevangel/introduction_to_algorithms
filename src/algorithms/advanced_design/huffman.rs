//! @title Huffman Codes
//! @chapter 16.3 (1e–3e) / 15.3 (4e)
//! @editions 1,2,3,4
//! @summary Optimal prefix-free codes by greedily merging the two least frequent symbols.
//! @order 170
use crate::outln;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(Debug, Clone)]
struct Node { freq: u64, sym: Option<char>, left: usize, right: usize }

/// A Huffman tree stored in an arena; `root` is the index of the root node.
#[derive(Debug, Clone)]
pub struct HuffmanTree { nodes: Vec<Node>, root: usize }

/// HUFFMAN(C): repeatedly extract the two minimum-frequency nodes (min-priority queue) and merge them.
pub fn huffman(freqs: &[(char, u64)]) -> HuffmanTree {
    assert!(!freqs.is_empty());
    let mut nodes: Vec<Node> = freqs.iter().map(|&(c, f)| Node { freq: f, sym: Some(c), left: usize::MAX, right: usize::MAX }).collect();
    let mut q: BinaryHeap<Reverse<(u64, usize)>> = (0..nodes.len()).map(|i| Reverse((nodes[i].freq, i))).collect();
    while q.len() > 1 {
        let Reverse((f1, a)) = q.pop().unwrap();
        let Reverse((f2, b)) = q.pop().unwrap();
        nodes.push(Node { freq: f1 + f2, sym: None, left: a, right: b });
        q.push(Reverse((f1 + f2, nodes.len() - 1)));
    }
    let Reverse((_, root)) = q.pop().unwrap();
    HuffmanTree { nodes, root }
}

impl HuffmanTree {
    /// Codeword for each symbol ('0' = left, '1' = right). A single-symbol alphabet gets "0".
    pub fn codes(&self) -> BTreeMap<char, String> {
        let mut m = BTreeMap::new();
        let mut st = vec![(self.root, String::new())];
        while let Some((v, code)) = st.pop() {
            let n = &self.nodes[v];
            if let Some(c) = n.sym { m.insert(c, if code.is_empty() { "0".to_string() } else { code }); }
            else {
                st.push((n.left, format!("{code}0")));
                st.push((n.right, format!("{code}1")));
            }
        }
        m
    }
    /// B(T) = Σ f(c)·d_T(c): total encoded length in bits.
    pub fn cost(&self) -> u64 {
        // the sum of all internal-node frequencies equals the weighted path length
        let internal: u64 = self.nodes.iter().filter(|n| n.sym.is_none()).map(|n| n.freq).sum();
        if internal == 0 { self.nodes[self.root].freq } else { internal }
    }
    pub fn encode(&self, text: &str) -> String {
        let codes = self.codes();
        text.chars().map(|c| codes[&c].as_str()).collect()
    }
    pub fn decode(&self, bits: &str) -> String {
        let mut out = String::new();
        let mut v = self.root;
        for b in bits.chars() {
            if self.nodes[self.root].sym.is_none() { v = if b == '0' { self.nodes[v].left } else { self.nodes[v].right }; }
            if let Some(c) = self.nodes[v].sym { out.push(c); v = self.root; }
            else if self.nodes[self.root].sym.is_some() { out.push(self.nodes[self.root].sym.unwrap()); }
        }
        out
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let f = [('a', 45), ('b', 13), ('c', 12), ('d', 16), ('e', 9), ('f', 5)];
    let t = huffman(&f);
    outln!(o, "frequencies (thousands): {:?}", f);
    for (c, code) in t.codes() { outln!(o, "  {c} → {code}"); }
    outln!(o, "Huffman cost = {} bits per 100 symbols (fixed 3-bit code would use 300)", t.cost());
    let msg = "abacabadabacaba";
    let t2 = huffman(&[('a', 8), ('b', 4), ('c', 2), ('d', 1)]);
    let enc = t2.encode(msg);
    outln!(o, "encode {:?} → {} ({} bits vs {} fixed)", msg, enc, enc.len(), msg.len() * 2);
    outln!(o, "decode → {:?}", t2.decode(&enc));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example_cost() {
        let t = huffman(&[('a', 45), ('b', 13), ('c', 12), ('d', 16), ('e', 9), ('f', 5)]);
        assert_eq!(t.cost(), 224);
        let codes = t.codes();
        assert_eq!(codes[&'a'].len(), 1);
        assert_eq!(codes[&'e'].len(), 4);
        assert_eq!(codes[&'f'].len(), 4);
    }
    #[test]
    fn prefix_free_roundtrip_and_cost_formula() {
        let mut r = Rng::new(114);
        for _ in 0..40 {
            let n = r.range(2, 12);
            let freqs: Vec<(char, u64)> = (0..n).map(|i| ((b'a' + i as u8) as char, r.range(1, 100) as u64)).collect();
            let t = huffman(&freqs);
            let codes = t.codes();
            for (a, ca) in &codes { for (b, cb) in &codes { if a != b { assert!(!cb.starts_with(ca.as_str())); } } }
            let direct: u64 = freqs.iter().map(|&(c, f)| f * codes[&c].len() as u64).sum();
            assert_eq!(direct, t.cost());
            let text: String = freqs.iter().flat_map(|&(c, f)| std::iter::repeat_n(c, (f % 5 + 1) as usize)).collect();
            assert_eq!(t.decode(&t.encode(&text)), text);
        }
    }
    #[test]
    fn optimality_on_two_symbol_and_single_symbol_alphabets() {
        assert_eq!(huffman(&[('x', 3), ('y', 7)]).cost(), 10);
        let single = huffman(&[('z', 5)]);
        assert_eq!(single.codes()[&'z'], "0");
        assert_eq!(single.decode(&single.encode("zzz")), "zzz");
    }
}
