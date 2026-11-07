//! @title Akra–Bazzi Recurrence Method
//! @chapter 4.7 (4e)
//! @editions 4
//! @summary Solves recurrences with unequal subproblem sizes: find p with Σ aᵢ·bᵢ^p = 1, then integrate.
//! @order 130
use crate::outln;

/// Find `p` such that Σ aᵢ·bᵢ^p = 1 for T(n) = Σ aᵢ·T(bᵢ·n) + g(n); each (aᵢ, bᵢ) has aᵢ>0, 0<bᵢ<1.
pub fn find_p(terms: &[(f64, f64)]) -> f64 {
    let f = |p: f64| terms.iter().map(|&(a, b)| a * b.powf(p)).sum::<f64>() - 1.0;
    // f is strictly decreasing in p because every bᵢ < 1
    let (mut lo, mut hi) = (-50.0, 50.0);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if f(mid) > 0.0 { lo = mid } else { hi = mid }
    }
    (lo + hi) / 2.0
}

/// Asymptotic solution for g(n) = n^k·lg^q n as text.
pub fn solve(terms: &[(f64, f64)], k: f64, q: f64) -> (f64, String) {
    let p = find_p(terms);
    let fmt = |e: f64, l: f64| format!("Θ(n^{:.3}{})", e, if l == 0.0 { String::new() } else { format!(" lg^{} n", l) });
    let eps = 1e-6;
    let s = if (k - p).abs() < eps {
        if q > -1.0 { fmt(p, q + 1.0) } else { fmt(p, 0.0) + " (q ≤ −1: lg lg n or constant factor)" }
    } else if k < p {
        fmt(p, 0.0)
    } else {
        fmt(k, q)
    };
    (p, s)
}

/// Numeric T(n)=Σ aᵢ·T(⌊bᵢ n⌋) + n (T(n)=1 for n<2), memoised.
pub fn evaluate(terms: &[(f64, f64)], n: u64, memo: &mut std::collections::HashMap<u64, f64>) -> f64 {
    if n < 2 {
        return 1.0;
    }
    if let Some(&v) = memo.get(&n) {
        return v;
    }
    let v = terms.iter().map(|&(a, b)| a * evaluate(terms, (b * n as f64).floor() as u64, memo)).sum::<f64>() + n as f64;
    memo.insert(n, v);
    v
}

pub fn demo() -> String {
    let mut o = String::new();
    let t1 = [(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)];
    let (p, s) = solve(&t1, 1.0, 0.0);
    outln!(o, "T(n)=T(n/3)+T(2n/3)+n      : p={:.4} → {}", p, s);
    let t2 = [(2.0, 0.5)];
    let (p, s) = solve(&t2, 1.0, 0.0);
    outln!(o, "T(n)=2T(n/2)+n             : p={:.4} → {}", p, s);
    let t3 = [(1.0, 0.5), (1.0, 0.25)];
    let (p, s) = solve(&t3, 1.0, 0.0);
    outln!(o, "T(n)=T(n/2)+T(n/4)+n       : p={:.4} → {}", p, s);
    let mut m = std::collections::HashMap::new();
    let n = 1u64 << 20;
    outln!(o, "numeric T(2^20)/(n lg n) for the first recurrence = {:.3}", evaluate(&t1, n, &mut m) / (n as f64 * 20.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p_values() {
        assert!((find_p(&[(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)]) - 1.0).abs() < 1e-9);
        assert!((find_p(&[(2.0, 0.5)]) - 1.0).abs() < 1e-9);
        assert!((find_p(&[(7.0, 0.5)]) - 7f64.log2()).abs() < 1e-9);
    }
    #[test]
    fn solutions() {
        assert_eq!(solve(&[(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)], 1.0, 0.0).1, "Θ(n^1.000 lg^1 n)");
        assert_eq!(solve(&[(1.0, 0.5), (1.0, 0.25)], 1.0, 0.0).1, "Θ(n^1.000)");
    }
    #[test]
    fn numeric_matches() {
        let t = [(1.0, 1.0 / 3.0), (1.0, 2.0 / 3.0)];
        let mut m = std::collections::HashMap::new();
        let n = 1u64 << 18;
        let r = evaluate(&t, n, &mut m) / (n as f64 * 18.0);
        assert!((0.5..2.0).contains(&r), "ratio {r}");
    }
}
