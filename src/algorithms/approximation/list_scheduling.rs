//! @title Scheduling on Parallel Machines (List Scheduling, LPT)
//! @chapter Problem 35-5 (1e–4e)
//! @editions 1,2,3,4
//! @summary Assign jobs to m identical machines to minimise the makespan: Graham's list scheduling is a (2 − 1/m)-approximation, and longest-processing-time-first improves it to 4/3 − 1/(3m).
//! @order 55
use crate::outln;

/// GRAHAM'S LIST SCHEDULING: give each job (in the given order) to the currently least-loaded machine. Returns (makespan, machine of each job).
pub fn list_schedule(jobs: &[u32], m: usize) -> (u32, Vec<usize>) {
    let mut load = vec![0u32; m];
    let mut assign = vec![0; jobs.len()];
    for (j, &p) in jobs.iter().enumerate() {
        let i = (0..m).min_by_key(|&i| load[i]).unwrap();
        load[i] += p;
        assign[j] = i;
    }
    (*load.iter().max().unwrap_or(&0), assign)
}

/// LONGEST PROCESSING TIME FIRST: sort jobs in decreasing order, then list scheduling.
pub fn lpt(jobs: &[u32], m: usize) -> u32 {
    let mut v = jobs.to_vec();
    v.sort_by(|a, b| b.cmp(a));
    list_schedule(&v, m).0
}

/// Optimal makespan by exhaustive search with pruning (small inputs only).
pub fn optimal_makespan(jobs: &[u32], m: usize) -> u32 {
    fn go(jobs: &[u32], i: usize, load: &mut Vec<u32>, best: &mut u32) {
        if *load.iter().max().unwrap() >= *best { return; }
        if i == jobs.len() { *best = *load.iter().max().unwrap(); return; }
        let mut seen = vec![];
        for k in 0..load.len() {
            if seen.contains(&load[k]) { continue; } // machines with equal load are interchangeable
            seen.push(load[k]);
            load[k] += jobs[i];
            go(jobs, i + 1, load, best);
            load[k] -= jobs[i];
        }
    }
    let mut sorted = jobs.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    let mut best = sorted.iter().sum::<u32>() + 1;
    go(&sorted, 0, &mut vec![0; m], &mut best);
    best
}

pub fn demo() -> String {
    let mut o = String::new();
    let m = 3;
    // the classic bad input for list scheduling: m(m−1) unit jobs followed by one job of size m
    let bad: Vec<u32> = std::iter::repeat_n(1, m * (m - 1)).chain([m as u32]).collect();
    outln!(o, "jobs {:?} on {m} machines", bad);
    outln!(o, "list scheduling {}, LPT {}, optimal {} (ratio {:.3}; bound 2 − 1/m = {:.3})", list_schedule(&bad, m).0, lpt(&bad, m), optimal_makespan(&bad, m), list_schedule(&bad, m).0 as f64 / optimal_makespan(&bad, m) as f64, 2.0 - 1.0 / m as f64);
    let jobs = [7u32, 7, 6, 6, 5, 5, 4, 4, 4];
    outln!(o, "jobs {:?}: list {}, LPT {}, optimal {}", jobs, list_schedule(&jobs, 4).0, lpt(&jobs, 4), optimal_makespan(&jobs, 4));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn graham_bounds_against_the_optimum() {
        let mut r = Rng::new(415);
        for _ in 0..300 {
            let m = r.range(1, 4);
            let jobs: Vec<u32> = (0..r.range(1, 9)).map(|_| r.range(1, 20) as u32).collect();
            let opt = optimal_makespan(&jobs, m);
            let (ls, assign) = list_schedule(&jobs, m);
            let mut load = vec![0; m];
            for (j, &i) in assign.iter().enumerate() { load[i] += jobs[j]; }
            assert_eq!(*load.iter().max().unwrap(), ls);
            assert!(opt >= (jobs.iter().sum::<u32>()).div_ceil(m as u32) && opt >= *jobs.iter().max().unwrap());
            assert!(m as u32 * ls <= (2 * m as u32 - 1) * opt, "list scheduling ≤ (2 − 1/m) OPT");
            assert!(3 * m as u32 * lpt(&jobs, m) <= (4 * m as u32 - 1) * opt, "LPT ≤ (4/3 − 1/(3m)) OPT");
        }
    }
    #[test]
    fn worst_case_instance_is_tight() {
        for m in 2..=5usize {
            let bad: Vec<u32> = std::iter::repeat_n(1, m * (m - 1)).chain([m as u32]).collect();
            assert_eq!(list_schedule(&bad, m).0, 2 * m as u32 - 1);
            assert_eq!(optimal_makespan(&bad, m), m as u32);
        }
    }
}
