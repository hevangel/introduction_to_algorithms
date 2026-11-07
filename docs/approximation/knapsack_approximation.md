# Approximating the 0-1 Knapsack Problem

## What it is
Choose items, each with a weight and a value, to maximise total value without exceeding a weight capacity `W`. The [0-1 knapsack](#/knapsack_01) DP runs in `O(nW)` — **pseudo-polynomial**, since `W` may be exponential in the input length — and the decision problem is NP-complete. Problem 35-7 shows two ways to approximate it, ending with a **fully polynomial-time approximation scheme (FPTAS)**, just like [subset sum](#/approx_subset_sum).

## How it works
* **2-approximation.** Sort items by value density `v_i/w_i` and take the longest prefix that fits; this equals the optimal *fractional* knapsack ([Section 16.2](#/fractional_knapsack)) minus a fraction of the first item that did not fit. Let `V_prefix` be its value and `v_max` the best single item. Then `OPT ≤ OPT_fractional ≤ V_prefix + v_max`, so `max(V_prefix, v_max) ≥ OPT/2`.
* **FPTAS.** Flip the DP: let `m[i][v]` be the *minimum weight* needed to reach total value `v` using the first `i` items. This is polynomial if values are small. So **scale the values**: replace `v_i` by `⌊v_i / K⌋` with `K = ε·v_max/n`. The DP table has `n · (n²/ε)` entries (total scaled value at most `n²/ε`). Each item loses less than `K` in rounding, so the whole solution loses less than `nK = ε·v_max ≤ ε·OPT`: the answer is at least `(1 − ε)·OPT`, in time `O(n³/ε)` — polynomial in both `n` and `1/ε`.

```mermaid
flowchart LR
  I["items (wᵢ, vᵢ)"] -->|"scale vᵢ ← ⌊vᵢ/K⌋, K = ε·v_max/n"| S["small integer values"] -->|"DP: min weight for each value"| R["≥ (1 − ε)·OPT"]
```

## Complexity
2-approximation: `O(n lg n)`; FPTAS `O(n³/ε)`; exact DP `O(nW)`.

## When to use it
Budgeted selection (ads, capital budgeting, cargo loading) when `W` is huge or values are real numbers, and as the textbook example of a problem with an FPTAS (strongly NP-hard problems cannot have one unless P = NP).

## In this implementation
Tests compare both algorithms with the exact DP on 300 random instances: the greedy result is at least `OPT/2`, and the FPTAS result is between `(1 − ε)·OPT` and `OPT` for `ε ∈ {0.5, 0.2, 0.1}`; with `ε = 0.01` the three-item example is solved exactly.
