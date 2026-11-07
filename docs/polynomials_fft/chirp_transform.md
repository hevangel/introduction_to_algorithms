# Chirp Transform: Evaluating a Polynomial at Many Points

## What it is
The [FFT](#/fft_recursive) evaluates a polynomial at the `n` complex **roots of unity**. Problem 30-5 generalises this: evaluate `A(x) = Σ_{j<n} a_j xʲ` at the `n` points `ω⁰, ω¹, …, ω^{n−1}` for an **arbitrary** `ω` — a geometric progression, no root-of-unity property needed. This is the **chirp transform** (a.k.a. Bluestein's algorithm when `ω` is a root of unity and `n` is not a power of two), in `O(n lg n)` time.

## How it works
Use the identity `jk = C(j+k, 2) − C(j, 2) − C(k, 2)`, where `C(m, 2) = m(m−1)/2`. Then

`y_k = A(ω^k) = Σ_j a_j ω^{jk} = ω^{−C(k,2)} · Σ_j [ a_j ω^{−C(j,2)} ] · ω^{C(j+k,2)}`.

The inner sum is a **correlation** of the sequences `f_j = a_j ω^{−C(j,2)}` and `g_r = ω^{C(r,2)}` — i.e. the coefficients of a polynomial product (reverse `f`), computed with FFT/[NTT](#/number_theoretic_transform) in `O(n lg n)`. Multiply the result by the "chirp" `ω^{−C(k,2)}` (the name comes from the chirped signal `ω^{k²/2}`). Over the integers modulo `998 244 353` the computation is exact; only `ω ≠ 0` is needed.

```mermaid
flowchart LR
  A["a_j"] -->|"× ω^(−C(j,2))"| F["f_j"] -->|"convolve with g_r = ω^(C(r,2))"| Z["z_k"] -->|"× ω^(−C(k,2))"| Y["A(ω^k)"]
```

## Complexity
`O(n lg n)` for all `n` values, versus `O(n²)` by repeated [Horner](#/horner) evaluation and `Θ(n lg n)` only for roots of unity by the plain FFT.

## When to use it
Spectral analysis at arbitrary frequencies (zoom FFT), DFTs of non-power-of-two lengths (Bluestein), fast evaluation at geometric progressions, and `q`-analogue computations.

## In this implementation
Exponents `C(r, 2)` are reduced modulo `P − 1`. Tests compare with direct evaluation on 150 random polynomials and random `ω ∈ [1, P−1]`, plus the degenerate cases `ω = 1` (every value is the coefficient sum) and `n = 1`.
