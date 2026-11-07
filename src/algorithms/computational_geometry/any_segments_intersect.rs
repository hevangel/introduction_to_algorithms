//! @title Sweep Line: Do Any Two Segments Intersect?
//! @chapter 33.2 (1e–3e)
//! @editions 1,2,3
//! @summary Shamos-Hoey sweep: keep segments ordered by y along a moving vertical line and test only neighbours: O(n lg n) probes.
//! @order 20
use super::segment_intersection::{segments_intersect, Pt};
use crate::outln;
use std::cmp::Ordering;

pub type Seg = (Pt, Pt);

/// Compare y of two (non-vertical) segments at abscissa x, exactly, via cross-multiplication.
fn cmp_at(a: &Seg, b: &Seg, x: i64) -> Ordering {
    let num = |s: &Seg| -> (i128, i128) {
        let ((x1, y1), (x2, y2)) = *s;
        let den = (x2 - x1) as i128;
        (y1 as i128 * den + (y2 - y1) as i128 * (x - x1) as i128, den) // y = num/den, den > 0 after normalisation
    };
    let (na, da) = num(a);
    let (nb, db) = num(b);
    (na * db).cmp(&(nb * da))
}

/// ANY-SEGMENTS-INTERSECT(S): assumes no vertical segments and no three segments through one point (as in the book).
/// Returns true iff some two segments intersect. Falls back to the quadratic test if a vertical segment is present.
pub fn any_segments_intersect(segs: &[Seg]) -> bool {
    let s: Vec<Seg> = segs.iter().map(|&(p, q)| if p <= q { (p, q) } else { (q, p) }).collect();
    if s.iter().any(|&(p, q)| p.0 == q.0) {
        return (0..s.len()).any(|i| (i + 1..s.len()).any(|j| segments_intersect(s[i].0, s[i].1, s[j].0, s[j].1)));
    }
    // events: (x, kind, y, segment) with left endpoints (kind 0) before right endpoints (kind 1) at equal x
    let mut ev: Vec<(i64, u8, i64, usize)> = vec![];
    for (i, &(p, q)) in s.iter().enumerate() {
        ev.push((p.0, 0, p.1, i));
        ev.push((q.0, 1, q.1, i));
    }
    ev.sort();
    let mut t: Vec<usize> = vec![]; // the sweep-line status, ordered from bottom to top
    let hit = |a: usize, b: usize| segments_intersect(s[a].0, s[a].1, s[b].0, s[b].1);
    for (x, kind, _, i) in ev {
        if kind == 0 {
            // INSERT: find the position by comparing with the status, then test the new neighbours
            let pos = t.partition_point(|&j| cmp_at(&s[j], &s[i], x) != Ordering::Greater);
            if pos > 0 && hit(t[pos - 1], i) { return true; }
            if pos < t.len() && hit(t[pos], i) { return true; }
            t.insert(pos, i);
        } else {
            let pos = t.iter().position(|&j| j == i).unwrap();
            if pos > 0 && pos + 1 < t.len() && hit(t[pos - 1], t[pos + 1]) { return true; } // DELETE exposes a new adjacent pair
            t.remove(pos);
        }
    }
    false
}

pub fn demo() -> String {
    let mut o = String::new();
    let disjoint: Vec<Seg> = vec![((0, 0), (10, 1)), ((1, 5), (9, 6)), ((2, 10), (8, 12)), ((0, 20), (10, 22))];
    outln!(o, "four stacked non-crossing segments: {}", any_segments_intersect(&disjoint));
    let mut crossing = disjoint.clone();
    crossing.push(((3, -2), (7, 11)));
    outln!(o, "adding a segment that crosses several: {}", any_segments_intersect(&crossing));
    outln!(o, "the sweep keeps at most n segments in the status and probes O(n) neighbour pairs → O(n lg n) with a balanced tree");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(s: &[Seg]) -> bool { (0..s.len()).any(|i| (i + 1..s.len()).any(|j| segments_intersect(s[i].0, s[i].1, s[j].0, s[j].1))) }
    #[test]
    fn agrees_with_all_pairs_on_random_inputs() {
        let mut r = Rng::new(231);
        let (mut yes, mut no) = (0, 0);
        for _ in 0..600 {
            let n = r.range(1, 9);
            // distinct x for all endpoints so there are no vertical segments or shared abscissae
            let mut xs: Vec<i64> = (0..2 * n as i64).collect();
            r.shuffle(&mut xs);
            let segs: Vec<Seg> = (0..n).map(|i| ((xs[2 * i], r.range(0, 1000) as i64), (xs[2 * i + 1], r.range(0, 1000) as i64))).collect();
            let want = brute(&segs);
            assert_eq!(any_segments_intersect(&segs), want, "{segs:?}");
            if want { yes += 1 } else { no += 1 }
        }
        assert!(yes > 50 && no > 50, "{yes} {no}");
    }
    #[test]
    fn simple_cases() {
        assert!(!any_segments_intersect(&[]));
        assert!(!any_segments_intersect(&[((0, 0), (5, 5))]));
        assert!(any_segments_intersect(&[((0, 0), (4, 4)), ((0, 4), (4, 0))]));
        assert!(any_segments_intersect(&[((0, 0), (4, 0)), ((2, 0), (2, 3))])); // vertical fallback
    }
}
