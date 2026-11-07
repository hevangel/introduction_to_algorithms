//! @title Matrix-Chain Multiplication
//! @chapter 15.2 (1e–3e: 16.1/15.2), 14.2 (4e)
//! @editions 1,2,3,4
//! @summary Choose the parenthesization minimising scalar multiplications: Θ(n³) DP.
//! @order 30
use crate::outln;

/// MATRIX-CHAIN-ORDER: matrix `A_i` is `p[i-1] × p[i]`. Returns (m, s) with 1-based indices.
pub fn matrix_chain_order(p: &[usize]) -> (Vec<Vec<u64>>, Vec<Vec<usize>>) {
    let n = p.len() - 1;
    let mut m = vec![vec![0u64; n + 1]; n + 1];
    let mut s = vec![vec![0usize; n + 1]; n + 1];
    for l in 2..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            m[i][j] = u64::MAX;
            for k in i..j {
                let q = m[i][k] + m[k + 1][j] + (p[i - 1] * p[k] * p[j]) as u64;
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k;
                }
            }
        }
    }
    (m, s)
}

/// PRINT-OPTIMAL-PARENS
pub fn optimal_parens(s: &[Vec<usize>], i: usize, j: usize) -> String {
    if i == j { format!("A{i}") } else { format!("({}{})", optimal_parens(s, i, s[i][j]), optimal_parens(s, s[i][j] + 1, j)) }
}

/// RECURSIVE-MATRIX-CHAIN: Θ(2ⁿ) — included to show the blow-up.
pub fn recursive_matrix_chain(p: &[usize], i: usize, j: usize) -> u64 {
    if i == j { return 0; }
    (i..j).map(|k| recursive_matrix_chain(p, i, k) + recursive_matrix_chain(p, k + 1, j) + (p[i - 1] * p[k] * p[j]) as u64).min().unwrap()
}

/// MEMOIZED-MATRIX-CHAIN
pub fn memoized_matrix_chain(p: &[usize]) -> u64 {
    fn look(p: &[usize], i: usize, j: usize, m: &mut Vec<Vec<u64>>) -> u64 {
        if m[i][j] != u64::MAX { return m[i][j]; }
        m[i][j] = if i == j { 0 } else {
            (i..j).map(|k| look(p, i, k, m) + look(p, k + 1, j, m) + (p[i - 1] * p[k] * p[j]) as u64).min().unwrap()
        };
        m[i][j]
    }
    let n = p.len() - 1;
    look(p, 1, n, &mut vec![vec![u64::MAX; n + 1]; n + 1])
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = [30, 35, 15, 5, 10, 20, 25];
    let (m, s) = matrix_chain_order(&p);
    outln!(o, "dimensions p = {:?}", p);
    outln!(o, "minimum scalar multiplications m[1,6] = {}", m[1][6]);
    outln!(o, "optimal parenthesization: {}", optimal_parens(&s, 1, 6));
    outln!(o, "memoized agrees: {}; naive recursion agrees: {}", memoized_matrix_chain(&p) == m[1][6], recursive_matrix_chain(&p, 1, 6) == m[1][6]);
    let left_to_right: u64 = (2..p.len()).map(|k| (p[0] * p[k - 1] * p[k]) as u64).sum();
    outln!(o, "naive left-to-right order would cost {}", left_to_right);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        let p = [30, 35, 15, 5, 10, 20, 25];
        let (m, s) = matrix_chain_order(&p);
        assert_eq!(m[1][6], 15125);
        assert_eq!(optimal_parens(&s, 1, 6), "((A1(A2A3))((A4A5)A6))");
    }
    #[test]
    fn three_strategies_agree() {
        let mut r = Rng::new(101);
        for n in 1..=8 {
            let p: Vec<usize> = (0..=n).map(|_| r.range(1, 20)).collect();
            let (m, _) = matrix_chain_order(&p);
            assert_eq!(m[1][n], memoized_matrix_chain(&p));
            assert_eq!(m[1][n], recursive_matrix_chain(&p, 1, n));
        }
    }
    #[test]
    fn single_matrix_costs_nothing() {
        let (m, s) = matrix_chain_order(&[5, 7]);
        assert_eq!(m[1][1], 0);
        assert_eq!(optimal_parens(&s, 1, 1), "A1");
    }
}
