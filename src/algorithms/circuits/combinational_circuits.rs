//! @title Combinational Circuits
//! @chapter 29.1 (1e)
//! @editions 1
//! @summary A gate-level circuit model with size and depth: the building block for adders and multipliers.
//! @order 10
use crate::outln;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gate { Input, Const(bool), Not(usize), And(usize, usize), Or(usize, usize), Xor(usize, usize) }

/// A combinational circuit as a DAG of gates; wire `i` is the output of gate `i`.
/// Size = number of logic gates; depth of a wire = longest gate path from any input.
#[derive(Debug, Clone, Default)]
pub struct Circuit {
    pub gates: Vec<Gate>,
    depth: Vec<usize>,
    pub n_inputs: usize,
}

impl Circuit {
    pub fn new() -> Self { Circuit::default() }
    fn push(&mut self, g: Gate, d: usize) -> usize { self.gates.push(g); self.depth.push(d); self.gates.len() - 1 }
    pub fn input(&mut self) -> usize { self.n_inputs += 1; self.push(Gate::Input, 0) }
    pub fn inputs(&mut self, n: usize) -> Vec<usize> { (0..n).map(|_| self.input()).collect() }
    pub fn constant(&mut self, b: bool) -> usize { self.push(Gate::Const(b), 0) }
    pub fn not(&mut self, a: usize) -> usize { let d = self.depth[a] + 1; self.push(Gate::Not(a), d) }
    pub fn and(&mut self, a: usize, b: usize) -> usize { let d = self.depth[a].max(self.depth[b]) + 1; self.push(Gate::And(a, b), d) }
    pub fn or(&mut self, a: usize, b: usize) -> usize { let d = self.depth[a].max(self.depth[b]) + 1; self.push(Gate::Or(a, b), d) }
    pub fn xor(&mut self, a: usize, b: usize) -> usize { let d = self.depth[a].max(self.depth[b]) + 1; self.push(Gate::Xor(a, b), d) }
    pub fn depth_of(&self, w: usize) -> usize { self.depth[w] }
    pub fn max_depth(&self, outs: &[usize]) -> usize { outs.iter().map(|&w| self.depth[w]).max().unwrap_or(0) }
    /// Number of logic gates (inputs and constants are free).
    pub fn size(&self) -> usize { self.gates.iter().filter(|g| !matches!(g, Gate::Input | Gate::Const(_))).count() }

    /// Evaluate every wire for the given input values (in creation order).
    pub fn eval(&self, inputs: &[bool]) -> Vec<bool> {
        assert_eq!(inputs.len(), self.n_inputs);
        let mut v: Vec<bool> = Vec::with_capacity(self.gates.len());
        let mut it = inputs.iter();
        for g in &self.gates {
            let x = match *g {
                Gate::Input => *it.next().unwrap(),
                Gate::Const(b) => b,
                Gate::Not(a) => !v[a],
                Gate::And(a, b) => v[a] && v[b],
                Gate::Or(a, b) => v[a] || v[b],
                Gate::Xor(a, b) => v[a] ^ v[b],
            };
            v.push(x);
        }
        v
    }
    /// Half adder: (sum, carry).
    pub fn half_adder(&mut self, a: usize, b: usize) -> (usize, usize) { (self.xor(a, b), self.and(a, b)) }
    /// Full adder: (sum, carry-out) — 5 gates, depth 2 for the sum and 3 for the carry.
    pub fn full_adder(&mut self, a: usize, b: usize, cin: usize) -> (usize, usize) {
        let x = self.xor(a, b);
        let sum = self.xor(x, cin);
        let c1 = self.and(a, b);
        let c2 = self.and(x, cin);
        (sum, self.or(c1, c2))
    }
}

/// Little-endian bits of `x` (n bits).
pub fn to_bits(x: u64, n: usize) -> Vec<bool> { (0..n).map(|i| x >> i & 1 == 1).collect() }
pub fn from_bits(b: &[bool]) -> u64 { b.iter().enumerate().map(|(i, &x)| (x as u64) << i).sum() }

pub fn demo() -> String {
    let mut o = String::new();
    let mut c = Circuit::new();
    let (a, b, cin) = (c.input(), c.input(), c.input());
    let (s, cout) = c.full_adder(a, b, cin);
    outln!(o, "full adder: size {} gates, depth(sum) {}, depth(carry) {}", c.size(), c.depth_of(s), c.depth_of(cout));
    outln!(o, " a b cin | sum cout");
    for x in 0..8u8 {
        let v = c.eval(&[x & 1 == 1, x >> 1 & 1 == 1, x >> 2 & 1 == 1]);
        outln!(o, " {} {}  {}  |  {}   {}", x & 1, x >> 1 & 1, x >> 2 & 1, v[s] as u8, v[cout] as u8);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_adder_truth_table() {
        let mut c = Circuit::new();
        let (a, b, cin) = (c.input(), c.input(), c.input());
        let (s, co) = c.full_adder(a, b, cin);
        assert_eq!((c.size(), c.depth_of(s), c.depth_of(co)), (5, 2, 3));
        for x in 0..8u8 {
            let bits = [x & 1 == 1, x >> 1 & 1 == 1, x >> 2 & 1 == 1];
            let v = c.eval(&bits);
            let total = bits.iter().filter(|&&b| b).count();
            assert_eq!(v[s], total % 2 == 1);
            assert_eq!(v[co], total >= 2);
        }
    }
    #[test]
    fn bit_helpers_and_constants() {
        assert_eq!(from_bits(&to_bits(0b1011, 6)), 11);
        let mut c = Circuit::new();
        let t = c.constant(true);
        let x = c.input();
        let n = c.not(x);
        let o = c.and(t, n);
        assert_eq!(c.eval(&[false])[o], true);
        assert_eq!(c.eval(&[true])[o], false);
        assert_eq!(c.size(), 2);
    }
}
