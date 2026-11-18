"""Natural Cubic Spline Interpolation — CLRS Problem 28-2 (2e–4e) (2, 3, 4).

Fit C² piecewise-cubic polynomials through n+1 knots in O(n) time using the Thomas tridiagonal solver on second derivatives.
"""

from __future__ import annotations

from bisect import bisect_right
from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _solve_tridiagonal(
    sub: list[float], diag: list[float], sup: list[float], rhs: list[float]
) -> list[float] | None:
    n = len(diag)
    assert len(sub) + 1 == n and len(sup) + 1 == n and len(rhs) == n
    c = [0.0] * n
    d = [0.0] * n
    if abs(diag[0]) < 1e-14:
        return None
    c[0] = sup[0] / diag[0] if n > 1 else 0.0
    d[0] = rhs[0] / diag[0]
    for i in range(1, n):
        denom = diag[i] - sub[i - 1] * c[i - 1]
        if abs(denom) < 1e-14:
            return None
        if i < n - 1:
            c[i] = sup[i] / denom
        d[i] = (rhs[i] - sub[i - 1] * d[i - 1]) / denom
    x = [0.0] * n
    x[n - 1] = d[n - 1]
    for i in range(n - 2, -1, -1):
        x[i] = d[i] - c[i] * x[i + 1]
    return x


@dataclass
class CubicSpline:
    """Natural cubic spline through knots (x_0, y_0), ..., (x_n, y_n) with S''(x_0) = S''(x_n) = 0."""

    xs: list[float]
    ys: list[float]
    d: list[float]

    @classmethod
    def fit(cls, xs: list[float], ys: list[float]) -> CubicSpline:
        """Fit a natural cubic spline in O(n) time using the Thomas tridiagonal solver."""
        m = len(xs)
        assert m >= 2 and m == len(ys)
        assert all(xs[i] < xs[i + 1] for i in range(m - 1)), "knots must be strictly increasing"
        n = m - 1
        if n == 1:
            return cls(xs=list(xs), ys=list(ys), d=[0.0, 0.0])
        h = [xs[i + 1] - xs[i] for i in range(n)]
        k = n - 1
        sub = [0.0] * max(k - 1, 0)
        diag = [0.0] * k
        sup = [0.0] * max(k - 1, 0)
        rhs = [0.0] * k

        for row in range(k):
            i = row + 1
            diag[row] = 2.0 * (h[i - 1] + h[i])
            rhs[row] = 6.0 * ((ys[i + 1] - ys[i]) / h[i] - (ys[i] - ys[i - 1]) / h[i - 1])
            if row + 1 < k:
                sup[row] = h[i]
                sub[row] = h[i]

        interior_d = _solve_tridiagonal(sub, diag, sup, rhs)
        assert interior_d is not None, "SPD spline system"
        d = [0.0, *interior_d, 0.0]
        return cls(xs=list(xs), ys=list(ys), d=d)

    def eval(self, x: float) -> tuple[float, float, float]:
        """Evaluate (S(x), S'(x), S''(x)) at any x."""
        n = len(self.xs) - 1
        if x <= self.xs[0]:
            i = 0
        elif x >= self.xs[n]:
            i = n - 1
        else:
            pos = bisect_right(self.xs, x)
            i = min(pos - 1, n - 1)
        return self._eval_segment(i, x - self.xs[i])

    def _eval_segment(self, seg: int, t: float) -> tuple[float, float, float]:
        hi = self.xs[seg + 1] - self.xs[seg]
        di = self.d[seg]
        di1 = self.d[seg + 1]
        bi = (self.ys[seg + 1] - self.ys[seg]) / hi - hi * (2.0 * di + di1) / 6.0
        ci = di / 2.0
        ei = (di1 - di) / (6.0 * hi)
        val = self.ys[seg] + t * (bi + t * (ci + t * ei))
        d1 = bi + t * (2.0 * ci + 3.0 * ei * t)
        d2 = di + (di1 - di) * (t / hi)
        return (val, d1, d2)


def demo() -> str:
    lines: list[str] = []
    xs = [0.0, 1.0, 2.5, 4.0, 5.0]
    ys = [math.sin(x * 1.1) for x in xs]
    sp = CubicSpline.fit(xs, ys)
    lines.append(f"natural second derivatives D = {[round(v * 1000.0) / 1000.0 for v in sp.d]}")
    for probe in [0.5, 1.0, 2.0, 3.25, 4.5]:
        v, d1, d2 = sp.eval(probe)
        lines.append(f"  x = {probe:>4.2f}: S={v:>7.4f}  S'={d1:>7.4f}  S''={d2:>7.4f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_interpolates_knots_with_c2_continuity_and_natural_boundaries(self) -> None:
        r = Rng(2802)
        for _ in range(40):
            m = r.range(3, 18)
            xs = [0.0]
            for _ in range(1, m):
                xs.append(xs[-1] + 0.3 + r.f64() * 2.5)
            ys = [r.f64() * 20.0 - 10.0 for _ in range(m)]
            sp = CubicSpline.fit(xs, ys)

            self.assertLess(abs(sp.eval(xs[0])[2]), 1e-10)
            self.assertLess(abs(sp.eval(xs[m - 1])[2]), 1e-10)

            for i in range(m):
                self.assertLess(abs(sp.eval(xs[i])[0] - ys[i]), 1e-10)

            for i in range(1, m - 1):
                h_prev = xs[i] - xs[i - 1]
                left = sp._eval_segment(i - 1, h_prev)
                right = sp._eval_segment(i, 0.0)
                self.assertLess(abs(left[0] - right[0]), 1e-9)
                self.assertLess(abs(left[1] - right[1]), 1e-9)
                self.assertLess(abs(left[2] - right[2]), 1e-9)

    def test_reproduces_linear_function_exactly(self) -> None:
        xs = [0.0, 0.7, 1.9, 3.3, 5.0]
        ys = [2.5 * x - 1.25 for x in xs]
        sp = CubicSpline.fit(xs, ys)
        self.assertTrue(all(abs(v) < 1e-11 for v in sp.d))
        for probe in [0.3, 1.5, 2.8, 4.4]:
            v, d1, d2 = sp.eval(probe)
            self.assertLess(abs(v - (2.5 * probe - 1.25)), 1e-10)
            self.assertLess(abs(d1 - 2.5), 1e-10)
            self.assertLess(abs(d2), 1e-10)


if __name__ == "__main__":
    print(demo(), end="")
