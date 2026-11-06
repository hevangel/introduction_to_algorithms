# Borůvka's Algorithm

## What it is
Borůvka's 1926 algorithm — the oldest MST algorithm, designed to electrify Moravia — works in **rounds**: in each round every connected component of the current forest selects the **lightest edge leaving it**, and all selected edges are added at once. It appears in the problems of Chapter 23 (23-2, as the first phase of an `O(E lg lg V)` MST algorithm) and is the engine behind many parallel and external-memory MST algorithms.

## How it works
1. Start with every vertex its own component.
2. **Round:** for each component find the minimum-weight edge to a different component (one pass over the edges).
3. Add all those edges (each is a **safe edge** by the cut property of [GENERIC-MST](#/generic_mst)); merge components with [union-find](#/disjoint_set).
4. Repeat until one component remains.

Because each component merges with at least one other, the number of components at least **halves** per round: at most `⌈lg V⌉` rounds, each costing `O(E)`: **`O(E lg V)`**. With tied weights break ties consistently (here by edge index), otherwise the chosen edges could form a cycle.

```mermaid
flowchart LR
  R0["n components"] -->|"each picks cheapest outgoing edge"| R1["≤ n/2 components"] -->|"…"| R2["≤ n/4"] --> Rk["1 component = MST"]
```

## Complexity
`O(E lg V)` time; after `lg lg V` Borůvka rounds, contracting components leaves a graph with `V/lg V` vertices on which Prim with a Fibonacci heap finishes in `O(E)` — total `O(E lg lg V)`.

## When to use it
Parallel/distributed MST (each component acts independently), GPU graph processing, and as the contraction step in randomised linear-time MST algorithms (Karger–Klein–Tarjan).

## In this implementation
Tie-breaking by `(weight, edge index)`; tests compare the weight and edge count with [Kruskal](#/mst_kruskal) on 150 random graphs with *many tied weights* (some disconnected) and assert at most `⌈lg n⌉ + 1` rounds.
