# Checkerboard Optimization & Profit-Deadline Scheduling

## What it is
Two classic dynamic programming exercises that share pseudo-polynomial or table-driven optimal substructure, juxtaposed with NP-completeness when durations grow exponentially:
1. **Moving on a Checkerboard (Problem 15-6)**: Find a bottom-to-top path moving up-left, straight up, or up-right on an $m \times n$ grid that maximizes collected square rewards.
2. **Maximum-Profit Scheduling with Deadlines and Durations (Problem 15-7 / 34-4)**: Given jobs $(t_i, d_i, p_i)$ with integer durations, deadlines, and profits, choose a subset schedulable by its deadlines maximizing total profit in $O(nD)$ time (strongly contrasting with unit-duration [matroid greedy](#/matroid_greedy), which runs in $O(n \lg n)$).

## How it works
* **Checkerboard (`checkerboard_max_profit`)**: Row $r$ transitions depend only on adjacent columns $\{c-1, c, c+1\}$ in row $r-1$, propagating $\mathrm{dp}[r][c] = \mathrm{profit}[r][c] + \max_{|c'-c|\le 1} \mathrm{dp}[r-1][c']$ upward and backtracking the winning column path.
* **Profit-deadline scheduling (`max_profit_scheduling`)**: By an exchange argument, any feasible subset remains feasible when sorted by earliest deadline first (EDD). Processing jobs in ascending $d_i$ updates $\mathrm{dp}[i+1][t] = \max(\mathrm{dp}[i][t], \mathrm{dp}[i][t - t_i] + p_i)$ for $t_i \le t \le d_i$.

```mermaid
flowchart LR
  Jobs["Sort jobs by deadline d_i"] --> DP["For each job: update dp[t] for t_i <= t <= d_i"]
  DP --> Opt["Take max_{t <= D} dp[n][t] & backtrack"]
```

## Complexity
* **Checkerboard DP**: $\Theta(mn)$ time and space (or $\Theta(n)$ space if only value is needed).
* **Profit-deadline scheduling**: $O(nD)$ time and space where $D = \max_i d_i$ — pseudo-polynomial (weakly NP-complete when durations are encoded in binary).

## When to use it
Use checkerboard DP for bounded-slope path planning (related to [seam carving](#/seam_carving)), and $O(nD)$ scheduling when job durations and deadlines are small integers compared to exponential subset search.

## In this implementation
`checkerboard_max_profit` and `max_profit_scheduling` reconstruct explicit optimal schedules/paths and validate exact agreement against exhaustive DFS and $2^n$ subset enumeration on randomized instances.
