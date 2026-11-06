# Columnsort

## What it is
**Columnsort** (Tom Leighton, 1985) sorts `n = r·s` numbers arranged in a matrix of `r` rows and `s` columns, using only **column sorts** and fixed data movements — which makes it attractive for parallel machines where each processor sorts a column. Problem 8-7 of the 2nd–4th editions walks through its 8 steps and asks you to prove it correct.

## How it works
Requirements: `s` divides `r`, `r` is even, and `r ≥ 2(s − 1)²`.

1. Sort each column.
2. "Transpose": pick the entries up in column-major order, lay them down in row-major order (keeping the `r × s` shape).
3. Sort each column.
4. Perform the inverse of step 2.
5. Sort each column.
6. Shift the column-major sequence down by `r/2` (opening an extra column; pad with `−∞` at the top and `+∞` at the bottom).
7. Sort each column.
8. Undo the shift. The matrix is now sorted in column-major order.

**Why it works:** the proof uses the [0-1 principle](#/comparison_network): after steps 1–4 every element is within `(s−1)²` positions of its final place, and steps 5–8 clean up the "dirty" region, which fits in half a column when `r ≥ 2(s−1)²`.

```mermaid
flowchart LR
  S1["sort cols"] --> T2["transpose"] --> S3["sort cols"] --> T4["inverse transpose"] --> S5["sort cols"] --> SH6["shift r/2"] --> S7["sort cols"] --> U8["unshift"]
```

## Complexity
Four sorts of `s` columns of length `r`, so `O(n lg r)` work and four fully parallel column-sorting phases; fewer rounds than [bitonic sorting](#/bitonic_sorter) when `r` is large (the constraint `r ≥ 2(s−1)²` means `s = O(n^{1/3})`).

## When to use it
Sorting in parallel or out-of-core machines with limited per-processor memory (each column fits in one processor or one disk block), e.g. distributed sample-sort-like pipelines.

## In this implementation
Column-major `Vec<i64>` with explicit index maps for the two permutation steps; tests sort random and reversed inputs for `s = 2…7` at the minimal legal height, a taller matrix, and check that illegal shapes panic.
