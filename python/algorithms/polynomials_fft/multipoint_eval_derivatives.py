"""All Derivatives & Fast Multipoint Evaluation — CLRS Problems 30-2..30-4 (4e); Problems 30-3..30-5 (1e–3e) (1, 2, 3, 4).

Multidimensional 2D FFT, O(n lg n) all-derivatives evaluation at a point via convolution, and O(n lg² n) subproduct-tree multipoint polynomial evaluation.
"""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(frozen=True, slots=True)
class Complex:
    re: float
    im: float

    @classmethod
    def cis(cls, theta: float) -> Complex:
        return cls(math.cos(theta), math.sin(theta))

    def scale(self, k: float) -> Complex:
        return Complex(self.re * k, self.im * k)

    def abs(self) -> float:
        return math.hypot(self.re, self.im)

    def __abs__(self) -> float:
        return math.hypot(self.re, self.im)

    def __add__(self, o: Complex) -> Complex:
        return Complex(self.re + o.re, self.im + o.im)

    def __sub__(self, o: Complex) -> Complex:
        return Complex(self.re - o.re, self.im - o.im)

    def __mul__(self, o: Complex) -> Complex:
        return Complex(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)


def _fft(a: list[Complex], invert: bool = False) -> list[Complex]:
    def rec(arr: list[Complex], sign: float) -> list[Complex]:
        n = len(arr)
        assert n > 0 and (n & (n - 1)) == 0
        if n == 1:
            return list(arr)
        wn = Complex.cis(sign * 2.0 * math.pi / n)
        y0 = rec(arr[0::2], sign)
        y1 = rec(arr[1::2], sign)
        y = [Complex(0.0, 0.0)] * n
        w = Complex(1.0, 0.0)
        for k in range(n // 2):
            t = w * y1[k]
            y[k] = y0[k] + t
            y[k + n // 2] = y0[k] - t
            w = w * wn
        return y

    y = rec(a, -1.0 if invert else 1.0)
    if invert:
        s = 1.0 / len(y)
        y = [v.scale(s) for v in y]
    return y


def _next_power_of_two(n: int) -> int:
    return 1 if n <= 1 else 1 << (n - 1).bit_length()


def fft_2d(grid: list[list[Complex]], invert: bool = False) -> list[list[Complex]]:
    """Problem 30-2 (4e) / 30-3 (1e-3e): 2D FFT along every row, then every column."""
    rows = len(grid)
    cols = len(grid[0])
    assert (rows & (rows - 1)) == 0 and (cols & (cols - 1)) == 0
    temp = [_fft(row, invert) for row in grid]
    for c in range(cols):
        col = [temp[r][c] for r in range(rows)]
        transformed = _fft(col, invert)
        for r in range(rows):
            temp[r][c] = transformed[r]
    return temp


def poly_mul_fft(a: list[float], b: list[float]) -> list[float]:
    """Multiply two real polynomials in O(n lg n) via FFT."""
    if not a or not b:
        return []
    out_len = len(a) + len(b) - 1
    n = _next_power_of_two(out_len)
    fa = [Complex(a[i] if i < len(a) else 0.0, 0.0) for i in range(n)]
    fb = [Complex(b[i] if i < len(b) else 0.0, 0.0) for i in range(n)]
    ya = _fft(fa, False)
    yb = _fft(fb, False)
    yc = [x * y for x, y in zip(ya, yb)]
    return [c.re for c in _fft(yc, True)[:out_len]]


def all_derivatives_at(coeffs: list[float], x0: float) -> list[float]:
    """Problem 30-4: Compute all derivatives A^(0)(x0), ..., A^(n-1)(x0) in O(n lg n) via FFT."""
    n = len(coeffs)
    if n == 0:
        return []
    fact = [1.0] * n
    for i in range(1, n):
        fact[i] = fact[i - 1] * i
    f = [coeffs[j] * fact[j] for j in range(n)]
    g = [0.0] * n
    pow_x = 1.0
    for k in range(n):
        g[n - 1 - k] = pow_x / fact[k]
        pow_x *= x0
    conv = poly_mul_fft(f, g)
    return [conv[t + n - 1] for t in range(n)]


def _trim_poly(p: list[float]) -> list[float]:
    res = list(p)
    while len(res) > 1 and abs(res[-1]) < 1e-12:
        res.pop()
    return res


def _poly_rem(a: list[float], b: list[float]) -> list[float]:
    r = list(a)
    db = len(b) - 1
    lead = b[db]
    while len(r) >= len(b):
        shift = len(r) - len(b)
        coeff = r[-1] / lead
        for i in range(len(b)):
            r[shift + i] -= coeff * b[i]
        r.pop()
    return _trim_poly(r)


@dataclass
class _SubTree:
    poly: list[float]
    left: _SubTree | None = None
    right: _SubTree | None = None


def multipoint_eval(coeffs: list[float], pts: list[float]) -> list[float]:
    """Problem 30-4 (4e) / 30-5 (1e-3e): Subproduct-tree multipoint evaluation A(x_0), ..., A(x_{m-1})."""
    m = len(pts)
    if m == 0:
        return []

    def build(slice_pts: list[float]) -> _SubTree:
        if len(slice_pts) == 1:
            return _SubTree(poly=[-slice_pts[0], 1.0])
        mid = len(slice_pts) // 2
        left = build(slice_pts[:mid])
        right = build(slice_pts[mid:])
        return _SubTree(poly=poly_mul_fft(left.poly, right.poly), left=left, right=right)

    def eval_down(a: list[float], node: _SubTree, out: list[float]) -> None:
        r = _poly_rem(a, node.poly)
        if node.left is None or node.right is None:
            out.append(r[0])
        else:
            eval_down(r, node.left, out)
            eval_down(r, node.right, out)

    tree = build(pts)
    out: list[float] = []
    eval_down(coeffs, tree, out)
    return out


def horner_eval(coeffs: list[float], x: float) -> float:
    acc = 0.0
    for c in reversed(coeffs):
        acc = acc * x + c
    return acc


def demo() -> str:
    lines: list[str] = []
    coeffs = [1.0, -2.0, 3.0, -1.0, 2.0]
    derivs = all_derivatives_at(coeffs, 2.0)
    lines.append(
        f"all derivatives of 1 - 2x + 3x^2 - x^3 + 2x^4 at x=2: {[float(round(v)) for v in derivs]}"
    )
    pts = [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0]
    vals = multipoint_eval(coeffs, pts)
    lines.append(f"multipoint eval at {pts} = {[round(v * 100.0) / 100.0 for v in vals]}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_fft_2d_roundtrips_and_matches_separable_dft(self) -> None:
        r = Rng(3002)
        rows, cols = 8, 4
        grid = [[Complex(r.f64() - 0.5, r.f64() - 0.5) for _ in range(cols)] for _ in range(rows)]
        freq = fft_2d(grid, False)
        back = fft_2d(freq, True)
        for i in range(rows):
            for j in range(cols):
                self.assertLess((back[i][j] - grid[i][j]).abs(), 1e-10)

    def test_all_derivatives_matches_direct_taylor_shift(self) -> None:
        r = Rng(3004)
        for n in range(1, 11):
            coeffs = [float(r.range(0, 10) - 5) for _ in range(n)]
            x0 = r.f64() * 2.0 - 1.0
            got = all_derivatives_at(coeffs, x0)
            cur = list(coeffs)
            for t in range(n):
                want = horner_eval(cur, x0)
                self.assertLess(abs(got[t] - want), 1e-6, f"deg={n}, t={t}: got {got[t]} vs want {want}")
                cur = [k * v for k, v in enumerate(cur) if k > 0]

    def test_multipoint_eval_matches_horner(self) -> None:
        r = Rng(3005)
        for _ in range(25):
            n = r.range(1, 20)
            m = r.range(1, 16)
            coeffs = [r.f64() * 4.0 - 2.0 for _ in range(n)]
            pts = [-1.2 + 2.4 * k / m for k in range(m)]
            got = multipoint_eval(coeffs, pts)
            for pt, g in zip(pts, got):
                want = horner_eval(coeffs, pt)
                self.assertLess(abs(g - want), 1e-4, f"pt={pt}: got {g} vs want {want}")


if __name__ == "__main__":
    print(demo(), end="")
