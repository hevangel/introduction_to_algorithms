//! @title Randomly Built Binary Search Trees
//! @chapter 12.4 (1e–3e)
//! @editions 1,2,3
//! @summary Inserting keys in random order gives expected height O(lg n); empirical check against the closed forms.
//! @order 110
use super::binary_search_tree::Bst;
use crate::common::Rng;
use crate::outln;

/// Build a BST by inserting a uniformly random permutation of 1..=n.
pub fn build_random(n: usize, rng: &mut Rng) -> Bst {
    let mut keys: Vec<i64> = (1..=n as i64).collect();
    rng.shuffle(&mut keys);
    let mut t = Bst::new();
    for k in keys { t.insert(k); }
    t
}

/// Average node depth (root = depth 0).
pub fn average_depth(t: &Bst) -> f64 {
    let mut total = 0usize;
    let mut level = vec![t.root];
    let mut d = 0;
    while !level.is_empty() {
        total += d * level.len();
        level = level.iter().flat_map(|&x| [t.nodes[x].left, t.nodes[x].right]).filter(|&c| c != super::binary_search_tree::NIL).collect();
        d += 1;
    }
    total as f64 / t.len() as f64
}

/// Expected average depth of a random BST: 2(1 + 1/n)·H_n − 4.
pub fn expected_average_depth(n: usize) -> f64 {
    let h: f64 = (1..=n).map(|i| 1.0 / i as f64).sum();
    2.0 * (1.0 + 1.0 / n as f64) * h - 4.0
}

/// Reed's asymptotic expected height: 4.311·ln n − 1.953·ln ln n.
pub fn expected_height_asymptotic(n: usize) -> f64 {
    let ln = (n as f64).ln();
    4.311 * ln - 1.953 * ln.ln()
}

/// (mean height, mean average-depth) over `trials` random trees of size n.
pub fn simulate(n: usize, trials: usize, rng: &mut Rng) -> (f64, f64) {
    let (mut h, mut d) = (0.0, 0.0);
    for _ in 0..trials {
        let t = build_random(n, rng);
        h += t.height() as f64;
        d += average_depth(&t);
    }
    (h / trials as f64, d / trials as f64)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(12);
    outln!(o, "      n | mean height | Reed 4.311 ln n − 1.953 ln ln n | mean avg depth | 2(1+1/n)H_n − 4 | lg n");
    for n in [100usize, 1000, 10000] {
        let (h, d) = simulate(n, 30, &mut r);
        outln!(o, "{:>7} | {:>11.2} | {:>31.2} | {:>14.3} | {:>15.3} | {:.1}", n, h, expected_height_asymptotic(n), d, expected_average_depth(n), (n as f64).log2());
    }
    let mut sorted = Bst::new();
    for k in 0..200 { sorted.insert(k); }
    outln!(o, "contrast: inserting 0..200 in sorted order gives height {}", sorted.height());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn average_depth_matches_formula() {
        let mut r = Rng::new(61);
        let (_, d) = simulate(500, 40, &mut r);
        assert!((d - expected_average_depth(500)).abs() < 0.5, "{d} vs {}", expected_average_depth(500));
    }
    #[test]
    fn height_is_logarithmic() {
        let mut r = Rng::new(62);
        let (h, _) = simulate(1000, 20, &mut r);
        assert!(h < 4.311 * (1000f64).ln() && h > (1000f64).log2());
    }
    #[test]
    fn small_case_exact() {
        // n = 3: expected total depth = 8/3, average = 8/9
        assert!((expected_average_depth(3) - 8.0 / 9.0).abs() < 1e-9);
    }
}
