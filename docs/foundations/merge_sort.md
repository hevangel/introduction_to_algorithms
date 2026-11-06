# Merge Sort

## What it is
Merge sort is the canonical **divide-and-conquer** algorithm and the book's first example of a recurrence. It guarantees Θ(n lg n) time on every input.

## How it works
1. **Divide** the array into two halves.
2. **Conquer** by sorting each half recursively (an array of one element is already sorted).
3. **Combine** with `MERGE`: repeatedly take the smaller front element of the two sorted halves — linear time.

```mermaid
flowchart TB
  A["38 27 43 3 9 82 10"] --> B["38 27 43 3"]
  A --> C["9 82 10"]
  B --> D["38 27"]
  B --> E["43 3"]
  C --> F["9 82"]
  C --> G["10"]
  D --> H["27 38"]
  E --> I["3 43"]
  F --> J["9 82"]
  G --> K["10"]
  H --> L["3 27 38 43"]
  I --> L
  J --> M["9 10 82"]
  K --> M
  L --> N["3 9 10 27 38 43 82"]
  M --> N
```

The running time satisfies `T(n) = 2T(n/2) + Θ(n)`, which the [master theorem](#/master_theorem) solves to `Θ(n lg n)`.

## Complexity
Θ(n lg n) time in all cases; Θ(n) auxiliary space; stable.

## When to use it
* Guaranteed worst-case bounds and stability (e.g. sorting records by several keys).
* External sorting and linked lists, where sequential access matters.
* It underlies [counting inversions](#/inversions) and [parallel merge sort](#/p_merge_sort).

## In this implementation
`merge` implements MERGE(A,p,q,r) with auxiliary vectors (the 3rd/4th-edition style, without sentinels). `merge_sort` is the recursive version, `merge_sort_bottom_up` the iterative one from Section 2.3's discussion.
