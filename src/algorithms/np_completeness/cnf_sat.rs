//! @title Satisfiability: Brute Force, DPLL and 2-SAT
//! @chapter 34.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary SAT is NP-complete; compare exhaustive search, DPLL backtracking with unit propagation, and linear-time 2-SAT via SCCs.
//! @order 30
use super::np_verification::{verify_cnf, Cnf};
use crate::outln;

/// Try all 2ⁿ assignments.
pub fn brute_force_sat(n_vars: usize, f: &Cnf) -> Option<Vec<bool>> {
    (0u64..1 << n_vars).map(|m| (0..n_vars).map(|i| m >> i & 1 == 1).collect::<Vec<bool>>()).find(|a| verify_cnf(f, a))
}

/// DPLL: unit propagation + branching. Returns a satisfying assignment (unassigned variables default to false).
pub fn dpll(n_vars: usize, f: &Cnf) -> Option<Vec<bool>> {
    fn solve(f: Cnf, asg: &mut Vec<Option<bool>>) -> bool {
        let mut f = f;
        loop {
            // simplify under the current assignment
            let mut next: Cnf = vec![];
            let mut unit = None;
            for c in &f {
                if c.iter().any(|&l| asg[l.unsigned_abs() as usize - 1] == Some(l > 0)) { continue; } // clause satisfied
                let rest: Vec<i32> = c.iter().copied().filter(|&l| asg[l.unsigned_abs() as usize - 1].is_none()).collect();
                if rest.is_empty() { return false; } // conflict
                if rest.len() == 1 { unit = Some(rest[0]); }
                next.push(rest);
            }
            f = next;
            if f.is_empty() { return true; }
            match unit {
                Some(l) => asg[l.unsigned_abs() as usize - 1] = Some(l > 0), // unit propagation
                None => break,
            }
        }
        let v = f[0][0].unsigned_abs() as usize - 1;
        for val in [true, false] {
            let mut a2 = asg.clone();
            a2[v] = Some(val);
            if solve(f.clone(), &mut a2) { *asg = a2; return true; }
        }
        false
    }
    let mut asg = vec![None; n_vars];
    if solve(f.clone(), &mut asg) { Some(asg.into_iter().map(|x| x.unwrap_or(false)).collect()) } else { None }
}

/// 2-SAT in O(n + m): implication graph (¬a → b, ¬b → a) and strongly connected components (Exercise 34.4-7).
pub fn two_sat(n_vars: usize, f: &Cnf) -> Option<Vec<bool>> {
    let node = |l: i32| 2 * (l.unsigned_abs() as usize - 1) + (l < 0) as usize; // x_v ↦ 2v, ¬x_v ↦ 2v+1
    let mut adj = vec![vec![]; 2 * n_vars];
    for c in f {
        assert!(c.len() <= 2, "2-SAT requires clauses with at most two literals");
        let (a, b) = (c[0], *c.get(1).unwrap_or(&c[0]));
        adj[node(-a)].push(node(b));
        adj[node(-b)].push(node(a));
    }
    let comps = crate::algorithms::graphs::strongly_connected_components::tarjan(&adj);
    let mut id = vec![0; 2 * n_vars];
    for (i, comp) in comps.iter().enumerate() { for &v in comp { id[v] = i; } }
    (0..n_vars).map(|v| {
        if id[2 * v] == id[2 * v + 1] { None } else { Some(id[2 * v] < id[2 * v + 1]) } // Tarjan emits components in reverse topological order
    }).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let f: Cnf = vec![vec![1, 2, -3], vec![-1, 3], vec![-2, 3], vec![1, -2, -3]];
    outln!(o, "formula (x1∨x2∨¬x3)(¬x1∨x3)(¬x2∨x3)(x1∨¬x2∨¬x3)");
    outln!(o, "brute force: {:?}", brute_force_sat(3, &f));
    outln!(o, "DPLL       : {:?}", dpll(3, &f));
    let unsat: Cnf = vec![vec![1, 2], vec![-1, 2], vec![1, -2], vec![-1, -2]];
    outln!(o, "unsatisfiable (all four 2-clauses over x1,x2): DPLL = {:?}, 2-SAT = {:?}", dpll(2, &unsat), two_sat(2, &unsat));
    let two: Cnf = vec![vec![1, 2], vec![-1, 3], vec![-2, -3], vec![3, 4], vec![-4, 1]];
    outln!(o, "2-SAT instance → {:?}", two_sat(4, &two));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_cnf(r: &mut Rng, n: usize, m: usize, width: usize) -> Cnf {
        (0..m).map(|_| (0..r.range(1, width)).map(|_| { let v = r.range(1, n) as i32; if r.range(0, 1) == 0 { v } else { -v } }).collect()).collect()
    }
    #[test]
    fn dpll_agrees_with_brute_force() {
        let mut r = Rng::new(242);
        let (mut sat, mut unsat) = (0, 0);
        for _ in 0..400 {
            let (n, m) = (r.range(1, 7), r.range(1, 14));
            let f = random_cnf(&mut r, n, m, 3);
            let (b, d) = (brute_force_sat(n, &f), dpll(n, &f));
            assert_eq!(b.is_some(), d.is_some(), "{f:?}");
            if let Some(a) = d { assert!(verify_cnf(&f, &a)); sat += 1; } else { unsat += 1; }
        }
        assert!(sat > 40 && unsat > 40, "{sat}/{unsat}");
    }
    #[test]
    fn two_sat_matches_brute_force() {
        let mut r = Rng::new(243);
        let (mut sat, mut unsat) = (0, 0);
        for _ in 0..400 {
            let (n, m) = (r.range(1, 8), r.range(1, 12));
            let f = random_cnf(&mut r, n, m, 2);
            let t = two_sat(n, &f);
            assert_eq!(t.is_some(), brute_force_sat(n, &f).is_some(), "{f:?}");
            if let Some(a) = t { assert!(verify_cnf(&f, &a), "{f:?} {a:?}"); sat += 1 } else { unsat += 1 }
        }
        assert!(sat > 40 && unsat > 40, "{sat}/{unsat}");
    }
}
