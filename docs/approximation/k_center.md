# k-Center Clustering

## What it is
Given `n` points in a metric space and an integer `k`, choose `k` **centers** so that the largest distance from any point to its nearest center — the **radius** — is as small as possible. It is the "build `k` fire stations to minimise the worst response distance" problem. The decision version is NP-complete (it contains vertex cover/dominating set), yet a very simple greedy algorithm is a **2-approximation**, and by a reduction from dominating set no algorithm can guarantee a factor below 2 unless P = NP — a rare case where the approximation ratio is tight and fully classified (compare [the metric TSP](#/approx_tsp) and [vertex cover](#/approx_vertex_cover)).

## How it works
**Farthest-first traversal (Gonzalez, 1985).** Pick any point as the first center. Repeat `k−1` times: add the point whose distance to its nearest chosen center is largest.

**Why 2?** Let `r` be the radius of the greedy solution, attained by some point `q`. The `k` centers together with `q` are `k + 1` points that are pairwise at distance `≥ r` (each was chosen as the farthest, so it is at least `r` from all earlier centers). By pigeonhole two of them share the same *optimal* center, and by the triangle inequality `r ≤ 2·OPT`.

```mermaid
flowchart LR
  A["any first center"] --> B["farthest point from the centers so far"] --> C["add as center"] --> B
  C --> D["after k centers: radius ≤ 2·OPT"]
```

## Complexity
`O(n·k)` time, `O(n)` space (maintain each point's distance to its nearest center).

## When to use it
Facility location, data summarisation (choosing representatives), seeding for [k-means](#/k_means) (k-means++ uses a randomised version of the same idea), and bounding schemes in clustering libraries.

## In this implementation
Integer 2-D points with squared distances (so the 2-approximation reads `radius² ≤ 4·OPT²`). Tests compare with exhaustive optimal centers drawn from the input points on 200 random instances, and check that three well-separated clusters are found.
