# Dijkstra with a Bucket Queue (Dial's Algorithm)

## What it is
[Dijkstra's algorithm](#/dijkstra) needs a min-priority queue. If all edge weights are **integers in `0…W`**, the queue can be replaced by an array of **buckets** indexed by distance (Dial, 1969). Exercises 24.3-7 and 24.3-8 ask for exactly this speed-up (and the `O(V lg lg V)`-type refinement with a [van Emde Boas tree](#/van_emde_boas_tree)).

## How it works
Every finite distance is an integer at most `W(V−1)`. Keep buckets `B[0], B[1], …` where `B[k]` holds the vertices whose current estimate is `k`. Scan `k = 0, 1, 2, …`: any vertex found in `B[k]` has the smallest remaining estimate, so it is *finalised*; relaxing an edge `(u, v)` appends `v` to the bucket of its new estimate. Estimates inserted while scanning bucket `k` land in buckets `≥ k`, so a single forward scan suffices (for weight-0 edges the same bucket is revisited — handled by the inner `pop` loop). Stale entries (a vertex whose estimate later dropped) are skipped.

* **EXTRACT-MIN** is amortised `O(1)` — moving a pointer along the buckets.
* There are `E` insertions and the pointer moves at most `W·V` buckets in total.

```mermaid
flowchart LR
  B0["bucket 0: s"] --> B1["bucket 1"] --> B2["bucket 2: …"] --> Bk["bucket k"]
  B0 -- "relax w=2" --> B2
```

## Complexity
`O(E + W·V)`: linear in the graph size when `W` is constant. (A *circular* array of `W + 1` buckets reduces space to `O(W + V)`.)

## When to use it
Road networks with small integer travel times, grid pathfinding, and as the inner routine of [Gabow's scaling algorithm](#/gabow_scaling), where all keys are small.

## In this implementation
Buckets grow on demand; zero-weight edges are supported. Tests compare with the binary-heap Dijkstra on 200 random graphs including unreachable vertices.
