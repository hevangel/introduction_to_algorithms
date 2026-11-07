//! @title Parallel Fibonacci & Work-Span Analysis
//! @chapter 27.1 (3e) / 26.1 (4e)
//! @editions 3,4
//! @summary spawn/sync Fibonacci, with work T₁, span T∞ and parallelism T₁/T∞ computed from the computation DAG.
//! @order 10
use crate::outln;
use std::thread;

/// Serial Fibonacci (the baseline).
pub fn fib(n: u32) -> u64 { if n < 2 { n as u64 } else { fib(n - 1) + fib(n - 2) } }

/// P-FIB(n): `spawn` the first recursive call, compute the second in the parent, then `sync` (join).
/// Subtrees below `cutoff` run serially to keep thread counts sane.
pub fn p_fib(n: u32, cutoff: u32) -> u64 {
    if n < 2 { return n as u64; }
    if n <= cutoff { return fib(n); }
    thread::scope(|s| {
        let x = s.spawn(|| p_fib(n - 1, cutoff)); // spawn
        let y = p_fib(n - 2, cutoff);
        x.join().unwrap() + y                      // sync
    })
}

/// Work T₁(n): total strands = number of calls: T₁(n) = T₁(n−1) + T₁(n−2) + 1.
pub fn work(n: u32) -> u64 { if n < 2 { 1 } else { work(n - 1) + work(n - 2) + 1 } }
/// Span T∞(n): longest dependency chain: T∞(n) = max(T∞(n−1), T∞(n−2)) + 1 = n.
pub fn span(n: u32) -> u64 { if n < 2 { 1 } else { span(n - 1).max(span(n - 2)) + 1 } }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "P-FIB(30) = {} (serial {})", p_fib(30, 20), fib(30));
    outln!(o, "   n |      work T1 |  span T∞ | parallelism T1/T∞");
    for n in [10u32, 20, 30, 40] {
        outln!(o, "{:>4} | {:>12} | {:>8} | {:>12.1}", n, work(n), span(n), work(n) as f64 / span(n) as f64);
    }
    outln!(o, "speedup bound on P processors: T_P ≥ max(T1/P, T∞) — plenty of parallelism since T1/T∞ grows like φⁿ/n");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parallel_matches_serial() {
        for n in 0..28 { assert_eq!(p_fib(n, 12), fib(n)); }
        assert_eq!(p_fib(32, 20), 2178309);
    }
    #[test]
    fn work_and_span_recurrences() {
        for n in 2..25 { assert_eq!(span(n), n as u64); }
        // work is 2·F(n+1) − 1
        for n in 0..25 { assert_eq!(work(n), 2 * fib(n + 1) - 1); }
        assert!(work(30) / span(30) > 10_000);
    }
}
