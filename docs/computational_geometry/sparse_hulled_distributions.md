# Sparse-Hulled Distributions and Linear-Time Hulls

## What it is
The convex hull of `n` points needs `Ω(n lg n)` time in the worst case, but for *typical* inputs the hull has very few vertices. A distribution is **sparse-hulled** if the expected number of hull vertices of `n` independent points is `o(n)`. Problem 33-5 shows that points drawn uniformly from the unit **disk** have expected hull size `Θ(n^(1/3))`, while for a **square** (or any convex polygon) it is only `Θ(lg n)` — and asks for an algorithm whose *expected* running time is `O(n)` on such inputs.

## How it works
Throw away points that cannot be hull vertices before sorting anything (the **Akl–Toussaint heuristic**):
1. In one pass find the extreme points in a few fixed directions (here `x`, `y`, `x+y`, `x−y`, both signs), at most 8 points forming a convex octagon `P` that lies inside the hull.
2. In a second pass discard every point strictly inside `P` — one cross-product test per edge of `P`, `O(1)` per point.
3. Run [Graham's scan](#/convex_hull_graham) on the survivors.

For a uniform disk the octagon covers about 90 % of the area, for a square nearly 100 %, so almost every point is thrown away in step 2 and the `O(m lg m)` scan sees only `m ≪ n` survivors: the expected total is `O(n)`. (The original problem uses a bucketing method over the polar angle with the same effect.)

```mermaid
flowchart LR
  Q["n points"] -->|"8 extreme points"| O["octagon"] -->|"drop points strictly inside"| S["m ≪ n survivors"] -->|"Graham scan"| H["hull"]
```

## Complexity
`O(n)` expected for sparse-hulled distributions (plus `O(m lg m)` for the survivors); worst case `O(n lg n)` — points on a circle survive the filter.

## When to use it
Large point clouds (GIS, LIDAR, collision detection) where a quick `O(n)` rejection pass saves most of the sorting work; the same filtering idea accelerates [Jarvis's march](#/jarvis_march) and quickhull.

## In this implementation
Integer coordinates with exact predicates. Tests check that the filtered hull equals plain Graham scan on 200 random sets (including duplicates and collinear points) and that, for 20 000 uniform points, fewer than 20 % (disk) and 5 % (square) survive the filter, with hull sizes consistent with `n^(1/3)` and `lg n` scaling.
