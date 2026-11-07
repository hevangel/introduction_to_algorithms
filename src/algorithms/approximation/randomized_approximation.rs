//! @title Randomized Approximation: MAX-3-CNF & MAX-CUT
//! @chapter 35.4 (3e, 4e)
//! @editions 2,3,4
//! @summary A random assignment satisfies 7/8 of clauses in expectation; derandomise with conditional expectations.
//! @order 60
use crate::common::Rng;
use crate::outln;

/// A 3-CNF clause set: literals are ±(variable+1).
pub type Cnf = Vec<Vec<i32>>;

pub fn satisfied(f: &Cnf, a: &[bool]) -> usize {
    f.iter().filter(|c| c.iter().any(|&l| a[l.unsigned_abs() as usize - 1] == (l > 0))).count()
}

/// RANDOMIZED MAX-3-CNF: set every variable to true/false by a fair coin. Each clause with 3 distinct variables is
/// satisfied with probability 7/8, so E[#satisfied] = 7m/8.
pub fn random_assignment(n: usize, rng: &mut Rng) -> Vec<bool> { (0..n).map(|_| rng.next_u64() & 1 == 1).collect() }

/// Method of conditional expectations: fix variables one at a time to the value that keeps the expected number of
/// satisfied clauses highest. Deterministically achieves ≥ 7m/8 (when every clause has 3 distinct variables).
pub fn derandomized(n: usize, f: &Cnf) -> Vec<bool> {
    // expected satisfied clauses given a partial assignment (free variables are fair coins)
    let expect = |asg: &[Option<bool>]| -> f64 {
        f.iter().map(|c| {
            if c.iter().any(|&l| asg[l.unsigned_abs() as usize - 1] == Some(l > 0)) { return 1.0; }
            let free = c.iter().filter(|&&l| asg[l.unsigned_abs() as usize - 1].is_none()).count();
            1.0 - 0.5f64.powi(free as i32)
        }).sum()
    };
    let mut asg: Vec<Option<bool>> = vec![None; n];
    for v in 0..n {
        asg[v] = Some(true);
        let e_true = expect(&asg);
        asg[v] = Some(false);
        let e_false = expect(&asg);
        asg[v] = Some(e_true >= e_false);
    }
    asg.into_iter().map(|x| x.unwrap()).collect()
}

/// Random MAX-CUT: each vertex joins side S with probability ½; expected cut ≥ |E|/2 (Problem 35-3).
pub fn cut_size(edges: &[(usize, usize)], side: &[bool]) -> usize { edges.iter().filter(|&&(u, v)| side[u] != side[v]).count() }

/// Greedy local search for MAX-CUT: flip a vertex if more than half of its edges are uncut. Ends with a cut ≥ |E|/2.
pub fn local_search_cut(n: usize, edges: &[(usize, usize)]) -> Vec<bool> {
    let mut side = vec![false; n];
    loop {
        let mut improved = false;
        for v in 0..n {
            let (same, diff) = edges.iter().filter(|e| e.0 == v || e.1 == v).fold((0, 0), |(s, d), &(a, b)| { let o = if a == v { b } else { a }; if side[o] == side[v] { (s + 1, d) } else { (s, d + 1) } });
            if same > diff { side[v] = !side[v]; improved = true; }
        }
        if !improved { return side; }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(8);
    let (n, m) = (20, 200);
    let f: Cnf = (0..m).map(|_| { let mut vs: Vec<i32> = vec![]; while vs.len() < 3 { let v = r.range(1, n) as i32; if !vs.contains(&v) { vs.push(v); } } vs.into_iter().map(|v| if r.range(0, 1) == 0 { v } else { -v }).collect() }).collect();
    let trials = 2000;
    let avg = (0..trials).map(|_| satisfied(&f, &random_assignment(n, &mut r)) as f64).sum::<f64>() / trials as f64;
    outln!(o, "MAX-3-CNF with n={n}, m={m}: random assignment satisfies {:.1} on average (7m/8 = {})", avg, 7 * m / 8);
    outln!(o, "derandomised assignment satisfies {} (≥ 7m/8 = {})", satisfied(&f, &derandomized(n, &f)), 7 * m / 8);
    let edges: Vec<(usize, usize)> = (0..60).map(|_| (r.range(0, 14), r.range(0, 14))).filter(|e| e.0 != e.1).collect();
    outln!(o, "MAX-CUT on {} edges: local search cut {} ≥ |E|/2 = {}", edges.len(), cut_size(&edges, &local_search_cut(15, &edges)), edges.len() / 2);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn random_3cnf(r: &mut Rng, n: usize, m: usize) -> Cnf {
        (0..m).map(|_| { let mut vs: Vec<i32> = vec![]; while vs.len() < 3 { let v = r.range(1, n) as i32; if !vs.contains(&v) { vs.push(v); } } vs.into_iter().map(|v| if r.range(0, 1) == 0 { v } else { -v }).collect() }).collect()
    }
    #[test]
    fn expectation_is_seven_eighths() {
        let mut r = Rng::new(255);
        let f = random_3cnf(&mut r, 15, 120);
        let avg = (0..4000).map(|_| satisfied(&f, &random_assignment(15, &mut r)) as f64).sum::<f64>() / 4000.0;
        assert!((avg - 105.0).abs() < 2.0, "{avg}");
    }
    #[test]
    fn derandomization_meets_the_guarantee() {
        let mut r = Rng::new(256);
        for _ in 0..100 {
            let (n, m) = (r.range(3, 12), r.range(1, 60));
            let f = random_3cnf(&mut r, n, m);
            let s = satisfied(&f, &derandomized(n, &f));
            assert!(8 * s >= 7 * m, "{s}/{m}");
        }
    }
    #[test]
    fn local_search_cuts_at_least_half_the_edges() {
        let mut r = Rng::new(257);
        for _ in 0..100 {
            let n = r.range(2, 12);
            let edges: Vec<(usize, usize)> = (0..r.range(1, 30)).map(|_| (r.range(0, n - 1), r.range(0, n - 1))).filter(|e| e.0 != e.1).collect();
            let c = cut_size(&edges, &local_search_cut(n, &edges));
            assert!(2 * c >= edges.len());
        }
    }
}
