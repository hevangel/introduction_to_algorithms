# Multidimensional FFT

## What it is
The **d-dimensional DFT** of an array `A(j₁, …, j_d)` with dimensions `n₁ × ⋯ × n_d` is

`Y(k₁, …, k_d) = Σ_{j₁} ⋯ Σ_{j_d} A(j₁,…,j_d) · ω_{n₁}^{j₁k₁} ⋯ ω_{n_d}^{j_dk_d}`.

It transforms images (`d = 2`), volumes and video (`d = 3`, `4`), and computes products of multivariate polynomials. Problem 30-3 asks to show that it can be computed by performing ordinary one-dimensional [FFTs](#/fft_recursive) along each dimension in turn.

## How it works
The sum factorises. Take the 1-D FFT of **every line parallel to axis `d`** (replace `j_d` by `k_d`), then of every line parallel to axis `d − 1`, and so on. Each pass costs `(N/n_i) · O(n_i lg n_i) = O(N lg n_i)` for `N = n₁⋯n_d` points, so the total is

`O(N (lg n₁ + ⋯ + lg n_d)) = O(N lg N)`.

The order of axes is irrelevant. The inverse is the same procedure with inverse 1-D transforms. A **2-D convolution** (blurring an image) is then one forward transform per input, a pointwise product, and one inverse — and the product of bivariate polynomials is the same computation on the coefficient matrix, padded to avoid wrap-around.

```mermaid
flowchart LR
  A["n₁×n₂ array"] -->|"FFT of every row"| B["rows transformed"] -->|"FFT of every column"| C["2-D spectrum"]
```

## Complexity
`Θ(N lg N)` with `N` the total number of points.

## When to use it
Image filtering and compression, PDE solvers with periodic boundaries, medical imaging (MRI reconstruction), and multi-dimensional convolutions in signal processing; Kronecker-substitution turns any multivariate product into a 1-D one but a direct transform is usually faster.

## In this implementation
`fft_nd` works on a row-major slice with any list of power-of-two dimensions, using strides to gather each line. Tests compare the 2-D transform with the defining double sum, check forward/inverse round trips on a `4×2×8` array, and multiply 50 random bivariate polynomials against schoolbook multiplication.
