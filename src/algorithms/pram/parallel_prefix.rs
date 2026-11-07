//! @title Parallel Prefix (Scan)
//! @chapter 30.2 (1e)
//! @editions 1
//! @summary All prefix sums in ⌈lg n⌉ steps (recursive doubling) or with O(n) work (up-sweep / down-sweep).
//! @order 20
use crate::outln;

/// Hillis–Steele inclusive scan: in step k every processor i ≥ 2^k combines x[i−2^k] ⊕ x[i]. ⌈lg n⌉ rounds, Θ(n lg n) work.
pub fn prefix_doubling<T: Copy>(x: &[T], op: impl Fn(T, T) -> T) -> (Vec<T>, usize) {
    let n = x.len();
    let (mut cur, mut step, mut rounds) = (x.to_vec(), 1, 0);
    while step < n {
        let old = cur.clone();
        for i in step..n { cur[i] = op(old[i - step], old[i]); }
        step *= 2;
        rounds += 1;
    }
    (cur, rounds)
}

/// Work-efficient (Blelloch) exclusive scan with an identity element: up-sweep then down-sweep, 2·lg n rounds, O(n) work.
/// Input is padded internally to a power of two.
pub fn prefix_work_efficient(x: &[i64], op: impl Fn(i64, i64) -> i64, identity: i64) -> (Vec<i64>, usize, usize) {
    let n = x.len().next_power_of_two().max(1);
    let mut a = x.to_vec();
    a.resize(n, identity);
    let (mut rounds, mut ops) = (0, 0);
    let mut d = 1;
    while d < n {                                  // up-sweep (reduce)
        for i in (0..n).step_by(2 * d) { a[i + 2 * d - 1] = op(a[i + d - 1], a[i + 2 * d - 1]); ops += 1; }
        d *= 2;
        rounds += 1;
    }
    a[n - 1] = identity;
    while d > 1 {                                  // down-sweep
        d /= 2;
        for i in (0..n).step_by(2 * d) {
            let t = a[i + d - 1];
            a[i + d - 1] = a[i + 2 * d - 1];
            a[i + 2 * d - 1] = op(t, a[i + 2 * d - 1]);
            ops += 1;
        }
        rounds += 1;
    }
    a.truncate(x.len());
    (a, rounds, ops)
}

pub fn demo() -> String {
    let mut o = String::new();
    let x: Vec<i64> = (1..=8).collect();
    let (p, rounds) = prefix_doubling(&x, |a, b| a + b);
    outln!(o, "x                    = {:?}", x);
    outln!(o, "inclusive scan (doubling) = {:?} in {} rounds", p, rounds);
    let (e, r2, ops) = prefix_work_efficient(&x, |a, b| a + b, 0);
    outln!(o, "exclusive scan (Blelloch) = {:?} in {} rounds, {} operations", e, r2, ops);
    let m: Vec<i64> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    outln!(o, "prefix maxima        = {:?}", prefix_doubling(&m, i64::max).0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_sequential_scans() {
        let mut r = Rng::new(161);
        for n in [0usize, 1, 2, 7, 8, 33, 100] {
            let x = r.vec(n, -20, 20);
            let mut seq = x.clone();
            for i in 1..n { seq[i] += seq[i - 1]; }
            let (p, rounds) = prefix_doubling(&x, |a, b| a + b);
            assert_eq!(p, seq);
            assert_eq!(rounds, if n <= 1 { 0 } else { (n as f64).log2().ceil() as usize });
            let (e, _, ops) = prefix_work_efficient(&x, |a, b| a + b, 0);
            let excl: Vec<i64> = (0..n).map(|i| if i == 0 { 0 } else { seq[i - 1] }).collect();
            assert_eq!(e, excl);
            assert!(ops <= 2 * n.next_power_of_two());
        }
    }
    #[test]
    fn works_for_any_associative_operator() {
        let x = [3i64, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(prefix_doubling(&x, i64::max).0, vec![3, 3, 4, 4, 5, 9, 9, 9]);
        assert_eq!(prefix_doubling(&x, i64::min).0, vec![3, 1, 1, 1, 1, 1, 1, 1]);
    }
}
