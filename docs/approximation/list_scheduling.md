# Scheduling on Parallel Machines (List Scheduling, LPT)

## What it is
`n` jobs with processing times `p₁,…,pₙ` must run on `m` identical machines, each job entirely on one machine. The **makespan** — the finishing time of the busiest machine — should be as small as possible. The problem (`P||C_max`) is NP-hard even for `m = 2` (it contains [PARTITION](#/subset_sum_np)). Problem 35-5 asks for approximation algorithms and their analysis.

## How it works
* **List scheduling (Graham, 1966).** Process the jobs in the given order, always giving the next job to the machine with the **least current load**. Let `j` be a job finishing last, with start time `s`. Every machine was busy until time `s`, hence `s ≤ (Σp − p_j)/m`, and
  `makespan = s + p_j ≤ Σp/m + (1 − 1/m) p_j ≤ OPT + (1 − 1/m) OPT = (2 − 1/m)·OPT`,
  using the two lower bounds `OPT ≥ Σp/m` and `OPT ≥ max p`. The bound is tight (the demo's `m(m−1)` unit jobs followed by one job of size `m`).
* **LPT — longest processing time first.** Sort in decreasing order first. The worst job is then smaller than `OPT/2` unless there are only `m` jobs, which improves the ratio to **`4/3 − 1/(3m)`**.
* A **PTAS** exists (Hochbaum–Shmoys).

```mermaid
flowchart LR
  J["jobs (sorted for LPT)"] --> M["least loaded machine"] --> A["assign, load += p"] --> J
```

## Complexity
`O(n lg m)` with a heap of machine loads (`O(nm)` here); LPT adds `O(n lg n)`.

## When to use it
Load balancing in clusters and thread pools, assembly lines, batch job queues — also the textbook example of an *online* algorithm (list scheduling needs no look-ahead) with a competitive ratio of `2 − 1/m`.

## In this implementation
Tests check both Graham bounds in exact integer form against a pruned exhaustive optimum on 300 random instances, validate assignments, and verify that the bad instance achieves exactly `2m − 1` versus the optimum `m` for `m = 2…5`.
