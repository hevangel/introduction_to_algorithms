//! @title Closed-Semiring All-Pairs Path Problems
//! @chapter 26.4 (1e)
//! @editions 1
//! @summary Unify transitive closure, all-pairs shortest paths, widest bottleneck paths, and highest-reliability paths over an algebraic closed semiring (S, ⊕, ⊙, 0̄, 1̄, *).
//! @order 155
use crate::outln;

/// Algebraic closed semiring (S, add/⊕, mul/⊙, zero/0̄, one/1̄, closure/*).
pub trait ClosedSemiring: Clone + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    fn closure(&self) -> Self;
}

/// Generic closed-semiring all-pairs path solver (1e §26.4 Kleene / Floyd-Warshall generalization).
pub fn semiring_apsp<S: ClosedSemiring>(base: &[Vec<S>]) -> Vec<Vec<S>> {
    let n = base.len();
    let mut m: Vec<Vec<S>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        S::one().add(&base[i][j])
                    } else {
                        base[i][j].clone()
                    }
                })
                .collect()
        })
        .collect();

    for k in 0..n {
        let m_prev = m.clone();
        let kk_star = m_prev[k][k].closure();
        for i in 0..n {
            for j in 0..n {
                let through_k = m_prev[i][k].mul(&kk_star).mul(&m_prev[k][j]);
                m[i][j] = m_prev[i][j].add(&through_k);
            }
        }
    }
    m
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BooleanClosure(pub bool);

impl ClosedSemiring for BooleanClosure {
    fn zero() -> Self { Self(false) }
    fn one() -> Self { Self(true) }
    fn add(&self, rhs: &Self) -> Self { Self(self.0 || rhs.0) }
    fn mul(&self, rhs: &Self) -> Self { Self(self.0 && rhs.0) }
    fn closure(&self) -> Self { Self(true) }
}

pub const INF: i64 = i64::MAX / 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TropicalMinPlus(pub i64);

impl ClosedSemiring for TropicalMinPlus {
    fn zero() -> Self { Self(INF) }
    fn one() -> Self { Self(0) }
    fn add(&self, rhs: &Self) -> Self { Self(self.0.min(rhs.0)) }
    fn mul(&self, rhs: &Self) -> Self {
        if self.0 >= INF || rhs.0 >= INF {
            Self(INF)
        } else {
            Self(self.0 + rhs.0)
        }
    }
    fn closure(&self) -> Self { Self(0) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BottleneckMaxMin(pub i64);

impl ClosedSemiring for BottleneckMaxMin {
    fn zero() -> Self { Self(0) }
    fn one() -> Self { Self(INF) }
    fn add(&self, rhs: &Self) -> Self { Self(self.0.max(rhs.0)) }
    fn mul(&self, rhs: &Self) -> Self { Self(self.0.min(rhs.0)) }
    fn closure(&self) -> Self { Self(INF) }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReliabilityMaxMul(pub f64);

impl ClosedSemiring for ReliabilityMaxMul {
    fn zero() -> Self { Self(0.0) }
    fn one() -> Self { Self(1.0) }
    fn add(&self, rhs: &Self) -> Self { Self(self.0.max(rhs.0)) }
    fn mul(&self, rhs: &Self) -> Self { Self(self.0 * rhs.0) }
    fn closure(&self) -> Self { Self(1.0) }
}

pub fn demo() -> String {
    let mut o = String::new();
    let n = 4;
    let edges = [(0, 1, 5, 0.9), (0, 2, 3, 0.7), (1, 3, 2, 0.95), (2, 1, 4, 0.8), (2, 3, 6, 0.6)];

    let mut min_w = vec![vec![TropicalMinPlus::zero(); n]; n];
    let mut cap_w = vec![vec![BottleneckMaxMin::zero(); n]; n];
    let mut rel_w = vec![vec![ReliabilityMaxMul::zero(); n]; n];
    for &(u, v, c, p) in &edges {
        min_w[u][v] = TropicalMinPlus(c);
        cap_w[u][v] = BottleneckMaxMin(c);
        rel_w[u][v] = ReliabilityMaxMul(p);
    }

    let sp = semiring_apsp(&min_w);
    let bp = semiring_apsp(&cap_w);
    let rp = semiring_apsp(&rel_w);
    outln!(o, "4-vertex DAG edges: {edges:?}");
    outln!(o, "shortest path 0 -> 3 (min, +):   {}", sp[0][3].0);
    outln!(o, "widest path   0 -> 3 (max, min): {}", bp[0][3].0);
    outln!(o, "most reliable 0 -> 3 (max, *):   {:.4}", rp[0][3].0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::graphs::floyd_warshall::{floyd_warshall, matrix, transitive_closure};
    use crate::common::Rng;

    #[test]
    fn matches_floyd_warshall_and_transitive_closure() {
        let edges = [(0, 1, 3), (0, 2, 8), (0, 4, -4), (1, 3, 1), (1, 4, 7), (2, 1, 4), (3, 0, 2), (3, 2, -5), (4, 3, 6)];
        let fw_mat = matrix(5, &edges);
        let (fw_dist, _) = floyd_warshall(&fw_mat);

        let mut trop = vec![vec![TropicalMinPlus::zero(); 5]; 5];
        let mut bool_m = vec![vec![BooleanClosure::zero(); 5]; 5];
        let mut adj = vec![vec![false; 5]; 5];
        for &(u, v, c) in &edges {
            trop[u][v] = TropicalMinPlus(c);
            bool_m[u][v] = BooleanClosure(true);
            adj[u][v] = true;
        }
        let sp = semiring_apsp(&trop);
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(sp[i][j].0, fw_dist[i][j]);
            }
        }
        let tc_ref = transitive_closure(&adj);
        let tc_semi = semiring_apsp(&bool_m);
        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(tc_semi[i][j].0, tc_ref[i][j]);
            }
        }
    }

    #[test]
    fn bottleneck_widest_path_matches_simple_path_brute_force() {
        let mut r = Rng::new(264);
        for _ in 0..40 {
            let n = 5;
            let mut cap = vec![vec![BottleneckMaxMin::zero(); n]; n];
            let mut raw = vec![vec![0i64; n]; n];
            for i in 0..n {
                for j in 0..n {
                    if i != j && r.f64() < 0.55 {
                        let c = r.range(1, 50) as i64;
                        cap[i][j] = BottleneckMaxMin(c);
                        raw[i][j] = c;
                    }
                }
            }
            let bp = semiring_apsp(&cap);
            for s in 0..n {
                // Bellman-Ford-style n-step relaxation for widest capacity path from s.
                let mut best = vec![0i64; n];
                best[s] = INF;
                for _ in 0..n {
                    for u in 0..n {
                        for v in 0..n {
                            if raw[u][v] > 0 {
                                best[v] = best[v].max(best[u].min(raw[u][v]));
                            }
                        }
                    }
                }
                for t in 0..n {
                    assert_eq!(bp[s][t].0, best[t]);
                }
            }
        }
    }
}
