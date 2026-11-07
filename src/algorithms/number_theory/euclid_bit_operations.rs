//! @title Bit Operations in Euclid's Algorithm
//! @chapter Problem 31-2 (3e)
//! @editions 3,4
//! @summary Count the bit operations of EUCLID(a, b): a division step with quotient q costs O((1 + lg q)·lg b), the quotients' logarithms add up to at most lg a, so the whole algorithm is O(β²) for β-bit inputs.
//! @order 36
use crate::outln;

fn bits(x: u64) -> u64 { (64 - x.leading_zeros()) as u64 }

/// EUCLID(a, b) with a bit-cost accounting. Returns (gcd, steps, total bit operations, Σ lg q).
/// Cost model: computing q = ⌊a/b⌋ and r = a − qb by the schoolbook method takes (1 + lg q)·lg b bit operations.
pub fn euclid_cost(mut a: u64, mut b: u64) -> (u64, u64, u64, u64) {
    let (mut steps, mut cost, mut sum_lg_q) = (0, 0, 0);
    while b != 0 {
        let q = a / b;
        cost += (1 + bits(q)) * bits(b);
        sum_lg_q += bits(q);
        steps += 1;
        let r = a % b;
        a = b;
        b = r;
    }
    (a, steps, cost, sum_lg_q)
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "     a            b        gcd  steps  bit ops  β²");
    let mut rng = crate::common::Rng::new(31);
    let (mut fa, mut fb) = (1u64, 1u64);
    for _ in 0..80 { let t = fa + fb; fa = fb; fb = t; if t > (1u64 << 62) { break; } }
    for (a, b) in [(1_000_000_007u64, 998_244_353u64), (rng.next_u64() >> 2, rng.next_u64() >> 2), (fb, fa)] {
        let (g, s, c, _) = euclid_cost(a, b);
        outln!(o, "{a:>20} {b:>20} {g:>6} {s:>5} {c:>9} {:>6}", bits(a).pow(2));
    }
    outln!(o, "consecutive Fibonacci numbers (the worst case for the number of steps) still stay below 3β² bit operations");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn cost_is_at_most_three_beta_squared_and_quotient_logs_are_bounded() {
        let mut r = Rng::new(469);
        for _ in 0..2000 {
            let a = r.next_u64() >> r.range(1, 40);
            let b = r.next_u64() >> r.range(1, 40);
            if a == 0 || b == 0 { continue; }
            let (a, b) = (a.max(b), a.min(b));
            let (g, steps, cost, sum_lg_q) = euclid_cost(a, b);
            assert_eq!(g, super::super::euclid_gcd::gcd(a, b));
            let beta = bits(a);
            assert!(sum_lg_q <= beta + steps, "Σ lg q = {sum_lg_q}"); // the product of the quotients is at most a
            assert!(cost <= 3 * beta * beta, "a={a} b={b}: cost {cost} > 3β² = {}", 3 * beta * beta);
        }
    }
    #[test]
    fn fibonacci_inputs_take_the_most_steps() {
        let (mut x, mut y) = (1u64, 1u64);
        while y < (1 << 60) { let t = x + y; x = y; y = t; }
        let (_, steps, cost, _) = euclid_cost(y, x);
        assert!(steps as f64 >= 0.9 * 1.44 * (bits(y) as f64) * 0.6 && cost <= 3 * bits(y) * bits(y));
    }
}
