//! @title Greedy Scheduling and Brent's Theorem
//! @chapter 27.1 (3e) / 26.1 (4e)
//! @editions 3,4
//! @summary A greedy scheduler runs min(P, #ready) strands each step, achieving T_P ≤ T₁/P + T∞ (Theorem 27.1), within a factor 2 of optimal.
//! @order 30
use crate::outln;

/// A computation DAG of unit-time strands: `deps[v]` lists the strands that must finish before v starts (all indices < v).
/// Returns (T₁ = work, T∞ = span, T_P of a greedy schedule).
pub fn analyse(deps: &[Vec<usize>], p: usize) -> (usize, usize, usize) {
    let n = deps.len();
    let work = n;
    let mut depth = vec![1usize; n];
    for v in 0..n { for &u in &deps[v] { depth[v] = depth[v].max(depth[u] + 1); } }
    let span = depth.iter().copied().max().unwrap_or(0);
    // greedy schedule: at each step run up to p ready strands
    let mut remaining: Vec<usize> = deps.iter().map(|d| d.len()).collect();
    let mut succ = vec![vec![]; n];
    for v in 0..n { for &u in &deps[v] { succ[u].push(v); } }
    let mut ready: Vec<usize> = (0..n).filter(|&v| remaining[v] == 0).collect();
    let (mut done, mut steps) = (0, 0);
    while done < n {
        let batch: Vec<usize> = ready.drain(..p.min(ready.len())).collect();
        steps += 1;
        done += batch.len();
        for u in batch { for &v in &succ[u] { remaining[v] -= 1; if remaining[v] == 0 { ready.push(v); } } }
    }
    (work, span, steps)
}

pub fn demo() -> String {
    let mut o = String::new();
    // a fork-join diamond repeated: 1 → 4 parallel → 1 → 4 parallel → 1
    let mut deps: Vec<Vec<usize>> = vec![vec![]];
    for stage in 0..2 {
        let root = deps.len() - 1;
        let first = deps.len();
        for _ in 0..4 { deps.push(vec![root]); }
        deps.push((first..first + 4).collect());
        let _ = stage;
    }
    outln!(o, "fork-join DAG with {} strands", deps.len());
    for p in [1usize, 2, 3, 4, 8] {
        let (t1, tinf, tp) = analyse(&deps, p);
        outln!(o, "P = {p}: T_P = {tp:>2}  (lower bound max(T1/P, T∞) = {:.1}, upper bound T1/P + T∞ = {:.1})", (t1 as f64 / p as f64).max(tinf as f64), t1 as f64 / p as f64 + tinf as f64);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn greedy_bound_holds_on_random_dags() {
        let mut r = Rng::new(433);
        for _ in 0..300 {
            let n = r.range(1, 40);
            let deps: Vec<Vec<usize>> = (0..n).map(|v| (0..v).filter(|_| r.f64() < 0.15).collect()).collect();
            for p in [1usize, 2, 3, 5, 100] {
                let (t1, tinf, tp) = analyse(&deps, p);
                assert!(tp >= t1.div_ceil(p) && tp >= tinf, "work and span laws");
                assert!(tp * p <= t1 + tinf * p, "T_P ≤ T1/P + T∞ (Theorem 27.1): {tp} {t1} {tinf} {p}");
                assert!(tp <= 2 * t1.div_ceil(p).max(tinf), "within factor 2 of the optimum lower bound");
            }
            let (t1, tinf, t1p) = analyse(&deps, 1);
            assert_eq!(t1p, t1);
            assert!(tinf <= t1);
        }
    }
    #[test]
    fn infinite_processors_reach_the_span() {
        let deps: Vec<Vec<usize>> = (0..10).map(|v| if v == 0 { vec![] } else { vec![v - 1] }).collect();
        assert_eq!(analyse(&deps, 64), (10, 10, 10)); // a chain cannot be sped up
    }
}
