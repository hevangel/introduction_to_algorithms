# Augmentation Problems: Maximum Overlap & Josephus

## What it is
Two problems from Chapter 14 (augmenting data structures) that are solved by the same strategy as [order-statistic](#/order_statistic_tree) and [interval trees](#/interval_tree): add a little information to a balanced tree.

* **Problem 14-1 — point of maximum overlap.** Given a set of intervals, find a point covered by the largest number of them.
* **Problem 14-2 — Josephus permutation.** `n` people stand in a circle; starting at person 1, every `m`-th person still present is eliminated until nobody remains. Output the elimination order.

## How it works
**Maximum overlap.** Keep the endpoints in order (left endpoints before right endpoints at the same coordinate, because intervals are closed). Sweep left to right with a counter: `+1` at a left endpoint, `−1` after a right endpoint; the maximum counter value is the answer. With a [red-black tree](#/red_black_tree) augmented by "sum of ±1 in the subtree" and "maximum prefix sum in the subtree", the structure supports dynamic insertion/deletion of intervals in `O(lg n)` each.

**Josephus with an order-statistic tree.** Store `1..n` in an OS-tree. If the next victim is at (0-based) position `p` among `k` survivors, the following one is at `(p + m − 1) mod (k − 1)` after removal; each step is one `OS-SELECT` and one delete: `O(n lg n)` total. For *only the survivor* the recurrence `J(1) = 0`, `J(k) = (J(k−1) + m) mod k` solves it in `Θ(n)` (and `O(lg n)` for `m = 2`).

```mermaid
flowchart LR
  C["circle of survivors"] --> S["pos = (pos + m − 1) mod size"] --> Q["OS-SELECT(pos+1)"] --> D["delete it, record"] --> C
```

## Complexity
Maximum overlap `O(n lg n)` offline; Josephus `O(n lg n)` (all eliminations) or `Θ(n)` (survivor).

## When to use it
Scheduling conflicts (peak concurrent load of intervals), calendar capacity checks, circular elimination games, rotating queues.

## In this implementation
Both problems are verified against brute force: the sweep against a scan of all points, and the OS-tree Josephus against the recurrence for every `n < 40`, `m < 8`, plus the classic `n = 7, m = 3` order `3 6 2 7 5 1 4`.
