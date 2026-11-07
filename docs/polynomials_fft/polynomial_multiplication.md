# Polynomial Multiplication via FFT

## What it is
Multiplying two polynomials `A(x)·B(x)` of degree `n−1` by the schoolbook method costs `Θ(n²)` coefficient products. The product's coefficient vector is the **convolution** `c_j = Σ_k a_k b_{j−k}`. Chapter 30 shows how the [FFT](#/fft_recursive) reduces it to `Θ(n lg n)`.

## How it works
```mermaid
flowchart LR
  A["coefficients a, b<br/>(pad to 2n, power of two)"] -->|"FFT: Θ(n lg n)"| P["point-value form<br/>A(ωᵏ), B(ωᵏ)"]
  P -->|"pointwise product: Θ(n)"| C["C(ωᵏ) = A(ωᵏ)B(ωᵏ)"]
  C -->|"inverse FFT: Θ(n lg n)"| R["coefficients of C"]
```
Key fact: the product of polynomials of degree-bound `n` has degree-bound `2n−1`, so `2n` sample points are needed — we pad to the next power of two. Point-value multiplication is `Θ(n)` because `C(xₖ) = A(xₖ)B(xₖ)`.

**Big-integer multiplication** is the same thing: an integer is a polynomial evaluated at the base (10 or 2¹⁶…); convolve the digit vectors and propagate carries (the Schönhage–Strassen algorithm uses exact number-theoretic transforms to avoid rounding).

## Complexity
Θ(n lg n) versus Θ(n²) (and Θ(n^1.585) for Karatsuba). Floating-point FFT is exact only while rounding errors stay below ½ — fine for moderate coefficients and sizes.

## When to use it
Huge-integer arithmetic (GMP, computer algebra), string matching with wildcards, convolution in signal/image processing, counting problems (generating functions), Bluestein/Rader reductions of other transforms.

## In this implementation
`multiply_fft` rounds the real parts of the inverse transform; it is verified against `multiply_naive` on random polynomials, and the demo multiplies 12345 × 6789 by convolving digit vectors.
