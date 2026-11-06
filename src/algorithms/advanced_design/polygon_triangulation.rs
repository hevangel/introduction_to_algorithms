//! @title Optimal Polygon Triangulation
//! @chapter 16.4 (1e)
//! @editions 1
//! @summary Minimum-weight triangulation of a convex polygon: the matrix-chain structure in geometry.
//! @order 120
use crate::outln;

pub type Point = (f64, f64);

fn dist(a: Point, b: Point) -> f64 { ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt() }

/// Weight of triangle (i, k, j): its perimeter.
pub fn perimeter(p: &[Point], i: usize, k: usize, j: usize) -> f64 { dist(p[i], p[k]) + dist(p[k], p[j]) + dist(p[i], p[j]) }

/// Minimum total triangle perimeter over all triangulations of the convex polygon p[0..n].
/// Returns (cost, list of triangles (i,k,j)).
pub fn optimal_triangulation(p: &[Point]) -> (f64, Vec<(usize, usize, usize)>) {
    let n = p.len();
    if n < 3 { return (0.0, vec![]); }
    let mut t = vec![vec![0.0f64; n]; n];
    let mut s = vec![vec![0usize; n]; n];
    for len in 2..n {
        for i in 0..n - len {
            let j = i + len;
            t[i][j] = f64::INFINITY;
            for k in i + 1..j {
                let q = t[i][k] + t[k][j] + perimeter(p, i, k, j);
                if q < t[i][j] { t[i][j] = q; s[i][j] = k; }
            }
        }
    }
    fn collect(s: &[Vec<usize>], i: usize, j: usize, out: &mut Vec<(usize, usize, usize)>) {
        if j - i < 2 { return; }
        let k = s[i][j];
        out.push((i, k, j));
        collect(s, i, k, out);
        collect(s, k, j, out);
    }
    let mut tris = vec![];
    collect(&s, 0, n - 1, &mut tris);
    (t[0][n - 1], tris)
}

/// Number of triangulations of an n-gon: the Catalan number C_{n−2}.
pub fn count_triangulations(n: usize) -> u64 {
    let m = n - 2;
    let mut c = 1u64;
    for i in 0..m { c = c * 2 * (2 * i as u64 + 1) / (i as u64 + 2); }
    c
}

pub fn demo() -> String {
    let mut o = String::new();
    let hexagon: Vec<Point> = (0..6).map(|k| { let a = std::f64::consts::PI * 2.0 * k as f64 / 6.0; (a.cos() * 3.0 + 0.3 * (k % 2) as f64, a.sin() * 2.0) }).collect();
    let (cost, tris) = optimal_triangulation(&hexagon);
    outln!(o, "hexagon: optimal total perimeter = {:.4}, triangles = {:?}", cost, tris);
    outln!(o, "number of triangulations of an n-gon (Catalan): n=6 → {}, n=10 → {}", count_triangulations(6), count_triangulations(10));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn brute(p: &[Point], i: usize, j: usize) -> f64 {
        if j - i < 2 { return 0.0; }
        (i + 1..j).map(|k| brute(p, i, k) + brute(p, k, j) + perimeter(p, i, k, j)).fold(f64::INFINITY, f64::min)
    }
    #[test]
    fn triangle_and_square() {
        let tri = [(0.0, 0.0), (4.0, 0.0), (0.0, 3.0)];
        assert!((optimal_triangulation(&tri).0 - 12.0).abs() < 1e-9);
        let sq = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        // either diagonal: two triangles with perimeter 2 + √2 each
        assert!((optimal_triangulation(&sq).0 - 2.0 * (2.0 + 2f64.sqrt())).abs() < 1e-9);
        assert_eq!(optimal_triangulation(&sq).1.len(), 2);
    }
    #[test]
    fn matches_recursive_and_catalan() {
        let pts: Vec<Point> = (0..8).map(|k| { let a = 2.0 * std::f64::consts::PI * k as f64 / 8.0; (a.cos() * (1.0 + 0.1 * k as f64 % 0.3), a.sin()) }).collect();
        let (c, tris) = optimal_triangulation(&pts);
        assert!((c - brute(&pts, 0, 7)).abs() < 1e-9);
        assert_eq!(tris.len(), 6); // n−2 triangles
        assert_eq!(count_triangulations(3), 1);
        assert_eq!(count_triangulations(4), 2);
        assert_eq!(count_triangulations(6), 14);
    }
}
