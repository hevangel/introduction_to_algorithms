# Cubic Spline Interpolation

## What it is
Given points `(x₀, y₀), …, (x_n, y_n)`, a **cubic spline** is a function that is a cubic polynomial on every interval `[x_i, x_{i+1}]`, passes through all the points, and has continuous **first and second derivatives** at the interior knots. It is the smoothest interpolant of its kind (it minimises curvature energy), and the standard tool for drawing smooth curves through data. Problem 28-2 derives it as the solution of a **tridiagonal system** — the structure of [Problem 28-1](#/tridiagonal_solver).

## How it works
Let `M_i = S″(x_i)` and `h_i = x_{i+1} − x_i`. On each interval the cubic is determined by its end values and the two `M`s:

`S(x) = M_i (x_{i+1}−x)³/(6h_i) + M_{i+1}(x−x_i)³/(6h_i) + (y_i/h_i − M_i h_i/6)(x_{i+1}−x) + (y_{i+1}/h_i − M_{i+1} h_i/6)(x−x_i)`.

Demanding continuity of `S′` at the interior knots gives `n − 1` equations

`h_{i−1}M_{i−1} + 2(h_{i−1}+h_i)M_i + h_i M_{i+1} = 6( (y_{i+1}−y_i)/h_i − (y_i−y_{i−1})/h_{i−1} )`.

Two more conditions close the system: **natural** (`M₀ = M_n = 0`) or **clamped** (prescribed slopes at both ends). The matrix is tridiagonal and strictly diagonally dominant, so the [Thomas algorithm](#/tridiagonal_solver) solves it in `O(n)` without pivoting.

```mermaid
flowchart LR
  P["points (xᵢ, yᵢ)"] --> T["tridiagonal system for M₀…Mₙ"] --> S["Thomas algorithm O(n)"] --> C["cubic on each interval"]
```

## Complexity
`O(n)` to fit, `O(lg n)` per evaluation (binary search for the interval).

## When to use it
Curve fitting and animation paths, CAD/CAM, data resampling, and as the 1-D model for smoothing splines and thin-plate splines.

## In this implementation
Natural and clamped boundary conditions. Tests check, on 100 random knot sets, that the spline interpolates and that value, slope and second derivative are continuous at interior knots (natural splines have zero curvature at the ends), and that a clamped spline reproduces an arbitrary cubic exactly.
