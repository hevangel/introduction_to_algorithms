# Approximate Metric TSP

## What it is
The **travelling-salesman problem** is NP-hard — and in general it cannot even be approximated within any constant factor unless P = NP. If the distances satisfy the **triangle inequality** (`c(u,w) ≤ c(u,v) + c(v,w)`, true for Euclidean distances) a simple algorithm gets within a factor of **2** of optimal (Section 35.2). Christofides' refinement achieves 3/2.

## How it works
1. Compute a **minimum spanning tree** `T` (use [Prim](#/mst_prim) from a root `r`).
2. List the vertices in a **preorder walk** of `T`.
3. Return that cyclic order as the tour `H`.

**Proof of the factor 2.** Deleting one edge from an optimal tour leaves a spanning tree, so `c(T) ≤ c(H*)`. A *full* walk `W` of the tree traverses every edge twice: `c(W) = 2c(T) ≤ 2c(H*)`. The preorder tour `H` is obtained from `W` by **shortcutting** repeated vertices; by the triangle inequality shortcuts never increase cost, so `c(H) ≤ c(W) ≤ 2c(H*)`.

```mermaid
flowchart LR
  P["points"] --> T["MST T:  c(T) ≤ OPT"]
  T --> W["full walk W:  c(W) = 2c(T)"]
  W --> H["shortcut repeats (preorder):  c(H) ≤ c(W)"]
  H --> R["c(H) ≤ 2·OPT"]
```

## Complexity
`Θ(V²)` with Prim on a dense (complete) graph.

## When to use it
Fast tours for routing, drilling, plotting where the solution is later refined by 2-opt / Lin–Kernighan local search. With *non*-metric costs, no polynomial approximation exists (Theorem 35.3) — which is the point of the reduction from [Hamiltonian cycle](#/np_reductions).

## In this implementation
Verified on random Euclidean instances against the exact Held–Karp optimum from [`hamiltonian_cycle`](#/hamiltonian_cycle): `OPT ≤ tour ≤ 2·OPT` and `MST ≤ OPT`.
