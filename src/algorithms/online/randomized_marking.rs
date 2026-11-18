//! @title Randomized Marking for Online Paging
//! @chapter 27.3.2 (4e)
//! @editions 4
//! @summary Epoch-based MARKING (k-competitive) and RANDOMIZED-MARKING (2 H_k = O(lg k) expected competitive ratio against an oblivious adversary).
//! @order 35
use crate::common::Rng;
use crate::outln;

/// Deterministic MARKING(k): every cached page starts an epoch unmarked. Accessing a page marks it;
/// when all k slots are marked on a miss, clear all marks before evicting an unmarked slot.
pub fn marking_misses(k: usize, requests: &[usize]) -> usize {
    assert!(k > 0);
    let mut cache: Vec<(usize, bool)> = Vec::new();
    let mut misses = 0;
    for &p in requests {
        if let Some(slot) = cache.iter_mut().find(|(q, _)| *q == p) {
            slot.1 = true;
            continue;
        }
        misses += 1;
        if cache.len() == k {
            if cache.iter().all(|&(_, marked)| marked) {
                for (_, marked) in &mut cache {
                    *marked = false;
                }
            }
            let pos = cache.iter().position(|&(_, marked)| !marked).unwrap();
            cache.swap_remove(pos);
        }
        cache.push((p, true));
    }
    misses
}

/// RANDOMIZED-MARKING(k): identical epoch marking, choosing the unmarked eviction victim uniformly at random.
pub fn randomized_marking_misses(k: usize, requests: &[usize], seed: u64) -> usize {
    assert!(k > 0);
    let mut rng = Rng::new(seed);
    let mut cache: Vec<(usize, bool)> = Vec::new();
    let mut misses = 0;
    for &p in requests {
        if let Some(slot) = cache.iter_mut().find(|(q, _)| *q == p) {
            slot.1 = true;
            continue;
        }
        misses += 1;
        if cache.len() == k {
            if cache.iter().all(|&(_, marked)| marked) {
                for (_, marked) in &mut cache {
                    *marked = false;
                }
            }
            let unmarked: Vec<usize> = cache
                .iter()
                .enumerate()
                .filter_map(|(i, &(_, marked))| if !marked { Some(i) } else { None })
                .collect();
            let victim = unmarked[rng.range(0, unmarked.len() - 1)];
            cache.swap_remove(victim);
        }
        cache.push((p, true));
    }
    misses
}

pub fn harmonic(k: usize) -> f64 {
    (1..=k).map(|i| 1.0 / i as f64).sum()
}

pub fn demo() -> String {
    let mut o = String::new();
    let k = 6;
    // Oblivious adversary cycling through k+1 pages: LRU hits k * OPT misses, while randomized marking beats k.
    let seq: Vec<usize> = (0..600).map(|i| i % (k + 1)).collect();
    let opt = super::online_caching::belady_misses(k, &seq);
    let lru = super::online_caching::lru_misses(k, &seq);
    let det_m = marking_misses(k, &seq);
    let avg_rand: f64 = (1..=20)
        .map(|s| randomized_marking_misses(k, &seq, s) as f64)
        .sum::<f64>()
        / 20.0;
    outln!(o, "cyclic (k+1={}) workload with k={k}: OPT={opt}, LRU={lru}, MARKING={det_m}, avg RAND-MARK={avg_rand:.1}", k + 1);
    outln!(o, "competitive bound 2*H_{k} = {:.2} vs observed rand/OPT = {:.2}", 2.0 * harmonic(k), avg_rand / opt as f64);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::online::online_caching::{belady_misses, lru_misses};

    #[test]
    fn randomized_marking_beats_deterministic_k_adversary_within_2_hk() {
        for k in [4usize, 6, 8] {
            let seq: Vec<usize> = (0..800).map(|i| i % (k + 1)).collect();
            let opt = belady_misses(k, &seq);
            let lru = lru_misses(k, &seq);
            assert_eq!(lru, seq.len());
            let det = marking_misses(k, &seq);
            assert!(det <= k * opt + k);
            let avg_rand: f64 = (1..=30)
                .map(|s| randomized_marking_misses(k, &seq, 100 + s) as f64)
                .sum::<f64>()
                / 30.0;
            assert!(avg_rand <= 2.0 * harmonic(k) * opt as f64 + k as f64);
            assert!(avg_rand < 0.7 * lru as f64);
        }
    }
}
