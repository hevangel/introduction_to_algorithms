//! @title The Simplex Algorithm & Duality
//! @chapter 29 (2e–4e); detailed pivoting 29.3, 29.5 (3e)
//! @editions 2,3,4
//! @summary Maximise cᵀx subject to Ax ≤ b, x ≥ 0 by pivoting between basic feasible solutions; initialisation and LP duality.
//! @order 10
use crate::outln;

const EPS: f64 = 1e-9;

#[derive(Debug, Clone, PartialEq)]
pub enum LpResult {
    Optimal { value: f64, x: Vec<f64> },
    Unbounded,
    Infeasible,
}

/// Tableau for  maximise cᵀx  s.t.  Ax ≤ b, x ≥ 0  (m constraints, n variables).
/// Rows hold the slack form; `basic[i]` / `nonbasic[j]` name the variables currently basic / nonbasic
/// (original variables 0..n, slacks n..n+m, and −1 for the auxiliary variable x₀ of INITIALIZE-SIMPLEX).
struct Lp {
    m: usize,
    n: usize,
    basic: Vec<isize>,
    nonbasic: Vec<isize>,
    d: Vec<Vec<f64>>,
}

impl Lp {
    fn new(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Lp {
        let (m, n) = (b.len(), c.len());
        let mut d = vec![vec![0.0; n + 2]; m + 2];
        for i in 0..m {
            for j in 0..n { d[i][j] = a[i][j]; }
            d[i][n] = -1.0;           // coefficient of the auxiliary variable x₀
            d[i][n + 1] = b[i];
        }
        let basic = (0..m).map(|i| (n + i) as isize).collect();
        let mut nonbasic: Vec<isize> = (0..n as isize).collect();
        nonbasic.push(-1);
        for j in 0..n { d[m][j] = -c[j]; }
        d[m + 1][n] = 1.0;
        Lp { m, n, basic, nonbasic, d }
    }
    /// PIVOT(N, B, A, b, c, v, l, e): exchange the leaving basic variable (row r) and entering nonbasic variable (column s).
    fn pivot(&mut self, r: usize, s: usize) {
        let inv = 1.0 / self.d[r][s];
        for i in 0..self.m + 2 {
            if i != r {
                for j in 0..self.n + 2 {
                    if j != s { self.d[i][j] -= self.d[r][j] * self.d[i][s] * inv; }
                }
            }
        }
        for j in 0..self.n + 2 { if j != s { self.d[r][j] *= inv; } }
        for i in 0..self.m + 2 { if i != r { self.d[i][s] *= -inv; } }
        self.d[r][s] = inv;
        std::mem::swap(&mut self.basic[r], &mut self.nonbasic[s]);
    }
    /// SIMPLEX main loop with Bland's rule (smallest-index entering and leaving variables), which cannot cycle.
    /// Returns false if the LP is unbounded.
    fn run(&mut self, phase: u8) -> bool {
        let x = if phase == 1 { self.m + 1 } else { self.m };
        loop {
            let mut s: Option<usize> = None;
            for j in 0..=self.n {
                if phase == 2 && self.nonbasic[j] == -1 { continue; }
                if self.d[x][j] < -EPS && s.is_none_or(|t| self.nonbasic[j] < self.nonbasic[t]) { s = Some(j); }
            }
            let Some(s) = s else { return true }; // all coefficients non-negative ⇒ optimal
            let mut r: Option<usize> = None;
            for i in 0..self.m {
                if self.d[i][s] < EPS { continue; }
                let better = match r {
                    None => true,
                    Some(t) => {
                        let (a, b) = (self.d[i][self.n + 1] / self.d[i][s], self.d[t][self.n + 1] / self.d[t][s]);
                        a < b - EPS || ((a - b).abs() <= EPS && self.basic[i] < self.basic[t])
                    }
                };
                if better { r = Some(i); }
            }
            let Some(r) = r else { return false }; // no limiting row ⇒ unbounded
            self.pivot(r, s);
        }
    }
    fn solve(&mut self) -> LpResult {
        let (m, n) = (self.m, self.n);
        // INITIALIZE-SIMPLEX: if some b_i < 0 the trivial basic solution is infeasible; solve the auxiliary LP first.
        let r = (0..m).min_by(|&i, &j| self.d[i][n + 1].partial_cmp(&self.d[j][n + 1]).unwrap()).unwrap();
        if self.d[r][n + 1] < -EPS {
            self.pivot(r, n);
            if !self.run(1) || self.d[m + 1][n + 1] < -EPS { return LpResult::Infeasible; }
            for i in 0..m {
                if self.basic[i] == -1 {
                    let s = (0..=n).min_by(|&a, &b| {
                        self.d[i][a].partial_cmp(&self.d[i][b]).unwrap().then(self.nonbasic[a].cmp(&self.nonbasic[b]))
                    }).unwrap();
                    self.pivot(i, s);
                }
            }
        }
        if !self.run(2) { return LpResult::Unbounded; }
        let mut x = vec![0.0; n];
        for i in 0..m { if self.basic[i] >= 0 && (self.basic[i] as usize) < n { x[self.basic[i] as usize] = self.d[i][n + 1]; } }
        LpResult::Optimal { value: self.d[m][n + 1], x }
    }
}

/// Maximise cᵀx subject to Ax ≤ b, x ≥ 0.
pub fn simplex(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> LpResult {
    assert!(!b.is_empty());
    Lp::new(a, b, c).solve()
}

/// The dual of "max cᵀx, Ax ≤ b, x ≥ 0" is "min bᵀy, Aᵀy ≥ c, y ≥ 0"; expressed in standard maximisation form
/// (max −bᵀy s.t. −Aᵀy ≤ −c, y ≥ 0). Returns (A′, b′, c′).
pub fn dual(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let (m, n) = (b.len(), c.len());
    let at: Vec<Vec<f64>> = (0..n).map(|j| (0..m).map(|i| -a[i][j]).collect()).collect();
    (at, c.iter().map(|x| -x).collect(), b.iter().map(|x| -x).collect())
}

pub fn demo() -> String {
    let mut o = String::new();
    // CLRS Section 29.3: maximise 3x1 + x2 + 2x3
    let a = vec![vec![1.0, 1.0, 3.0], vec![2.0, 2.0, 5.0], vec![4.0, 1.0, 2.0]];
    let (b, c) = ([30.0, 24.0, 36.0], [3.0, 1.0, 2.0]);
    outln!(o, "primal: max 3x1 + x2 + 2x3  s.t.  x1+x2+3x3 ≤ 30, 2x1+2x2+5x3 ≤ 24, 4x1+x2+2x3 ≤ 36");
    outln!(o, "  -> {:?}", simplex(&a, &b, &c));
    let (da, db, dc) = dual(&a, &b, &c);
    if let LpResult::Optimal { value, x } = simplex(&da, &db, &dc) {
        outln!(o, "dual optimum (min bᵀy) = {:.4} at y = {:?}  — equal to the primal value (strong duality)", -value, x.iter().map(|v| (v * 1e4).round() / 1e4).collect::<Vec<_>>());
    }
    outln!(o, "infeasible: x ≤ −1, x ≥ 0 → {:?}", simplex(&[vec![1.0]], &[-1.0], &[1.0]));
    outln!(o, "unbounded : max x s.t. −x ≤ 1 → {:?}", simplex(&[vec![-1.0]], &[1.0], &[1.0]));
    outln!(o, "needs initialisation (b < 0): max −x1−x2 s.t. −x1−x2 ≤ −2 → {:?}", simplex(&[vec![-1.0, -1.0]], &[-2.0], &[-1.0, -1.0]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn feasible(a: &[Vec<f64>], b: &[f64], x: &[f64]) -> bool {
        x.iter().all(|&v| v > -1e-7) && a.iter().zip(b).all(|(row, &bi)| row.iter().zip(x).map(|(p, q)| p * q).sum::<f64>() <= bi + 1e-6)
    }
    #[test]
    fn book_example() {
        let a = vec![vec![1.0, 1.0, 3.0], vec![2.0, 2.0, 5.0], vec![4.0, 1.0, 2.0]];
        match simplex(&a, &[30.0, 24.0, 36.0], &[3.0, 1.0, 2.0]) {
            LpResult::Optimal { value, x } => {
                assert!((value - 28.0).abs() < 1e-7);
                for (g, w) in x.iter().zip([8.0, 4.0, 0.0]) { assert!((g - w).abs() < 1e-7); }
            }
            r => panic!("{r:?}"),
        }
    }
    #[test]
    fn status_codes() {
        assert_eq!(simplex(&[vec![1.0]], &[-1.0], &[1.0]), LpResult::Infeasible);
        assert_eq!(simplex(&[vec![-1.0]], &[1.0], &[1.0]), LpResult::Unbounded);
        match simplex(&[vec![-1.0, -1.0]], &[-2.0], &[-1.0, -1.0]) {
            LpResult::Optimal { value, .. } => assert!((value + 2.0).abs() < 1e-7),
            r => panic!("{r:?}"),
        }
    }
    /// Brute force for two variables: the optimum lies at an intersection of two constraint boundaries.
    fn brute2(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<f64> {
        let mut lines: Vec<(f64, f64, f64)> = a.iter().zip(b).map(|(r, &bi)| (r[0], r[1], bi)).collect();
        lines.push((1.0, 0.0, 0.0));
        lines.push((0.0, 1.0, 0.0));
        let mut best: Option<f64> = None;
        for i in 0..lines.len() { for j in i + 1..lines.len() {
            let (l1, l2) = (lines[i], lines[j]);
            let det = l1.0 * l2.1 - l1.1 * l2.0;
            if det.abs() < 1e-12 { continue; }
            let x = [(l1.2 * l2.1 - l1.1 * l2.2) / det, (l1.0 * l2.2 - l1.2 * l2.0) / det];
            if feasible(a, b, &x) {
                let v = c[0] * x[0] + c[1] * x[1];
                best = Some(best.map_or(v, |bb: f64| bb.max(v)));
            }
        } }
        best
    }
    #[test]
    fn matches_vertex_enumeration_and_duality() {
        let mut r = Rng::new(190);
        let mut checked = 0;
        for _ in 0..200 {
            let m = r.range(1, 5);
            let a: Vec<Vec<f64>> = (0..m).map(|_| (0..2).map(|_| r.range(0, 8) as f64 - 2.0).collect()).collect();
            let b: Vec<f64> = (0..m).map(|_| r.range(0, 20) as f64 - 3.0).collect();
            let c: Vec<f64> = (0..2).map(|_| r.range(0, 8) as f64 - 2.0).collect();
            // add a bounding box so brute force and simplex both see a bounded region
            let mut a2 = a.clone(); let mut b2 = b.clone();
            a2.push(vec![1.0, 0.0]); b2.push(30.0);
            a2.push(vec![0.0, 1.0]); b2.push(30.0);
            match (simplex(&a2, &b2, &c), brute2(&a2, &b2, &c)) {
                (LpResult::Optimal { value, x }, Some(want)) => {
                    assert!((value - want).abs() < 1e-6, "{value} vs {want}");
                    assert!(feasible(&a2, &b2, &x));
                    let (da, db, dc) = dual(&a2, &b2, &c);
                    if let LpResult::Optimal { value: dv, .. } = simplex(&da, &db, &dc) { assert!((value + dv).abs() < 1e-5, "duality gap {value} vs {}", -dv); } else { panic!("dual not optimal"); }
                    checked += 1;
                }
                (LpResult::Infeasible, None) => {}
                other => panic!("mismatch {other:?} for A={a2:?} b={b2:?} c={c:?}"),
            }
        }
        assert!(checked > 50);
    }
}
