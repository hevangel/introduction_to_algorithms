//! @title Difference Constraints
//! @chapter 24.4 (3e) / 22.4 (4e) / 25.5 (1e)
//! @editions 1,2,3,4
//! @summary Solve systems xⱼ − xᵢ ≤ w by shortest paths in a constraint graph (Bellman-Ford).
//! @order 130
use super::bellman_ford::bellman_ford;
use crate::outln;

/// A constraint `x[j] − x[i] ≤ w`.
#[derive(Debug, Clone, Copy)]
pub struct Constraint { pub j: usize, pub i: usize, pub w: i64 }

/// Returns a feasible solution (x₁ … xₙ) or None if the system is infeasible.
pub fn solve(n: usize, cs: &[Constraint]) -> Option<Vec<i64>> {
    // constraint graph: vertex n is the virtual source with 0-weight edges to all; edge (i → j) of weight w per constraint
    let mut edges: Vec<(usize, usize, i64)> = cs.iter().map(|c| (c.i, c.j, c.w)).collect();
    for v in 0..n { edges.push((n, v, 0)); }
    let (d, _) = bellman_ford(n + 1, &edges, n)?;
    Some(d[..n].to_vec())
}

pub fn satisfies(x: &[i64], cs: &[Constraint]) -> bool { cs.iter().all(|c| x[c.j] - x[c.i] <= c.w) }

pub fn demo() -> String {
    let mut o = String::new();
    // Figure 24.8 (indices 0..4 for x1..x5)
    let cs = [
        Constraint { j: 0, i: 1, w: 0 }, Constraint { j: 0, i: 4, w: -1 }, Constraint { j: 1, i: 4, w: 1 }, Constraint { j: 2, i: 0, w: 5 },
        Constraint { j: 3, i: 0, w: 4 }, Constraint { j: 3, i: 2, w: -1 }, Constraint { j: 4, i: 2, w: -3 }, Constraint { j: 4, i: 3, w: -3 },
    ];
    let x = solve(5, &cs).unwrap();
    outln!(o, "solution x = {:?}", x);
    outln!(o, "all 8 constraints satisfied: {}", satisfies(&x, &cs));
    let shifted: Vec<i64> = x.iter().map(|v| v + 10).collect();
    outln!(o, "adding a constant to every variable stays feasible: {}", satisfies(&shifted, &cs));
    let bad = [Constraint { j: 0, i: 1, w: -1 }, Constraint { j: 1, i: 0, w: 0 }];
    outln!(o, "x0−x1 ≤ −1 and x1−x0 ≤ 0 → {:?} (negative cycle)", solve(2, &bad));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_system() {
        let cs = [
            Constraint { j: 0, i: 1, w: 0 }, Constraint { j: 0, i: 4, w: -1 }, Constraint { j: 1, i: 4, w: 1 }, Constraint { j: 2, i: 0, w: 5 },
            Constraint { j: 3, i: 0, w: 4 }, Constraint { j: 3, i: 2, w: -1 }, Constraint { j: 4, i: 2, w: -3 }, Constraint { j: 4, i: 3, w: -3 },
        ];
        assert_eq!(solve(5, &cs).unwrap(), vec![-5, -3, 0, -1, -4]);
    }
    #[test]
    fn solutions_satisfy_and_infeasible_detected() {
        let mut r = Rng::new(133);
        for _ in 0..80 {
            let n = r.range(1, 6);
            let cs: Vec<Constraint> = (0..r.range(0, 10)).map(|_| Constraint { j: r.range(0, n - 1), i: r.range(0, n - 1), w: r.range(0, 14) as i64 - 4 }).collect();
            match solve(n, &cs) {
                Some(x) => assert!(satisfies(&x, &cs)),
                None => {
                    // no integer solution within a generous box (brute force over [-12,12]ⁿ, n ≤ 3 to stay cheap)
                    if n <= 3 {
                        let range = -12..=12i64;
                        let mut found = false;
                        for a in range.clone() { for b in if n > 1 { range.clone() } else { 0..=0 } { for c in if n > 2 { range.clone() } else { 0..=0 } {
                            let x = [a, b, c];
                            if satisfies(&x[..n].to_vec(), &cs) { found = true; }
                        } } }
                        assert!(!found, "{cs:?}");
                    }
                }
            }
        }
    }
}
