//! @title Online Caching: LRU, FIFO and Belady's Optimum
//! @chapter 27.3 (4e); Problem 16-5 (3e)
//! @editions 3,4
//! @summary Cache replacement policies compared by cache misses; LRU and FIFO are k-competitive, furthest-in-future is optimal offline.
//! @order 30
use crate::outln;
use std::collections::VecDeque;

/// Least-recently-used replacement: evict the page unused for the longest time. Returns the number of misses.
pub fn lru_misses(k: usize, requests: &[usize]) -> usize {
    let mut cache: VecDeque<usize> = VecDeque::new(); // front = most recently used
    let mut misses = 0;
    for &p in requests {
        if let Some(i) = cache.iter().position(|&x| x == p) { cache.remove(i); }
        else { misses += 1; if cache.len() == k { cache.pop_back(); } }
        cache.push_front(p);
    }
    misses
}

/// First-in-first-out: evict the page that has been in the cache longest.
pub fn fifo_misses(k: usize, requests: &[usize]) -> usize {
    let mut cache: VecDeque<usize> = VecDeque::new();
    let mut misses = 0;
    for &p in requests {
        if !cache.contains(&p) {
            misses += 1;
            if cache.len() == k { cache.pop_front(); }
            cache.push_back(p);
        }
    }
    misses
}

/// Flush-when-full: when the cache is full and a miss occurs, empty it entirely.
pub fn fwf_misses(k: usize, requests: &[usize]) -> usize {
    let mut cache: Vec<usize> = vec![];
    let mut misses = 0;
    for &p in requests {
        if !cache.contains(&p) {
            misses += 1;
            if cache.len() == k { cache.clear(); }
            cache.push(p);
        }
    }
    misses
}

/// Belady's offline-optimal policy (Problem 16-5): on a miss with a full cache, evict the page whose next request is furthest in the future.
pub fn belady_misses(k: usize, requests: &[usize]) -> usize {
    let mut cache: Vec<usize> = vec![];
    let mut misses = 0;
    for (t, &p) in requests.iter().enumerate() {
        if cache.contains(&p) { continue; }
        misses += 1;
        if cache.len() == k {
            let next_use = |q: usize| requests[t + 1..].iter().position(|&x| x == q).unwrap_or(usize::MAX);
            let victim = (0..cache.len()).max_by_key(|&i| next_use(cache[i])).unwrap();
            cache.swap_remove(victim);
        }
        cache.push(p);
    }
    misses
}

pub fn demo() -> String {
    let mut o = String::new();
    // adversarial: cycle through k+1 pages — LRU and FIFO miss every time
    let k = 4;
    let cyc: Vec<usize> = (0..200).map(|i| i % (k + 1)).collect();
    outln!(o, "cycling through {} pages with a cache of {k}: LRU {} misses, FIFO {}, FWF {}, optimal {}", k + 1, lru_misses(k, &cyc), fifo_misses(k, &cyc), fwf_misses(k, &cyc), belady_misses(k, &cyc));
    outln!(o, "→ ratio LRU/OPT = {:.2} ≈ k = {k}: the k-competitive bound is tight", lru_misses(k, &cyc) as f64 / belady_misses(k, &cyc) as f64);
    let mut r = crate::common::Rng::new(9);
    let loc: Vec<usize> = (0..5000).map(|_| { let u = r.f64(); ((u * u * u) * 40.0) as usize }).collect();
    for k in [4usize, 8, 16] {
        outln!(o, "locality-heavy workload, k={:>2}: LRU {:>4}  FIFO {:>4}  FWF {:>4}  OPT {:>4}", k, lru_misses(k, &loc), fifo_misses(k, &loc), fwf_misses(k, &loc), belady_misses(k, &loc));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn optimal_by_search(k: usize, req: &[usize]) -> usize {
        fn go(k: usize, req: &[usize], t: usize, cache: &mut Vec<usize>) -> usize {
            if t == req.len() { return 0; }
            if cache.contains(&req[t]) { return go(k, req, t + 1, cache); }
            if cache.len() < k { cache.push(req[t]); let r = 1 + go(k, req, t + 1, cache); cache.pop(); return r; }
            let mut best = usize::MAX;
            for i in 0..cache.len() { let old = cache[i]; cache[i] = req[t]; best = best.min(1 + go(k, req, t + 1, cache)); cache[i] = old; }
            best
        }
        go(k, req, 0, &mut vec![])
    }
    #[test]
    fn belady_is_optimal() {
        let mut r = Rng::new(271);
        for _ in 0..200 {
            let (k, len) = (r.range(1, 3), r.range(0, 11));
            let req: Vec<usize> = (0..len).map(|_| r.range(0, 4)).collect();
            assert_eq!(belady_misses(k, &req), optimal_by_search(k, &req), "k={k} {req:?}");
        }
    }
    #[test]
    fn online_policies_are_k_competitive() {
        let mut r = Rng::new(272);
        for _ in 0..100 {
            let (k, len) = (r.range(1, 6), r.range(1, 300));
            let req: Vec<usize> = (0..len).map(|_| r.range(0, 9)).collect();
            let opt = belady_misses(k, &req);
            assert!(opt <= lru_misses(k, &req) && opt <= fifo_misses(k, &req) && opt <= fwf_misses(k, &req));
            // misses ≤ k·OPT (the additive constant is at most k from the cold start)
            assert!(lru_misses(k, &req) <= k * opt, "lru");
            assert!(fifo_misses(k, &req) <= k * opt, "fifo");
            assert!(fwf_misses(k, &req) <= k * opt, "fwf");
        }
    }
    #[test]
    fn adversarial_cycle_attains_ratio_k() {
        let k = 5;
        let req: Vec<usize> = (0..600).map(|i| i % (k + 1)).collect();
        assert_eq!(lru_misses(k, &req), 600);
        let opt = belady_misses(k, &req);
        assert!(opt <= 600 / k + k + 1, "{opt}");
        assert!(lru_misses(k, &req) as f64 / opt as f64 > k as f64 - 1.0);
    }
}
