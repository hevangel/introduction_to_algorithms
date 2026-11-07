//! @title NP Verification Algorithms
//! @chapter 34.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Polynomial-time verifiers for Hamiltonian cycle, clique, vertex cover, CNF-SAT and subset sum — the definition of NP.
//! @order 10
use crate::outln;

/// A CNF formula: each clause lists literals; literal +v means variable v (1-based) is true, −v means it is false.
pub type Cnf = Vec<Vec<i32>>;

/// Verify a Hamiltonian-cycle certificate (a permutation of all vertices whose consecutive pairs are edges). O(n).
pub fn verify_hamiltonian_cycle(adj: &[Vec<bool>], cert: &[usize]) -> bool {
    let n = adj.len();
    if cert.len() != n { return false; }
    let mut seen = vec![false; n];
    for &v in cert { if v >= n || std::mem::replace(&mut seen[v], true) { return false; } }
    (0..n).all(|i| adj[cert[i]][cert[(i + 1) % n]])
}
/// A k-clique: k distinct vertices that are pairwise adjacent.
pub fn verify_clique(adj: &[Vec<bool>], set: &[usize], k: usize) -> bool {
    set.len() == k && (0..k).all(|i| (i + 1..k).all(|j| set[i] != set[j] && adj[set[i]][set[j]]))
}
/// A vertex cover of size ≤ k: every edge has an endpoint in the set.
pub fn verify_vertex_cover(edges: &[(usize, usize)], set: &[usize], k: usize) -> bool {
    set.len() <= k && edges.iter().all(|&(u, v)| set.contains(&u) || set.contains(&v))
}
/// SAT certificate = a truth assignment (`a[v-1]` is the value of variable v).
pub fn verify_cnf(f: &Cnf, a: &[bool]) -> bool {
    f.iter().all(|c| c.iter().any(|&l| a[l.unsigned_abs() as usize - 1] == (l > 0)))
}
/// SUBSET-SUM certificate = indices of numbers adding to exactly the target.
pub fn verify_subset_sum(nums: &[i64], target: i64, idx: &[usize]) -> bool {
    let mut seen = vec![false; nums.len()];
    idx.iter().all(|&i| i < nums.len() && !std::mem::replace(&mut seen[i], true)) && idx.iter().map(|&i| nums[i]).sum::<i64>() == target
}

/// NP ⊆ EXP: decide by trying every certificate of `bits` bits with the verifier — exponential, but each check is polynomial.
pub fn decide_by_exhaustive_certificates(bits: u32, verifier: impl Fn(&[bool]) -> bool) -> Option<Vec<bool>> {
    (0u64..1 << bits).map(|m| (0..bits).map(|i| m >> i & 1 == 1).collect::<Vec<bool>>()).find(|c| verifier(c))
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut adj = vec![vec![false; 5]; 5];
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 2)] { adj[a][b] = true; adj[b][a] = true; }
    outln!(o, "Hamiltonian-cycle certificate [0,1,2,3,4]: {}", verify_hamiltonian_cycle(&adj, &[0, 1, 2, 3, 4]));
    outln!(o, "bad certificate [0,2,1,3,4]: {}", verify_hamiltonian_cycle(&adj, &[0, 2, 1, 3, 4]));
    outln!(o, "3-clique {{0,1,2}}: {}", verify_clique(&adj, &[0, 1, 2], 3));
    let edges = [(0, 1), (1, 2), (2, 3)];
    outln!(o, "vertex cover {{1,2}} of the path 0-1-2-3: {}", verify_vertex_cover(&edges, &[1, 2], 2));
    let f: Cnf = vec![vec![1, 2], vec![-1, 3], vec![-2, -3]];
    outln!(o, "(x1∨x2)∧(¬x1∨x3)∧(¬x2∨¬x3) with x=(T,F,T): {}", verify_cnf(&f, &[true, false, true]));
    outln!(o, "found by exhaustive certificate search: {:?}", decide_by_exhaustive_certificates(3, |a| verify_cnf(&f, a)));
    outln!(o, "subset sum {{3,34,4,12,5,2}} target 9 with items [2,3]→4+5: {}", verify_subset_sum(&[3, 34, 4, 12, 5, 2], 9, &[2, 4]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hamiltonian_certificates() {
        let mut adj = vec![vec![false; 4]; 4];
        for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] { adj[a][b] = true; adj[b][a] = true; }
        assert!(verify_hamiltonian_cycle(&adj, &[0, 1, 2, 3]));
        assert!(verify_hamiltonian_cycle(&adj, &[2, 1, 0, 3]));
        assert!(!verify_hamiltonian_cycle(&adj, &[0, 2, 1, 3]));
        assert!(!verify_hamiltonian_cycle(&adj, &[0, 1, 2]));
        assert!(!verify_hamiltonian_cycle(&adj, &[0, 1, 1, 3]));
    }
    #[test]
    fn clique_cover_sat_and_subset_sum() {
        let mut adj = vec![vec![false; 4]; 4];
        for (a, b) in [(0, 1), (1, 2), (0, 2), (2, 3)] { adj[a][b] = true; adj[b][a] = true; }
        assert!(verify_clique(&adj, &[0, 1, 2], 3));
        assert!(!verify_clique(&adj, &[0, 1, 3], 3));
        assert!(!verify_clique(&adj, &[0, 0, 1], 3));
        assert!(verify_vertex_cover(&[(0, 1), (1, 2), (2, 3)], &[1, 2], 2));
        assert!(!verify_vertex_cover(&[(0, 1), (1, 2), (2, 3)], &[1], 2));
        let f: Cnf = vec![vec![1, -2], vec![2, 3], vec![-1, -3]];
        assert!(verify_cnf(&f, &[true, true, false]));
        assert!(!verify_cnf(&f, &[true, false, true]));
        assert!(verify_subset_sum(&[3, 34, 4, 12, 5, 2], 9, &[2, 4]));
        assert!(!verify_subset_sum(&[3, 34, 4, 12, 5, 2], 9, &[2, 2, 3]));
        let unsat: Cnf = vec![vec![1], vec![-1]];
        assert!(decide_by_exhaustive_certificates(1, |a| verify_cnf(&unsat, a)).is_none());
    }
}
