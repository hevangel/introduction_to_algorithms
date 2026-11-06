# Signing Free-Agent Players

## What it is
A baseball manager has `X` dollars to spend on `N` positions; for each position several free agents are available, each with a requested **salary** and a **VORP** (value over replacement player). Exactly one player must be signed for every position. Problem 15-12 asks for an algorithm that maximises the total VORP without exceeding the budget.

## How it works
This is a **multiple-choice knapsack**: from each *group* (position) pick exactly one item. Let `V[i, x]` be the best total VORP using positions `1…i` with total salary at most `x`:

`V[i, x] = max over players p at position i with salary(p) ≤ x of ( V[i−1, x − salary(p)] + vorp(p) )`.

`V[0, x] = 0`; positions with no affordable candidate make a state infeasible. Back-pointers reconstruct who is signed. Like [0-1 knapsack](#/knapsack_01) the cost is pseudo-polynomial in the budget.

```mermaid
flowchart LR
  S["V[0][·] = 0"] --> P1["position 1: one candidate"] --> P2["position 2: one candidate"] --> Pn["position N"] --> R["V[N][X]"]
```

## Complexity
`O(N·X·P)` for `P` candidates per position; `O(N·X)` space (or `O(X)` for the value alone).

## When to use it
Team/portfolio selection under a budget when each slot needs exactly one choice (configurations with one component per category, menu planning, ad slot allocation).

## In this implementation
Tests compare with enumeration of all combinations on 200 random rosters and check that the reported assignment respects the budget and sums to the claimed VORP; an unaffordable roster returns `None`.
