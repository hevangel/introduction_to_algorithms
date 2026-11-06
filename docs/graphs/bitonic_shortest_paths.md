# Bitonic Shortest Paths

## What it is
A sequence is **bitonic** if it monotonically increases and then monotonically decreases (either part may be empty). A path is *bitonic* if the weights of its consecutive edges form a bitonic sequence — e.g. a flight itinerary where each hop is longer and then shorter. Problem 24-6 asks for an `O(E lg V)`-time algorithm for the shortest bitonic path from `s` to every vertex, assuming all edge weights are distinct.

## How it works
Sort the edges by weight once (`O(E lg E)`). Then:
1. **Increasing phase.** Relax every edge **once, in ascending weight order**: `inc[v] = min(inc[v], inc[u] + w)`. Because each edge is processed after all lighter ones, any path produced uses edges in increasing weight order, and every increasing-weight path is found (its edges appear in the sorted order).
2. **Decreasing phase.** Starting from the `inc` values, relax every edge once in **descending** weight order. A path built from an increasing prefix followed by a decreasing suffix is bitonic whether or not the junction weights fit — if the first descending edge is heavier than the last ascending one the path simply peaks one edge later.

Each edge is touched twice: no priority queue is needed, so the total is the sorting cost. Distinct weights make "increasing/decreasing" strict.

```mermaid
flowchart LR
  S["sort edges by weight"] --> A["pass up: relax ascending → inc[·]"] --> D["pass down: relax descending from inc → answer"]
```

## Complexity
`O(E lg E)` (sorting) plus `O(E)` relaxations — better than Dijkstra's `O(E lg V)` when the weights are pre-sorted.

## When to use it
Itinerary/route problems with monotonicity constraints (e.g. "climb then descend" mountain routes, ordered-fare routing), and as an illustration that restricting path structure can replace the priority queue by *sorting the edges* — compare [DAG shortest paths](#/dag_shortest_paths).

## In this implementation
Tests compare with exhaustive enumeration of all simple bitonic paths on 300 random graphs with distinct weights.
