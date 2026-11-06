//! @title Convex Hull: Graham's Scan
//! @chapter 33.3 (1e–3e)
//! @editions 1,2,3
//! @summary Sort points by polar angle around the lowest point, then discard non-left turns with a stack: O(n lg n).
//! @order 30
use super::segment_intersection::{cross, Pt};
use crate::outln;

/// GRAHAM-SCAN(Q): the vertices of the convex hull in counter-clockwise order, starting at the lowest (then leftmost) point.
/// Collinear points on hull edges are dropped.
pub fn graham_scan(points: &[Pt]) -> Vec<Pt> {
    let mut pts: Vec<Pt> = points.to_vec();
    pts.sort();
    pts.dedup();
    if pts.len() < 3 { return pts; }
    // p0 = lowest y, leftmost on ties
    let p0 = *pts.iter().min_by_key(|p| (p.1, p.0)).unwrap();
    let mut rest: Vec<Pt> = pts.into_iter().filter(|&p| p != p0).collect();
    let dist2 = |p: Pt| (p.0 - p0.0).pow(2) + (p.1 - p0.1).pow(2);
    // sort by polar angle around p0 (via cross products); on equal angles keep the farthest point only
    rest.sort_by(|&a, &b| {
        let c = cross(p0, a, b);
        if c > 0 { std::cmp::Ordering::Less } else if c < 0 { std::cmp::Ordering::Greater } else { dist2(a).cmp(&dist2(b)) }
    });
    let mut filtered: Vec<Pt> = vec![];
    for p in rest {
        match filtered.last() {
            Some(&q) if cross(p0, q, p) == 0 => { *filtered.last_mut().unwrap() = p; } // same angle: replace with the farther one
            _ => filtered.push(p),
        }
    }
    let mut stack = vec![p0];
    for p in filtered {
        while stack.len() >= 2 && cross(stack[stack.len() - 2], stack[stack.len() - 1], p) <= 0 { stack.pop(); } // not a left turn
        stack.push(p);
    }
    stack
}

pub fn hull_area2(h: &[Pt]) -> i64 {
    (0..h.len()).map(|i| h[i].0 * h[(i + 1) % h.len()].1 - h[(i + 1) % h.len()].0 * h[i].1).sum()
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(0, 3), (1, 1), (2, 2), (4, 4), (0, 0), (1, 2), (3, 1), (3, 3), (2, 0), (4, 0)];
    let h = graham_scan(&pts);
    outln!(o, "points: {:?}", pts);
    outln!(o, "convex hull (CCW from the lowest point): {:?}", h);
    outln!(o, "hull has {} of {} points; doubled area = {}", h.len(), pts.len(), hull_area2(&h));
    let mut r = crate::common::Rng::new(7);
    let big: Vec<Pt> = (0..10_000).map(|_| (r.range(0, 1000) as i64, r.range(0, 1000) as i64)).collect();
    outln!(o, "10000 random points in a square: hull size {}", graham_scan(&big).len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn square_with_interior_and_collinear_points() {
        let pts = vec![(0, 0), (2, 0), (4, 0), (4, 4), (0, 4), (2, 2), (1, 1), (4, 2), (0, 2)];
        assert_eq!(graham_scan(&pts), vec![(0, 0), (4, 0), (4, 4), (0, 4)]);
    }
    #[test]
    fn hull_contains_all_points_and_is_convex() {
        let mut r = Rng::new(232);
        for _ in 0..200 {
            let n = r.range(0, 40);
            let pts: Vec<Pt> = (0..n).map(|_| (r.range(0, 15) as i64, r.range(0, 15) as i64)).collect();
            let h = graham_scan(&pts);
            if h.len() >= 3 {
                for i in 0..h.len() { assert!(cross(h[i], h[(i + 1) % h.len()], h[(i + 2) % h.len()]) > 0, "strictly convex"); }
                for &p in &pts { assert!((0..h.len()).all(|i| cross(h[i], h[(i + 1) % h.len()], p) >= 0), "{p:?} outside {h:?}"); }
            }
        }
    }
    #[test]
    fn degenerate_inputs() {
        assert!(graham_scan(&[]).is_empty());
        assert_eq!(graham_scan(&[(1, 1), (1, 1)]), vec![(1, 1)]);
        assert_eq!(graham_scan(&[(0, 0), (1, 1), (2, 2), (3, 3)]).len(), 2); // all collinear: the two extreme points
    }
}
