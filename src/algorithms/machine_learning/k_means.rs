//! @title k-Means Clustering (Lloyd's Algorithm)
//! @chapter 33.1 (4e)
//! @editions 4
//! @summary Alternate between assigning points to the nearest center and moving centers to cluster means; cost never increases.
//! @order 10
use crate::common::Rng;
use crate::outln;

pub type Point = Vec<f64>;

fn d2(a: &[f64], b: &[f64]) -> f64 { a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum() }

/// k-means++ seeding: first center uniform, later centers chosen with probability ∝ squared distance to the nearest chosen center.
pub fn init_centers(points: &[Point], k: usize, rng: &mut Rng) -> Vec<Point> {
    let mut centers = vec![points[rng.range(0, points.len() - 1)].clone()];
    while centers.len() < k {
        let d: Vec<f64> = points.iter().map(|p| centers.iter().map(|c| d2(p, c)).fold(f64::INFINITY, f64::min)).collect();
        let total: f64 = d.iter().sum();
        if total == 0.0 { centers.push(points[rng.range(0, points.len() - 1)].clone()); continue; }
        let mut t = rng.f64() * total;
        let mut pick = d.len() - 1;
        for (i, &x) in d.iter().enumerate() { if t < x { pick = i; break; } t -= x; }
        centers.push(points[pick].clone());
    }
    centers
}

/// Sum of squared distances from each point to its assigned center — the k-means objective.
pub fn cost(points: &[Point], centers: &[Point], assign: &[usize]) -> f64 { points.iter().zip(assign).map(|(p, &c)| d2(p, &centers[c])).sum() }

/// LLOYD(points, k): returns (centers, assignment, cost history per iteration).
pub fn lloyd(points: &[Point], k: usize, rng: &mut Rng, max_iter: usize) -> (Vec<Point>, Vec<usize>, Vec<f64>) {
    let dim = points[0].len();
    let mut centers = init_centers(points, k, rng);
    let mut assign = vec![0usize; points.len()];
    let mut history = vec![];
    for _ in 0..max_iter {
        // assignment step
        let mut changed = false;
        for (i, p) in points.iter().enumerate() {
            let best = (0..k).min_by(|&a, &b| d2(p, &centers[a]).partial_cmp(&d2(p, &centers[b])).unwrap()).unwrap();
            if assign[i] != best { assign[i] = best; changed = true; }
        }
        history.push(cost(points, &centers, &assign));
        // update step: each center becomes the centroid of its cluster (an empty cluster keeps its center)
        let (mut sums, mut counts) = (vec![vec![0.0; dim]; k], vec![0usize; k]);
        for (p, &c) in points.iter().zip(&assign) { counts[c] += 1; for j in 0..dim { sums[c][j] += p[j]; } }
        for c in 0..k { if counts[c] > 0 { centers[c] = sums[c].iter().map(|s| s / counts[c] as f64).collect(); } }
        if !changed { break; }
    }
    history.push(cost(points, &centers, &assign));
    (centers, assign, history)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(11);
    let blobs = [(0.0, 0.0), (10.0, 0.0), (5.0, 9.0)];
    let mut pts: Vec<Point> = vec![];
    for &(cx, cy) in &blobs { for _ in 0..40 { pts.push(vec![cx + (r.f64() - 0.5) * 3.0, cy + (r.f64() - 0.5) * 3.0]); } }
    let (centers, assign, hist) = lloyd(&pts, 3, &mut r, 50);
    outln!(o, "120 points in 3 blobs; Lloyd's algorithm converged in {} iterations", hist.len() - 1);
    outln!(o, "cost per iteration: {:?}", hist.iter().map(|c| (c * 10.0).round() / 10.0).collect::<Vec<_>>());
    for (i, c) in centers.iter().enumerate() { outln!(o, "center {i}: ({:.2}, {:.2}) with {} points", c[0], c[1], assign.iter().filter(|&&a| a == i).count()); }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn blobs(r: &mut Rng) -> Vec<Point> {
        let mut pts = vec![];
        for &(cx, cy) in &[(0.0, 0.0), (20.0, 0.0), (10.0, 18.0)] { for _ in 0..30 { pts.push(vec![cx + r.f64() * 2.0, cy + r.f64() * 2.0]); } }
        pts
    }
    #[test]
    fn cost_never_increases() {
        let mut r = Rng::new(260);
        for _ in 0..20 {
            let pts: Vec<Point> = (0..80).map(|_| vec![r.f64() * 10.0, r.f64() * 10.0]).collect();
            let (_, _, h) = lloyd(&pts, 4, &mut r, 100);
            assert!(h.windows(2).all(|w| w[1] <= w[0] + 1e-9), "{h:?}");
        }
    }
    #[test]
    fn recovers_well_separated_clusters() {
        let mut r = Rng::new(261);
        for _ in 0..10 {
            let pts = blobs(&mut r);
            let (_, assign, _) = lloyd(&pts, 3, &mut r, 100);
            for b in 0..3 {
                let first = assign[b * 30];
                assert!(assign[b * 30..(b + 1) * 30].iter().all(|&a| a == first), "blob {b} split");
            }
            let mut ids = vec![assign[0], assign[30], assign[60]];
            ids.sort();
            ids.dedup();
            assert_eq!(ids.len(), 3);
        }
    }
    #[test]
    fn single_cluster_is_the_mean() {
        let pts = vec![vec![0.0], vec![2.0], vec![4.0]];
        let (c, _, _) = lloyd(&pts, 1, &mut Rng::new(1), 10);
        assert!((c[0][0] - 2.0).abs() < 1e-12);
    }
}
