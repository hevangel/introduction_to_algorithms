"""Fast Fourier Transform (Recursive) — CLRS 30.2–30.3 (1e–4e) (1, 2, 3, 4).

RECURSIVE-FFT evaluates a polynomial at the n complex n-th roots of unity in Θ(n lg n); the inverse DFT interpolates.
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
        """e^{i*theta}."""
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


Complex.ZERO = Complex(0.0, 0.0)  # type: ignore[attr-defined]


def _fft_rec(a: list[Complex], sign: float) -> list[Complex]:
    n = len(a)
    assert n > 0 and (n & (n - 1)) == 0, "length must be a power of two"
    if n == 1:
        return list(a)
    wn = Complex.cis(sign * 2.0 * math.pi / n)
    even = a[0::2]
    odd = a[1::2]
    y0 = _fft_rec(even, sign)
    y1 = _fft_rec(odd, sign)
    y = [Complex(0.0, 0.0)] * n
    w = Complex(1.0, 0.0)
    for k in range(n // 2):
        t = w * y1[k]
        y[k] = y0[k] + t
        y[k + n // 2] = y0[k] - t
        w = w * wn
    return y


def fft(a: list[Complex], invert: bool = False) -> list[Complex]:
    """RECURSIVE-FFT(a): n must be a power of 2. Set invert for the inverse DFT (scaled by 1/n)."""
    y = _fft_rec(a, -1.0 if invert else 1.0)
    if invert:
        s = 1.0 / len(y)
        y = [v.scale(s) for v in y]
    return y


def dft_naive(a: list[Complex]) -> list[Complex]:
    """The O(n^2) definition of the DFT, for comparison."""
    n = len(a)
    out: list[Complex] = []
    for k in range(n):
        acc = Complex(0.0, 0.0)
        for j in range(n):
            acc = acc + a[j] * Complex.cis(2.0 * math.pi * (j * k) / n)
        out.append(acc)
    return out


def demo() -> str:
    lines: list[str] = []
    a = [Complex(float(x), 0.0) for x in (1.0, 2.0, 3.0, 4.0)]
    y = fft(a, False)
    for k, v in enumerate(y):
        lines.append(f"A(ω⁴^{k}) = {v.re:>7.3f} {v.im:+.3f}i")
    back = fft(y, True)
    lines.append(f"inverse FFT recovers coefficients: {[round(c.re * 1e9) / 1e9 for c in back]}")
    lines.append("Θ(n lg n) vs Θ(n²): for n = 2^20 that is ≈ 2·10⁷ vs 10¹² butterfly operations")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_definition_and_roundtrips(self) -> None:
        r = Rng(200)
        for k in range(8):
            n = 1 << k
            a = [Complex(r.f64() - 0.5, r.f64() - 0.5) for _ in range(n)]
            f = fft(a, False)
            d = dft_naive(a)
            for i in range(n):
                self.assertLess((f[i] - d[i]).abs(), 1e-9, f"n={n} i={i}")
            back = fft(f, True)
            for i in range(n):
                self.assertLess((back[i] - a[i]).abs(), 1e-9)

    def test_evaluates_at_roots_of_unity(self) -> None:
        y = fft([Complex(x, 0.0) for x in (1.0, 2.0, 3.0, 4.0)], False)
        want = [(10.0, 0.0), (-2.0, -2.0), (-2.0, 0.0), (-2.0, 2.0)]
        for g, w in zip(y, want):
            self.assertLess(abs(g.re - w[0]), 1e-12)
            self.assertLess(abs(g.im - w[1]), 1e-12)


if __name__ == "__main__":
    print(demo(), end="")
