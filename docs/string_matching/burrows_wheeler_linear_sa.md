# Linear-Time Suffix Array & Burrows-Wheeler Transform

## What it is
Two cornerstone linear-time string indexing and compression algorithms introduced in the 4th edition (extending [suffix arrays](#/suffix_array) from $O(n \lg^2 n)$ prefix doubling to $\Theta(n)$):
1. **DC3 / Skew Suffix Array (`dc3_suffix_array`)**: Kärkkäinen–Sanders divide-and-conquer construction sorting all suffixes in $\Theta(n)$ worst-case time on integer alphabets.
2. **Burrows–Wheeler Transform (`bwt_encode` & `bwt_decode`)**: An invertible permutation placing all cyclic predecessors of sorted suffixes together, boosting run-length and entropy compression while remaining exactly invertible in $\Theta(n)$ time via LF-mapping.

## How it works
* **DC3 / Skew (`dc3_rec`)**:
  1. Partition positions modulo 3 into sample indices $S_{12} = \{i : i \bmod 3 \ne 0\}$ and non-sample $S_0 = \{i : i \bmod 3 = 0\}$.
  2. Radix-sort triples $(T[i], T[i+1], T[i+2])$ for $i \in S_{12}$ in $O(n)$ time; if duplicates occur, assign lexicographic rank names and recurse on the $2n/3$-length string of names.
  3. Induce $S_0$ order in one counting-sort pass by pairing $T[i]$ with the known rank of $i+1 \in S_{12}$.
  4. Merge sorted $S_{12}$ and $S_0$ in $O(n)$ comparisons using at most $1$–$2$ character reads and one rank lookup.
* **BWT encode & decode**: Compute the DC3 suffix array over `text + $`, emit each predecessor byte `text[sa[r] - 1]` plus the sentinel row, and invert in $O(n)$ via cumulative character offsets $C[c]$ and occurrence ranks (`LF(r) = C[L[r]] + rank(r)`).

```mermaid
flowchart LR
  S["Text T[0..n]"] -->|"i mod 3 != 0"| S12["Radix sort 3-tuples & recurse (2n/3)"]
  S12 -->|"induce i mod 3 == 0"| S0["Counting sort S_0"]
  S12 --> Merge["O(n) linear merge -> SA"]
  S0 --> Merge
```

## Complexity
* **DC3 suffix array**: $T(n) = T(\lceil 2n/3 \rceil) + \Theta(n) = \Theta(n)$ time and space.
* **BWT encode & decode**: $\Theta(n)$ time and $\Theta(n)$ space.

## When to use it
Use DC3 when building large full-text substring indices without $O(n \lg n)$ doubling overhead, and BWT as the reversible front-end for `bzip2`-style compressors and FM-indexes.

## In this implementation
`dc3_suffix_array` is verified against `suffix_array::suffix_array` across periodic strings, Fibonacci words, and 80 random inputs; `bwt_encode` / `bwt_decode` roundtrip arbitrary binary byte slices across lengths `0..120`.
