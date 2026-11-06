# Prim's Minimum Spanning Tree

## What it is
Prim's algorithm (Jarník 1930, Prim 1957, Dijkstra 1959) is the other classic MST method. Whereas [Kruskal](#/mst_kruskal) grows a forest, Prim grows **a single tree** from an arbitrary root `r`, always adding the lightest edge that leaves the tree — the *safe edge* for the cut `(tree, rest)`.

## How it works
Maintain for every vertex outside the tree its `key[v]` = weight of the lightest edge connecting it to the tree and `π[v]` = the other endpoint, in a **min-priority queue**:
1. Initialise `key[r] = 0`, all other keys `∞`.
2. Extract the vertex `u` with minimum key and add it to the tree.
3. For each neighbour `v` still in the queue, if `w(u,v) < key[v]` update `π[v] = u` and `DECREASE-KEY(v, w(u,v))`.

```mermaid
flowchart LR
  T["tree so far"] -- "lightest crossing edge" --> v["new vertex"]
  v --> T2["tree grows by one vertex"]
```
The loop invariant is that the edges `{(v, π[v])}` for vertices not in the queue form a subset of some MST.

## Complexity
| Priority queue | Time |
|---|---|
| Array | Θ(V²) |
| Binary heap | O(E lg V) |
| [Fibonacci heap](#/fibonacci_heap) | O(E + V lg V) |

## When to use it
Dense graphs (array version, no heap overhead) and graphs given as adjacency lists. Structurally almost identical to [Dijkstra](#/dijkstra) — only the key update rule differs (`w(u,v)` instead of `d[u] + w(u,v)`).

## In this implementation
`prim` uses a `BinaryHeap` with lazy deletion (stale entries skipped) instead of DECREASE-KEY; `prim_array` is the Θ(V²) version. Both are checked against Kruskal on random connected graphs.
