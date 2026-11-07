# Insertion Sort

## What it is
Insertion sort is the first algorithm in CLRS and the model for how the book analyses algorithms. It sorts the way many people sort a hand of playing cards: keep the left part sorted, pick up the next card, and slide it left until it sits in the right place.

## How it works
For `j = 2 … n`, take `A[j]` as the **key** and shift every larger element of the sorted prefix `A[1..j-1]` one slot to the right, then drop the key into the gap.

```mermaid
flowchart LR
  A["5 2 4 6 1 3"] --> B["2 5 | 4 6 1 3"]
  B --> C["2 4 5 | 6 1 3"]
  C --> D["2 4 5 6 | 1 3"]
  D --> E["1 2 4 5 6 | 3"]
  E --> F["1 2 3 4 5 6"]
```

**Loop invariant** (the book's proof technique): at the start of each iteration of the outer loop, `A[1..j-1]` holds the original first `j-1` elements in sorted order.

## Complexity
| Case | Time |
|---|---|
| Best (already sorted) | Θ(n) |
| Worst (reverse sorted) | Θ(n²) |
| Average | Θ(n²) |
| Extra space | O(1) — in place, stable |

## When to use it
* Small inputs or nearly sorted data (cost is Θ(n + inversions)).
* As the base case of faster hybrid sorts (e.g. quicksort/merge sort cut over below ~16 elements; also used in bucket sort).
* Online setting: it can sort elements as they arrive.

## In this implementation
`insertion_sort` and the non-increasing variant `insertion_sort_desc` (Exercise 2.1-2) work on any `PartialOrd + Clone` slice; a test checks stability.
