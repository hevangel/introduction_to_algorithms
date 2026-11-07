//! @title Carry-Save Adder
//! @chapter 29.3 (1e)
//! @editions 1
//! @summary Reduce three n-bit numbers to two in constant depth (a row of independent full adders).
//! @order 40
use super::combinational_circuits::{from_bits, to_bits, Circuit};
use crate::outln;

/// Append a carry-save adder to `c`: x + y + z = s + t. Returns (s: n bits, t: n+1 bits where t₀ = 0).
pub fn build_csa(c: &mut Circuit, x: &[usize], y: &[usize], z: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let n = x.len();
    let zero = c.constant(false);
    let (mut s, mut t) = (vec![], vec![zero]);
    for i in 0..n {
        let (sum, carry) = c.full_adder(x[i], y[i], z[i]); // all n full adders work in parallel
        s.push(sum);
        t.push(carry);
    }
    (s, t)
}

/// Standalone circuit; inputs are x, y, z (n bits each); outputs are the sum vector `s` followed by the carry vector `t`.
pub fn carry_save_adder(n: usize) -> (Circuit, Vec<usize>, Vec<usize>) {
    let mut c = Circuit::new();
    let (x, y, z) = (c.inputs(n), c.inputs(n), c.inputs(n));
    let (s, t) = build_csa(&mut c, &x, &y, &z);
    (c, s, t)
}

/// Evaluate: returns (s, t) as integers with s + t = x + y + z.
pub fn add3(n: usize, x: u64, y: u64, z: u64) -> (u64, u64) {
    let (c, s, t) = carry_save_adder(n);
    let mut inp = to_bits(x, n);
    inp.extend(to_bits(y, n));
    inp.extend(to_bits(z, n));
    let v = c.eval(&inp);
    (from_bits(&s.iter().map(|&w| v[w]).collect::<Vec<_>>()), from_bits(&t.iter().map(|&w| v[w]).collect::<Vec<_>>()))
}

pub fn demo() -> String {
    let mut o = String::new();
    let (s, t) = add3(6, 37, 21, 50);
    outln!(o, "37 + 21 + 50 = {} ; carry-save form: s = {s}, t = {t}, s + t = {}", 37 + 21 + 50, s + t);
    for n in [4usize, 16, 64] {
        let (c, s, t) = carry_save_adder(n);
        let outs: Vec<usize> = s.iter().chain(&t).copied().collect();
        outln!(o, "n = {:>2}: size {:>3} gates, depth {} (independent of n!)", n, c.size(), c.max_depth(&outs));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sum_of_outputs_equals_sum_of_inputs() {
        let mut r = Rng::new(172);
        for n in 1..=16 {
            for _ in 0..30 {
                let m = (1u64 << n) - 1;
                let (x, y, z) = (r.next_u64() & m, r.next_u64() & m, r.next_u64() & m);
                let (s, t) = add3(n, x, y, z);
                assert_eq!(s + t, x + y + z);
            }
        }
    }
    #[test]
    fn constant_depth() {
        for n in [1usize, 8, 100] {
            let (c, s, t) = carry_save_adder(n);
            let outs: Vec<usize> = s.iter().chain(&t).copied().collect();
            assert_eq!(c.max_depth(&outs), 3);
            assert_eq!(c.size(), 5 * n);
        }
    }
}
