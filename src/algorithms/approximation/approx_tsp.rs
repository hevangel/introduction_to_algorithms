//! @title Approximate Metric TSP
//! @chapter 35.2 (3e) / 37.2 (4e)
//! @editions 2,3,4
//! @summary Preorder walk of a minimum spanning tree: a 2-approximation when distances satisfy the triangle inequality.
//! @order 20
use crate::outln;

pub type Pt = (f64, f64);

pub fn dist(a: Pt, b: Pt) -> f64 { (a.0 - b.0).hypot(a.1 - b.1) }

/// MST (Prim, O(V²)) as a parent array rooted at 0.
fn mst_parents(pts: &[Pt]) -> Vec<usize> {
    let n = pts.len();
    let (mut key, mut parent, mut done) = (vec![f64::INFINITY; n], vec![usize::MAX; n], vec![false; n]);
    key[0] = 0.0;
    for _ in 0..n {
        let u = (0..n).filter(|&v| !done[v]).min_by(|&a, &b| key[a].partial_cmp(&key[b]).unwrap()).unwrap();
        done[u] = true;
        for v in 0..n { let d = dist(pts[u], pts[v]); if !done[v] && d < key[v] { key[v] = d; parent[v] = u; } }
    }
    parent
}

/// APPROX-TSP-TOUR(G, c, r): build an MST, list its vertices in preorder, and return that order as the tour.
pub fn approx_tsp_tour(pts: &[Pt]) -> (f64, Vec<usize>) {
    let n = pts.len();
    if n == 0 { return (0.0, vec![]); }
    let parent = mst_parents(pts);
    let mut children = vec![vec![]; n];
    for v in 1..n { children[parent[v]].push(v); }
    let (mut tour, mut stack) = (vec![], vec![0usize]);
    while let Some(u) = stack.pop() {
        tour.push(u);                           // preorder = full walk with repeated vertices shortcut away
        for &c in children[u].iter().rev() { stack.push(c); }
    }
    let len = (0..n).map(|i| dist(pts[tour[i]], pts[tour[(i + 1) % n]])).sum();
    (len, tour)
}

pub fn mst_weight(pts: &[Pt]) -> f64 {
    let p = mst_parents(pts);
    (1..pts.len()).map(|v| dist(pts[v], pts[p[v]])).sum()
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (0.0, 3.0), (2.0, 1.5), (6.0, 1.0), (1.0, 5.0)];
    let (approx, tour) = approx_tsp_tour(&pts);
    let d: Vec<Vec<f64>> = pts.iter().map(|&a| pts.iter().map(|&b| dist(a, b)).collect()).collect();
    let (opt, opt_tour) = super::super::np_completeness::hamiltonian_cycle::traveling_salesman_exact(&d).unwrap();
    outln!(o, "MST weight          = {:.3}  (lower bound on OPT)", mst_weight(&pts));
    outln!(o, "approximate tour    = {:.3}  {:?}", approx, tour);
    outln!(o, "optimal tour (DP)   = {:.3}  {:?}", opt, opt_tour);
    outln!(o, "ratio = {:.3}  (guaranteed ≤ 2)", approx / opt);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::np_completeness::hamiltonian_cycle::traveling_salesman_exact;
    use crate::common::Rng;
    #[test]
    fn two_approximation_on_random_euclidean_instances() {
        let mut r = Rng::new(251);
        for _ in 0..60 {
            let n = r.range(3, 9);
            let pts: Vec<Pt> = (0..n).map(|_| (r.f64() * 100.0, r.f64() * 100.0)).collect();
            let (len, tour) = approx_tsp_tour(&pts);
            let mut s = tour.clone();
            s.sort();
            assert_eq!(s, (0..n).collect::<Vec<_>>());
            let d: Vec<Vec<f64>> = pts.iter().map(|&a| pts.iter().map(|&b| dist(a, b)).collect()).collect();
            let opt = traveling_salesman_exact(&d).unwrap().0;
            assert!(len >= opt - 1e-9 && len <= 2.0 * opt + 1e-9, "{len} vs {opt}");
            assert!(mst_weight(&pts) <= opt + 1e-9); // removing an edge from the optimal tour leaves a spanning tree
        }
    }
    #[test]
    fn trivial_cases() {
        assert_eq!(approx_tsp_tour(&[]).0, 0.0);
        assert_eq!(approx_tsp_tour(&[(1.0, 1.0)]).1, vec![0]);
        assert!((approx_tsp_tour(&[(0.0, 0.0), (3.0, 4.0)]).0 - 10.0).abs() < 1e-9);
    }
}
