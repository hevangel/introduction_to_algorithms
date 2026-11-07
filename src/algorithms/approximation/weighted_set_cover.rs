//! @title Weighted Set Cover
//! @chapter Problem 35-3 (3e)
//! @editions 3,4
//! @summary Greedily pick the set with the lowest cost per newly covered element: an H(d)-approximation, where d is the size of the largest set and H the harmonic number.
//! @order 50
use crate::outln;

/// Greedy weighted set cover over the universe 0..n. Returns (chosen set indices, total weight), or None if the sets do not cover the universe.
pub fn greedy_weighted_cover(n: usize, sets: &[Vec<usize>], weights: &[f64]) -> Option<(Vec<usize>, f64)> {
    let mut covered = vec![false; n];
    let (mut left, mut chosen, mut total) = (n, vec![], 0.0);
    while left > 0 {
        // cost-effectiveness: weight per newly covered element
        let best = (0..sets.len()).filter_map(|s| { let new = sets[s].iter().filter(|&&e| !covered[e]).count(); (new > 0).then(|| (weights[s] / new as f64, s)) }).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())?;
        chosen.push(best.1);
        total += weights[best.1];
        for &e in &sets[best.1] { if !covered[e] { covered[e] = true; left -= 1; } }
    }
    Some((chosen, total))
}

/// The optimal weight by exhaustive search over subsets of sets (small instances only).
pub fn optimal_weighted_cover(n: usize, sets: &[Vec<usize>], weights: &[f64]) -> Option<f64> {
    (0u32..1 << sets.len()).filter(|m| { let mut c = vec![false; n]; for s in 0..sets.len() { if m >> s & 1 == 1 { for &e in &sets[s] { c[e] = true; } } } c.iter().all(|&x| x) }).map(|m| (0..sets.len()).filter(|&s| m >> s & 1 == 1).map(|s| weights[s]).sum::<f64>()).min_by(|a, b| a.partial_cmp(b).unwrap())
}

pub fn harmonic(d: usize) -> f64 { (1..=d).map(|i| 1.0 / i as f64).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    let sets = vec![vec![0, 1, 2, 3], vec![0, 1], vec![2, 3], vec![4, 5], vec![3, 4, 5], vec![0, 5]];
    let w = [5.0, 2.0, 2.0, 3.0, 4.0, 2.5];
    let (chosen, total) = greedy_weighted_cover(6, &sets, &w).unwrap();
    outln!(o, "greedy picks sets {:?} for total weight {total}", chosen);
    outln!(o, "optimal weight {:?}; guarantee H(4) = {:.3}", optimal_weighted_cover(6, &sets, &w), harmonic(4));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn within_harmonic_factor_of_the_optimum() {
        let mut r = Rng::new(481);
        for _ in 0..300 {
            let n = r.range(1, 10);
            let m = r.range(1, 10);
            let mut sets: Vec<Vec<usize>> = (0..m).map(|_| (0..n).filter(|_| r.f64() < 0.35).collect()).collect();
            sets.push((0..n).collect::<Vec<_>>().into_iter().filter(|e| e % 2 == 0).collect()); // make covering likely but not guaranteed
            let weights: Vec<f64> = (0..sets.len()).map(|_| 1.0 + r.f64() * 9.0).collect();
            match (greedy_weighted_cover(n, &sets, &weights), optimal_weighted_cover(n, &sets, &weights)) {
                (Some((chosen, total)), Some(opt)) => {
                    let d = sets.iter().map(|s| s.len()).max().unwrap();
                    assert!(total <= harmonic(d) * opt + 1e-9, "greedy {total} opt {opt} H({d})");
                    let mut cov = vec![false; n];
                    for &s in &chosen { for &e in &sets[s] { cov[e] = true; } }
                    assert!(cov.iter().all(|&x| x));
                    assert!((chosen.iter().map(|&s| weights[s]).sum::<f64>() - total).abs() < 1e-9);
                }
                (None, None) => {}
                other => panic!("greedy and exhaustive search disagree on feasibility: {other:?}"),
            }
        }
    }
    #[test]
    fn unit_weights_reduce_to_the_unweighted_greedy_algorithm() {
        let sets = vec![vec![0, 1, 2], vec![2, 3], vec![3, 4, 5], vec![0, 5]];
        let (chosen, total) = greedy_weighted_cover(6, &sets, &[1.0; 4]).unwrap();
        assert_eq!(total, 2.0);
        assert_eq!(chosen.len(), 2);
    }
}
