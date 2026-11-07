//! @title Scheduling with Profits and Deadlines
//! @chapter Problem 34-4 (3e)
//! @editions 3,4
//! @summary Tasks with processing times, profits and deadlines on one machine: NP-complete for arbitrary times, but a pseudo-polynomial DP over total processing time solves it in O(n · D).
//! @order 80
use crate::outln;

/// A task: processing time `t`, profit `p` earned only if it completes by deadline `d`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Task { pub t: usize, pub p: i64, pub d: usize }

/// Maximum total profit and the chosen tasks (indices, in the order they run). Tasks that are scheduled are run back to back in deadline order
/// (earliest deadline first is optimal for a fixed set); dp[τ] = best profit with total processing time exactly τ.
pub fn max_profit(tasks: &[Task]) -> (i64, Vec<usize>) {
    let mut order: Vec<usize> = (0..tasks.len()).collect();
    order.sort_by_key(|&i| tasks[i].d);
    let horizon = tasks.iter().map(|t| t.d).max().unwrap_or(0);
    let neg = i64::MIN / 4;
    let n = tasks.len();
    let mut dp = vec![vec![neg; horizon + 1]; n + 1];
    dp[0][0] = 0;
    for (k, &i) in order.iter().enumerate() {
        let Task { t, p, d } = tasks[i];
        for tau in 0..=horizon {
            dp[k + 1][tau] = dp[k][tau]; // skip the task
            if tau >= t && tau <= d && dp[k][tau - t] > neg { dp[k + 1][tau] = dp[k + 1][tau].max(dp[k][tau - t] + p); } // run it last, finishing at τ ≤ d
        }
    }
    let (mut tau, &best) = dp[n].iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
    let mut chosen = vec![];
    for k in (0..n).rev() {
        if dp[k + 1][tau] == dp[k][tau] { continue; }
        let i = order[k];
        chosen.push(i);
        tau -= tasks[i].t;
    }
    chosen.reverse();
    (best, chosen)
}

/// Is the set schedulable so that each task meets its deadline (run in deadline order)?
pub fn feasible(tasks: &[Task], set: &[usize]) -> bool {
    let mut s = set.to_vec();
    s.sort_by_key(|&i| tasks[i].d);
    let mut time = 0;
    s.iter().all(|&i| { time += tasks[i].t; time <= tasks[i].d })
}

pub fn demo() -> String {
    let mut o = String::new();
    let tasks = [Task { t: 3, p: 10, d: 4 }, Task { t: 2, p: 7, d: 3 }, Task { t: 4, p: 15, d: 9 }, Task { t: 1, p: 4, d: 2 }, Task { t: 5, p: 12, d: 10 }];
    let (p, chosen) = max_profit(&tasks);
    outln!(o, "tasks {:?}", tasks);
    outln!(o, "best profit {p} with tasks {:?} (feasible: {})", chosen, feasible(&tasks, &chosen));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_exhaustive_subset_search() {
        let mut r = Rng::new(477);
        for _ in 0..300 {
            let n = r.range(1, 9);
            let tasks: Vec<Task> = (0..n).map(|_| Task { t: r.range(1, 5), p: r.range(0, 20) as i64, d: r.range(1, 20) }).collect();
            let best = (0u32..1 << n).filter(|m| feasible(&tasks, &(0..n).filter(|&i| m >> i & 1 == 1).collect::<Vec<_>>())).map(|m| (0..n).filter(|&i| m >> i & 1 == 1).map(|i| tasks[i].p).sum::<i64>()).max().unwrap();
            let (p, chosen) = max_profit(&tasks);
            assert_eq!(p, best, "{tasks:?}");
            assert!(feasible(&tasks, &chosen));
            assert_eq!(chosen.iter().map(|&i| tasks[i].p).sum::<i64>(), p);
        }
    }
    #[test]
    fn unit_tasks_agree_with_the_matroid_greedy_algorithm() {
        let mut r = Rng::new(478);
        for _ in 0..100 {
            let n = r.range(1, 10);
            let (d, p): (Vec<usize>, Vec<i64>) = ((0..n).map(|_| r.range(1, 8)).collect(), (0..n).map(|_| r.range(1, 30) as i64).collect());
            let tasks: Vec<Task> = (0..n).map(|i| Task { t: 1, p: p[i], d: d[i] }).collect();
            let (_, penalty) = crate::algorithms::advanced_design::matroid_greedy::schedule_unit_tasks(&d, &p);
            assert_eq!(max_profit(&tasks).0, p.iter().sum::<i64>() - penalty);
        }
    }
}
