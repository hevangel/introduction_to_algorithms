# Activity Selection

## What it is
Given `n` activities `aᵢ = [sᵢ, fᵢ)` that all need the same resource (a lecture hall), select a **maximum-size set of mutually compatible** (non-overlapping) activities. It is the introductory example of a **greedy algorithm**: make the locally best choice and never reconsider.

## How it works
**Greedy choice:** among activities sorted by finish time, the one that finishes first can always be included in some optimal solution. Proof (exchange argument): take an optimal set, replace its first activity by the one finishing earliest — the schedule stays feasible and has the same size. After committing to that activity, what remains is the same problem restricted to activities starting after it finishes (optimal substructure).

```mermaid
flowchart LR
  A["sort by finish"] --> B["take first"] --> C["drop activities that overlap it"] --> D["repeat on the rest"]
```

* `RECURSIVE-ACTIVITY-SELECTOR` and `GREEDY-ACTIVITY-SELECTOR` do this in Θ(n) after sorting.
* The DP formulation `c[i,j] = max_k c[i,k] + c[k,j] + 1` needs Θ(n³) — a good contrast showing what greedy saves.
* Exercise 16.1-4 asks for the fewest lecture halls so every activity is scheduled: the answer equals the maximum number of simultaneously active activities.

## Complexity
Θ(n lg n) (sorting) + Θ(n); DP version Θ(n³).

## When to use it
Meeting-room scheduling, job scheduling on one machine, bandwidth reservation. Fails for the *weighted* variant (maximise total value), where DP is required.

## In this implementation
Greedy, recursive, DP and lecture-hall versions; randomised tests ensure greedy and DP return the same size and that the greedy set is conflict-free.
