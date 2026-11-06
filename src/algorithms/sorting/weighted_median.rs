//! @title Weighted Median
//! @chapter Problem 9-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary The element whose lighter side carries < half the total weight, via sorting or linear-time selection.
//! @order 190
use crate::outln;

/// Weighted lower median of `(value, weight)` pairs: smallest x with cumulative weight(≤ x) ≥ half. Θ(n lg n).
pub fn weighted_median_sorting(items: &[(i64, u64)]) -> i64 {
    let mut v = items.to_vec();
    v.sort();
    let total: u64 = v.iter().map(|x| x.1).sum();
    let mut cum = 0;
    for &(x, w) in &v {
        cum += w;
        if 2 * cum >= total { return x; }
    }
    unreachable!()
}

/// Θ(n) weighted median by repeatedly selecting the median value and discarding the side that cannot hold the answer.
pub fn weighted_median_linear(items: &[(i64, u64)]) -> i64 {
    let mut cur: Vec<(i64, u64)> = items.to_vec();
    let total: u64 = cur.iter().map(|x| x.1).sum();
    let mut below = 0u64; // weight of already discarded elements smaller than every remaining one
    loop {
        let vals: Vec<i64> = cur.iter().map(|x| x.0).collect();
        let m = super::select_linear_time::median(&vals);
        let wl: u64 = cur.iter().filter(|x| x.0 < m).map(|x| x.1).sum();
        let we: u64 = cur.iter().filter(|x| x.0 == m).map(|x| x.1).sum();
        if 2 * (below + wl) >= total {
            cur.retain(|x| x.0 < m); // answer is strictly smaller than m
        } else if 2 * (below + wl + we) >= total {
            return m;
        } else {
            below += wl + we;
            cur.retain(|x| x.0 > m);
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let items = [(1, 10), (2, 5), (3, 20), (4, 5), (5, 15), (6, 5), (7, 40)];
    outln!(o, "(value, weight) = {:?}", items);
    outln!(o, "total weight = {}", items.iter().map(|x| x.1).sum::<u64>());
    outln!(o, "weighted median (sorting) = {}", weighted_median_sorting(&items));
    outln!(o, "weighted median (linear)  = {}", weighted_median_linear(&items));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn linear_matches_sorting() {
        let mut r = Rng::new(43);
        for n in 1..60 {
            let items: Vec<(i64, u64)> = (0..n).map(|_| (r.vec(1, -20, 20)[0], r.range(1, 9) as u64)).collect();
            assert_eq!(weighted_median_linear(&items), weighted_median_sorting(&items), "{items:?}");
        }
    }
    #[test]
    fn equal_weights_give_ordinary_lower_median() {
        let items: Vec<(i64, u64)> = (1..=9).map(|x| (x, 1)).collect();
        assert_eq!(weighted_median_sorting(&items), 5);
        assert_eq!(weighted_median_linear(&items), 5);
    }
}
