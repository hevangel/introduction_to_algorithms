//! @title Clocked Circuits: Bit-Serial Arithmetic
//! @chapter 29.4 (1e)
//! @editions 1
//! @summary Sequential circuits with registers: a one-full-adder serial adder (n cycles) and a shift-and-add multiplier.
//! @order 60
use crate::outln;

/// Bit-serial adder: one full adder plus a carry flip-flop; processes one bit pair per clock tick, LSB first.
#[derive(Debug, Clone, Default)]
pub struct BitSerialAdder {
    carry: bool, // the register
    pub cycles: usize,
}

impl BitSerialAdder {
    pub fn new() -> Self { Self::default() }
    /// One clock tick: combinational logic computes (sum, next carry); the register latches the carry at the clock edge.
    pub fn tick(&mut self, a: bool, b: bool) -> bool {
        let sum = a ^ b ^ self.carry;
        let next = (a & b) | (self.carry & (a ^ b));
        self.carry = next;
        self.cycles += 1;
        sum
    }
    pub fn carry(&self) -> bool { self.carry }
}

/// Add two n-bit numbers serially: n + 1 cycles (the last flushes the carry).
pub fn serial_add(a: u64, b: u64, n: usize) -> (u64, usize) {
    let mut adder = BitSerialAdder::new();
    let mut out = 0u64;
    for i in 0..n { out |= (adder.tick(a >> i & 1 == 1, b >> i & 1 == 1) as u64) << i; }
    out |= (adder.tick(false, false) as u64) << n;
    (out, adder.cycles)
}

/// Sequential shift-and-add multiplier: a 2n-bit accumulator and n clock cycles, each adding `a << i` when bit i of b is set.
/// Returns (product, cycles).
pub fn shift_add_multiply(a: u64, b: u64, n: usize) -> (u64, usize) {
    let (mut acc, mut multiplicand, mut multiplier) = (0u64, a, b);
    for _ in 0..n {
        if multiplier & 1 == 1 { acc += multiplicand; } // one parallel adder per cycle
        multiplicand <<= 1;                              // shift registers
        multiplier >>= 1;
    }
    (acc, n)
}

/// Fully serial multiplier built from bit-serial adders: n additions of ≈ 2n cycles each.
pub fn serial_multiply(a: u64, b: u64, n: usize) -> (u64, usize) {
    let (mut acc, mut cycles) = (0u64, 0);
    for i in 0..n {
        if b >> i & 1 == 1 {
            let (s, c) = serial_add(acc, a << i, 2 * n);
            acc = s & ((1 << (2 * n)) - 1);
            cycles += c;
        }
    }
    (acc, cycles)
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "bit-serial 179 + 94 (8 bits): {:?}", serial_add(179, 94, 8));
    outln!(o, "shift-and-add 13 × 11 (4 bits): {:?}", shift_add_multiply(13, 11, 4));
    outln!(o, "fully serial 13 × 11 (4 bits): {:?}", serial_multiply(13, 11, 4));
    outln!(o, "  n | serial add cycles | shift-add cycles | serial multiply cycles (worst case)");
    for n in [4usize, 8, 16] {
        let all_ones = (1u64 << n) - 1;
        outln!(o, "{:>3} | {:>17} | {:>16} | {:>6}", n, serial_add(all_ones, 1, n).1, shift_add_multiply(all_ones, all_ones, n).1, serial_multiply(all_ones, all_ones, n).1);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn serial_add_matches_integer_addition() {
        let mut r = Rng::new(174);
        for n in 1..=20 {
            let m = (1u64 << n) - 1;
            let (x, y) = (r.next_u64() & m, r.next_u64() & m);
            assert_eq!(serial_add(x, y, n), (x + y, n + 1));
        }
    }
    #[test]
    fn multipliers_match() {
        let mut r = Rng::new(175);
        for n in 1..=10 {
            let m = (1u64 << n) - 1;
            let (x, y) = (r.next_u64() & m, r.next_u64() & m);
            assert_eq!(shift_add_multiply(x, y, n), (x * y, n));
            assert_eq!(serial_multiply(x, y, n).0, x * y);
        }
    }
    #[test]
    fn register_holds_the_carry_between_cycles() {
        let mut a = BitSerialAdder::new();
        assert!(!a.tick(true, true)); // 1+1 = 0 carry 1
        assert!(a.carry());
        assert!(a.tick(false, false)); // carry flows into the next bit
        assert!(!a.carry());
    }
}
