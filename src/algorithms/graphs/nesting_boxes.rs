//! @title Nesting Boxes
//! @chapter Problem 15-? (1e–2e); 24-2 (3e) / 22-2 (4e)
//! @editions 2,3,4
//! @summary d-dimensional boxes nest if some rotation fits strictly inside; sort each box's dimensions and find the longest chain in the nesting DAG.
//! @order 126
use crate::outln;

/// Does box `a` nest inside box `b` (after some permutation of a's dimensions)? Equivalent to comparing sorted dimensions component-wise.
pub fn nests(a: &[i64], b: &[i64]) -> bool {
    let (mut x, mut y) = (a.to_vec(), b.to_vec());
    x.sort();
    y.sort();
    x.iter().zip(&y).all(|(p, q)| p < q)
}

/// Longest sequence B₁ ⊂ B₂ ⊂ … (each nests in the next). Returns indices of one longest chain, innermost first.
/// O(n²·d + n·d lg d): sort dimensions once, then DP over boxes ordered by volume-like key (the nesting relation is acyclic).
pub fn longest_nesting_chain(boxes: &[Vec<i64>]) -> Vec<usize> {
    let sorted: Vec<Vec<i64>> = boxes.iter().map(|b| { let mut s = b.clone(); s.sort(); s }).collect();
    let n = boxes.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| sorted[i].iter().sum::<i64>()); // if a nests in b then sum(a) < sum(b): a topological order
    let (mut best, mut pred) = (vec![1usize; n], vec![usize::MAX; n]);
    for (pos, &j) in order.iter().enumerate() {
        for &i in &order[..pos] {
            if sorted[i].iter().zip(&sorted[j]).all(|(p, q)| p < q) && best[i] + 1 > best[j] { best[j] = best[i] + 1; pred[j] = i; }
        }
    }
    let Some(end) = (0..n).max_by_key(|&i| best[i]) else { return vec![] };
    let (mut chain, mut x) = (vec![], end);
    while x != usize::MAX { chain.push(x); x = pred[x]; }
    chain.reverse();
    chain
}

pub fn demo() -> String {
    let mut o = String::new();
    let boxes = vec![vec![1, 5, 3], vec![4, 2, 6], vec![7, 8, 9], vec![2, 6, 4], vec![3, 7, 5], vec![10, 2, 2]];
    outln!(o, "boxes {:?}", boxes);
    outln!(o, "box 0 nests in box 1? {}   box 1 nests in 4? {}   box 5 nests in 2? {}", nests(&boxes[0], &boxes[1]), nests(&boxes[1], &boxes[4]), nests(&boxes[5], &boxes[2]));
    let chain = longest_nesting_chain(&boxes);
    outln!(o, "longest nesting chain (innermost first): {:?} = {:?}", chain, chain.iter().map(|&i| &boxes[i]).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn permutations(d: usize) -> Vec<Vec<usize>> {
        if d == 0 { return vec![vec![]]; }
        let mut out = vec![];
        for p in permutations(d - 1) { for pos in 0..=p.len() { let mut q = p.clone(); q.insert(pos, d - 1); out.push(q); } }
        out
    }
    #[test]
    fn sorted_comparison_equals_trying_all_rotations() {
        let mut r = Rng::new(360);
        for _ in 0..500 {
            let d = r.range(1, 4);
            let (a, b) = (r.vec(d, 1, 8), r.vec(d, 1, 8));
            let brute = permutations(d).iter().any(|p| (0..d).all(|i| a[p[i]] < b[i]));
            assert_eq!(nests(&a, &b), brute, "{a:?} {b:?}");
        }
    }
    #[test]
    fn chain_matches_exhaustive_subset_search() {
        let mut r = Rng::new(361);
        for _ in 0..80 {
            let (n, d) = (r.range(1, 8), r.range(1, 3));
            let boxes: Vec<Vec<i64>> = (0..n).map(|_| r.vec(d, 1, 8)).collect();
            let chain = longest_nesting_chain(&boxes);
            assert!(chain.windows(2).all(|w| nests(&boxes[w[0]], &boxes[w[1]])));
            let brute = (0u32..1 << n).filter(|m| {
                let mut ids: Vec<usize> = (0..n).filter(|i| m >> i & 1 == 1).collect();
                ids.sort_by_key(|&i| { let mut s = boxes[i].clone(); s.sort(); s.iter().sum::<i64>() });
                ids.windows(2).all(|w| nests(&boxes[w[0]], &boxes[w[1]]))
            }).map(|m| m.count_ones() as usize).max().unwrap();
            assert_eq!(chain.len(), brute, "{boxes:?}");
        }
    }
}
