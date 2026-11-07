//! @title Matroids & Weighted Greedy
//! @chapter 16.4–16.5 (1e–3e)
//! @editions 1,2,3
//! @summary The matroid greedy algorithm and its instances: maximum spanning tree and unit-time task scheduling with penalties.
//! @order 180
use crate::outln;

/// An independence system over the ground set {0, …, size−1}.
pub trait Matroid {
    fn size(&self) -> usize;
    fn independent(&self, subset: &[usize]) -> bool;
}

/// GREEDY(M, w): consider elements by decreasing weight and keep each that preserves independence.
/// For a matroid the result is a maximum-weight independent set (Theorem 16.11).
pub fn greedy<M: Matroid>(m: &M, weights: &[i64]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..m.size()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(weights[i]));
    let mut a: Vec<usize> = vec![];
    for x in order {
        a.push(x);
        if !m.independent(&a) { a.pop(); }
    }
    a.sort();
    a
}

/// Graphic matroid M_G: independent sets are acyclic edge sets (forests).
pub struct GraphicMatroid { pub vertices: usize, pub edges: Vec<(usize, usize)> }
impl Matroid for GraphicMatroid {
    fn size(&self) -> usize { self.edges.len() }
    fn independent(&self, subset: &[usize]) -> bool {
        let mut p: Vec<usize> = (0..self.vertices).collect();
        fn find(p: &mut Vec<usize>, x: usize) -> usize { if p[x] != x { let r = find(p, p[x]); p[x] = r; } p[x] }
        for &e in subset {
            let (a, b) = (find(&mut p, self.edges[e].0), find(&mut p, self.edges[e].1));
            if a == b { return false; }
            p[a] = b;
        }
        true
    }
}

/// Uniform matroid U_{k,n}: any set of at most k elements is independent.
pub struct UniformMatroid { pub n: usize, pub k: usize }
impl Matroid for UniformMatroid {
    fn size(&self) -> usize { self.n }
    fn independent(&self, subset: &[usize]) -> bool { subset.len() <= self.k }
}

/// Section 16.5: sets of unit-time tasks that can all finish by their deadlines.
/// A is independent iff for every t, N_t(A) = |{a ∈ A : deadline(a) ≤ t}| ≤ t (Lemma 16.12).
pub struct SchedulingMatroid { pub deadlines: Vec<usize> }
impl Matroid for SchedulingMatroid {
    fn size(&self) -> usize { self.deadlines.len() }
    fn independent(&self, subset: &[usize]) -> bool {
        let maxd = self.deadlines.iter().copied().max().unwrap_or(0);
        (1..=maxd).all(|t| subset.iter().filter(|&&a| self.deadlines[a] <= t).count() <= t)
    }
}

/// Schedule unit tasks with deadlines and late-penalties; returns (early tasks in schedule order, total penalty of late tasks).
pub fn schedule_unit_tasks(deadlines: &[usize], penalties: &[i64]) -> (Vec<usize>, i64) {
    let m = SchedulingMatroid { deadlines: deadlines.to_vec() };
    let early = greedy(&m, penalties);
    let mut sched = early.clone();
    sched.sort_by_key(|&a| deadlines[a]); // earliest deadline first
    let late: i64 = (0..deadlines.len()).filter(|a| !early.contains(a)).map(|a| penalties[a]).sum();
    (sched, late)
}

pub fn demo() -> String {
    let mut o = String::new();
    let g = GraphicMatroid { vertices: 5, edges: vec![(0, 1), (1, 2), (0, 2), (2, 3), (3, 4), (2, 4)] };
    let w = [4, 8, 3, 7, 2, 9];
    outln!(o, "maximum spanning tree edges (greedy on the graphic matroid): {:?}", greedy(&g, &w));
    let (sched, penalty) = schedule_unit_tasks(&[4, 2, 4, 3, 1, 4, 6], &[70, 60, 50, 40, 30, 20, 10]);
    outln!(o, "unit-task scheduling: early tasks {:?} (0-based), total penalty {}", sched, penalty);
    outln!(o, "uniform matroid U(2,5) with weights [5,1,9,3,7] picks {:?}", greedy(&UniformMatroid { n: 5, k: 2 }, &[5, 1, 9, 3, 7]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_scheduling_example() {
        let (sched, penalty) = schedule_unit_tasks(&[4, 2, 4, 3, 1, 4, 6], &[70, 60, 50, 40, 30, 20, 10]);
        assert_eq!(penalty, 50);
        assert_eq!(sched.len(), 5);
        // every early task meets its deadline when run in deadline order
        for (slot, &t) in sched.iter().enumerate() { assert!(slot + 1 <= [4, 2, 4, 3, 1, 4, 6][t]); }
    }
    fn brute_best<M: Matroid>(m: &M, w: &[i64]) -> i64 {
        (0..1u32 << m.size()).filter(|mask| {
            let s: Vec<usize> = (0..m.size()).filter(|i| mask >> i & 1 == 1).collect();
            m.independent(&s)
        }).map(|mask| (0..m.size()).filter(|i| mask >> i & 1 == 1).map(|i| w[i]).sum()).max().unwrap()
    }
    #[test]
    fn greedy_is_optimal_on_matroids() {
        let mut r = Rng::new(115);
        for _ in 0..40 {
            let n = r.range(2, 6);
            let edges: Vec<(usize, usize)> = (0..r.range(1, 9)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            let g = GraphicMatroid { vertices: n, edges };
            let w = r.vec(g.size(), 1, 20);
            let sel = greedy(&g, &w);
            assert_eq!(sel.iter().map(|&i| w[i]).sum::<i64>(), brute_best(&g, &w));
            let u = UniformMatroid { n: 6, k: r.range(0, 6) };
            let w2 = r.vec(6, 1, 20);
            assert_eq!(greedy(&u, &w2).iter().map(|&i| w2[i]).sum::<i64>(), brute_best(&u, &w2));
            let dl: Vec<usize> = (0..6).map(|_| r.range(1, 5)).collect();
            let s = SchedulingMatroid { deadlines: dl };
            assert_eq!(greedy(&s, &w2).iter().map(|&i| w2[i]).sum::<i64>(), brute_best(&s, &w2));
        }
    }
}
