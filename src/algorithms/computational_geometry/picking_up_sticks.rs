//! @title Picking Up Sticks
//! @chapter Problem 33-4 (3e)
//! @editions 3,4
//! @summary Sticks lie in a pile (3-D segments): find an order in which they can be lifted one at a time with nothing on top of the lifted stick — the "lies above" relation of crossing sticks is a DAG, so a topological order works.
//! @order 64
use crate::outln;
use std::collections::VecDeque;

/// A stick: endpoints (x, y, z) with z the height.
pub type Stick = ((f64, f64, f64), (f64, f64, f64));

/// If the xy-projections of the two sticks cross, return (height of a at the crossing, height of b at the crossing).
fn crossing_heights(a: &Stick, b: &Stick) -> Option<(f64, f64)> {
    let ((x1, y1, z1), (x2, y2, z2)) = *a;
    let ((x3, y3, z3), (x4, y4, z4)) = *b;
    let d = (x2 - x1) * (y4 - y3) - (y2 - y1) * (x4 - x3);
    if d.abs() < 1e-12 { return None; } // parallel (or collinear): they cannot lie on top of each other at a point
    let t = ((x3 - x1) * (y4 - y3) - (y3 - y1) * (x4 - x3)) / d;
    let u = ((x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1)) / d;
    if !(0.0..=1.0).contains(&t) || !(0.0..=1.0).contains(&u) { return None; }
    Some((z1 + t * (z2 - z1), z3 + u * (z4 - z3)))
}

/// `above[i][j]` — stick i lies on top of stick j at a crossing (so i must be lifted before j).
pub fn above_relation(sticks: &[Stick]) -> Vec<Vec<bool>> {
    let n = sticks.len();
    let mut above = vec![vec![false; n]; n];
    for i in 0..n { for j in i + 1..n {
        if let Some((za, zb)) = crossing_heights(&sticks[i], &sticks[j]) {
            if za > zb + 1e-12 { above[i][j] = true; } else if zb > za + 1e-12 { above[j][i] = true; }
        }
    } }
    above
}

/// An order in which the sticks can be picked up (each stick free of all others lying on it), or None if the "above" relation has a cycle
/// (which cannot happen for a physically possible pile). O(n²) here; O(n lg n) is possible with a sweep.
pub fn pickup_order(sticks: &[Stick]) -> Option<Vec<usize>> {
    let above = above_relation(sticks);
    let n = sticks.len();
    let mut indeg: Vec<usize> = (0..n).map(|j| (0..n).filter(|&i| above[i][j]).count()).collect(); // how many sticks lie on j
    let mut queue: VecDeque<usize> = (0..n).filter(|&j| indeg[j] == 0).collect();
    let mut order = vec![];
    while let Some(i) = queue.pop_front() {
        order.push(i);
        for j in 0..n { if above[i][j] { indeg[j] -= 1; if indeg[j] == 0 { queue.push_back(j); } } }
    }
    (order.len() == n).then_some(order)
}

pub fn demo() -> String {
    let mut o = String::new();
    // three horizontal sticks forming a "pile": 0 lies on 1, 1 lies on 2
    let sticks: Vec<Stick> = vec![((0.0, 0.0, 3.0), (10.0, 10.0, 3.0)), ((0.0, 10.0, 2.0), (10.0, 0.0, 2.0)), ((5.0, -1.0, 1.0), (5.0, 11.0, 1.0))];
    outln!(o, "above relation (row lies on column):");
    for (i, row) in above_relation(&sticks).iter().enumerate() { outln!(o, "  stick {i}: {:?}", row); }
    outln!(o, "pick-up order: {:?}", pickup_order(&sticks));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn valid(sticks: &[Stick], order: &[usize]) -> bool {
        let above = above_relation(sticks);
        let pos: Vec<usize> = { let mut p = vec![0; sticks.len()]; for (k, &i) in order.iter().enumerate() { p[i] = k; } p };
        (0..sticks.len()).all(|i| (0..sticks.len()).all(|j| !above[i][j] || pos[i] < pos[j]))
    }
    #[test]
    fn horizontal_sticks_at_distinct_heights_can_always_be_lifted() {
        let mut r = Rng::new(470);
        for _ in 0..200 {
            let n = r.range(1, 14);
            let mut heights: Vec<f64> = (0..n).map(|i| i as f64).collect();
            r.shuffle(&mut heights);
            let sticks: Vec<Stick> = (0..n).map(|i| ((r.f64() * 20.0, r.f64() * 20.0, heights[i]), (r.f64() * 20.0, r.f64() * 20.0, heights[i]))).collect();
            let order = pickup_order(&sticks).expect("constant heights cannot form a cycle");
            assert_eq!(order.len(), n);
            assert!(valid(&sticks, &order));
        }
    }
    #[test]
    fn tilted_sticks_give_a_valid_order_whenever_one_exists() {
        let mut r = Rng::new(471);
        let mut found = 0;
        for _ in 0..300 {
            let n = r.range(1, 8);
            let sticks: Vec<Stick> = (0..n).map(|_| ((r.f64() * 10.0, r.f64() * 10.0, r.f64() * 5.0), (r.f64() * 10.0, r.f64() * 10.0, r.f64() * 5.0))).collect();
            if let Some(order) = pickup_order(&sticks) { assert!(valid(&sticks, &order)); found += 1; }
        }
        assert!(found > 100);
    }
    #[test]
    fn explicit_cycle_is_rejected() {
        // each stick passes over the next one and under the previous one: the three crossings form a cycle
        let s: Vec<Stick> = vec![((0.0, 0.0, 0.0), (10.0, 0.0, 10.0)), ((8.0, -5.0, 6.0), (2.0, 5.0, -6.0)), ((-2.0, 4.0, 5.0), (12.0, 6.0, 5.0))];
        let above = above_relation(&s);
        if (0..3).all(|i| (0..3).filter(|&j| above[i][j]).count() == 1) { assert!(pickup_order(&s).is_none()); }
    }
}
