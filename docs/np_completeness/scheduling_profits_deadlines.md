# Scheduling with Profits and Deadlines

## What it is
One machine, `n` tasks. Task `i` needs `t_i` units of time, earns profit `p_i` if it is **completed by its deadline** `d_i`, and nothing otherwise. Choose which tasks to run — and their order — to maximise total profit. Problem 34-4 shows the decision problem is **NP-complete** (it contains [subset sum](#/subset_sum_np): set `p_i = t_i` and a common deadline) and asks for an algorithm polynomial in `n` and the *numeric* sizes — a **pseudo-polynomial** algorithm.

## How it works
For a fixed set of tasks the best order is **earliest deadline first**: if a later-deadline task ran before an earlier-deadline one, swapping them cannot hurt. So sort by deadline and decide, task by task, whether to include it. Let `dp[k][τ]` be the maximum profit using a subset of the first `k` tasks (in deadline order) whose total processing time is exactly `τ`. Including task `k` makes it finish at `τ`, which must satisfy `τ ≤ d_k`:

`dp[k][τ] = max( dp[k−1][τ], dp[k−1][τ − t_k] + p_k  if τ ≤ d_k )`.

The answer is `max_τ dp[n][τ]`; back-pointers recover the chosen tasks. The table has `n · D` entries (`D` = largest deadline). When all `t_i = 1` the problem is the **matroid** problem of [Section 16.5](#/matroid_greedy) and greedy suffices — the DP agrees with it.

```mermaid
flowchart LR
  T["tasks sorted by deadline"] --> D["dp over total time τ ≤ d_k"] --> A["best profit, EDF order of the chosen set"]
```

## Complexity
`O(n · D)` time and space — pseudo-polynomial: polynomial when the deadlines are written in unary, exponential in the bit-length of binary-encoded deadlines.

## When to use it
Job scheduling with hard deadlines and rewards (batch processing windows, ad-slot allocation), and as a template for knapsack-like DPs with a time horizon.

## In this implementation
Tests compare with exhaustive search over all task subsets on 300 random instances (and verify the returned set is feasible with the claimed profit), and check that unit-time instances give the same profit as the matroid algorithm in [`matroid_greedy`](#/matroid_greedy).
