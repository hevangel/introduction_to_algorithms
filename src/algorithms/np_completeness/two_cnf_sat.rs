//! @title 2-CNF Satisfiability in Polynomial Time
//! @chapter Exercise 34.4-7 / 34.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Unlike 3-CNF-SAT, 2-CNF-SAT is in P: build the implication graph, find strongly connected components, and unsatisfiable ⇔ some x and ¬x share a component. O(n + m).
//! @order 25
use crate::outln;

/// A clause (a ∨ b) over variables 1..=n; a literal is +v for x_v and −v for ¬x_v.
pub type Clause = (i64, i64);

fn node(lit: i64) -> usize { let v = lit.unsigned_abs() as usize - 1; 2 * v + (lit < 0) as usize }

/// Tarjan's SCC on the implication graph (components are numbered in reverse topological order: sinks first).
fn scc(adj: &[Vec<usize>]) -> Vec<usize> {
    struct S<'a> { adj: &'a [Vec<usize>], idx: Vec<usize>, low: Vec<usize>, on: Vec<bool>, stack: Vec<usize>, comp: Vec<usize>, counter: usize, ncomp: usize }
    fn visit(s: &mut S, u: usize) {
        s.counter += 1;
        s.idx[u] = s.counter; s.low[u] = s.counter;
        s.stack.push(u); s.on[u] = true;
        for k in 0..s.adj[u].len() {
            let v = s.adj[u][k];
            if s.idx[v] == 0 { visit(s, v); s.low[u] = s.low[u].min(s.low[v]); }
            else if s.on[v] { s.low[u] = s.low[u].min(s.idx[v]); }
        }
        if s.low[u] == s.idx[u] {
            while let Some(v) = s.stack.pop() { s.on[v] = false; s.comp[v] = s.ncomp; if v == u { break; } }
            s.ncomp += 1;
        }
    }
    let n = adj.len();
    let mut s = S { adj, idx: vec![0; n], low: vec![0; n], on: vec![false; n], stack: vec![], comp: vec![0; n], counter: 0, ncomp: 0 };
    for u in 0..n { if s.idx[u] == 0 { visit(&mut s, u); } }
    s.comp
}

/// A satisfying assignment (index 0 ↔ x₁) or None. Each clause (a ∨ b) gives implications ¬a → b and ¬b → a.
pub fn solve_2sat(n: usize, clauses: &[Clause]) -> Option<Vec<bool>> {
    let mut adj = vec![vec![]; 2 * n];
    for &(a, b) in clauses { adj[node(-a)].push(node(b)); adj[node(-b)].push(node(a)); }
    let comp = scc(&adj);
    let mut val = vec![false; n];
    for v in 0..n {
        if comp[2 * v] == comp[2 * v + 1] { return None; } // x and ¬x are equivalent ⇒ contradiction
        val[v] = comp[2 * v] < comp[2 * v + 1]; // pick the literal whose component comes later in topological order
    }
    Some(val)
}

pub fn satisfies(clauses: &[Clause], val: &[bool]) -> bool {
    let lit = |l: i64| val[l.unsigned_abs() as usize - 1] == (l > 0);
    clauses.iter().all(|&(a, b)| lit(a) || lit(b))
}

pub fn demo() -> String {
    let mut o = String::new();
    let sat = [(1, 2), (-1, 3), (-2, -3), (2, 3)];
    outln!(o, "(x1∨x2)(¬x1∨x3)(¬x2∨¬x3)(x2∨x3): {:?}", solve_2sat(3, &sat));
    let unsat = [(1, 1), (-1, -1)];
    outln!(o, "(x1)(¬x1): {:?}", solve_2sat(1, &unsat));
    let forced = [(1, 2), (1, -2), (-1, 2), (-1, -2)];
    outln!(o, "all four clauses on x1,x2: {:?}", solve_2sat(2, &forced));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn agrees_with_brute_force() {
        let mut r = Rng::new(411);
        for _ in 0..400 {
            let n = r.range(1, 7);
            let lit = |r: &mut Rng| { let v = r.range(1, n) as i64; if r.range(0, 1) == 0 { v } else { -v } };
            let clauses: Vec<Clause> = (0..r.range(0, 14)).map(|_| (lit(&mut r), lit(&mut r))).collect();
            let brute = (0u32..1 << n).any(|m| satisfies(&clauses, &(0..n).map(|i| m >> i & 1 == 1).collect::<Vec<_>>()));
            match solve_2sat(n, &clauses) {
                Some(v) => { assert!(brute); assert!(satisfies(&clauses, &v), "{clauses:?} {v:?}"); }
                None => assert!(!brute, "{clauses:?}"),
            }
        }
    }
    #[test]
    fn large_chain_of_implications() {
        let n = 5000;
        let mut clauses: Vec<Clause> = (1..n as i64).map(|i| (-i, i + 1)).collect(); // x1 → x2 → … → xn
        clauses.push((1, 1));
        let v = solve_2sat(n, &clauses).unwrap();
        assert!(v.iter().all(|&b| b));
        clauses.push((-(n as i64), -(n as i64)));
        assert!(solve_2sat(n, &clauses).is_none());
    }
}
