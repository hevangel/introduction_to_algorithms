# Radix Sort

## What it is
Radix sort is the algorithm behind old punched-card sorting machines. It sorts `n` numbers of `d` digits (each digit has up to `k` values) in Θ(d(n + k)) time by sorting on one digit at a time.

## How it works
Sort by the **least significant digit first**, using a **stable** sort ([counting sort](#/counting_sort)) for each pass. Stability guarantees that earlier passes act as tie-breakers for later ones.

```mermaid
flowchart LR
  A["329 457 657 839 436 720 355"] -->|"digit 1 (ones)"| B["720 355 436 457 657 329 839"]
  B -->|"digit 2 (tens)"| C["720 329 436 839 355 457 657"]
  C -->|"digit 3 (hundreds)"| D["329 355 436 457 657 720 839"]
```
For `b`-bit keys split into `r`-bit digits the cost is `Θ((b/r)(n + 2ʳ))`; the best choice is `r ≈ lg n`, giving Θ(bn / lg n) — linear for keys of `O(lg n)` bits, or Θ(n) for keys in `0…n^c`.

## Complexity
Θ(d(n + k)) time, Θ(n + k) extra space.

## When to use it
Fixed-width integer keys, IP addresses, dates (Exercise 8.3-1), fixed-length strings, suffix-array construction. Not suited to variable-length keys without padding.

## In this implementation
`radix_sort(a, base)` supports any base ≥ 2 (tests use 2, 10, 16, 256 and 1000); `radix_sort_strings` pads with zero bytes and reuses the stable counting sort.
