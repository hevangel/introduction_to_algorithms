//! @title Augmentation Problems: Maximum Overlap & Josephus
//! @chapter Problems 14-1, 14-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary The point covered by the most intervals via sorted endpoints, and the Josephus permutation in O(n lg n) with an order-statistic tree.
//! @order 135
use crate::algorithms::data_structures::order_statistic_tree::OsTree;
use crate::outln;

/// Problem 14-1: a point of maximum overlap among closed integer intervals and the number of intervals covering it.
/// Sort endpoints (starts before ends at equal coordinates), sweep keeping a running count: O(n lg n).
pub fn max_overlap(intervals: &[(i64, i64)]) -> Option<(i64, usize)> {
    let mut ev: Vec<(i64, u8)> = intervals.iter().flat_map(|&(l, r)| [(l, 0u8), (r, 1u8)]).collect();
    ev.sort();
    let (mut cur, mut best) = (0usize, None);
    for (x, kind) in ev {
        if kind == 0 { cur += 1; if best.is_none_or(|(_, c)| cur > c) { best = Some((x, cur)); } } else { cur -= 1; }
    }
    best
}

/// Problem 14-2, (b): Josephus permutation of {1..n} with step m using an order-statistic tree: O(n lg n).
pub fn josephus_os_tree(n: usize, m: usize) -> Vec<usize> {
    let mut t = OsTree::new();
    for i in 1..=n as i64 { t.insert(i); }
    let (mut pos, mut out) = (0usize, vec![]);
    for left in (1..=n).rev() {
        pos = (pos + m - 1) % left;                 // 0-based index of the next victim among the survivors
        let victim = t.select(pos + 1).unwrap();    // OS-SELECT
        t.delete(victim);
        out.push(victim as usize);
    }
    out
}

/// Problem 14-2, (c): only the survivor via J(1) = 0, J(k) = (J(k−1) + m) mod k in Θ(n).
pub fn josephus_survivor(n: usize, m: usize) -> usize { (1..=n).fold(0, |j, k| (j + m) % k) + 1 }

pub fn demo() -> String {
    let mut o = String::new();
    let iv = [(1, 5), (3, 8), (4, 6), (7, 10), (9, 12), (2, 4)];
    outln!(o, "intervals {:?}", iv);
    outln!(o, "point of maximum overlap: {:?}", max_overlap(&iv));
    outln!(o, "Josephus(7, 3) elimination order = {:?}", josephus_os_tree(7, 3));
    outln!(o, "survivor of Josephus(41, 3) = {} (closed form recurrence)", josephus_survivor(41, 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn overlap_matches_brute_force() {
        let mut r = Rng::new(330);
        for _ in 0..200 {
            let iv: Vec<(i64, i64)> = (0..r.range(1, 15)).map(|_| { let a = r.range(0, 30) as i64; (a, a + r.range(0, 10) as i64) }).collect();
            let (_, c) = max_overlap(&iv).unwrap();
            let brute = (0..45).map(|x| iv.iter().filter(|&&(l, rr)| l <= x && x <= rr).count()).max().unwrap();
            assert_eq!(c, brute);
            let (p, c2) = max_overlap(&iv).unwrap();
            assert_eq!(iv.iter().filter(|&&(l, rr)| l <= p && p <= rr).count(), c2);
        }
        assert_eq!(max_overlap(&[]), None);
    }
    #[test]
    fn josephus_agrees_across_methods() {
        assert_eq!(josephus_os_tree(7, 3), vec![3, 6, 2, 7, 5, 1, 4]);
        for n in 1..40 {
            for m in 1..8 {
                let order = josephus_os_tree(n, m);
                assert_eq!(*order.last().unwrap(), josephus_survivor(n, m), "n={n} m={m}");
                let mut s = order.clone();
                s.sort();
                assert_eq!(s, (1..=n).collect::<Vec<_>>());
            }
        }
    }
}
