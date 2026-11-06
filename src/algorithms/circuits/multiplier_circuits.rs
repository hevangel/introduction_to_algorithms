//! @title Array & Wallace-Tree Multipliers
//! @chapter 29.3 (1e)
//! @editions 1
//! @summary Multiply n-bit numbers: array multiplier with ripple adders (Θ(n) depth) vs carry-save Wallace tree (Θ(lg n) depth).
//! @order 50
use super::carry_lookahead_adder::build_cla;
use super::combinational_circuits::{from_bits, to_bits, Circuit};
use super::ripple_carry_adder::build_ripple;
use crate::outln;

/// Array multiplier: add the n shifted partial products one after another with ripple-carry adders.
/// Returns (circuit, 2n product wires). Inputs: a (n bits), then b (n bits).
pub fn array_multiplier(n: usize) -> (Circuit, Vec<usize>) {
    let mut c = Circuit::new();
    let (a, b) = (c.inputs(n), c.inputs(n));
    let zero = c.constant(false);
    let mut acc: Vec<usize> = vec![zero; 2 * n];
    for i in 0..n {
        // partial product row i = (a AND b_i) << i, added into the accumulator
        let row: Vec<usize> = (0..n).map(|j| c.and(a[j], b[i])).collect();
        let (sum, cout) = build_ripple(&mut c, &acc[i..i + n].to_vec(), &row, zero);
        for k in 0..n { acc[i + k] = sum[k]; }
        if i + n < 2 * n { acc[i + n] = cout; }
    }
    (c, acc)
}

/// Wallace-tree multiplier: all n² partial-product bits are reduced column-wise with 3:2 compressors
/// (carry-save adders) until two rows remain; a carry-lookahead adder finishes the job. Depth Θ(lg n).
pub fn wallace_multiplier(n: usize) -> (Circuit, Vec<usize>) {
    let mut c = Circuit::new();
    let (a, b) = (c.inputs(n), c.inputs(n));
    let zero = c.constant(false);
    let mut cols: Vec<Vec<usize>> = vec![vec![]; 2 * n + 1];
    for i in 0..n { for j in 0..n { let pp = c.and(a[j], b[i]); cols[i + j].push(pp); } }
    // Dadda-style schedule of target column heights 2, 3, 4, 6, 9, 13, 19, …: each stage uses only as many
    // 3:2 (full adder) and 2:2 (half adder) compressors as needed to reach the next target, so carries never ripple.
    let max_h = cols.iter().map(|x| x.len()).max().unwrap_or(0);
    let mut targets = vec![2usize];
    while *targets.last().unwrap() < max_h { let t = *targets.last().unwrap(); targets.push(t * 3 / 2); }
    targets.pop(); // the last target is >= max_h: nothing to do for it
    for &d in targets.iter().rev() {
        let mut next: Vec<Vec<usize>> = vec![vec![]; cols.len() + 1];
        let mut carries_in: Vec<usize> = vec![];
        for k in 0..cols.len() {
            let mut bits = cols[k].clone();
            let mut out: Vec<usize> = vec![];
            let mut carry_out: Vec<usize> = vec![];
            let mut h = bits.len() + carries_in.len();
            while h > d && bits.len() >= 2 {
                if h - d >= 2 && bits.len() >= 3 {
                    let (x, y, z) = (bits.pop().unwrap(), bits.pop().unwrap(), bits.pop().unwrap());
                    let (s, co) = c.full_adder(x, y, z);
                    out.push(s);
                    carry_out.push(co);
                    h -= 2;
                } else {
                    let (x, y) = (bits.pop().unwrap(), bits.pop().unwrap());
                    let (s, co) = c.half_adder(x, y);
                    out.push(s);
                    carry_out.push(co);
                    h -= 1;
                }
            }
            next[k].extend(bits);
            next[k].extend(out);
            next[k].extend(carries_in.drain(..));
            carries_in = carry_out;
        }
        next[cols.len()].extend(carries_in);
        next.truncate(cols.len());
        cols = next;
    }
    let row = |idx: usize| -> Vec<usize> { cols.iter().take(2 * n).map(|col| *col.get(idx).unwrap_or(&zero)).collect() };
    let (r0, r1) = (row(0), row(1));
    let mut sum = build_cla(&mut c, &r0, &r1);
    sum.truncate(2 * n);
    (c, sum)
}

pub fn multiply(build: fn(usize) -> (Circuit, Vec<usize>), n: usize, x: u64, y: u64) -> u64 {
    let (c, outs) = build(n);
    let mut inp = to_bits(x, n);
    inp.extend(to_bits(y, n));
    let v = c.eval(&inp);
    from_bits(&outs.iter().map(|&w| v[w]).collect::<Vec<_>>())
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "13 × 11: array = {}, Wallace = {}", multiply(array_multiplier, 4, 13, 11), multiply(wallace_multiplier, 4, 13, 11));
    outln!(o, "  n | array size/depth | wallace size/depth");
    for n in [4usize, 8, 16, 32] {
        let (a, ao) = array_multiplier(n);
        let (w, wo) = wallace_multiplier(n);
        outln!(o, "{:>3} | {:>6} / {:>4}    | {:>6} / {:>4}", n, a.size(), a.max_depth(&ao), w.size(), w.max_depth(&wo));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn both_multiply_correctly() {
        let mut r = Rng::new(173);
        for n in 1..=8 {
            for _ in 0..30 {
                let m = (1u64 << n) - 1;
                let (x, y) = (r.next_u64() & m, r.next_u64() & m);
                assert_eq!(multiply(array_multiplier, n, x, y), x * y, "array n={n} {x}*{y}");
                assert_eq!(multiply(wallace_multiplier, n, x, y), x * y, "wallace n={n} {x}*{y}");
            }
        }
        assert_eq!(multiply(wallace_multiplier, 8, 255, 255), 65025);
    }
    #[test]
    fn wallace_is_shallower_for_wide_operands() {
        let (a, ao) = array_multiplier(32);
        let (w, wo) = wallace_multiplier(32);
        assert!(w.max_depth(&wo) * 2 < a.max_depth(&ao), "{} vs {}", w.max_depth(&wo), a.max_depth(&ao));
    }
}
