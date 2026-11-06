//! @title Optimal Binary Search Trees
//! @chapter 15.5 (2e–3e), 14.5 (4e)
//! @editions 2,3,4
//! @summary Minimise expected search cost given key and miss probabilities: O(n³) DP, O(n²) with Knuth's monotonicity.
//! @order 50
use crate::outln;

/// OPTIMAL-BST(p, q, n): `p[1..=n]` key probabilities (p[0] unused), `q[0..=n]` dummy-key probabilities.
/// Returns (e, root) where e[1][n] is the expected cost and root[i][j] the optimal root of keys i..=j.
pub fn optimal_bst(p: &[f64], q: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    let n = q.len() - 1;
    let mut e = vec![vec![0.0; n + 1]; n + 2];
    let mut w = vec![vec![0.0; n + 1]; n + 2];
    let mut root = vec![vec![0usize; n + 1]; n + 2];
    for i in 1..=n + 1 {
        e[i][i - 1] = q[i - 1];
        w[i][i - 1] = q[i - 1];
    }
    for l in 1..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            e[i][j] = f64::INFINITY;
            w[i][j] = w[i][j - 1] + p[j] + q[j];
            for r in i..=j {
                let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
                if t < e[i][j] {
                    e[i][j] = t;
                    root[i][j] = r;
                }
            }
        }
    }
    (e, root)
}

/// Knuth's O(n²) refinement: root[i][j-1] ≤ root[i][j] ≤ root[i+1][j] (Exercise 15.5-4).
pub fn optimal_bst_knuth(p: &[f64], q: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<usize>>) {
    let n = q.len() - 1;
    let mut e = vec![vec![0.0; n + 1]; n + 2];
    let mut w = vec![vec![0.0; n + 1]; n + 2];
    let mut root = vec![vec![0usize; n + 1]; n + 2];
    for i in 1..=n + 1 {
        e[i][i - 1] = q[i - 1];
        w[i][i - 1] = q[i - 1];
    }
    for l in 1..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            e[i][j] = f64::INFINITY;
            w[i][j] = w[i][j - 1] + p[j] + q[j];
            let (lo, hi) = if l == 1 { (i, j) } else { (root[i][j - 1], root[i + 1][j]) };
            for r in lo..=hi {
                let t = e[i][r - 1] + e[r + 1][j] + w[i][j];
                if t < e[i][j] {
                    e[i][j] = t;
                    root[i][j] = r;
                }
            }
        }
    }
    (e, root)
}

/// Parenthesised description of the optimal tree, e.g. "k2(k1(d0,d1),k5(...))".
pub fn describe(root: &[Vec<usize>], i: usize, j: usize) -> String {
    if j + 1 == i { return format!("d{j}"); }
    let r = root[i][j];
    format!("k{r}({},{})", describe(root, i, r - 1), describe(root, r + 1, j))
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = [0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
    let q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
    let (e, root) = optimal_bst(&p, &q);
    outln!(o, "p = {:?}", &p[1..]);
    outln!(o, "q = {:?}", q);
    outln!(o, "expected search cost e[1,5] = {:.2}", e[1][5]);
    outln!(o, "optimal tree: {}", describe(&root, 1, 5));
    let (e2, _) = optimal_bst_knuth(&p, &q);
    outln!(o, "Knuth's O(n²) variant gives {:.2}", e2[1][5]);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        let p = [0.0, 0.15, 0.10, 0.05, 0.10, 0.20];
        let q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10];
        let (e, root) = optimal_bst(&p, &q);
        assert!((e[1][5] - 2.75).abs() < 1e-9);
        assert_eq!(root[1][5], 2);
        assert_eq!(describe(&root, 1, 5), "k2(k1(d0,d1),k5(k4(k3(d2,d3),d4),d5))");
    }
    #[test]
    fn knuth_matches_cubic() {
        let mut r = Rng::new(103);
        for n in 1..=12 {
            let raw: Vec<f64> = (0..2 * n + 1).map(|_| r.f64() + 0.01).collect();
            let s: f64 = raw.iter().sum();
            let mut p = vec![0.0];
            p.extend(raw[..n].iter().map(|x| x / s));
            let q: Vec<f64> = raw[n..].iter().map(|x| x / s).collect();
            let (a, _) = optimal_bst(&p, &q);
            let (b, _) = optimal_bst_knuth(&p, &q);
            assert!((a[1][n] - b[1][n]).abs() < 1e-9);
        }
    }
    #[test]
    fn single_key() {
        let (e, root) = optimal_bst(&[0.0, 0.5], &[0.25, 0.25]);
        assert!((e[1][1] - 1.5).abs() < 1e-12); // 0.5·1 + 2·(0.25·2)
        assert_eq!(root[1][1], 1);
    }
}
