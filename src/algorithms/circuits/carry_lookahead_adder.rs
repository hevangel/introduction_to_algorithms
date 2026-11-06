//! @title Carry-Lookahead Adder
//! @chapter 29.2 (1e)
//! @editions 1
//! @summary Compute all carries with a parallel-prefix network over (generate, propagate) pairs: Θ(lg n) depth.
//! @order 30
use super::combinational_circuits::{from_bits, to_bits, Circuit};
use crate::outln;

/// Append an n-bit carry-lookahead adder (carry-in = 0) to `c`: returns n+1 sum wires (the last is the carry out).
/// Uses a Kogge–Stone prefix network on (G, P) pairs with the associative operator
/// (G, P) ∘ (G', P') = (G ∨ (P ∧ G'), P ∧ P').
pub fn build_cla(c: &mut Circuit, a: &[usize], b: &[usize]) -> Vec<usize> {
    let n = a.len();
    let p0: Vec<usize> = (0..n).map(|i| c.xor(a[i], b[i])).collect(); // propagate
    let mut g: Vec<usize> = (0..n).map(|i| c.and(a[i], b[i])).collect(); // generate
    let mut p = p0.clone();
    let mut step = 1;
    while step < n {
        let (old_g, old_p) = (g.clone(), p.clone());
        for i in step..n {
            let t = c.and(old_p[i], old_g[i - step]);
            g[i] = c.or(old_g[i], t);
            p[i] = c.and(old_p[i], old_p[i - step]);
        }
        step *= 2;
    }
    // g[i] is now the carry OUT of bit i; the carry INTO bit i is g[i−1] (and 0 for bit 0)
    let mut sum = vec![p0[0]];
    for i in 1..n { sum.push(c.xor(p0[i], g[i - 1])); }
    sum.push(g[n - 1]);
    sum
}

pub fn carry_lookahead_adder(n: usize) -> (Circuit, Vec<usize>) {
    let mut c = Circuit::new();
    let a = c.inputs(n);
    let b = c.inputs(n);
    let sum = build_cla(&mut c, &a, &b);
    (c, sum)
}

pub fn add(n: usize, x: u64, y: u64) -> u64 {
    let (c, outs) = carry_lookahead_adder(n);
    let mut inp = to_bits(x, n);
    inp.extend(to_bits(y, n));
    let v = c.eval(&inp);
    from_bits(&outs.iter().map(|&w| v[w]).collect::<Vec<_>>())
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "100 + 155 with an 8-bit carry-lookahead adder = {}", add(8, 100, 155));
    outln!(o, "  n | ripple depth | lookahead depth | lookahead size");
    for n in [4usize, 8, 16, 32, 64, 128] {
        let (r, ro) = super::ripple_carry_adder::ripple_carry_adder(n);
        let (c, co) = carry_lookahead_adder(n);
        outln!(o, "{:>3} | {:>12} | {:>15} | {:>8}", n, r.max_depth(&ro), c.max_depth(&co), c.size());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn adds_correctly() {
        let mut r = Rng::new(171);
        for n in 1..=16 {
            for _ in 0..40 {
                let mask = (1u64 << n) - 1;
                let (x, y) = (r.next_u64() & mask, r.next_u64() & mask);
                assert_eq!(add(n, x, y), x + y, "n={n}");
            }
            assert_eq!(add(n, (1 << n) - 1, (1 << n) - 1), 2 * ((1 << n) - 1));
        }
    }
    #[test]
    fn depth_is_logarithmic() {
        for k in 2..=7usize {
            let n = 1usize << k;
            let (c, outs) = carry_lookahead_adder(n);
            assert!(c.max_depth(&outs) <= 2 * k + 3, "n={n} depth={}", c.max_depth(&outs));
            let (r, ro) = super::super::ripple_carry_adder::ripple_carry_adder(n);
            if n >= 16 { assert!(c.max_depth(&outs) < r.max_depth(&ro) / 2); }
        }
    }
}
