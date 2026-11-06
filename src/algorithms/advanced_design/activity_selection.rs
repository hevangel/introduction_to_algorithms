//! @title Activity Selection
//! @chapter 16.1 (1e–3e) / 15.1 (4e)
//! @editions 1,2,3,4
//! @summary Greedy choice (earliest finish time) picks a maximum set of compatible activities; compared with the DP.
//! @order 150
use crate::outln;

pub type Activity = (i64, i64); // (start, finish), half-open [s, f)

/// GREEDY-ACTIVITY-SELECTOR: sort by finish time and take every activity that starts after the last chosen one finishes.
pub fn greedy_activity_selector(acts: &[Activity]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..acts.len()).collect();
    idx.sort_by_key(|&i| (acts[i].1, acts[i].0));
    let (mut chosen, mut last_finish) = (vec![], i64::MIN);
    for i in idx {
        if acts[i].0 >= last_finish {
            chosen.push(i);
            last_finish = acts[i].1;
        }
    }
    chosen
}

/// RECURSIVE-ACTIVITY-SELECTOR over activities already sorted by finish time.
pub fn recursive_activity_selector(sorted: &[Activity], k: usize, out: &mut Vec<usize>) {
    // k is the number of activities already considered (sentinel a0 has finish -∞)
    let mut m = k;
    let fk = if k == 0 { i64::MIN } else { sorted[k - 1].1 };
    while m < sorted.len() && sorted[m].0 < fk { m += 1; }
    if m < sorted.len() {
        out.push(m);
        recursive_activity_selector(sorted, m + 1, out);
    }
}

/// Θ(n³) dynamic programming of Section 16.1: c[i][j] = size of a maximum compatible subset of S_ij.
pub fn dp_activity_selection(acts: &[Activity]) -> usize {
    let mut a = acts.to_vec();
    a.sort_by_key(|x| x.1);
    let n = a.len();
    // index 0 = sentinel a_0 (finish −∞), index n+1 = sentinel a_{n+1} (start +∞)
    let fin = |i: usize| if i == 0 { i64::MIN } else { a[i - 1].1 };
    let sta = |i: usize| if i == n + 1 { i64::MAX } else { a[i - 1].0 };
    let mut c = vec![vec![0usize; n + 2]; n + 2];
    for len in 2..=n + 1 {
        for i in 0..=n + 1 - len {
            let j = i + len;
            for k in i + 1..j {
                if sta(k) >= fin(i) && fin(k) <= sta(j) {
                    c[i][j] = c[i][j].max(c[i][k] + c[k][j] + 1);
                }
            }
        }
    }
    c[0][n + 1]
}

/// Exercise 16.1-4: partition activities among the fewest lecture halls (interval-graph colouring).
pub fn min_lecture_halls(acts: &[Activity]) -> usize {
    let mut events: Vec<(i64, i32)> = acts.iter().flat_map(|&(s, f)| [(s, 1), (f, -1)]).collect();
    events.sort(); // at equal times, finishes (-1) sort before starts (+1)
    let (mut cur, mut best) = (0, 0);
    for (_, d) in events { cur += d; best = best.max(cur); }
    best as usize
}

pub fn demo() -> String {
    let mut o = String::new();
    let acts: Vec<Activity> = vec![(1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9), (6, 10), (8, 11), (8, 12), (2, 14), (12, 16)];
    let g = greedy_activity_selector(&acts);
    outln!(o, "activities (s,f): {:?}", acts);
    outln!(o, "greedy picks (by finish order): {:?}", g.iter().map(|&i| acts[i]).collect::<Vec<_>>());
    outln!(o, "greedy size = {}, dynamic programming size = {}", g.len(), dp_activity_selection(&acts));
    let mut sorted = acts.clone();
    sorted.sort_by_key(|a| a.1);
    let mut rec = vec![];
    recursive_activity_selector(&sorted, 0, &mut rec);
    outln!(o, "recursive selector picks {:?}", rec.iter().map(|&i| sorted[i]).collect::<Vec<_>>());
    outln!(o, "lecture halls needed for all activities: {}", min_lecture_halls(&acts));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_example() {
        let acts = vec![(1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9), (6, 10), (8, 11), (8, 12), (2, 14), (12, 16)];
        assert_eq!(greedy_activity_selector(&acts).len(), 4);
        assert_eq!(dp_activity_selection(&acts), 4);
    }
    #[test]
    fn greedy_equals_dp_and_is_compatible() {
        let mut r = Rng::new(112);
        for _ in 0..80 {
            let n = r.range(0, 12);
            let acts: Vec<Activity> = (0..n).map(|_| { let s = r.range(0, 20) as i64; (s, s + r.range(1, 8) as i64) }).collect();
            let g = greedy_activity_selector(&acts);
            let mut chosen: Vec<_> = g.iter().map(|&i| acts[i]).collect();
            chosen.sort();
            assert!(chosen.windows(2).all(|w| w[0].1 <= w[1].0));
            assert_eq!(g.len(), dp_activity_selection(&acts));
            let mut sorted = acts.clone();
            sorted.sort_by_key(|a| a.1);
            let mut rec = vec![];
            recursive_activity_selector(&sorted, 0, &mut rec);
            assert_eq!(rec.len(), g.len());
        }
    }
    #[test]
    fn lecture_halls_equal_maximum_overlap() {
        assert_eq!(min_lecture_halls(&[(0, 3), (3, 6), (6, 9)]), 1);
        assert_eq!(min_lecture_halls(&[(0, 5), (1, 6), (2, 7)]), 3);
        assert_eq!(min_lecture_halls(&[]), 0);
    }
}
