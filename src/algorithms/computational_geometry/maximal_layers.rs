//! @title Maximal Layers
//! @chapter Problem 33-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Peel off the maximal (non-dominated) points repeatedly; layer numbers come from one sort and a binary search over the layers' topmost y-values: O(n lg n).
//! @order 61
use crate::outln;

pub type Pt = (i64, i64);

/// Point p dominates q if p ≠ q, p.x ≥ q.x and p.y ≥ q.y. The layer of a point is 1 + the largest layer of any point that dominates it
/// (layer 0 = maximal points). O(n lg n).
pub fn maximal_layers(points: &[Pt]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..points.len()).collect();
    order.sort_by(|&a, &b| points[b].cmp(&points[a])); // x descending, then y descending
    let mut top: Vec<i64> = vec![]; // top[k] = largest y so far in layer k; non-increasing in k
    let mut layer = vec![0; points.len()];
    for &i in &order {
        let y = points[i].1;
        let k = top.partition_point(|&t| t >= y); // first layer whose topmost y is below y: nobody there dominates
        if k == top.len() { top.push(y); } else { top[k] = top[k].max(y); }
        layer[i] = k;
    }
    layer
}

/// Quadratic reference: repeatedly remove all maximal points.
pub fn maximal_layers_naive(points: &[Pt]) -> Vec<usize> {
    let mut layer = vec![usize::MAX; points.len()];
    let mut left: Vec<usize> = (0..points.len()).collect();
    let mut k = 0;
    while !left.is_empty() {
        let (maximal, rest): (Vec<usize>, Vec<usize>) = left.iter().partition(|&&i| !left.iter().any(|&j| j != i && points[j].0 >= points[i].0 && points[j].1 >= points[i].1));
        for &i in &maximal { layer[i] = k; }
        left = rest;
        k += 1;
    }
    layer
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(1, 9), (3, 7), (2, 5), (5, 6), (6, 2), (4, 3), (8, 1), (7, 4), (9, 0), (2, 2)];
    outln!(o, "points {:?}", pts);
    outln!(o, "layers (fast)  {:?}", maximal_layers(&pts));
    outln!(o, "layers (naive) {:?}", maximal_layers_naive(&pts));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn fast_equals_naive() {
        let mut r = Rng::new(407);
        for _ in 0..300 {
            let mut pts: Vec<Pt> = (0..r.range(0, 30)).map(|_| (r.range(0, 12) as i64, r.range(0, 12) as i64)).collect();
            pts.sort();
            pts.dedup();
            r.shuffle(&mut pts);
            assert_eq!(maximal_layers(&pts), maximal_layers_naive(&pts), "{pts:?}");
        }
    }
    #[test]
    fn staircase_has_one_layer_and_chain_has_many() {
        let stair: Vec<Pt> = (0..10).map(|i| (i, 10 - i)).collect();
        assert!(maximal_layers(&stair).iter().all(|&l| l == 0));
        let chain: Vec<Pt> = (0..10).map(|i| (i, i)).collect();
        assert_eq!(maximal_layers(&chain), (0..10).rev().collect::<Vec<_>>());
    }
}
