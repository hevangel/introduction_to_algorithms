//! @title Minimizing Average Completion Time
//! @chapter Problem 16-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Shortest-processing-time-first minimises the average completion time on one machine; with release times and preemption, run the shortest remaining time (SRPT).
//! @order 275
use crate::outln;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Problem 16-2(a): schedule tasks without release times in order of increasing processing time (SPT). Returns the order.
pub fn spt_order(p: &[u64]) -> Vec<usize> { let mut o: Vec<usize> = (0..p.len()).collect(); o.sort_by_key(|&i| p[i]); o }

/// Average completion time of running the tasks in the given order, back to back from time 0.
pub fn average_completion(p: &[u64], order: &[usize]) -> f64 {
    let (mut t, mut sum) = (0u64, 0u64);
    for &i in order { t += p[i]; sum += t; }
    sum as f64 / order.len().max(1) as f64
}

/// Problem 16-2(c): with release times r[i] and preemption, always run the available task with the shortest remaining time (SRPT).
/// Returns the completion time of every task.
pub fn srpt(r: &[u64], p: &[u64]) -> Vec<u64> {
    let n = p.len();
    let mut jobs: Vec<usize> = (0..n).collect();
    jobs.sort_by_key(|&i| r[i]);
    let (mut next, mut t) = (0, 0u64);
    let mut heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new(); // (remaining, id)
    let mut done = vec![0u64; n];
    while next < n || !heap.is_empty() {
        if heap.is_empty() { t = t.max(r[jobs[next]]); }
        while next < n && r[jobs[next]] <= t { heap.push(Reverse((p[jobs[next]], jobs[next]))); next += 1; }
        let Reverse((rem, id)) = heap.pop().unwrap();
        let until = if next < n { r[jobs[next]] } else { u64::MAX };
        let run = rem.min(until.saturating_sub(t).max(0)).max(if until <= t { rem } else { 0 });
        let run = if until > t { rem.min(until - t) } else { run };
        t += run;
        if run == rem { done[id] = t; } else { heap.push(Reverse((rem - run, id))); }
    }
    done
}

/// Best non-preemptive schedule with release times by exhaustive search (for validation).
pub fn nonpreemptive_optimum(r: &[u64], p: &[u64]) -> f64 {
    fn go(r: &[u64], p: &[u64], used: &mut Vec<bool>, t: u64, sum: u64, best: &mut u64) {
        if used.iter().all(|&u| u) { *best = (*best).min(sum); return; }
        for i in 0..p.len() {
            if !used[i] { used[i] = true; let start = t.max(r[i]); go(r, p, used, start + p[i], sum + start + p[i], best); used[i] = false; }
        }
    }
    let mut best = u64::MAX;
    go(r, p, &mut vec![false; p.len()], 0, 0, &mut best);
    best as f64 / p.len().max(1) as f64
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = [8u64, 3, 10, 2, 5];
    let spt = spt_order(&p);
    outln!(o, "processing times {:?}: SPT order {:?}, average completion {:.2}", p, spt, average_completion(&p, &spt));
    outln!(o, "given order 0..5 instead: {:.2}", average_completion(&p, &[0, 1, 2, 3, 4]));
    let r = [0u64, 2, 4, 5, 6];
    let done = srpt(&r, &p);
    outln!(o, "with release times {:?}: SRPT completion times {:?}, average {:.2}", r, done, done.iter().sum::<u64>() as f64 / 5.0);
    outln!(o, "best non-preemptive schedule: {:.2}", nonpreemptive_optimum(&r, &p));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn permutations(n: usize) -> Vec<Vec<usize>> {
        if n == 0 { return vec![vec![]]; }
        let mut out = vec![];
        for p in permutations(n - 1) { for pos in 0..=p.len() { let mut q = p.clone(); q.insert(pos, n - 1); out.push(q); } }
        out
    }
    #[test]
    fn spt_is_optimal_without_release_times() {
        let mut r = Rng::new(345);
        for _ in 0..100 {
            let n = r.range(1, 7);
            let p: Vec<u64> = (0..n).map(|_| r.range(1, 20) as u64).collect();
            let best = permutations(n).iter().map(|o| average_completion(&p, o)).fold(f64::INFINITY, f64::min);
            assert!((average_completion(&p, &spt_order(&p)) - best).abs() < 1e-9);
            assert!(((srpt(&vec![0; n], &p).iter().sum::<u64>() as f64 / n as f64) - best).abs() < 1e-9);
        }
    }
    #[test]
    fn srpt_beats_every_nonpreemptive_schedule_with_release_times() {
        let mut r = Rng::new(346);
        for _ in 0..100 {
            let n = r.range(1, 6);
            let (rel, p): (Vec<u64>, Vec<u64>) = ((0..n).map(|_| r.range(0, 15) as u64).collect(), (0..n).map(|_| r.range(1, 10) as u64).collect());
            let done = srpt(&rel, &p);
            assert!(done.iter().zip(rel.iter().zip(&p)).all(|(&c, (&ri, &pi))| c >= ri + pi));
            assert!(done.iter().sum::<u64>() as f64 / n as f64 <= nonpreemptive_optimum(&rel, &p) + 1e-9);
        }
    }
}
