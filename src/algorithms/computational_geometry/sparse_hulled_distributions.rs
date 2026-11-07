//! @title Sparse-Hulled Distributions and Linear-Time Hulls
//! @chapter Problem 33-5 (3e)
//! @editions 3,4
//! @summary For points drawn uniformly from a disk or square the expected hull has far fewer than n vertices; discarding points inside the octagon of extreme points, then running Graham's scan, gives expected O(n) time.
//! @order 65
use super::convex_hull_graham::graham_scan;
use super::segment_intersection::{cross, Pt};
use crate::outln;

/// Convex hull with the Akl–Toussaint pre-filter: find the extreme points in the directions x, y, x+y, x−y (both signs); every point strictly inside
/// their convex polygon cannot be a hull vertex and is discarded in O(1) per point. Returns (hull, number of points that survived the filter).
pub fn filtered_hull(points: &[Pt]) -> (Vec<Pt>, usize) {
    if points.len() < 8 { return (graham_scan(points), points.len()); }
    let dirs: [fn(&Pt) -> i64; 4] = [|p| p.0, |p| p.1, |p| p.0 + p.1, |p| p.0 - p.1];
    let mut extremes: Vec<Pt> = vec![];
    for f in dirs { extremes.push(*points.iter().min_by_key(|p| f(p)).unwrap()); extremes.push(*points.iter().max_by_key(|p| f(p)).unwrap()); }
    let poly = graham_scan(&extremes);
    if poly.len() < 3 { return (graham_scan(points), points.len()); }
    let survivors: Vec<Pt> = points.iter().copied().filter(|&p| !(0..poly.len()).all(|i| cross(poly[i], poly[(i + 1) % poly.len()], p) > 0)).collect();
    (graham_scan(&survivors), survivors.len())
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = crate::common::Rng::new(33);
    let r = 1_000_000i64;
    outln!(o, "     n | shape  | hull size | survivors after the octagon filter");
    for n in [1_000usize, 10_000, 100_000] {
        let disk: Vec<Pt> = std::iter::repeat_with(|| loop { let p = (rng.range(0, 2 * r as usize) as i64 - r, rng.range(0, 2 * r as usize) as i64 - r); if p.0 * p.0 + p.1 * p.1 <= r * r { break p; } }).take(n).collect();
        let square: Vec<Pt> = (0..n).map(|_| (rng.range(0, 2 * r as usize) as i64, rng.range(0, 2 * r as usize) as i64)).collect();
        for (name, pts) in [("disk", &disk), ("square", &square)] {
            let (h, s) = filtered_hull(pts);
            outln!(o, "{n:>6} | {name:<6} | {:>9} | {s}", h.len());
        }
    }
    outln!(o, "expected hull size: Θ(n^(1/3)) for a disk, Θ(lg n) for a square");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn same_hull_as_plain_graham() {
        let mut r = Rng::new(472);
        for _ in 0..200 {
            let n = r.range(0, 120);
            let pts: Vec<Pt> = (0..n).map(|_| (r.range(0, 60) as i64, r.range(0, 60) as i64)).collect();
            let (h, _) = filtered_hull(&pts);
            assert_eq!(h, graham_scan(&pts));
        }
    }
    #[test]
    fn the_filter_removes_most_points_for_uniform_distributions() {
        let mut r = Rng::new(473);
        let n = 20_000;
        let rad = 100_000i64;
        let disk: Vec<Pt> = std::iter::repeat_with(|| loop { let p = (r.range(0, 2 * rad as usize) as i64 - rad, r.range(0, 2 * rad as usize) as i64 - rad); if p.0 * p.0 + p.1 * p.1 <= rad * rad { break p; } }).take(n).collect();
        let square: Vec<Pt> = (0..n).map(|_| (r.range(0, 200_000) as i64, r.range(0, 200_000) as i64)).collect();
        let (hd, sd) = filtered_hull(&disk);
        let (hs, ss) = filtered_hull(&square);
        assert!(sd < n / 5, "disk survivors {sd}");
        assert!(ss < n / 20, "square survivors {ss}");
        assert!(hd.len() < 200 && hs.len() < 60, "hull sizes {} {}", hd.len(), hs.len());
    }
}
