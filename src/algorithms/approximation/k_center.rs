//! @title k-Center Clustering
//! @chapter Problem 35-? / Section 35.2 (clustering)
//! @editions 3,4
//! @summary Choose k centers to minimise the largest distance from any point to its nearest center: the farthest-first traversal is a 2-approximation, and nothing better is possible unless P = NP.
//! @order 60
use crate::outln;

pub type Pt = (i64, i64);

fn d2(a: Pt, b: Pt) -> i64 { (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) }

/// Radius² of a set of centers: the max over all points of the squared distance to the nearest center.
pub fn radius2(points: &[Pt], centers: &[usize]) -> i64 {
    points.iter().map(|&p| centers.iter().map(|&c| d2(p, points[c])).min().unwrap()).max().unwrap_or(0)
}

/// GONZALEZ'S FARTHEST-FIRST TRAVERSAL: start with point 0; repeatedly add the point farthest from the current centers.
/// Returns the indices of k centers. 2-approximation. O(nk).
pub fn farthest_first(points: &[Pt], k: usize) -> Vec<usize> {
    let mut centers = vec![0];
    let mut dist: Vec<i64> = points.iter().map(|&p| d2(p, points[0])).collect();
    while centers.len() < k.min(points.len()) {
        let next = (0..points.len()).max_by_key(|&i| dist[i]).unwrap();
        centers.push(next);
        for i in 0..points.len() { dist[i] = dist[i].min(d2(points[i], points[next])); }
    }
    centers
}

/// The optimal choice of k centers **among the input points** by exhaustive search (small inputs).
pub fn optimal_centers(points: &[Pt], k: usize) -> (i64, Vec<usize>) {
    fn go(points: &[Pt], k: usize, start: usize, chosen: &mut Vec<usize>, best: &mut (i64, Vec<usize>)) {
        if chosen.len() == k { let r = radius2(points, chosen); if r < best.0 { *best = (r, chosen.clone()); } return; }
        for i in start..points.len() { chosen.push(i); go(points, k, i + 1, chosen, best); chosen.pop(); }
    }
    let mut best = (i64::MAX, vec![]);
    go(points, k.min(points.len()), 0, &mut vec![], &mut best);
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(0, 0), (1, 1), (0, 2), (10, 0), (11, 1), (10, 2), (5, 10), (6, 11), (4, 9)];
    for k in 1..=4 {
        let g = farthest_first(&pts, k);
        let (opt, oc) = optimal_centers(&pts, k);
        outln!(o, "k = {k}: greedy centers {:?} radius {:.2}; optimal {:?} radius {:.2}", g, (radius2(&pts, &g) as f64).sqrt(), oc, (opt as f64).sqrt());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn two_approximation_against_exhaustive_optimum() {
        let mut r = Rng::new(416);
        for _ in 0..200 {
            let n = r.range(1, 9);
            let pts: Vec<Pt> = (0..n).map(|_| (r.range(0, 30) as i64, r.range(0, 30) as i64)).collect();
            let k = r.range(1, n.min(4));
            let g = farthest_first(&pts, k);
            assert_eq!(g.len(), k);
            let (opt, _) = optimal_centers(&pts, k);
            assert!(radius2(&pts, &g) <= 4 * opt, "radius² {} vs 4·OPT² {}", radius2(&pts, &g), 4 * opt);
        }
    }
    #[test]
    fn well_separated_clusters_are_found() {
        let pts: Vec<Pt> = vec![(0, 0), (1, 0), (100, 0), (101, 1), (0, 100), (1, 101)];
        let g = farthest_first(&pts, 3);
        assert!(radius2(&pts, &g) <= 2);
        assert_eq!(radius2(&pts, &farthest_first(&pts, 6)), 0);
    }
}
