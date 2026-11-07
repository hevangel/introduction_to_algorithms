# Minimizing Average Completion Time

## What it is
A single machine must run `n` tasks; task `aᵢ` needs processing time `pᵢ`. If task `i` finishes at time `cᵢ`, the **average completion time** is `(1/n) Σ cᵢ`. Problem 16-2 shows that a simple greedy rule is optimal, then extends it with **release times** and **preemption**.

## How it works
* **(a) SPT — shortest processing time first.** Schedule tasks in increasing order of `pᵢ`. *Exchange argument:* if a longer task precedes a shorter adjacent one, swapping them decreases the sum — task `j` placed first contributes its `pⱼ` to the completion time of every later task, so the smaller `pⱼ` should come earlier. Sorting dominates: `O(n lg n)`.
* **(c) SRPT — shortest remaining processing time first.** With release times `rᵢ` and *preemption* allowed, at every moment run the available task with the least remaining time; a new arrival may interrupt the current task. A heap keyed by remaining time implements it in `O(n lg n)`; it minimises the average completion time (the greedy-choice proof is again an exchange argument). Without preemption the problem with release times is NP-hard, so SRPT also lower-bounds every non-preemptive schedule.

```mermaid
flowchart LR
  A["arrivals"] --> H["heap by remaining time"] --> R["run the shortest until it finishes or a task arrives"] --> H
```

## Complexity
`O(n lg n)` for both.

## When to use it
Job scheduling in operating systems (SJF/SRTF policies minimise mean response time), request scheduling in servers (SRPT is optimal for mean latency), and the basis of the scheduling matroid ideas of [matroid greedy](#/matroid_greedy).

## In this implementation
`spt_order`, `average_completion`, `srpt` and an exhaustive non-preemptive optimum. Tests check SPT against all permutations, and that SRPT never exceeds the best non-preemptive schedule with release times (and equals SPT when all release times are 0).
