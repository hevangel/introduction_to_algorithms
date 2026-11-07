//! @title Greedy Set Cover
//! @chapter 35.3 (2e–4e)
//! @editions 2,3,4
//! @summary Repeatedly pick the set covering the most uncovered elements: an H(d)-approximation, H(d) ≤ ln d + 1.
//! @order 30
use crate::outln;

/// GREEDY-SET-COVER(X, F): `sets` are subsets of {0, …, universe−1}. Returns indices of the chosen sets, or None if some element is uncoverable.
pub fn greedy_set_cover(universe: usize, sets: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut uncovered = vec![true; universe];
    let mut left = universe;
    let mut chosen = vec![];
    while left > 0 {
        let (best, gain) = sets.iter().enumerate().map(|(i, s)| (i, s.iter().filter(|&&e| uncovered[e]).count())).max_by_key(|&(_, g)| g)?;
        if gain == 0 { return None; }
        for &e in &sets[best] { if uncovered[e] { uncovered[e] = false; left -= 1; } }
        chosen.push(best);
    }
    Some(chosen)
}

/// Optimal cover size by exhaustive search over subfamilies (exponential), for validation.
pub fn optimal_cover_size(universe: usize, sets: &[Vec<usize>]) -> Option<usize> {
    (0u32..1 << sets.len()).filter(|m| {
        let mut cov = vec![false; universe];
        for (i, s) in sets.iter().enumerate() { if m >> i & 1 == 1 { for &e in s { cov[e] = true; } } }
        cov.iter().all(|&c| c)
    }).map(|m| m.count_ones() as usize).min()
}

pub fn harmonic(d: usize) -> f64 { (1..=d).map(|i| 1.0 / i as f64).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    let sets: Vec<Vec<usize>> = vec![vec![0, 1, 2, 3, 4, 5], vec![0, 3, 6, 9], vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![6, 7, 8, 9, 10, 11]];
    outln!(o, "universe 0..12 with {} sets", sets.len());
    outln!(o, "greedy picks {:?}", greedy_set_cover(12, &sets));
    outln!(o, "optimal size = {:?}", optimal_cover_size(12, &sets));
    // classic bad instance: greedy takes the 3 big sets while 2 sets suffice
    let bad: Vec<Vec<usize>> = vec![vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 7, 8, 9], vec![3, 4, 5, 6, 7, 8, 9]];
    outln!(o, "adversarial: greedy {:?}, optimal {:?}", greedy_set_cover(10, &bad).map(|v| v.len()), optimal_cover_size(10, &bad));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn covers_everything_within_harmonic_bound() {
        let mut r = Rng::new(252);
        let mut checked = 0;
        for _ in 0..300 {
            let (u, m) = (r.range(1, 12), r.range(1, 10));
            let sets: Vec<Vec<usize>> = (0..m).map(|_| (0..u).filter(|_| r.f64() < 0.35).collect()).collect();
            let opt = optimal_cover_size(u, &sets);
            let g = greedy_set_cover(u, &sets);
            assert_eq!(opt.is_some(), g.is_some());
            if let (Some(o), Some(g)) = (opt, g) {
                let mut cov = vec![false; u];
                for &i in &g { for &e in &sets[i] { cov[e] = true; } }
                assert!(cov.iter().all(|&c| c));
                let d = sets.iter().map(|s| s.len()).max().unwrap();
                assert!(g.len() as f64 <= o as f64 * harmonic(d) + 1e-9, "{} vs {} * H({d})", g.len(), o);
                checked += 1;
            }
        }
        assert!(checked > 100);
    }
    #[test]
    fn greedy_can_be_suboptimal() {
        let bad: Vec<Vec<usize>> = vec![vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 7, 8, 9], vec![3, 4, 5, 6, 7, 8, 9]];
        assert_eq!(optimal_cover_size(10, &bad), Some(2));
        assert_eq!(greedy_set_cover(10, &bad).unwrap().len(), 2); // 7 elements first, then the remaining three need the second set
        assert_eq!(greedy_set_cover(3, &[vec![0], vec![1]]), None);
    }
}
