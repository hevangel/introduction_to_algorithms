//! @title MST with Mergeable Heaps
//! @chapter Problem 19-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Keep every component's outgoing edges in a mergeable heap; repeatedly extract a component's lightest edge, skip edges that became internal, union, and merge the heaps: O(E lg V) with leftist heaps.
//! @order 86
use crate::outln;
use std::collections::VecDeque;

pub type Edge = (usize, usize, i64);

/// A leftist heap (mergeable min-heap) of (weight, edge id).
#[derive(Debug)]
struct Heap { key: (i64, usize), rank: usize, left: Option<Box<Heap>>, right: Option<Box<Heap>> }
type H = Option<Box<Heap>>;

fn rank(h: &H) -> usize { h.as_ref().map_or(0, |n| n.rank) }

/// MERGE in O(lg n): merge along the right spines, swapping children to keep the right spine short.
fn merge(a: H, b: H) -> H {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(mut x), Some(mut y)) => {
            if x.key > y.key { std::mem::swap(&mut x, &mut y); }
            x.right = merge(x.right.take(), Some(y));
            if rank(&x.left) < rank(&x.right) { std::mem::swap(&mut x.left, &mut x.right); }
            x.rank = rank(&x.right) + 1;
            Some(x)
        }
    }
}

fn pop(h: H) -> Option<((i64, usize), H)> { h.map(|n| { let n = *n; (n.key, merge(n.left, n.right)) }) }

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }

/// MST-MERGEABLE-HEAP: returns (total weight, chosen edge ids). On a disconnected graph it returns a minimum spanning forest.
pub fn mst_mergeable_heaps(n: usize, edges: &[Edge]) -> (i64, Vec<usize>) {
    let mut heaps: Vec<H> = (0..n).map(|_| None).collect();
    for (i, &(u, v, w)) in edges.iter().enumerate() {
        if u == v { continue; }
        for x in [u, v] { let single = Some(Box::new(Heap { key: (w, i), rank: 1, left: None, right: None })); heaps[x] = merge(heaps[x].take(), single); }
    }
    let mut p: Vec<usize> = (0..n).collect();
    let mut queue: VecDeque<usize> = (0..n).collect();
    let (mut total, mut chosen) = (0, vec![]);
    while let Some(c) = queue.pop_front() {
        if find(&mut p, c) != c { continue; } // already merged into another component
        // the lightest edge leaving component c is safe (cut property); discard edges that became internal
        let mut h = heaps[c].take();
        let found = loop {
            match pop(h) {
                None => { h = None; break None; }
                Some(((w, id), rest)) => {
                    h = rest;
                    let (a, b) = (find(&mut p, edges[id].0), find(&mut p, edges[id].1));
                    if a != b { break Some((w, id, if a == c { b } else { a })); }
                }
            }
        };
        match found {
            None => {} // no edge leaves this component: it is a finished tree of the spanning forest
            Some((w, id, other)) => {
                total += w;
                chosen.push(id);
                p[other] = c;
                heaps[c] = merge(h, heaps[other].take()); // UNION of the heaps
                queue.push_back(c);
            }
        }
    }
    (total, chosen)
}

pub fn demo() -> String {
    let mut o = String::new();
    let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let (w, ids) = mst_mergeable_heaps(9, &e);
    outln!(o, "MST weight {w} (Kruskal {}), edges {:?}", super::mst_kruskal::kruskal(9, &e).0, ids.iter().map(|&i| e[i]).collect::<Vec<_>>());
    let mut r = crate::common::Rng::new(2);
    let n = 20_000;
    let mut big: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 100_000) as i64)).collect();
    for _ in 0..60_000 { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { big.push((a, b, r.range(1, 100_000) as i64)); } }
    outln!(o, "random graph n = {n}, m = {}: weight {} = Kruskal {}", big.len(), mst_mergeable_heaps(n, &big).0, super::mst_kruskal::kruskal(n, &big).0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn same_weight_and_forest_size_as_kruskal() {
        let mut r = Rng::new(446);
        for _ in 0..200 {
            let n = r.range(1, 25);
            let mut edges: Vec<Edge> = vec![];
            for v in 1..n { if r.f64() < 0.9 { edges.push((r.range(0, v - 1), v, r.range(1, 20) as i64)); } }
            for _ in 0..r.range(0, 40) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); edges.push((a, b, r.range(1, 20) as i64)); } // may include self-loops and parallel edges
            let (w, ids) = mst_mergeable_heaps(n, &edges);
            let (kw, kt) = super::super::mst_kruskal::kruskal(n, &edges);
            assert_eq!((w, ids.len()), (kw, kt.len()), "{edges:?}");
        }
    }
    #[test]
    fn leftist_heap_sorts() {
        let mut r = Rng::new(447);
        let keys: Vec<i64> = r.vec(500, 0, 1000);
        let mut h: H = None;
        for (i, &k) in keys.iter().enumerate() { h = merge(h, Some(Box::new(Heap { key: (k, i), rank: 1, left: None, right: None }))); }
        let mut out = vec![];
        while let Some((k, rest)) = pop(h) { out.push(k.0); h = rest; }
        let mut e = keys.clone();
        e.sort();
        assert_eq!(out, e);
    }
}
