//! @title Fractional Knapsack
//! @chapter 16.2 (1e–3e) / 15.2 (4e)
//! @editions 1,2,3,4
//! @summary Greedy by value-per-pound is optimal when items may be split.
//! @order 160
use crate::outln;

/// Items are `(weight, value)`; returns (best total value, fraction of each item taken).
pub fn fractional_knapsack(items: &[(f64, f64)], cap: f64) -> (f64, Vec<f64>) {
    let mut idx: Vec<usize> = (0..items.len()).collect();
    idx.sort_by(|&a, &b| (items[b].1 / items[b].0).partial_cmp(&(items[a].1 / items[a].0)).unwrap());
    let (mut left, mut total, mut take) = (cap, 0.0, vec![0.0; items.len()]);
    for i in idx {
        if left <= 0.0 { break; }
        let f = (left / items[i].0).min(1.0);
        take[i] = f;
        total += f * items[i].1;
        left -= f * items[i].0;
    }
    (total, take)
}

pub fn demo() -> String {
    let mut o = String::new();
    let items = [(10.0, 60.0), (20.0, 100.0), (30.0, 120.0)];
    let (v, take) = fractional_knapsack(&items, 50.0);
    outln!(o, "items (weight, value) = {:?}, capacity 50", items);
    outln!(o, "fractional optimum = {:.2}, fractions taken = {:?}", v, take);
    let (zero_one, _) = super::knapsack_01::knapsack_01(&[(10, 60), (20, 100), (30, 120)], 50);
    outln!(o, "0-1 optimum        = {} (never larger than the fractional one)", zero_one);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn thief_example() {
        let (v, take) = fractional_knapsack(&[(10.0, 60.0), (20.0, 100.0), (30.0, 120.0)], 50.0);
        assert!((v - 240.0).abs() < 1e-9);
        assert_eq!(take, vec![1.0, 1.0, 2.0 / 3.0]);
    }
    #[test]
    fn upper_bounds_the_zero_one_optimum_and_fits() {
        let mut r = Rng::new(113);
        for _ in 0..60 {
            let n = r.range(1, 8);
            let ints: Vec<(usize, u64)> = (0..n).map(|_| (r.range(1, 9), r.range(1, 30) as u64)).collect();
            let cap = r.range(1, 30);
            let items: Vec<(f64, f64)> = ints.iter().map(|&(w, v)| (w as f64, v as f64)).collect();
            let (f, take) = fractional_knapsack(&items, cap as f64);
            assert!(take.iter().zip(&items).map(|(t, i)| t * i.0).sum::<f64>() <= cap as f64 + 1e-9);
            assert!(f + 1e-9 >= super::super::knapsack_01::knapsack_01(&ints, cap).0 as f64);
        }
    }
}
