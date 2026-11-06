//! @title Convex Hull: Jarvis's March
//! @chapter 33.3 (1e–3e)
//! @editions 1,2,3
//! @summary Gift wrapping: from each hull vertex pick the point with the smallest polar angle; output-sensitive O(nh).
//! @order 40
use super::segment_intersection::{cross, Pt};
use crate::outln;

/// JARVIS-MARCH(Q): hull vertices in counter-clockwise order starting at the lowest-leftmost point (collinear points omitted).
pub fn jarvis_march(points: &[Pt]) -> Vec<Pt> {
    let mut pts: Vec<Pt> = points.to_vec();
    pts.sort();
    pts.dedup();
    if pts.len() < 3 { return pts; }
    let start = *pts.iter().min_by_key(|p| (p.1, p.0)).unwrap();
    let dist2 = |a: Pt, b: Pt| (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2);
    let (mut hull, mut cur) = (vec![start], start);
    loop {
        // the next vertex is the point such that every other point lies to the left of cur → next
        let mut next = if pts[0] == cur { pts[1] } else { pts[0] };
        for &p in &pts {
            if p == cur { continue; }
            let c = cross(cur, next, p);
            if c < 0 || (c == 0 && dist2(cur, p) > dist2(cur, next)) { next = p; } // p is more clockwise, or farther on the same ray
        }
        if next == start { break; }
        hull.push(next);
        cur = next;
    }
    hull
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(0, 3), (1, 1), (2, 2), (4, 4), (0, 0), (1, 2), (3, 1), (3, 3), (2, 0), (4, 0)];
    outln!(o, "Jarvis hull   : {:?}", jarvis_march(&pts));
    outln!(o, "Graham hull   : {:?}", super::convex_hull_graham::graham_scan(&pts));
    let circle: Vec<Pt> = (0..360).step_by(10).map(|d| { let a = (d as f64).to_radians(); ((1000.0 * a.cos()).round() as i64, (1000.0 * a.sin()).round() as i64) }).collect();
    outln!(o, "points on a circle (h ≈ n): hull size {} of {}", jarvis_march(&circle).len(), circle.len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn agrees_with_graham_scan() {
        let mut r = Rng::new(233);
        for _ in 0..300 {
            let n = r.range(0, 40);
            let pts: Vec<Pt> = (0..n).map(|_| (r.range(0, 12) as i64, r.range(0, 12) as i64)).collect();
            assert_eq!(jarvis_march(&pts), super::super::convex_hull_graham::graham_scan(&pts), "{pts:?}");
        }
    }
    #[test]
    fn collinear_boundary_points_dropped() {
        assert_eq!(jarvis_march(&[(0, 0), (2, 0), (4, 0), (4, 4), (0, 4), (2, 2)]), vec![(0, 0), (4, 0), (4, 4), (0, 4)]);
    }
}
