//! @title Horner's Rule
//! @chapter Problem 2-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Evaluate a degree-n polynomial with n multiplications and n additions.
//! @order 70
use crate::outln;

/// HORNER: y = a0 + x(a1 + x(a2 + … + x·an)); `coeffs[i]` is aᵢ.
pub fn horner(coeffs: &[i64], x: i64) -> i64 {
    let mut y = 0;
    for &a in coeffs.iter().rev() {
        y = a + x * y;
    }
    y
}

/// Naive Θ(n²)-multiplication evaluation (computes every power from scratch).
pub fn naive_eval(coeffs: &[i64], x: i64) -> i64 {
    let mut y = 0;
    for (i, &a) in coeffs.iter().enumerate() {
        let mut p = 1;
        for _ in 0..i {
            p *= x;
        }
        y += a * p;
    }
    y
}

/// Θ(n) evaluation that keeps a running power of x.
pub fn running_power_eval(coeffs: &[i64], x: i64) -> i64 {
    let (mut y, mut p) = (0, 1);
    for &a in coeffs {
        y += a * p;
        p *= x;
    }
    y
}

pub fn demo() -> String {
    let mut o = String::new();
    let c = [1, -2, 0, 3]; // 1 - 2x + 3x^3
    for x in [-2, 0, 1, 3] {
        outln!(o, "P({:>2}) horner={:>4} naive={:>4} running={:>4}", x, horner(&c, x), naive_eval(&c, x), running_power_eval(&c, x));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_three_agree() {
        let c = [5, -3, 2, 0, 7, 1];
        for x in -4..5 {
            assert_eq!(horner(&c, x), naive_eval(&c, x));
            assert_eq!(horner(&c, x), running_power_eval(&c, x));
        }
        assert_eq!(horner(&[], 3), 0);
        assert_eq!(horner(&[1, -2, 0, 3], 2), 1 - 4 + 24);
    }
}
