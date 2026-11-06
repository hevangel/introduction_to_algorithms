# Parallel Randomized Quicksort

## What it is
[Randomized quicksort](#/randomized_quicksort) is naturally parallel: after partitioning around a pivot the two sub-arrays are independent, so the two recursive calls can be `spawn`ed. The chapter on multithreaded algorithms uses this to illustrate how the **span** of an algorithm can be dominated by a serial component.

## How it works
```
P-QUICKSORT(A, p, r):
  if p < r:
      q = RANDOMIZED-PARTITION(A, p, r)
      spawn P-QUICKSORT(A, p, q-1)
      P-QUICKSORT(A, q+1, r)
      sync
```
There is no race: the sub-arrays are disjoint (here enforced by Rust's `split_at_mut`).
* **Work:** `Θ(n lg n)` expected, the same as the serial algorithm.
* **Span:** the recursion tree has expected depth `Θ(lg n)`, but each level's partition is serial, so the first partition alone costs `Θ(n)`: `T∞(n) = T∞(n/2) + Θ(n) = Θ(n)` in the typical case. Parallelism is therefore only `Θ(lg n)`.
* **Better:** a parallel partition built from a [parallel prefix](#/parallel_prefix) (to compute where each element goes) has span `Θ(lg n)`, giving total span `Θ(lg² n)` and parallelism `Θ(n/lg n)`. Compare [parallel merge sort](#/p_merge_sort), whose merge is parallelised.

```mermaid
flowchart TB
  A["A[0..n)"] -->|partition (serial)| P["pivot"]
  P -->|spawn| L["sort left"]
  P --> R["sort right"]
  L & R -->|sync| D["done"]
```

## Complexity
Work `Θ(n lg n)`; span `Θ(n)` with a serial partition, `Θ(lg² n)` with a parallel one.

## When to use it
Shared-memory sorting where memory bandwidth is not the bottleneck; the pattern of "parallel recursion + serial cutoff" appears in every fork-join runtime.

## In this implementation
Sub-problems below a cutoff are sorted serially to bound the thread count; the function returns the recursion depth. Tests sort random (duplicate-heavy), sorted and reversed inputs and check the depth stays logarithmic for 50 000 random keys.
