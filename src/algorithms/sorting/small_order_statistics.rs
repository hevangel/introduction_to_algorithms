//! @title Selecting Small Order Statistics
//! @chapter Problem 9-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Find the i-th smallest of n elements for small i with about n + O(i lg(n/i)) comparisons by pairing elements, recursing on the pair minima, and examining at most 2i candidates.
//! @order 222
use crate::outln;

/// Sort `v` while counting comparisons.
fn counted_sort(v: &mut Vec<(i64, usize)>, cnt: &mut usize) { v.sort_by(|a, b| { *cnt += 1; a.cmp(b) }); }

/// i-th smallest (1-based) of distinct (value, id) items. Returns the item and increments `cnt` for every comparison made.
fn small_stat(a: &[(i64, usize)], i: usize, cnt: &mut usize) -> (i64, usize) {
    let n = a.len();
    if n <= 3 || 2 * i >= n { let mut v = a.to_vec(); counted_sort(&mut v, cnt); return v[i - 1]; }
    // pair up the elements: one comparison per pair; `mins` holds the smaller of each pair, `partner` the larger one
    let mut mins: Vec<(i64, usize)> = vec![];
    let mut partner = std::collections::HashMap::new();
    for ch in a.chunks(2) {
        if ch.len() == 2 {
            *cnt += 1;
            let (lo, hi) = if ch[0] < ch[1] { (ch[0], ch[1]) } else { (ch[1], ch[0]) };
            mins.push(lo);
            partner.insert(lo, hi);
        } else { mins.push(ch[0]); }
    }
    // the i-th smallest overall is among the i smallest pair-minima and their partners (at most 2i candidates)
    let m = small_stat(&mins, i, cnt);
    let mut cand: Vec<(i64, usize)> = vec![];
    for &x in mins.iter().filter(|&&x| x <= m) { // membership test x ≤ m costs one comparison each in a real implementation
        *cnt += 1;
        cand.push(x);
        if let Some(&h) = partner.get(&x) { cand.push(h); }
    }
    counted_sort(&mut cand, cnt);
    cand[i - 1]
}

/// The i-th smallest value (1 ≤ i ≤ n) and the number of comparisons used.
pub fn small_order_statistic(a: &[i64], i: usize) -> (i64, usize) {
    assert!((1..=a.len()).contains(&i));
    let items: Vec<(i64, usize)> = a.iter().copied().enumerate().map(|(id, v)| (v, id)).collect(); // ids break ties
    let mut cnt = 0;
    let r = small_stat(&items, i, &mut cnt);
    (r.0, cnt)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = crate::common::Rng::new(3);
    let n = 10_000;
    let a = rng.vec(n, 0, 1_000_000);
    let mut s = a.clone();
    s.sort();
    for i in [1usize, 2, 5, 20, 100] {
        let (v, c) = small_order_statistic(&a, i);
        outln!(o, "i = {i:>3}: value {v} (correct: {}), {c} comparisons = {:.2}·n  (full sort ≈ {:.1}·n)", v == s[i - 1], c as f64 / n as f64, (n as f64).log2());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_sorting() {
        let mut r = Rng::new(452);
        for _ in 0..300 {
            let n = r.range(1, 80);
            let a = r.vec(n, 0, 40); // duplicates allowed: ids break ties
            let mut s = a.clone();
            s.sort();
            for i in 1..=n.min(12) { assert_eq!(small_order_statistic(&a, i).0, s[i - 1], "n={n} i={i}"); }
        }
    }
    #[test]
    fn few_comparisons_for_small_i() {
        let mut r = Rng::new(453);
        let n = 4096;
        let a = r.vec(n, 0, 1 << 30);
        for (i, factor) in [(1usize, 2.5f64), (4, 3.5), (16, 6.0)] {
            let c = small_order_statistic(&a, i).1;
            assert!((c as f64) < factor * n as f64, "i={i} used {c} comparisons");
        }
    }
}
