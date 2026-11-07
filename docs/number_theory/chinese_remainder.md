# Chinese Remainder Theorem

## What it is
Over 1500 years ago Sun Tzu asked for a number that leaves remainder 2 when divided by 3, 3 by 5 and 2 by 7 (answer 23). The **Chinese remainder theorem (CRT)** says that if the moduli `n₁,…,n_k` are pairwise relatively prime and `n = n₁n₂⋯n_k`, then the map `x ↦ (x mod n₁, …, x mod n_k)` is a **bijection** between `Z_n` and `Z_{n₁} × … × Z_{n_k}` that respects addition and multiplication.

## How it works
Reconstruction formula: let `mᵢ = n/nᵢ` and `cᵢ = mᵢ·(mᵢ⁻¹ mod nᵢ)` (the inverse via [extended Euclid](#/extended_euclid)). Then `cᵢ ≡ 1 (mod nᵢ)` and `cᵢ ≡ 0 (mod nⱼ)` for `j ≠ i`, so

`x = Σ aᵢcᵢ  (mod n)`.

For moduli that are **not** coprime, congruences can be merged pairwise: `x ≡ a₁ (mod n₁)` and `x ≡ a₂ (mod n₂)` are consistent iff `gcd(n₁,n₂) | (a₂−a₁)`, and then the solution is unique modulo `lcm(n₁,n₂)`.

```mermaid
flowchart LR
  X["x mod 105"] <--> R["(x mod 3, x mod 5, x mod 7)"]
  R -- "compute modulo small primes in parallel" --> OP["+ , ×"]
  OP --> RES["recombine with CRT"]
```

## Complexity
`O(k lg² n)` arithmetic for reconstruction; the real benefit is **arithmetic on small residues**.

## When to use it
Speeding up big-integer / modular computations (RSA decryption with CRT is ~4× faster), multi-modular linear algebra, hash tables with moduli, calendar problems ("every 4 days and every 6 days"), secret sharing.

## In this implementation
`crt` implements the book's formula; `crt_general` merges any congruences. Tests check the bijection for `105 = 3·5·7` and non-coprime cases against brute-force search.
