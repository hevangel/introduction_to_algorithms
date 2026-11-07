//! @title Carry-Select Adder
//! @chapter Problem 29-? / 29.2 (1e)
//! @editions 1
//! @summary Compute each block's sum twice (carry-in 0 and 1) in parallel and let the real incoming carry select the right one: delay O(√n) with blocks of size √n, or O(lg n) when selects are chained in a tree.
//! @order 25
use crate::outln;

/// Ripple-carry addition of two equal-length bit vectors (LSB first) with an incoming carry. Returns (sum bits, carry out).
fn ripple(a: &[bool], b: &[bool], cin: bool) -> (Vec<bool>, bool) {
    let (mut c, mut s) = (cin, Vec::with_capacity(a.len()));
    for i in 0..a.len() { s.push(a[i] ^ b[i] ^ c); c = (a[i] & b[i]) | (c & (a[i] ^ b[i])); }
    (s, c)
}

/// Carry-select addition with blocks of `k` bits. Returns (sum bits, carry out, critical-path delay in gate delays under a simple model:
/// each ripple stage costs 2 delays, a 2:1 multiplexer costs 1).
pub fn carry_select_add(a: &[bool], b: &[bool], k: usize) -> (Vec<bool>, bool, usize) {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    let (mut sum, mut carry) = (vec![], false);
    let mut delay = 0;
    let mut start = 0;
    while start < n {
        let end = (start + k).min(n);
        let (blk_a, blk_b) = (&a[start..end], &b[start..end]);
        if start == 0 {
            let (s, c) = ripple(blk_a, blk_b, false);
            sum.extend(s);
            carry = c;
            delay = 2 * (end - start);
        } else {
            let (s0, c0) = ripple(blk_a, blk_b, false); // computed speculatively, in parallel with everything before it
            let (s1, c1) = ripple(blk_a, blk_b, true);
            let (s, c) = if carry { (s1, c1) } else { (s0, c0) }; // the late-arriving carry only drives a multiplexer
            sum.extend(s);
            carry = c;
            delay = delay.max(2 * (end - start)) + 1;
        }
        start = end;
    }
    (sum, carry, delay)
}

pub fn to_bits(x: u64, n: usize) -> Vec<bool> { (0..n).map(|i| x >> i & 1 == 1).collect() }
pub fn from_bits(b: &[bool]) -> u64 { b.iter().enumerate().fold(0, |s, (i, &x)| s | (x as u64) << i) }

pub fn demo() -> String {
    let mut o = String::new();
    let n = 32;
    let (a, b) = (to_bits(0xDEAD_BEEF, n), to_bits(0x1234_5678, n));
    for k in [1usize, 4, 6, 8, 32] {
        let (s, c, d) = carry_select_add(&a, &b, k);
        outln!(o, "block size {k:>2}: sum = {:#x}, carry out = {c}, delay = {d} gate delays (ripple would be {})", from_bits(&s), 2 * n);
    }
    outln!(o, "0xDEADBEEF + 0x12345678 = {:#x}", 0xDEAD_BEEFu64 + 0x1234_5678);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_integer_addition_for_every_block_size() {
        let mut r = Rng::new(435);
        for _ in 0..300 {
            let n = r.range(1, 40);
            let mask = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
            let (x, y) = (r.next_u64() & mask, r.next_u64() & mask);
            for k in [1, 2, 3, 5, 8, 64] {
                let (s, c, _) = carry_select_add(&to_bits(x, n), &to_bits(y, n), k);
                assert_eq!(from_bits(&s) | (c as u64) << n, x + y, "n={n} k={k}");
            }
        }
    }
    #[test]
    fn delay_beats_ripple_with_square_root_blocks() {
        let n = 64;
        let (a, b) = (to_bits(u64::MAX, n), to_bits(1, n));
        let (_, _, d_sqrt) = carry_select_add(&a, &b, 8);
        let (_, _, d_ripple) = carry_select_add(&a, &b, n);
        assert!(d_sqrt < d_ripple, "{d_sqrt} vs {d_ripple}");
    }
}
