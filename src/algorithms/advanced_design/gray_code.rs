//! @title Binary Reflected Gray Code
//! @chapter Problem 16-1 (4e)
//! @editions 4
//! @summary Generate all n-bit words changing one bit per step via recursive reflection, G(i) = i ^ (i >> 1), and amortized O(1) bit-position iteration.
//! @order 245
use crate::outln;

/// Recursive reflected construction of the n-bit binary Gray code.
pub fn gray_recursive(n: usize) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let half = gray_recursive(n - 1);
    let msb = 1u64 << (n - 1);
    let mut out = Vec::with_capacity(half.len() * 2);
    out.extend_from_slice(&half);
    for &code in half.iter().rev() {
        out.push(msb | code);
    }
    out
}

/// Closed-form i -> Gray(i) = i ^ (i >> 1).
pub fn gray_encode(i: u64) -> u64 {
    i ^ (i >> 1)
}

/// Inverse Gray(i) -> i via prefix XOR across bit positions.
pub fn gray_decode(mut g: u64) -> u64 {
    let mut shift = 1;
    while shift < 64 {
        g ^= g >> shift;
        shift <<= 1;
    }
    g
}

/// Bit position toggled at step `i -> i + 1` for i = 0..2^n - 2, plus total transitions work.
/// By Problem 16-1, total work across all 2^n words is sum_{k>=1} floor(2^n / 2^k) = Θ(2^n), i.e., O(1) amortized.
pub fn gray_transition_positions(n: usize) -> (Vec<usize>, usize) {
    if n == 0 {
        return (vec![], 0);
    }
    let total_steps = (1usize << n) - 1;
    let mut flips = Vec::with_capacity(total_steps);
    let mut trailing_checks = 0usize;
    for i in 0..total_steps {
        // Simulating bit-by-bit scan from bit 0 upward until first 0 bit in (i): cost = trailing_ones(i) + 1.
        let pos = (i + 1).trailing_zeros() as usize;
        trailing_checks += pos + 1;
        flips.push(pos);
    }
    (flips, trailing_checks)
}

pub fn demo() -> String {
    let mut o = String::new();
    let g4 = gray_recursive(4);
    let bin: Vec<String> = g4.iter().map(|x| format!("{:04b}", x)).collect();
    outln!(o, "4-bit reflected Gray code ({} words): {}", g4.len(), bin.join(" "));
    let (flips, work) = gray_transition_positions(4);
    outln!(o, "toggled bit indices: {:?}", flips);
    outln!(o, "total bit inspections across {} steps = {} ({:.2} per step)", flips.len(), work, work as f64 / (1 << 4) as f64);
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_matches_closed_form_and_hamming_distance_is_1() {
        for n in 0..=10 {
            let rec = gray_recursive(n);
            assert_eq!(rec.len(), 1usize << n);
            for (i, &val) in rec.iter().enumerate() {
                assert_eq!(val, gray_encode(i as u64));
                assert_eq!(gray_decode(val), i as u64);
            }
            for w in rec.windows(2) {
                assert_eq!((w[0] ^ w[1]).count_ones(), 1);
            }
            if n > 0 {
                assert_eq!((rec[0] ^ rec[rec.len() - 1]).count_ones(), 1); // cyclic
            }
        }
    }

    #[test]
    fn amortized_cost_is_bounded_by_2_per_step() {
        for n in 1..=12 {
            let (flips, work) = gray_transition_positions(n);
            let mut cur = 0u64;
            for (step, &bit) in flips.iter().enumerate() {
                cur ^= 1u64 << bit;
                assert_eq!(cur, gray_encode((step + 1) as u64));
            }
            assert!(work <= 2 * (1usize << n));
        }
    }
}
