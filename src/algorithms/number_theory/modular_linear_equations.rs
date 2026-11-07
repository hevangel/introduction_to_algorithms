//! @title Modular Linear Equations
//! @chapter 31.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Solve ax ≡ b (mod n): d = gcd(a, n) solutions mod n if d | b, none otherwise.
//! @order 30
use super::extended_euclid::extended_euclid;
use crate::outln;

/// MODULAR-LINEAR-EQUATION-SOLVER(a, b, n): all solutions x in 0..n of a·x ≡ b (mod n).
pub fn solve(a: i64, b: i64, n: i64) -> Vec<i64> {
    let (d, x1, _) = extended_euclid(a.rem_euclid(n), n);
    if b.rem_euclid(n) % d != 0 { return vec![]; }
    let x0 = (x1 * (b.rem_euclid(n) / d)).rem_euclid(n / d);
    (0..d).map(|i| (x0 + i * (n / d)) % n).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "14x ≡ 30 (mod 100): {:?}  (gcd = 2, so 2 solutions)", solve(14, 30, 100));
    outln!(o, "3x ≡ 2 (mod 7):    {:?}  (unique solution)", solve(3, 2, 7));
    outln!(o, "4x ≡ 3 (mod 8):    {:?}  (no solution: gcd 4 ∤ 3)", solve(4, 3, 8));
    outln!(o, "6x ≡ 0 (mod 9):    {:?}  (gcd 3 → 3 solutions)", solve(6, 0, 9));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn book_example() {
        let mut s = solve(14, 30, 100);
        s.sort();
        assert_eq!(s, vec![45, 95]);
    }
    #[test]
    fn exhaustive_check() {
        for n in 1..40i64 { for a in 0..n { for b in 0..n {
            let want: Vec<i64> = (0..n).filter(|&x| (a * x - b).rem_euclid(n) == 0).collect();
            let mut got = solve(a, b, n);
            got.sort();
            assert_eq!(got, want, "{a}x ≡ {b} (mod {n})");
        } } }
    }
}
