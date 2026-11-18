//! @title Checkerboard Optimization & Profit-Deadline Scheduling
//! @chapter Problems 15-6 & 15-7 (2e); Problem 34-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Bottom-up maximum-profit checkerboard traversal and pseudopolynomial O(nD) scheduling with profits, deadlines, and durations (contrasted with NP-completeness).
//! @order 145
use crate::outln;

/// Problem 15-6 (Moving on a checkerboard): `profit[r][c]` is the reward for stepping on square (r, c).
/// From (r, c) a checker may step to (r+1, c-1), (r+1, c), or (r+1, c+1).
/// Returns `(max_total_profit, chosen_col_per_row)` starting at row 0 and ending at row m-1.
pub fn checkerboard_max_profit(profit: &[Vec<i64>]) -> (i64, Vec<usize>) {
    let m = profit.len();
    if m == 0 {
        return (0, vec![]);
    }
    let n = profit[0].len();
    assert!(n > 0 && profit.iter().all(|row| row.len() == n));

    let mut dp = vec![vec![i64::MIN / 4; n]; m];
    let mut prev_col = vec![vec![0usize; n]; m];
    for c in 0..n {
        dp[0][c] = profit[0][c];
    }
    for r in 1..m {
        for c in 0..n {
            let lo = c.saturating_sub(1);
            let hi = (c + 1).min(n - 1);
            let mut best_p = i64::MIN / 4;
            let mut best_pc = lo;
            for pc in lo..=hi {
                if dp[r - 1][pc] > best_p {
                    best_p = dp[r - 1][pc];
                    best_pc = pc;
                }
            }
            dp[r][c] = best_p + profit[r][c];
            prev_col[r][c] = best_pc;
        }
    }
    let (mut cur_c, &best_total) = dp[m - 1]
        .iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v)
        .unwrap();
    let mut path = vec![0usize; m];
    for r in (0..m).rev() {
        path[r] = cur_c;
        if r > 0 {
            cur_c = prev_col[r][cur_c];
        }
    }
    (best_total, path)
}

/// Problem 15-7 / 34-4: Scheduling with durations, deadlines, and profits.
/// Each job is `(duration t_i, deadline d_i, profit p_i)`.
/// Pseudopolynomial O(n D) DP where D = max d_i; returns `(max_profit, selected_job_indices_in_order)`.
pub fn max_profit_scheduling(jobs: &[(usize, usize, i64)]) -> (i64, Vec<usize>) {
    let n = jobs.len();
    if n == 0 {
        return (0, vec![]);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&idx| (jobs[idx].1, jobs[idx].0, idx));

    let max_d = order.iter().map(|&idx| jobs[idx].1).max().unwrap_or(0);
    const NEG: i64 = i64::MIN / 4;
    let mut dp = vec![vec![NEG; max_d + 1]; n + 1];
    let mut took = vec![vec![false; max_d + 1]; n + 1];
    dp[0][0] = 0;

    for (step, &orig_idx) in order.iter().enumerate() {
        let (dur, dl, prof) = jobs[orig_idx];
        for t in 0..=max_d {
            dp[step + 1][t] = dp[step][t];
        }
        for t in dur..=dl {
            if dp[step][t - dur] > NEG / 2 {
                let cand = dp[step][t - dur] + prof;
                if cand > dp[step + 1][t] {
                    dp[step + 1][t] = cand;
                    took[step + 1][t] = true;
                }
            }
        }
    }

    let (mut best_t, &best_profit) = dp[n]
        .iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v)
        .unwrap();
    let mut chosen = Vec::new();
    for step in (0..n).rev() {
        if took[step + 1][best_t] {
            let orig_idx = order[step];
            chosen.push(orig_idx);
            best_t -= jobs[orig_idx].0;
        }
    }
    chosen.reverse();
    (best_profit, chosen)
}

pub fn demo() -> String {
    let mut o = String::new();
    let board = vec![
        vec![3, 1, 7, 2],
        vec![2, 9, 4, 8],
        vec![6, 1, 5, 3],
        vec![4, 8, 2, 7],
    ];
    let (b_prof, path) = checkerboard_max_profit(&board);
    outln!(o, "checkerboard max profit = {b_prof}, column path = {:?}", path);

    let jobs = vec![(3, 5, 40), (2, 4, 25), (4, 6, 50), (1, 2, 15)];
    let (s_prof, chosen) = max_profit_scheduling(&jobs);
    outln!(o, "jobs (dur, dl, profit) = {:?}", jobs);
    outln!(o, "max scheduling profit   = {s_prof}, execution order = {:?}", chosen);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn checkerboard_matches_exhaustive_dfs() {
        let mut r = Rng::new(1506);
        for _ in 0..50 {
            let m = r.range(1, 7);
            let n = r.range(1, 5);
            let board: Vec<Vec<i64>> = (0..m).map(|_| r.vec(n, -20, 40)).collect();
            let (got, path) = checkerboard_max_profit(&board);
            assert_eq!(path.len(), m);
            let mut sum = 0;
            for r_idx in 0..m {
                sum += board[r_idx][path[r_idx]];
                if r_idx > 0 {
                    assert!(path[r_idx].abs_diff(path[r_idx - 1]) <= 1);
                }
            }
            assert_eq!(sum, got);

            fn brute(b: &[Vec<i64>], r: usize, c: usize) -> i64 {
                if r + 1 == b.len() {
                    return b[r][c];
                }
                let lo = c.saturating_sub(1);
                let hi = (c + 1).min(b[0].len() - 1);
                b[r][c] + (lo..=hi).map(|nc| brute(b, r + 1, nc)).max().unwrap()
            }
            let expected = (0..n).map(|c| brute(&board, 0, c)).max().unwrap();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn scheduling_matches_exhaustive_subset_enumeration() {
        let mut r = Rng::new(1507);
        for _ in 0..60 {
            let n = r.range(1, 11);
            let jobs: Vec<(usize, usize, i64)> = (0..n)
                .map(|_| {
                    let dur = r.range(1, 6);
                    let dl = r.range(dur - 1, 14);
                    let prof = r.range(1, 50) as i64;
                    (dur, dl, prof)
                })
                .collect();
            let (got, chosen) = max_profit_scheduling(&jobs);

            // Verify schedule validity.
            let mut time = 0usize;
            let mut prof_sum = 0i64;
            for &idx in &chosen {
                let (dur, dl, prof) = jobs[idx];
                time += dur;
                assert!(time <= dl);
                prof_sum += prof;
            }
            assert_eq!(prof_sum, got);

            // Brute-force all 2^n subsets ordered by EDD.
            let mut brute = 0i64;
            for mask in 0..(1usize << n) {
                let mut sub: Vec<(usize, usize, i64)> = (0..n)
                    .filter(|&i| (mask >> i) & 1 == 1)
                    .map(|i| jobs[i])
                    .collect();
                sub.sort_by_key(|j| j.1);
                let mut t = 0;
                let mut ok = true;
                let mut p = 0;
                for (dur, dl, prof) in sub {
                    t += dur;
                    if t > dl {
                        ok = false;
                        break;
                    }
                    p += prof;
                }
                if ok {
                    brute = brute.max(p);
                }
            }
            assert_eq!(got, brute);
        }
    }
}
