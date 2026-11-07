//! @title Scheduling Unit-Time Tasks with Disjoint Sets
//! @chapter Problem 16-4 (3e)
//! @editions 3,4
//! @summary Consider tasks in decreasing penalty order and put each in the latest free slot not after its deadline, found with a union-find forest: O(n α(n)) after sorting, instead of the O(n²) slot scan.
//! @order 285
use crate::outln;

fn find(p: &mut Vec<usize>, x: usize) -> usize { let mut r = x; while p[r] != r { r = p[r]; } let mut c = x; while p[c] != r { let nx = p[c]; p[c] = r; c = nx; } r }

/// Returns (slot of each task — Some(1..=n) for early tasks, None for late ones, total penalty of late tasks).
/// Slot 0 is a sentinel meaning "no free slot"; find(d) returns the latest free slot ≤ d.
pub fn schedule(deadlines: &[usize], penalties: &[i64]) -> (Vec<Option<usize>>, i64) {
    let n = deadlines.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(penalties[i]));
    let mut parent: Vec<usize> = (0..=n).collect();
    let (mut slot, mut late_penalty) = (vec![None; n], 0);
    for i in order {
        let s = find(&mut parent, deadlines[i].min(n));
        if s == 0 { late_penalty += penalties[i]; } // no free slot at or before the deadline
        else { slot[i] = Some(s); parent[s] = s - 1; } // slot s is now taken: later queries skip to s − 1
    }
    (slot, late_penalty)
}

pub fn demo() -> String {
    let mut o = String::new();
    let d = [4, 2, 4, 3, 1, 4, 6];
    let w = [70, 60, 50, 40, 30, 20, 10];
    let (slot, penalty) = schedule(&d, &w);
    outln!(o, "deadlines {:?}, penalties {:?}", d, w);
    outln!(o, "slot of each task {:?}; total late penalty {penalty}", slot);
    let mut r = crate::common::Rng::new(16);
    let n = 200_000;
    let (dd, ww): (Vec<usize>, Vec<i64>) = ((0..n).map(|_| r.range(1, n)).collect(), (0..n).map(|_| r.range(1, 1000) as i64).collect());
    outln!(o, "n = {n} tasks scheduled in near-linear time; late penalty {}", schedule(&dd, &ww).1);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn same_penalty_as_the_matroid_greedy_algorithm_and_slots_are_valid() {
        let mut r = Rng::new(494);
        for _ in 0..300 {
            let n = r.range(1, 14);
            let (d, w): (Vec<usize>, Vec<i64>) = ((0..n).map(|_| r.range(1, n + 3)).collect(), (0..n).map(|_| r.range(1, 40) as i64).collect());
            let (slot, penalty) = schedule(&d, &w);
            let (_, want) = crate::algorithms::advanced_design::matroid_greedy::schedule_unit_tasks(&d, &w);
            assert_eq!(penalty, want);
            let mut used = vec![false; n + 1];
            for i in 0..n { if let Some(s) = slot[i] { assert!(s >= 1 && s <= d[i] && !used[s], "task {i} in slot {s}"); used[s] = true; } }
            assert_eq!(penalty, (0..n).filter(|&i| slot[i].is_none()).map(|i| w[i]).sum::<i64>());
        }
    }
    #[test]
    fn matches_exhaustive_search() {
        let mut r = Rng::new(495);
        for _ in 0..100 {
            let n = r.range(1, 8);
            let (d, w): (Vec<usize>, Vec<i64>) = ((0..n).map(|_| r.range(1, n)).collect(), (0..n).map(|_| r.range(1, 30) as i64).collect());
            // a set of tasks can all be early iff for every t at most t of them have deadline ≤ t
            let best_kept = (0u32..1 << n).filter(|m| (1..=n).all(|t| (0..n).filter(|&i| m >> i & 1 == 1 && d[i] <= t).count() <= t)).map(|m| (0..n).filter(|&i| m >> i & 1 == 1).map(|i| w[i]).sum::<i64>()).max().unwrap();
            assert_eq!(schedule(&d, &w).1, w.iter().sum::<i64>() - best_kept);
        }
    }
}
