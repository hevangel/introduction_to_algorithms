//! @title Constant-Time Maximum on a CRCW PRAM
//! @chapter 30.3 (1e)
//! @editions 1
//! @summary With n² processors and concurrent writes, find the maximum in O(1); contrast with the Θ(lg n) EREW tree.
//! @order 30
use crate::outln;

/// Common-CRCW maximum in 2 parallel steps with n² processors.
/// Step 1: processor (i, j) writes `false` to m[i] if a[i] < a[j] (concurrent writes all store the same value).
/// Step 2: the unique-valued survivors write a[i] to the result cell.
/// Returns (maximum, processors used, steps used).
pub fn crcw_maximum(a: &[i64]) -> Option<(i64, usize, usize)> {
    let n = a.len();
    if n == 0 { return None; }
    let mut m = vec![true; n];
    for i in 0..n { for j in 0..n { if a[i] < a[j] { m[i] = false; } } } // n² processors, one step
    let mut result = None;
    for i in 0..n { if m[i] { result = Some(a[i]); } }                    // n processors, one step (all write the same value)
    result.map(|r| (r, n * n, 2))
}

/// EREW tree reduction: ⌈lg n⌉ rounds with n/2 processors. Returns (max, rounds).
pub fn erew_maximum(a: &[i64]) -> Option<(i64, usize)> {
    if a.is_empty() { return None; }
    let mut cur = a.to_vec();
    let mut rounds = 0;
    while cur.len() > 1 {
        cur = cur.chunks(2).map(|c| *c.iter().max().unwrap()).collect();
        rounds += 1;
    }
    Some((cur[0], rounds))
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [7, 3, 9, 1, 9, 4, 2, 8];
    outln!(o, "A = {:?}", a);
    outln!(o, "CRCW: (max, processors, steps) = {:?}", crcw_maximum(&a));
    outln!(o, "EREW: (max, rounds)            = {:?}", erew_maximum(&a));
    for n in [8usize, 64, 1024] {
        let v: Vec<i64> = (0..n as i64).rev().collect();
        outln!(o, "n = {:>4}: CRCW 2 steps with {:>7} processors; EREW {} steps with {} processors", n, n * n, erew_maximum(&v).unwrap().1, n / 2);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn both_find_the_maximum() {
        let mut r = Rng::new(162);
        for n in 1..40 {
            let v = r.vec(n, -9, 9);
            let want = *v.iter().max().unwrap();
            assert_eq!(crcw_maximum(&v).unwrap().0, want);
            let (m, rounds) = erew_maximum(&v).unwrap();
            assert_eq!(m, want);
            assert_eq!(rounds, (n as f64).log2().ceil() as usize);
        }
        assert!(crcw_maximum(&[]).is_none() && erew_maximum(&[]).is_none());
    }
}
