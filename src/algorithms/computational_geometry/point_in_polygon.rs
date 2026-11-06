//! @title Point in Polygon
//! @chapter 33.1 exercises / 33.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Ray casting (even–odd rule) and the winding number decide whether a point lies inside a simple polygon in O(n), using only exact cross products.
//! @order 63
use super::segment_intersection::{cross, Pt};
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Where { Inside, Boundary, Outside }

fn on_segment(a: Pt, b: Pt, p: Pt) -> bool {
    cross(a, b, p) == 0 && p.0 >= a.0.min(b.0) && p.0 <= a.0.max(b.0) && p.1 >= a.1.min(b.1) && p.1 <= a.1.max(b.1)
}

/// Even–odd rule: count the edges crossed by the horizontal ray going right from p.
pub fn locate_even_odd(poly: &[Pt], p: Pt) -> Where {
    let n = poly.len();
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if on_segment(a, b, p) { return Where::Boundary; }
        if (a.1 > p.1) != (b.1 > p.1) {
            // x-coordinate of the edge at height p.y is to the right of p?  cross(a, b, p) has the sign of (p is left of a→b)
            let left = cross(a, b, p) > 0;
            if left == (b.1 > a.1) { inside = !inside; }
        }
    }
    if inside { Where::Inside } else { Where::Outside }
}

/// Winding number: signed number of times the polygon wraps around p (non-zero ⇒ inside).
pub fn locate_winding(poly: &[Pt], p: Pt) -> Where {
    let n = poly.len();
    let mut wn = 0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if on_segment(a, b, p) { return Where::Boundary; }
        if a.1 <= p.1 { if b.1 > p.1 && cross(a, b, p) > 0 { wn += 1; } }
        else if b.1 <= p.1 && cross(a, b, p) < 0 { wn -= 1; }
    }
    if wn != 0 { Where::Inside } else { Where::Outside }
}

/// Twice the signed area (shoelace formula): positive for counter-clockwise polygons.
pub fn area2(poly: &[Pt]) -> i64 { (0..poly.len()).map(|i| { let (a, b) = (poly[i], poly[(i + 1) % poly.len()]); a.0 * b.1 - b.0 * a.1 }).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    // a non-convex "arrow" polygon
    let poly: Vec<Pt> = vec![(0, 0), (8, 0), (8, 8), (4, 3), (0, 8)];
    outln!(o, "polygon {:?}, 2×area = {}", poly, area2(&poly));
    for p in [(4, 1), (4, 5), (1, 7), (4, 3), (9, 4), (0, 4), (6, 6)] {
        outln!(o, "{:?}: even-odd {:?}, winding {:?}", p, locate_even_odd(&poly, p), locate_winding(&poly, p));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn rectangles_and_triangles_against_direct_tests() {
        let mut r = Rng::new(409);
        for _ in 0..300 {
            let (x0, y0) = (r.range(0, 10) as i64, r.range(0, 10) as i64);
            let (w, h) = (r.range(1, 10) as i64, r.range(1, 10) as i64);
            let rect = [(x0, y0), (x0 + w, y0), (x0 + w, y0 + h), (x0, y0 + h)];
            let p = (r.range(0, 24) as i64 - 2, r.range(0, 24) as i64 - 2);
            let want = if p.0 > x0 && p.0 < x0 + w && p.1 > y0 && p.1 < y0 + h { Where::Inside }
                else if p.0 >= x0 && p.0 <= x0 + w && p.1 >= y0 && p.1 <= y0 + h { Where::Boundary } else { Where::Outside };
            assert_eq!(locate_even_odd(&rect, p), want);
            assert_eq!(locate_winding(&rect, p), want);
            let rev: Vec<Pt> = rect.iter().rev().copied().collect(); // clockwise orientation gives the same answers
            assert_eq!(locate_even_odd(&rev, p), want);
            assert_eq!(locate_winding(&rev, p), want);
        }
    }
    #[test]
    fn rules_agree_on_random_star_shaped_polygons() {
        let mut r = Rng::new(410);
        for _ in 0..100 {
            let n = r.range(3, 12);
            // star-shaped around the origin: increasing angles with random radii ⇒ a simple polygon
            let poly: Vec<Pt> = (0..n).map(|i| { let a = (i as f64 + 0.5 * r.f64()) / n as f64 * std::f64::consts::TAU; let rad = 5.0 + 20.0 * r.f64(); ((rad * a.cos()).round() as i64, (rad * a.sin()).round() as i64) }).collect();
            if area2(&poly) <= 0 { continue; }
            for _ in 0..30 {
                let p = (r.range(0, 60) as i64 - 30, r.range(0, 60) as i64 - 30);
                assert_eq!(locate_even_odd(&poly, p), locate_winding(&poly, p), "{poly:?} {p:?}");
            }
            assert_eq!(locate_even_odd(&poly, (0, 0)), Where::Inside);
        }
    }
    #[test]
    fn concave_notch_and_area() {
        let poly: Vec<Pt> = vec![(0, 0), (8, 0), (8, 8), (4, 3), (0, 8)];
        assert_eq!(locate_even_odd(&poly, (4, 5)), Where::Outside); // inside the notch
        assert_eq!(locate_even_odd(&poly, (4, 1)), Where::Inside);
        assert_eq!(locate_winding(&poly, (4, 3)), Where::Boundary);
        assert_eq!(area2(&[(0, 0), (4, 0), (4, 3)]), 12);
    }
}
