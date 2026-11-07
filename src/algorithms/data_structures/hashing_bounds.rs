//! @title Hashing Bounds: Longest Chain & Probe Tails
//! @chapter Problems 11-1, 11-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Experiments behind the probabilistic bounds: the longest probe sequence is O(lg n) w.h.p. and the longest chain is Θ(lg n / lg lg n).
//! @order 78
use crate::common::Rng;
use crate::outln;

/// Problem 11-2: throw n keys into m slots uniformly (simple uniform hashing); return the length of the longest chain.
pub fn max_chain_length(n: usize, m: usize, rng: &mut Rng) -> usize {
    let mut c = vec![0usize; m];
    for _ in 0..n { c[rng.range(0, m - 1)] += 1; }
    *c.iter().max().unwrap()
}

/// The expected-maximum scale lg n / lg lg n.
pub fn chain_scale(n: usize) -> f64 { (n as f64).log2() / (n as f64).log2().log2() }

/// Problem 11-1: under uniform hashing, insert keys into a table of m slots until it holds αm keys, then count probes needed by
/// one more unsuccessful search. Returns the empirical P(probes > k) for k = 0..=kmax, which should be ≈ αᵏ.
pub fn probe_tail(m: usize, alpha: f64, kmax: usize, trials: usize, rng: &mut Rng) -> Vec<f64> {
    let mut exceed = vec![0usize; kmax + 1];
    let filled = (alpha * m as f64) as usize;
    for _ in 0..trials {
        let mut occupied = vec![false; m];
        let mut fill = 0;
        while fill < filled { let s = rng.range(0, m - 1); if !occupied[s] { occupied[s] = true; fill += 1; } } // uniform random occupied set of size αm
        // an unsuccessful search for a new key probes a random sequence of distinct slots until an empty one
        let mut probes = 0;
        loop { probes += 1; if !occupied[rng.range(0, m - 1)] { break; } }
        for (k, e) in exceed.iter_mut().enumerate() { if probes > k { *e += 1; } }
    }
    exceed.iter().map(|&e| e as f64 / trials as f64).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(7);
    outln!(o, "chaining with n = m keys: average longest chain vs lg n / lg lg n");
    for k in [8, 10, 12, 14, 16] {
        let n = 1usize << k;
        let avg = (0..40).map(|_| max_chain_length(n, n, &mut r) as f64).sum::<f64>() / 40.0;
        outln!(o, "  n = 2^{:<2}: longest chain {:>5.2}   lg n/lg lg n = {:.2}   ratio {:.2}", k, avg, chain_scale(n), avg / chain_scale(n));
    }
    let tail = probe_tail(512, 0.5, 6, 20_000, &mut r);
    outln!(o, "open addressing at α = ½: P(probes > k) = {:?}  vs α^k = {:?}", tail.iter().map(|x| (x * 1000.0).round() / 1000.0).collect::<Vec<_>>(), (0..=6).map(|k| 0.5f64.powi(k)).collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_chain_grows_like_lg_n_over_lg_lg_n() {
        let mut r = Rng::new(331);
        let avg = |n: usize, r: &mut Rng| (0..60).map(|_| max_chain_length(n, n, r) as f64).sum::<f64>() / 60.0;
        for n in [1usize << 8, 1 << 12, 1 << 15] {
            let a = avg(n, &mut r);
            let s = chain_scale(n);
            assert!(a >= s && a <= 3.5 * s, "n={n}: avg {a}, scale {s}");
        }
        assert!(avg(1 << 15, &mut r) > avg(1 << 8, &mut r));
    }
    #[test]
    fn probe_tail_decays_geometrically() {
        let mut r = Rng::new(332);
        let t = probe_tail(256, 0.5, 4, 20_000, &mut r);
        for (k, p) in t.iter().enumerate() { assert!((p - 0.5f64.powi(k as i32)).abs() < 0.02, "k={k} p={p}"); }
        let t9 = probe_tail(256, 0.9, 3, 8_000, &mut r);
        assert!((t9[3] - 0.9f64.powi(3)).abs() < 0.03, "{t9:?}");
    }
}
