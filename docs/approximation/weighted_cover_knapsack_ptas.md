# Weighted Set Cover, Maximal Matching & 0-1 Knapsack FPTAS

## What it is
Four foundational approximation procedures from Chapter 35 problem sets complementing [greedy set cover](#/greedy_set_cover) and [approximate subset sum](#/approx_subset_sum):
1. **Weighted Greedy Set Cover (Problem 35-3)**: Picks sets minimizing cost per newly covered element, preserving the $H(d)$-approximation guarantee.
2. **2-Approximate Maximal Matching (Problem 35-4)**: Any inclusion-maximal matching satisfies $|M| \ge \frac{1}{2}|M^*|$ because every optimal edge shares an endpoint with at least one edge in $M$.
3. **Maximum Spanning Tree (Problem 35-6)**: Negating weights or sorting edges descending turns [Kruskal's MST](#/mst_kruskal) into an exact maximum-weight spanning tree (used to prove 2-approximability on induced subgraphs).
4. **0-1 Knapsack FPTAS (Problem 35-7)**: Scales item profits by $K = \frac{\varepsilon v_{\max}}{n}$ and solves minimum-weight DP over scaled integer values to guarantee profit $\ge (1 - \varepsilon)\mathrm{OPT}$ in $O(n^3/\varepsilon)$ time, upgrading pseudo-polynomial [0-1 knapsack](#/knapsack_01).

## How it works
* **Weighted set cover (`weighted_greedy_set_cover`)**: Iteratively picks $S_i$ minimizing $w_i / |S_i \setminus C|$ until all elements are covered.
* **Maximal matching (`maximal_matching`)**: Greedily accepts any edge whose both endpoints remain free.
* **Knapsack FPTAS (`knapsack_fptas`)**: Discards items heavier than `cap`, defines $v'_i = \lfloor v_i / K \rfloor$, computes minimum weight needed to reach each total scaled value $\le \lfloor n^2 / \varepsilon \rfloor$, and takes the largest feasible value.

```mermaid
flowchart LR
  Items["Items (w_i, v_i)"] --> Scale["Scale v'_i = floor(n * v_i / (eps * v_max))"]
  Scale --> DP["Min-weight DP over scaled values (O(n^3 / eps))"]
  DP --> Out["Output >= (1 - eps) OPT"]
```

## Complexity
| Algorithm | Approximation Ratio | Running Time |
|---|---|---|
| Weighted Greedy Set Cover | $H(d) \le \ln d + 1$ | $O(m \cdot |U|)$ |
| Maximal Matching | $2$-approx | $O(V + E)$ |
| Max Spanning Tree | Exact | $O(E \lg V)$ |
| 0-1 Knapsack FPTAS | $(1-\varepsilon)$-approx | $O(n^3 / \varepsilon)$ |

## When to use it
When unweighted set cover extends to heterogeneous costs, fast $O(E)$ linear matching avoids $O(E\sqrt{V})$ [Hopcroft–Karp](#/hopcroft_karp) overhead, or large knapsack capacities make $O(nW)$ DP infeasible.

## In this implementation
Validates weighted cover against $2^m$ brute-force optimum times $H(d)$, maximal matching $\ge |M^*|/2$ against `hopcroft_karp`, exact max spanning tree, and knapsack FPTAS $\ge (1-\varepsilon)\mathrm{OPT}$ against exact `knapsack_01`.
