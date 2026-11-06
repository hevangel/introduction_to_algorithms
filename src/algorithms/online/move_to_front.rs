//! @title Self-Organizing Lists: Move-to-Front
//! @chapter 27.2 (4e); Problem 17-5 (2e–3e)
//! @editions 2,3,4
//! @summary Access cost = position; moving each accessed item to the front is 4-competitive against any strategy.
//! @order 20
use crate::outln;

/// A list of distinct keys; accessing the key at position i (1-based) costs i. `move_to_front` is the MTF heuristic.
#[derive(Debug, Clone)]
pub struct MtfList { items: Vec<usize> }

impl MtfList {
    pub fn new(n: usize) -> Self { MtfList { items: (0..n).collect() } }
    pub fn order(&self) -> &[usize] { &self.items }
    /// Access `key`: pay its (1-based) position, then move it to the front.
    pub fn access(&mut self, key: usize) -> usize {
        let pos = self.items.iter().position(|&x| x == key).expect("key not in list");
        let k = self.items.remove(pos);
        self.items.insert(0, k);
        pos + 1
    }
}

/// Total access cost of move-to-front on a request sequence.
pub fn mtf_cost(n: usize, requests: &[usize]) -> usize {
    let mut l = MtfList::new(n);
    requests.iter().map(|&r| l.access(r)).sum()
}

/// Cost of a list that never changes (initial order 0..n).
pub fn static_cost(n: usize, requests: &[usize], order: &[usize]) -> usize {
    let _ = n;
    requests.iter().map(|&r| order.iter().position(|&x| x == r).unwrap() + 1).sum()
}

/// The best fixed order (sorted by decreasing access frequency) and its cost.
pub fn best_static_order(n: usize, requests: &[usize]) -> (Vec<usize>, usize) {
    let mut freq = vec![0usize; n];
    for &r in requests { freq[r] += 1; }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&k| std::cmp::Reverse(freq[k]));
    let c = static_cost(n, requests, &order);
    (order, c)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(6);
    let n = 30;
    // skewed (Zipf-like) request stream: key i is requested with weight 1/(i+1)
    let weights: Vec<f64> = (0..n).map(|i| 1.0 / (i as f64 + 1.0)).collect();
    let total: f64 = weights.iter().sum();
    let stream = |r: &mut crate::common::Rng| -> usize { let mut t = r.f64() * total; for (i, w) in weights.iter().enumerate() { if t < *w { return i; } t -= w; } n - 1 };
    let reqs: Vec<usize> = (0..5000).map(|_| stream(&mut r)).collect();
    let shuffled: Vec<usize> = { let mut o: Vec<usize> = (0..n).collect(); r.shuffle(&mut o); o };
    let (best, best_cost) = best_static_order(n, &reqs);
    outln!(o, "5000 Zipf requests over {n} keys");
    outln!(o, "move-to-front total cost        = {}", mtf_cost(n, &reqs));
    outln!(o, "fixed order, initial (0..n)     = {}", static_cost(n, &reqs, &(0..n).collect::<Vec<_>>()));
    outln!(o, "fixed random order              = {}", static_cost(n, &reqs, &shuffled));
    outln!(o, "best fixed order (offline)      = {} (order starts {:?})", best_cost, &best[..5]);
    outln!(o, "theorem: MTF ≤ 4 × cost of ANY algorithm (additive n² for the starting arrangement)");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn access_moves_to_front_and_costs_position() {
        let mut l = MtfList::new(5);
        assert_eq!(l.access(3), 4);
        assert_eq!(l.order(), &[3, 0, 1, 2, 4]);
        assert_eq!(l.access(3), 1);
        assert_eq!(l.access(4), 5);
        assert_eq!(l.order(), &[4, 3, 0, 1, 2]);
    }
    #[test]
    fn four_competitive_against_the_best_static_order() {
        let mut r = Rng::new(270);
        for _ in 0..100 {
            let n = r.range(2, 15);
            let len = r.range(1, 400);
            // skewed random requests
            let reqs: Vec<usize> = (0..len).map(|_| { let u = r.f64(); ((u * u) * n as f64) as usize % n }).collect();
            let (_, opt_static) = best_static_order(n, &reqs);
            // the optimal algorithm may first rearrange the list (≤ n² swaps) and then keep it fixed
            assert!(mtf_cost(n, &reqs) <= 4 * (opt_static + n * n), "n={n} len={len}");
        }
    }
    #[test]
    fn beats_a_bad_fixed_order_on_skewed_input() {
        let n = 20;
        let reqs: Vec<usize> = (0..2000).map(|i| if i % 5 == 0 { 19 } else { 18 }).collect(); // hot keys are last in the initial order
        assert!(mtf_cost(n, &reqs) * 4 < static_cost(n, &reqs, &(0..n).collect::<Vec<_>>()));
    }
    #[test]
    fn adversarial_sequence_makes_mtf_cost_n_per_request() {
        // always request the key at the back: MTF pays n each time, the same as any static list would for that sequence
        let n = 8;
        let mut l = MtfList::new(n);
        let mut total = 0;
        for _ in 0..50 { let last = *l.order().last().unwrap(); total += l.access(last); }
        assert_eq!(total, 50 * n);
    }
}
