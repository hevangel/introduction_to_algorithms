//! @title Searching a Sorted Compact List
//! @chapter Problem 10-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary A sorted linked list in array form can be searched in expected O(√n) probes by random sampling, beating the Θ(n) scan.
//! @order 47
use crate::common::Rng;
use crate::outln;

const NIL: usize = usize::MAX;

/// A sorted linked list stored in arrays (`key`, `next`) with arbitrary physical order — the "compact list" of Problem 10-3.
#[derive(Debug, Clone)]
pub struct CompactList { key: Vec<i64>, next: Vec<usize>, head: usize }

impl CompactList {
    /// Store the sorted keys at randomly permuted array positions.
    pub fn from_sorted(keys: &[i64], rng: &mut Rng) -> Self {
        let n = keys.len();
        let mut pos: Vec<usize> = (0..n).collect();
        rng.shuffle(&mut pos);
        let (mut key, mut next) = (vec![0; n], vec![NIL; n]);
        for (i, &k) in keys.iter().enumerate() {
            key[pos[i]] = k;
            if i + 1 < n { next[pos[i]] = pos[i + 1]; }
        }
        CompactList { key, next, head: if n > 0 { pos[0] } else { NIL } }
    }
    pub fn len(&self) -> usize { self.key.len() }
    pub fn is_empty(&self) -> bool { self.key.is_empty() }
    /// COMPACT-LIST-SEARCH: walk from the head. Returns (found, probes). Θ(n).
    pub fn search_linear(&self, k: i64) -> (bool, usize) {
        let (mut i, mut probes) = (self.head, 0);
        while i != NIL && self.key[i] < k { i = self.next[i]; probes += 1; }
        (i != NIL && self.key[i] == k, probes + 1)
    }
    /// COMPACT-LIST-SEARCH′: sample `t` random array slots, jump to the largest sampled key that is still < k, then walk.
    /// Expected O(t + n/t) probes — O(√n) for t = √n.
    pub fn search_randomized(&self, k: i64, t: usize, rng: &mut Rng) -> (bool, usize) {
        if self.is_empty() { return (false, 0); }
        let (mut i, mut probes) = (self.head, 0usize);
        for _ in 0..t {
            let j = rng.range(0, self.len() - 1);
            probes += 1;
            if self.key[i] < self.key[j] && self.key[j] <= k { i = j; if self.key[i] == k { return (true, probes); } }
        }
        while i != NIL && self.key[i] < k { i = self.next[i]; probes += 1; }
        (i != NIL && self.key[i] == k, probes + 1)
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(3);
    for n in [100usize, 10_000, 1_000_000] {
        let keys: Vec<i64> = (0..n as i64).map(|x| 2 * x).collect();
        let l = CompactList::from_sorted(&keys, &mut r);
        let t = (n as f64).sqrt() as usize;
        let trials = 200;
        let (mut lin, mut rnd) = (0, 0);
        for _ in 0..trials { let k = 2 * r.range(0, n - 1) as i64; lin += l.search_linear(k).1; rnd += l.search_randomized(k, t, &mut r).1; }
        outln!(o, "n = {:>7}: linear scan {:>8.0} probes on average; randomized with t = √n = {:>4}: {:>6.0} probes", n, lin as f64 / trials as f64, t, rnd as f64 / trials as f64);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_searches_are_correct() {
        let mut r = Rng::new(333);
        for n in [0usize, 1, 2, 17, 300] {
            let keys: Vec<i64> = (0..n as i64).map(|x| 3 * x).collect();
            let l = CompactList::from_sorted(&keys, &mut r);
            for q in -2..(3 * n as i64 + 3) {
                let want = keys.binary_search(&q).is_ok();
                assert_eq!(l.search_linear(q).0, want);
                assert_eq!(l.search_randomized(q, 8, &mut r).0, want, "n={n} q={q}");
            }
        }
    }
    #[test]
    fn randomized_search_probes_about_sqrt_n() {
        let mut r = Rng::new(334);
        let n = 40_000;
        let keys: Vec<i64> = (0..n as i64).collect();
        let l = CompactList::from_sorted(&keys, &mut r);
        let mean = |f: &dyn Fn(i64, &mut Rng) -> usize, r: &mut Rng| (0..300).map(|_| f(r.range(0, n - 1) as i64, r)).sum::<usize>() as f64 / 300.0;
        let lin = mean(&|q, _| l.search_linear(q).1, &mut r);
        let rnd = mean(&|q, r| l.search_randomized(q, 200, r).1, &mut r);
        assert!(lin > 15_000.0, "{lin}");
        assert!(rnd < 4.0 * 200.0 + 100.0, "{rnd}");
    }
}
