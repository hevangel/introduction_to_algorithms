//! @title Borůvka's Algorithm
//! @chapter Problem 23-2 (3e) / 21 problems (4e)
//! @editions 2,3,4
//! @summary Every component simultaneously picks its cheapest outgoing edge: at least half the components merge each round, O(E lg V) with simple data structures.
//! @order 84
use crate::outln;

pub type Edge = (usize, usize, i64);

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }

/// Borůvka: returns (total weight, chosen edge indices, number of rounds). Ties are broken by edge index, which prevents cycles.
/// On a disconnected graph it returns a minimum spanning forest.
pub fn boruvka(n: usize, edges: &[Edge]) -> (i64, Vec<usize>, usize) {
    let mut p: Vec<usize> = (0..n).collect();
    let (mut chosen, mut rounds) = (vec![], 0);
    loop {
        // cheapest[c] = lightest edge leaving component c (weight, index) — lexicographic order breaks ties consistently
        let mut cheapest: Vec<Option<(i64, usize)>> = vec![None; n];
        for (i, &(u, v, w)) in edges.iter().enumerate() {
            let (a, b) = (find(&mut p, u), find(&mut p, v));
            if a == b { continue; }
            for c in [a, b] { if cheapest[c].is_none_or(|x| (w, i) < x) { cheapest[c] = Some((w, i)); } }
        }
        let picks: Vec<usize> = cheapest.iter().flatten().map(|&(_, i)| i).collect();
        if picks.is_empty() { break; }
        rounds += 1;
        for i in picks {
            let (a, b) = (find(&mut p, edges[i].0), find(&mut p, edges[i].1));
            if a != b { p[a] = b; chosen.push(i); }
        }
    }
    (chosen.iter().map(|&i| edges[i].2).sum(), chosen, rounds)
}

pub fn demo() -> String {
    let mut o = String::new();
    let e: Vec<Edge> = vec![(0, 1, 4), (0, 7, 8), (1, 2, 8), (1, 7, 11), (2, 3, 7), (2, 5, 4), (2, 8, 2), (3, 4, 9), (3, 5, 14), (4, 5, 10), (5, 6, 2), (6, 7, 1), (6, 8, 6), (7, 8, 7)];
    let (w, edges, rounds) = boruvka(9, &e);
    outln!(o, "Borůvka MST weight {w} (Kruskal {}) after {rounds} rounds (≤ ⌈lg 9⌉ = 4)", super::mst_kruskal::kruskal(9, &e).0);
    outln!(o, "edges: {:?}", edges.iter().map(|&i| (e[i].0, e[i].1, e[i].2)).collect::<Vec<_>>());
    let mut r = crate::common::Rng::new(1);
    let n = 5000;
    let mut big: Vec<Edge> = (1..n).map(|v| (r.range(0, v - 1), v, r.range(1, 1000) as i64)).collect();
    for _ in 0..20_000 { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { big.push((a, b, r.range(1, 1000) as i64)); } }
    outln!(o, "random graph n = 5000, m = {}: {} rounds (lg n = {:.1})", big.len(), boruvka(n, &big).2, (n as f64).log2());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_kruskal_even_with_tied_weights() {
        let mut r = Rng::new(353);
        for _ in 0..150 {
            let n = r.range(1, 20);
            let mut edges: Vec<Edge> = vec![];
            for v in 1..n { if r.f64() < 0.9 { edges.push((r.range(0, v - 1), v, r.range(1, 6) as i64)); } } // small weight range → many ties; maybe disconnected
            for _ in 0..r.range(0, 30) { let (a, b) = (r.range(0, n - 1), r.range(0, n - 1)); if a != b { edges.push((a, b, r.range(1, 6) as i64)); } }
            let (w, chosen, rounds) = boruvka(n, &edges);
            let (kw, kt) = super::super::mst_kruskal::kruskal(n, &edges);
            assert_eq!((w, chosen.len()), (kw, kt.len()));
            assert!(rounds <= (n as f64).log2().ceil() as usize + 1, "rounds {rounds} n {n}");
        }
    }
}
