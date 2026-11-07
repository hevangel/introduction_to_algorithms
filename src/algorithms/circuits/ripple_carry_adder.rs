//! @title Ripple-Carry Adder
//! @chapter 29.2 (1e)
//! @editions 1
//! @summary n cascaded full adders: simple, Θ(n) size, but Θ(n) depth because the carry ripples.
//! @order 20
use super::combinational_circuits::{from_bits, to_bits, Circuit};
use crate::outln;

/// Append an n-bit ripple-carry adder to `c`. Returns (sum wires (n bits), carry-out wire).
pub fn build_ripple(c: &mut Circuit, a: &[usize], b: &[usize], cin: usize) -> (Vec<usize>, usize) {
    let mut carry = cin;
    let mut sum = vec![];
    for i in 0..a.len() {
        let (s, co) = c.full_adder(a[i], b[i], carry);
        sum.push(s);
        carry = co;
    }
    (sum, carry)
}

/// A standalone n-bit adder circuit plus its output wires (inputs: a₀..aₙ₋₁, b₀..bₙ₋₁).
pub fn ripple_carry_adder(n: usize) -> (Circuit, Vec<usize>) {
    let mut c = Circuit::new();
    let a = c.inputs(n);
    let b = c.inputs(n);
    let zero = c.constant(false);
    let (mut sum, cout) = build_ripple(&mut c, &a, &b, zero);
    sum.push(cout);
    (c, sum)
}

/// Add two numbers with the circuit.
pub fn add(n: usize, x: u64, y: u64) -> u64 {
    let (c, outs) = ripple_carry_adder(n);
    let mut inp = to_bits(x, n);
    inp.extend(to_bits(y, n));
    let v = c.eval(&inp);
    from_bits(&outs.iter().map(|&w| v[w]).collect::<Vec<_>>())
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "12 + 7 with a 4-bit ripple-carry adder = {}", add(4, 12, 7));
    outln!(o, "15 + 15 = {} (carry-out becomes bit 4)", add(4, 15, 15));
    outln!(o, "  n | size | depth of carry-out");
    for n in [4usize, 8, 16, 32, 64] {
        let (c, outs) = ripple_carry_adder(n);
        outln!(o, "{:>3} | {:>4} | {:>3}   (= 2n + 1)", n, c.size(), c.depth_of(*outs.last().unwrap()));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn adds_correctly() {
        let mut r = Rng::new(170);
        for n in 1..=12 {
            for _ in 0..40 {
                let (x, y) = (r.next_u64() & ((1 << n) - 1), r.next_u64() & ((1 << n) - 1));
                assert_eq!(add(n, x, y), x + y, "n={n}");
            }
        }
    }
    #[test]
    fn size_and_depth_are_linear() {
        for n in [1usize, 2, 8, 20] {
            let (c, outs) = ripple_carry_adder(n);
            assert_eq!(c.size(), 5 * n);
            assert_eq!(c.depth_of(*outs.last().unwrap()), 2 * n + 1);
        }
    }
}
