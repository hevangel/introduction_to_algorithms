//! @title Three Algorithms for Fibonacci Numbers
//! @chapter Problem 31-3 (3e)
//! @editions 3,4
//! @summary Naive recursion takes exponential time, iteration O(n) additions, and repeated squaring of the matrix [[1,1],[1,0]] only O(lg n) multiplications — compared by operation counts and bit-cost models.
//! @order 22
use crate::outln;

/// Problem 31-3(a): the naive recursion; returns (F_n, number of calls) — the call count satisfies T(n) = 2F_{n+1} − 1, i.e. Θ(φⁿ).
pub fn fib_recursive(n: u32) -> (u128, u64) {
    if n < 2 { return (n as u128, 1); }
    let (a, ca) = fib_recursive(n - 1);
    let (b, cb) = fib_recursive(n - 2);
    (a + b, ca + cb + 1)
}

/// Problem 31-3(b): memoised / iterative — n − 1 additions.
pub fn fib_iterative(n: u32) -> (u128, u32) {
    if n == 0 { return (0, 0); }
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 1..n { let t = a + b; a = b; b = t; } // b = F_i after the i-th pass
    (b, n.saturating_sub(1))
}

/// Problem 31-3(c): [[1,1],[1,0]]ⁿ by repeated squaring, ⌈lg n⌉ squarings. Returns (F_n, number of 2×2 matrix multiplications).
pub fn fib_matrix(n: u32) -> (u128, u32) {
    type M = [[u128; 2]; 2];
    let mul = |x: &M, y: &M| -> M { [[x[0][0] * y[0][0] + x[0][1] * y[1][0], x[0][0] * y[0][1] + x[0][1] * y[1][1]], [x[1][0] * y[0][0] + x[1][1] * y[1][0], x[1][0] * y[0][1] + x[1][1] * y[1][1]]] };
    let (mut result, mut base, mut e, mut mults) = ([[1, 0], [0, 1]], [[1, 1], [1, 0]], n, 0);
    while e > 0 {
        if e & 1 == 1 { result = mul(&result, &base); mults += 1; }
        e >>= 1;
        if e > 0 { base = mul(&base, &base); mults += 1; }
    }
    (result[0][1], mults)
}

/// Problem 31-3(d)/(e): bit-operation models for F_n, which has about 0.694·n bits.
/// Iteration performs n additions of numbers up to 0.694n bits: Θ(n²); the matrix method does O(lg n) multiplications of numbers
/// up to that size: Θ(M(n)) with M the cost of multiplication (n² schoolbook, n^1.585 Karatsuba, n lg n FFT).
pub fn bit_cost_models(n: u32) -> (f64, f64) {
    let bits = 0.694 * n as f64;
    (n as f64 * bits / 2.0, 3.0 * bits * bits) // iteration: Σ_i 0.694 i ≈ n·bits/2; matrix: ≈ 3 schoolbook multiplications of that size (dominant top level)
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "   n |        F_n | recursive calls | additions | matrix mults");
    for n in [1u32, 5, 10, 20, 30] { let (f, c) = fib_recursive(n); outln!(o, "{n:>4} | {f:>10} | {c:>15} | {:>9} | {:>12}", fib_iterative(n).1, fib_matrix(n).1); }
    let n = 180u32;
    outln!(o, "F_{n} = {} (iterative: {} additions, matrix: {} multiplications)", fib_iterative(n).0, fib_iterative(n).1, fib_matrix(n).1);
    let (it, mx) = bit_cost_models(n);
    outln!(o, "bit-operation models for n = {n}: iteration ≈ {it:.0}, matrix (schoolbook multiply) ≈ {mx:.0}");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_three_agree_and_call_count_is_2f_minus_1() {
        for n in 0..=184u32 { assert_eq!(fib_iterative(n).0, fib_matrix(n).0, "n = {n}"); }
        for n in 0..=25u32 {
            let (f, calls) = fib_recursive(n);
            assert_eq!(f, fib_iterative(n).0);
            assert_eq!(calls as u128, 2 * fib_iterative(n + 1).0 - 1, "n = {n}");
        }
    }
    #[test]
    fn matrix_method_needs_logarithmically_many_multiplications() {
        for n in [1u32, 2, 3, 100, 184] { assert!(fib_matrix(n).1 <= 2 * (32 - n.leading_zeros()), "n = {n}"); }
        assert_eq!(fib_iterative(90).0, 2_880_067_194_370_816_120);
    }
}
