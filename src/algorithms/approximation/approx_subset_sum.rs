//! @title FPTAS for Subset Sum
//! @chapter 35.5 (2e–4e)
//! @editions 2,3,4
//! @summary Trim the list of reachable sums by a factor (1+ε/2n): a fully polynomial-time approximation scheme.
//! @order 40
use crate::outln;

/// MERGE-LISTS: sorted union without duplicates.
fn merge_lists(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out: Vec<u64> = a.iter().chain(b).copied().collect();
    out.sort();
    out.dedup();
    out
}

/// EXACT-SUBSET-SUM(S, t): L_i = L_{i−1} ∪ (L_{i−1} + x_i), keeping sums ≤ t. Exponential lists.
pub fn exact_subset_sum(s: &[u64], t: u64) -> u64 {
    let mut l = vec![0u64];
    for &x in s {
        let shifted: Vec<u64> = l.iter().map(|&y| y + x).filter(|&y| y <= t).collect();
        l = merge_lists(&l, &shifted);
    }
    *l.last().unwrap()
}

/// TRIM(L, δ): drop y when a kept z ≤ y satisfies y/(1+δ) ≤ z.
pub fn trim(l: &[u64], delta: f64) -> Vec<u64> {
    let mut out = vec![l[0]];
    let mut last = l[0];
    for &y in &l[1..] {
        if (y as f64) > last as f64 * (1.0 + delta) { out.push(y); last = y; }
    }
    out
}

/// APPROX-SUBSET-SUM(S, t, ε): returns z ≤ t with z ≥ (1 − ε)·OPT, in time polynomial in n and 1/ε.
/// Also returns the maximum list length seen.
pub fn approx_subset_sum(s: &[u64], t: u64, eps: f64) -> (u64, usize) {
    let n = s.len().max(1);
    let mut l = vec![0u64];
    let mut max_len = 1;
    for &x in s {
        let shifted: Vec<u64> = l.iter().map(|&y| y + x).filter(|&y| y <= t).collect();
        l = trim(&merge_lists(&l, &shifted), eps / (2.0 * n as f64));
        max_len = max_len.max(l.len());
    }
    (*l.last().unwrap(), max_len)
}

pub fn demo() -> String {
    let mut o = String::new();
    let s = [104, 102, 201, 101];
    outln!(o, "S = {:?}, t = 308", s);
    outln!(o, "exact optimum        = {}", exact_subset_sum(&s, 308));
    outln!(o, "FPTAS with ε = 0.4   = {:?}", approx_subset_sum(&s, 308, 0.4));
    let mut r = crate::common::Rng::new(1);
    let big: Vec<u64> = (0..22).map(|_| r.range(1_000, 1_000_000) as u64).collect();
    let t = big.iter().sum::<u64>() / 2;
    outln!(o, "22 numbers, t = {t}: exact {}", exact_subset_sum(&big, t));
    for eps in [0.5, 0.1, 0.01] {
        let (z, len) = approx_subset_sum(&big, t, eps);
        outln!(o, "  ε = {:<4}: z = {}  (≥ (1−ε)·OPT), max list length {}", eps, z, len);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        // Section 35.5: S = {104,102,201,101}, t = 308, ε = 0.40 → z = 302; optimum 307
        let s = [104, 102, 201, 101];
        assert_eq!(exact_subset_sum(&s, 308), 307);
        let (z, _) = approx_subset_sum(&s, 308, 0.4);
        assert!(z <= 308 && z as f64 >= 0.6 * 307.0, "{z}");
    }
    #[test]
    fn approximation_guarantee_and_polynomial_list_size() {
        let mut r = Rng::new(253);
        for _ in 0..100 {
            let n = r.range(1, 14);
            let s: Vec<u64> = (0..n).map(|_| r.range(1, 5000) as u64).collect();
            let t = r.range(1, 40_000) as u64;
            let opt = exact_subset_sum(&s, t);
            for eps in [0.5, 0.2, 0.05] {
                let (z, len) = approx_subset_sum(&s, t, eps);
                assert!(z <= t && z as f64 >= (1.0 - eps) * opt as f64 - 1e-9, "eps={eps} z={z} opt={opt}");
                // list length ≤ ln t / δ + 2 with δ = ε/2n (the book's bound is O(n² ln t / ε))
                assert!(len as f64 <= (t as f64).ln() * 2.0 * n as f64 / eps + 3.0);
            }
        }
    }
}
