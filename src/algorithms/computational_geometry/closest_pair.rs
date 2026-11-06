//! @title Closest Pair of Points
//! @chapter 33.4 (1e–3e)
//! @editions 1,2,3
//! @summary Divide and conquer on x with a y-sorted strip: only 7 comparisons per point, O(n lg n).
//! @order 50
use super::segment_intersection::Pt;
use crate::outln;

fn d2(a: Pt, b: Pt) -> i64 { (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) }

/// Θ(n²) brute force: (squared distance, pair).
pub fn closest_pair_brute(p: &[Pt]) -> Option<(i64, Pt, Pt)> {
    let mut best: Option<(i64, Pt, Pt)> = None;
    for i in 0..p.len() { for j in i + 1..p.len() {
        let d = d2(p[i], p[j]);
        if best.is_none_or(|b| d < b.0) { best = Some((d, p[i], p[j])); }
    } }
    best
}

/// Divide and conquer: returns (squared distance, pair) of a closest pair; None if fewer than two points.
pub fn closest_pair(points: &[Pt]) -> Option<(i64, Pt, Pt)> {
    if points.len() < 2 { return None; }
    let mut px = points.to_vec();
    px.sort(); // sorted by x (then y)
    fn rec(px: &[Pt]) -> ((i64, Pt, Pt), Vec<Pt>) {
        let n = px.len();
        if n <= 3 {
            let mut ys = px.to_vec();
            ys.sort_by_key(|p| (p.1, p.0));
            return (closest_pair_brute(px).unwrap(), ys);
        }
        let mid = n / 2;
        let midx = px[mid].0;
        let (dl, yl) = rec(&px[..mid]);
        let (dr, yr) = rec(&px[mid..]);
        let mut best = if dl.0 <= dr.0 { dl } else { dr };
        // merge the y-sorted halves (as in merge sort)
        let mut ys = Vec::with_capacity(n);
        let (mut i, mut j) = (0, 0);
        while i < yl.len() || j < yr.len() {
            if j >= yr.len() || (i < yl.len() && (yl[i].1, yl[i].0) <= (yr[j].1, yr[j].0)) { ys.push(yl[i]); i += 1; } else { ys.push(yr[j]); j += 1; }
        }
        // strip of points within √best of the dividing line, in y order
        let strip: Vec<Pt> = ys.iter().copied().filter(|p| (p.0 - midx).pow(2) < best.0).collect();
        for a in 0..strip.len() {
            for b in a + 1..strip.len() {
                if (strip[b].1 - strip[a].1).pow(2) >= best.0 { break; } // at most 7 successors can be this close
                let d = d2(strip[a], strip[b]);
                if d < best.0 { best = (d, strip[a], strip[b]); }
            }
        }
        (best, ys)
    }
    Some(rec(&px).0)
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(2, 3), (12, 30), (40, 50), (5, 1), (12, 10), (3, 4)];
    let (d, a, b) = closest_pair(&pts).unwrap();
    outln!(o, "points {:?}", pts);
    outln!(o, "closest pair {:?} – {:?} at distance √{} = {:.4}", a, b, d, (d as f64).sqrt());
    let mut r = crate::common::Rng::new(3);
    let big: Vec<Pt> = (0..20_000).map(|_| (r.range(0, 1_000_000) as i64, r.range(0, 1_000_000) as i64)).collect();
    let (d, ..) = closest_pair(&big).unwrap();
    outln!(o, "20000 random points: squared distance {} (n lg n divide and conquer)", d);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_brute_force() {
        let mut r = Rng::new(234);
        for _ in 0..300 {
            let n = r.range(0, 60);
            let pts: Vec<Pt> = (0..n).map(|_| (r.range(0, 50) as i64, r.range(0, 50) as i64)).collect();
            assert_eq!(closest_pair(&pts).map(|x| x.0), closest_pair_brute(&pts).map(|x| x.0), "{pts:?}");
        }
    }
    #[test]
    fn duplicates_and_degenerate() {
        assert_eq!(closest_pair(&[(1, 1), (5, 5), (1, 1)]).unwrap().0, 0);
        assert!(closest_pair(&[(1, 1)]).is_none());
        assert_eq!(closest_pair(&[(0, 0), (3, 4)]).unwrap().0, 25);
        let line: Vec<Pt> = (0..100).map(|i| (i * 7, 0)).collect();
        assert_eq!(closest_pair(&line).unwrap().0, 49);
    }
}
