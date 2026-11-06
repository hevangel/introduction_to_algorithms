//! @title Bitonic Euclidean Traveling Salesman
//! @chapter Problem 15-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary The shortest bitonic closed tour of points sorted by x, found in O(n²) by DP.
//! @order 100
use crate::outln;

pub type Point = (f64, f64);

fn dist(a: Point, b: Point) -> f64 { ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt() }

/// Length of the shortest bitonic tour of the points (sorted internally by x; x-coordinates must be distinct).
pub fn bitonic_tour_length(points: &[Point]) -> f64 {
    let mut p = points.to_vec();
    p.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = p.len();
    match n { 0 | 1 => return 0.0, 2 => return 2.0 * dist(p[0], p[1]), _ => {} }
    // b[i][j] (i < j): shortest pair of disjoint paths from p0 to p_j and p_i covering points 0..=j
    let mut b = vec![vec![f64::INFINITY; n]; n];
    b[0][1] = dist(p[0], p[1]);
    for j in 2..n {
        for i in 0..j - 1 { b[i][j] = b[i][j - 1] + dist(p[j - 1], p[j]); }
        b[j - 1][j] = (0..j - 1).map(|k| b[k][j - 1] + dist(p[k], p[j])).fold(f64::INFINITY, f64::min);
    }
    b[n - 2][n - 1] + dist(p[n - 2], p[n - 1])
}

/// Exhaustive check: assign each interior point to the upper or lower path (2^(n−2) tours).
pub fn bitonic_tour_brute_force(points: &[Point]) -> f64 {
    let mut p = points.to_vec();
    p.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = p.len();
    if n < 3 { return bitonic_tour_length(points); }
    let mut best = f64::INFINITY;
    for mask in 0..1u32 << (n - 2) {
        let (mut up, mut down) = (vec![0usize], vec![0usize]);
        for i in 1..n - 1 { if mask >> (i - 1) & 1 == 1 { up.push(i); } else { down.push(i); } }
        up.push(n - 1);
        down.push(n - 1);
        let len: f64 = up.windows(2).chain(down.windows(2)).map(|w| dist(p[w[0]], p[w[1]])).sum();
        best = best.min(len);
    }
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts = [(0.0, 6.0), (1.0, 0.0), (2.0, 3.0), (5.0, 4.0), (6.0, 1.0), (7.0, 5.0), (8.0, 2.0)];
    outln!(o, "points: {:?}", pts);
    outln!(o, "shortest bitonic tour (DP)    = {:.4}", bitonic_tour_length(&pts));
    outln!(o, "shortest bitonic tour (brute) = {:.4}", bitonic_tour_brute_force(&pts));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn dp_matches_brute_force() {
        let mut r = Rng::new(108);
        for n in 3..=10 {
            let pts: Vec<Point> = (0..n).map(|i| (i as f64 + r.f64() * 0.5, r.f64() * 10.0)).collect();
            assert!((bitonic_tour_length(&pts) - bitonic_tour_brute_force(&pts)).abs() < 1e-9, "n={n}");
        }
    }
    #[test]
    fn tiny_cases() {
        assert_eq!(bitonic_tour_length(&[]), 0.0);
        assert_eq!(bitonic_tour_length(&[(1.0, 1.0)]), 0.0);
        assert!((bitonic_tour_length(&[(0.0, 0.0), (3.0, 4.0)]) - 10.0).abs() < 1e-9);
        // a triangle: its perimeter
        assert!((bitonic_tour_length(&[(0.0, 0.0), (3.0, 0.0), (0.0 + 1.0, 4.0)]) - (3.0 + 5f64.hypot(0.0).min(5.0) * 0.0 + 4f64.hypot(2.0) + 4f64.hypot(1.0))).abs() < 1e-9);
    }
}
