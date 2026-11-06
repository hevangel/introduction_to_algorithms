//! @title Rod Cutting
//! @chapter 15.1 (3e) / 14.1 (4e); 1e–2e had assembly-line instead
//! @editions 3,4
//! @summary The introductory dynamic program: naive recursion, memoization, bottom-up and solution reconstruction.
//! @order 10
use crate::outln;

/// CUT-ROD(p, n): plain recursion, Θ(2ⁿ). `p[i]` is the price of a piece of length i (`p[0] = 0`).
pub fn cut_rod(p: &[i64], n: usize) -> i64 {
    if n == 0 { return 0; }
    (1..=n).map(|i| p[i] + cut_rod(p, n - i)).max().unwrap()
}

/// MEMOIZED-CUT-ROD: top-down with a table, Θ(n²).
pub fn memoized_cut_rod(p: &[i64], n: usize) -> i64 {
    fn aux(p: &[i64], n: usize, r: &mut Vec<Option<i64>>) -> i64 {
        if let Some(v) = r[n] { return v; }
        let q = if n == 0 { 0 } else { (1..=n).map(|i| p[i] + aux(p, n - i, r)).max().unwrap() };
        r[n] = Some(q);
        q
    }
    aux(p, n, &mut vec![None; n + 1])
}

/// BOTTOM-UP-CUT-ROD: solve subproblems in order of size, Θ(n²).
pub fn bottom_up_cut_rod(p: &[i64], n: usize) -> i64 {
    let mut r = vec![0i64; n + 1];
    for j in 1..=n {
        r[j] = (1..=j).map(|i| p[i] + r[j - i]).max().unwrap();
    }
    r[n]
}

/// EXTENDED-BOTTOM-UP-CUT-ROD: returns the revenues `r` and the first-piece sizes `s`.
pub fn extended_bottom_up_cut_rod(p: &[i64], n: usize) -> (Vec<i64>, Vec<usize>) {
    let (mut r, mut s) = (vec![0i64; n + 1], vec![0usize; n + 1]);
    for j in 1..=n {
        let mut q = i64::MIN;
        for i in 1..=j {
            if q < p[i] + r[j - i] {
                q = p[i] + r[j - i];
                s[j] = i;
            }
        }
        r[j] = q;
    }
    (r, s)
}

/// PRINT-CUT-ROD-SOLUTION: the sizes of the pieces in an optimal cut.
pub fn cut_rod_solution(p: &[i64], mut n: usize) -> Vec<usize> {
    let (_, s) = extended_bottom_up_cut_rod(p, n);
    let mut pieces = vec![];
    while n > 0 {
        pieces.push(s[n]);
        n -= s[n];
    }
    pieces
}

/// Exercise 15.1-3: each cut costs `c`.
pub fn cut_rod_with_cost(p: &[i64], n: usize, c: i64) -> i64 {
    let mut r = vec![0i64; n + 1];
    for j in 1..=n {
        r[j] = (1..=j).map(|i| p[i] + r[j - i] - if i < j { c } else { 0 }).max().unwrap();
    }
    r[n]
}

pub const CLRS_PRICES: [i64; 11] = [0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "prices p[1..10] = {:?}", &CLRS_PRICES[1..]);
    outln!(o, " n | r_n | optimal pieces");
    for n in 1..=10 {
        outln!(o, "{:>2} | {:>3} | {:?}", n, bottom_up_cut_rod(&CLRS_PRICES, n), cut_rod_solution(&CLRS_PRICES, n));
    }
    outln!(o, "naive == memoized == bottom-up for n=10: {}", cut_rod(&CLRS_PRICES, 10) == memoized_cut_rod(&CLRS_PRICES, 10));
    outln!(o, "with a cut cost of 2, n=7 yields {}", cut_rod_with_cost(&CLRS_PRICES, 7, 2));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn book_values() {
        let expected = [0, 1, 5, 8, 10, 13, 17, 18, 22, 25, 30];
        for n in 0..=10 {
            assert_eq!(cut_rod(&CLRS_PRICES, n), expected[n]);
            assert_eq!(memoized_cut_rod(&CLRS_PRICES, n), expected[n]);
            assert_eq!(bottom_up_cut_rod(&CLRS_PRICES, n), expected[n]);
        }
    }
    #[test]
    fn reconstruction_is_consistent() {
        for n in 1..=10 {
            let pieces = cut_rod_solution(&CLRS_PRICES, n);
            assert_eq!(pieces.iter().sum::<usize>(), n);
            assert_eq!(pieces.iter().map(|&i| CLRS_PRICES[i]).sum::<i64>(), bottom_up_cut_rod(&CLRS_PRICES, n));
        }
        assert_eq!(cut_rod_solution(&CLRS_PRICES, 7), vec![1, 6]);
    }
    #[test]
    fn cut_cost_reduces_revenue() {
        assert_eq!(cut_rod_with_cost(&CLRS_PRICES, 10, 0), 30);
        assert_eq!(cut_rod_with_cost(&CLRS_PRICES, 4, 1), 9); // piece of 4 alone = 9 vs 5+5-1 = 9
        assert!(cut_rod_with_cost(&CLRS_PRICES, 7, 3) < 18);
    }
}
