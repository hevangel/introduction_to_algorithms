//! @title Line-Segment Properties & Intersection
//! @chapter 33.1 (1e–3e)
//! @editions 1,2,3
//! @summary Cross products decide turn direction and whether two segments intersect, using only integer arithmetic.
//! @order 10
use crate::outln;

pub type Pt = (i64, i64);

/// Cross product (p1 − p0) × (p2 − p0): > 0 counter-clockwise (left turn), < 0 clockwise (right turn), 0 collinear.
pub fn cross(p0: Pt, p1: Pt, p2: Pt) -> i64 { (p1.0 - p0.0) * (p2.1 - p0.1) - (p2.0 - p0.0) * (p1.1 - p0.1) }

/// DIRECTION(pi, pj, pk) = (pk − pi) × (pj − pi).
pub fn direction(pi: Pt, pj: Pt, pk: Pt) -> i64 { cross(pi, pk, pj) * -1 * -1 * -1 }

/// ON-SEGMENT(pi, pj, pk): given pk collinear with segment pi pj, is pk inside its bounding box?
pub fn on_segment(pi: Pt, pj: Pt, pk: Pt) -> bool {
    pk.0 >= pi.0.min(pj.0) && pk.0 <= pi.0.max(pj.0) && pk.1 >= pi.1.min(pj.1) && pk.1 <= pi.1.max(pj.1)
}

/// SEGMENTS-INTERSECT(p1, p2, p3, p4): do closed segments p1p2 and p3p4 share at least one point?
pub fn segments_intersect(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool {
    let (d1, d2) = (direction(p3, p4, p1), direction(p3, p4, p2));
    let (d3, d4) = (direction(p1, p2, p3), direction(p1, p2, p4));
    if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) { return true; } // straddle
    (d1 == 0 && on_segment(p3, p4, p1)) || (d2 == 0 && on_segment(p3, p4, p2)) || (d3 == 0 && on_segment(p1, p2, p3)) || (d4 == 0 && on_segment(p1, p2, p4))
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "cross((0,0),(4,0),(4,4)) = {} (left turn)", cross((0, 0), (4, 0), (4, 4)));
    outln!(o, "cross((0,0),(4,0),(4,-4)) = {} (right turn)", cross((0, 0), (4, 0), (4, -4)));
    outln!(o, "cross((0,0),(2,2),(4,4)) = {} (collinear)", cross((0, 0), (2, 2), (4, 4)));
    let cases: [(&str, Pt, Pt, Pt, Pt); 5] = [
        ("proper crossing        ", (0, 0), (4, 4), (0, 4), (4, 0)),
        ("touching at an endpoint", (0, 0), (2, 2), (2, 2), (4, 0)),
        ("collinear overlapping  ", (0, 0), (4, 0), (2, 0), (6, 0)),
        ("collinear disjoint     ", (0, 0), (1, 0), (2, 0), (3, 0)),
        ("parallel               ", (0, 0), (4, 0), (0, 1), (4, 1)),
    ];
    for (name, a, b, c, d) in cases { outln!(o, "{name}: intersect = {}", segments_intersect(a, b, c, d)); }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    /// Independent check: solve p1 + t·r = p3 + u·s with exact rationals (i128).
    fn reference(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool {
        let (r, s) = ((p2.0 - p1.0) as i128, (p2.1 - p1.1) as i128);
        let (a, b) = ((p4.0 - p3.0) as i128, (p4.1 - p3.1) as i128);
        let (qx, qy) = ((p3.0 - p1.0) as i128, (p3.1 - p1.1) as i128);
        let den = r * b - s * a;
        if den != 0 {
            let (tn, un) = (qx * b - qy * a, qx * s - qy * r);
            let (lo, hi) = if den > 0 { (0, den) } else { (den, 0) };
            return tn >= lo.min(hi) && tn <= lo.max(hi) && un >= lo.min(hi) && un <= lo.max(hi) && {
                let (t_ok, u_ok) = (if den > 0 { tn >= 0 && tn <= den } else { tn <= 0 && tn >= den }, if den > 0 { un >= 0 && un <= den } else { un <= 0 && un >= den });
                t_ok && u_ok
            };
        }
        if qx * s - qy * r != 0 { return false; } // parallel, not collinear
        // collinear: do the bounding boxes overlap?
        let ov = |a1: i64, a2: i64, b1: i64, b2: i64| a1.min(a2) <= b1.max(b2) && b1.min(b2) <= a1.max(a2);
        ov(p1.0, p2.0, p3.0, p4.0) && ov(p1.1, p2.1, p3.1, p4.1)
    }
    #[test]
    fn turn_directions() {
        assert!(cross((0, 0), (4, 0), (4, 4)) > 0);
        assert!(cross((0, 0), (4, 0), (4, -4)) < 0);
        assert_eq!(cross((0, 0), (2, 2), (4, 4)), 0);
    }
    #[test]
    fn named_cases() {
        assert!(segments_intersect((0, 0), (4, 4), (0, 4), (4, 0)));
        assert!(segments_intersect((0, 0), (2, 2), (2, 2), (4, 0)));
        assert!(segments_intersect((0, 0), (4, 0), (2, 0), (6, 0)));
        assert!(!segments_intersect((0, 0), (1, 0), (2, 0), (3, 0)));
        assert!(!segments_intersect((0, 0), (4, 0), (0, 1), (4, 1)));
        assert!(!segments_intersect((0, 0), (1, 1), (2, 2), (3, 0)));
    }
    #[test]
    fn matches_exact_reference_on_small_grids() {
        let mut r = Rng::new(230);
        for _ in 0..5000 {
            let mut p = || (r.range(0, 6) as i64, r.range(0, 6) as i64);
            let (a, b, c, d) = (p(), p(), p(), p());
            if a == b || c == d { continue; }
            assert_eq!(segments_intersect(a, b, c, d), reference(a, b, c, d), "{a:?} {b:?} {c:?} {d:?}");
        }
    }
}
