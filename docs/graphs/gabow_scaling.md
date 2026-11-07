# Gabow's Scaling Algorithm

## What it is
For single-source shortest paths with **non-negative integer weights** bounded by `W`, Gabow's scaling algorithm (1985) runs in `O(E lg W)` time — faster than [Dijkstra](#/dijkstra) with a binary heap when `W` is small. Problem 24-4 develops it as a divide-and-conquer on the *bits* of the weights.

## How it works
Let `k = ⌈lg(W+1)⌉` and define the truncated weights `wᵢ(u,v) = ⌊w(u,v) / 2^{k−i}⌋` — the `i` most significant bits. So `w₀ = 0` and `wᵢ = 2·wᵢ₋₁ + (the next bit)`. Let `δᵢ(s, v)` be shortest distances under `wᵢ`.

**Key lemma:** use the previous solution as a potential. The reweighted weights
`ŵᵢ(u,v) = wᵢ(u,v) + 2δᵢ₋₁(s,u) − 2δᵢ₋₁(s,v)`
are **non-negative** (by the triangle inequality for `δᵢ₋₁` applied to `wᵢ₋₁`, `2wᵢ₋₁(u,v) + 2δᵢ₋₁(u) − 2δᵢ₋₁(v) ≥ 0`), and the shortest-path distance in the reweighted graph is small: `δ̂ᵢ(s, v) ≤ |E|` (a shortest path under `wᵢ` is at most `2δᵢ₋₁ + |E|`, since each edge contributes at most one extra bit). Then

`δᵢ(s, v) = 2·δᵢ₋₁(s, v) + δ̂ᵢ(s, v)`.

Because `δ̂` is bounded by `|E|`, each phase can use a **bucket queue** ([Dial](#/dial_dijkstra)) in `O(E)` time — and there are `k = O(lg W)` phases.

```mermaid
flowchart LR
  P0["δ₀ = 0"] -->|"reweight with δ₀, Dial"| P1["δ₁"] -->|"reweight with δ₁, Dial"| P2["δ₂"] --> Pk["δ_k = exact distances"]
```

## Complexity
`O(E lg W)`; compare `O(E + V lg V)` with Fibonacci heaps.

## When to use it
Integer-weighted graphs with moderate `W` (road networks, unit-cost variants), and as an instance of the *scaling* technique that also gives capacity-scaling flow algorithms ([scaling max-flow](#/scaling_max_flow)), bit-scaling for matching, and number-theoretic algorithms.

## In this implementation
Reachability is computed first (unreachable vertices stay `None`); each phase reweights and calls the bucket-queue Dijkstra. Tests compare with binary-heap Dijkstra on 200 random graphs with maximum weights 1, 7, 100 and 5000.
