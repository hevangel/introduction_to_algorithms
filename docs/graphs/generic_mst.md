# The Generic MST Method (Safe Edges)

## What it is
Chapter 23 builds minimum spanning trees on one general scheme. **GENERIC-MST** maintains a set of edges `A` that is always a subset of *some* minimum spanning tree and grows it one edge at a time with a **safe edge** — an edge whose addition keeps `A` a subset of some MST. Both [Kruskal's](#/mst_kruskal) and [Prim's](#/mst_prim) algorithms are instances; they differ only in how they choose the safe edge.

## How it works
```
GENERIC-MST(G, w):
  A = ∅
  while A does not form a spanning tree:
      find an edge (u, v) that is safe for A
      A = A ∪ {(u, v)}
```
**Recognising safe edges (Theorem 23.1 — the cut property).** A *cut* `(S, V − S)` **respects** `A` if no edge of `A` crosses it. If `(u, v)` is a **light edge** (minimum weight) crossing such a cut, it is safe for `A`. Proof by exchange: take an MST `T ⊇ A` not containing `(u, v)`; adding `(u, v)` to `T` closes a cycle that crosses the cut at another edge `(x, y)`; swapping `(x, y)` for `(u, v)` gives a tree no heavier, also containing `A ∪ {(u,v)}`.
Here the cut is `(S, V − S)` with `S` the component of `A` containing vertex 0 — exactly Prim's choice; Kruskal instead picks the globally lightest edge between *any* two components of `A` (Corollary 23.2).

```mermaid
flowchart LR
  A["forest A"] --> C["cut (S, V−S) respecting A"] --> L["light edge across the cut"] --> A2["A ∪ {light edge}"]
```

## Complexity
`O(VE)` for this naive version (one component search and one edge scan per added edge); the specialised algorithms reach `O(E lg V)` or better.

## When to use it
To *understand* and prove MST algorithms; the same safe-edge reasoning underlies Borůvka's algorithm ([next](#/boruvka)) and the matroid proof in [matroid greedy](#/matroid_greedy).

## In this implementation
The cut is the component of vertex 0; tests assert equality of the resulting weight with Kruskal's on 100 random connected graphs, and early termination on disconnected inputs.
