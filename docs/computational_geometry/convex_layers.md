# Convex Layers

## What it is
The **convex layers** (or "onion peeling") of a set `Q` of points: layer 1 is the set of vertices of the convex hull of `Q`; layer 2 is the set of vertices of the hull of the points remaining after removing layer 1; and so on until no points are left. The *depth* of a point is the index of its layer. Problem 33-1 asks for an algorithm computing the layers in `O(n²)` time and to relate the number of layers to the hull of the full set.

## How it works
Repeat: compute the hull with [Graham's scan](#/convex_hull_graham) (or [Jarvis's march](#/jarvis_march)), record its vertices as the next layer, delete them, continue while points remain. Each round removes at least one point (at least three unless the remaining points are collinear), and costs `O(n lg n)`; so the straightforward method takes `O(n² lg n)`. The problem's `O(n²)` bound comes from presorting once and maintaining the sorted order across rounds; Chazelle (1985) achieves `O(n lg n)`.

Layers nest: every point of a deeper layer lies inside or on the hull of the previous layer, so areas strictly decrease.

```mermaid
flowchart LR
  Q["point set"] --> H1["hull → layer 0"] --> R1["remove it"] --> H2["hull of the rest → layer 1"] --> R2["…"] --> E["empty"]
```

## Complexity
`O(n² lg n)` here (`O(n²)` with presorting, `O(n lg n)` with Chazelle's algorithm); the number of layers is `O(n)`, and about `n^(2/3)` for uniformly random points in a square.

## When to use it
Statistics (depth contours and the "bagplot" generalise the median to the plane — *convex hull peeling depth*), outlier detection, and ranking of points by centrality.

## In this implementation
Points that lie on a hull edge but are not vertices stay for the next layer. Tests verify that layers partition the input, nest (every point is on the inner side of every previous edge, areas decrease), that layer 0 equals the set of extreme points found by an independent area test, and a known nested-squares example.
