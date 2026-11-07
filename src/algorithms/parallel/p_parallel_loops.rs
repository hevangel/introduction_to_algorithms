//! @title Implementing Parallel Loops with Nested Parallelism
//! @chapter Problem 27-1 (3e)
//! @editions 3,4
//! @summary A `parallel for` over n iterations compiles to a balanced tree of spawns (span Θ(lg n)); a loop that merely spawns each iteration one after another has span Θ(n); a grain size trades span against scheduling overhead.
//! @order 14
use crate::outln;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Run `body(i)` for every i in lo..hi by recursive halving (`spawn` the left half, run the right half, `sync`) until a chunk has at most
/// `grain` iterations, which run serially. Returns the depth of the spawn tree.
pub fn parallel_for<F: Fn(usize) + Sync>(lo: usize, hi: usize, grain: usize, body: &F) -> usize {
    if hi <= lo { return 0; }
    if hi - lo <= grain.max(1) { for i in lo..hi { body(i); } return 0; }
    let mid = lo + (hi - lo) / 2;
    let (dl, dr) = thread::scope(|s| {
        let left = s.spawn(|| parallel_for(lo, mid, grain, body)); // spawn
        let dr = parallel_for(mid, hi, grain, body);
        (left.join().unwrap(), dr)                                 // sync
    });
    1 + dl.max(dr)
}

/// Span (in units of one iteration, ignoring the scheduling constant) of the naive loop `for i … spawn body(i)` followed by a sync: the parent
/// executes n spawns one after the other, so the span is Θ(n) no matter how many processors there are.
pub fn span_spawn_loop(n: usize, iteration_span: usize) -> usize { n + iteration_span }

/// Span of the nested-parallelism implementation with grain size g: ⌈lg(n/g)⌉ levels of spawning plus g serial iterations plus the span of one iteration.
pub fn span_nested(n: usize, grain: usize, iteration_span: usize) -> usize {
    let chunks = n.div_ceil(grain.max(1));
    (usize::BITS - (chunks.max(1) - 1).leading_zeros()) as usize + grain.max(1) * iteration_span
}

pub fn demo() -> String {
    let mut o = String::new();
    let count = AtomicUsize::new(0);
    let depth = parallel_for(0, 1000, 8, &|i| { count.fetch_add(i, Ordering::Relaxed); });
    outln!(o, "parallel_for over 0..1000 with grain 8: sum of indices = {} (expected {}), spawn-tree depth {depth}", count.load(Ordering::Relaxed), 999 * 1000 / 2);
    outln!(o, "   n  | spawn loop span | nested span (grain 1) | grain lg n");
    for n in [16usize, 256, 4096, 65536] {
        let lg = n.ilog2() as usize;
        outln!(o, "{n:>6} | {:>15} | {:>21} | {}", span_spawn_loop(n, 1), span_nested(n, 1, 1), span_nested(n, lg, 1));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_index_runs_exactly_once_for_any_grain() {
        for n in [0usize, 1, 2, 7, 100, 1000] {
            for grain in [1usize, 2, 5, 64, 5000] {
                let hits: Vec<AtomicUsize> = (0..n).map(|_| AtomicUsize::new(0)).collect();
                parallel_for(0, n, grain, &|i| { hits[i].fetch_add(1, Ordering::SeqCst); });
                assert!(hits.iter().all(|h| h.load(Ordering::SeqCst) == 1), "n = {n}, grain = {grain}");
            }
        }
    }
    #[test]
    fn spawn_tree_depth_is_logarithmic_and_spans_compare_as_in_the_problem() {
        for n in [16usize, 1000, 4096] {
            let depth = parallel_for(0, n, 1, &|_| {});
            assert!(depth <= (n as f64).log2().ceil() as usize, "n = {n}: depth {depth}");
        }
        assert_eq!(parallel_for(0, 1000, 5000, &|_| {}), 0, "a grain larger than the loop means no spawning at all");
        assert!(span_nested(4096, 1, 1) < span_spawn_loop(4096, 1) / 100);
        assert!(span_nested(1 << 16, 16, 1) <= span_nested(1 << 16, 1, 1) + 16);
    }
}
