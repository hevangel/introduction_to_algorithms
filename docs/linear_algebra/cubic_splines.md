# Natural Cubic Spline Interpolation

## What it is
Given $n+1$ strictly increasing knots $(x_0, y_0), \dots, (x_n, y_n)$, a **natural cubic spline** fits a piecewise degree-3 polynomial $S_i(x)$ on every interval $[x_i, x_{i+1}]$ so that $S$, its slope $S'$, and its curvature $S''$ are all **continuous** ($C^2$-smooth) across every interior knot, while imposing natural zero-curvature boundary conditions $S''(x_0) = S''(x_n) = 0$.

## How it works
Parameterize each cubic piece by its endpoint second derivatives $D_i = S''(x_i)$ and widths $h_i = x_{i+1} - x_i$:
$$S_i(x) = y_i + \left(\frac{y_{i+1}-y_i}{h_i} - \frac{h_i}{6}(2D_i + D_{i+1})\right)t + \frac{D_i}{2}t^2 + \frac{D_{i+1}-D_i}{6h_i}t^3, \quad t = x - x_i.$$
Matching left and right first derivatives $S'_{i-1}(x_i) = S'_i(x_i)$ at each interior knot $i = 1, \dots, n-1$ yields a symmetric, strictly diagonally dominant tridiagonal system in the unknowns $(D_1, \dots, D_{n-1})$:
$$h_{i-1} D_{i-1} + 2(h_{i-1} + h_i) D_i + h_i D_{i+1} = 6\left(\frac{y_{i+1}-y_i}{h_i} - \frac{y_i - y_{i-1}}{h_{i-1}}\right).$$
Solving this system via the [tridiagonal Thomas solver](#/tridiagonal_solver) takes $\Theta(n)$ time.

```mermaid
flowchart LR
  Knots["Knots (x_i, y_i)"] --> System["Build (n-1)x(n-1) tridiagonal curvature system"]
  System -->|"Thomas O(n)"| D["Second derivatives D_0..D_n"]
  D --> Eval["O(lg n) segment lookup + O(1) eval S, S', S''"]
```

## Complexity
* **Fitting (`CubicSpline::fit`)**: $\Theta(n)$ time and space (even for non-uniform knot spacings $h_i$).
* **Evaluation (`CubicSpline::eval`)**: $O(\lg n)$ binary search for interval $[x_i, x_{i+1}]$ plus $O(1)$ Horner evaluation of $S(x)$, $S'(x)$, and $S''(x)$.

## When to use it
For smooth CAD/graphics curves, numerical differentiation/quadrature, and non-oscillatory interpolation where high-degree global polynomials suffer from Runge's phenomenon.

## In this implementation
`CubicSpline` supports non-uniform knot spacings, evaluates values and derivatives cleanly, reproduces linear functions with zero second derivatives, and verifies $C^0$, $C^1$, and $C^2$ continuity at every interior knot across randomized non-uniform meshes.
