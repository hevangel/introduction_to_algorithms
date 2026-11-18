"""Iterative FFT (Bit-Reversal Butterflies) — CLRS 30.3 (1e–3e) (1, 2, 3).

BIT-REVERSE-COPY then lg n in-place butterfly passes: the loop structure used by hardware FFT circuits.
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


def rev(k: int, bits: int) -> int:
    """rev(k): reverse the lowest `bits` bits of k."""
    out = 0
    for _ in range(bits):
        out = (out << 1) | (k & 1)
        k >>= 1
    return out


def bit_reverse_copy(a: list[Complex]) -> list[Complex]:
    """BIT-REVERSE-COPY(a, A)."""
    n = len(a)
    bits = (n & -n).bit_length() - 1
    if bits == 0:
        return list(a)
    return [a[rev(k, bits)] for k in range(n)]


def iterative_fft(a: list[Complex], invert: bool = False) -> list[Complex]:
    """ITERATIVE-FFT(a): in-place, Theta(n lg n); invert gives the inverse transform."""
    n = len(a)
    assert n > 0 and (n & (n - 1)) == 0
    x = bit_reverse_copy(a)
    sign = -1.0 if invert else 1.0
    m = 2
    while m <= n:
        wm = Complex.cis(sign * 2.0 * math.pi / m)
        for k in range(0, n, m):
            w = Complex(1.0, 0.0)
            for j in range(m // 2):
                t = w * x[k + j + m // 2]
                u = x[k + j]
                x[k + j] = u + t
                x[k + j + m // 2] = u - t
                w = w * wm
        m *= 2
    if invert:
        s = 1.0 / n
        x = [v.scale(s) for v in x]
    return x


def _recursive_fft(a: list[Complex], invert: bool = False) -> list[Complex]:
    def rec(arr: list[Complex], sign: float) -> list[Complex]:
        n = len(arr)
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


def demo() -> str:
    lines: list[str] = []
    lines.append(f"bit-reversal permutation for n = 8: {[rev(k, 3) for k in range(8)]}")
    a = [Complex(float(x), 0.0) for x in range(1, 9)]
    y = iterative_fft(a, False)
    lines.append(f"iterative FFT of 1..8 (re parts): {[round(c.re * 1e6) / 1e6 for c in y]}")
    same = _recursive_fft(a, False)
    lines.append(f"equals recursive FFT: {all((p - q).abs() < 1e-9 for p, q in zip(y, same))}")
    lines.append("passes: lg n = 3, each with n/2 = 4 butterflies → 12 butterflies total")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_bit_reversal(self) -> None:
        self.assertEqual([rev(k, 3) for k in range(8)], [0, 4, 2, 6, 1, 5, 3, 7])

    def test_equals_recursive_fft_and_inverts(self) -> None:
        r = Rng(201)
        for k in range(10):
            n = 1 << k
            a = [Complex(r.f64(), r.f64()) for _ in range(n)]
            it = iterative_fft(a, False)
            rc = _recursive_fft(a, False)
            for j in range(n):
                self.assertLess((it[j] - rc[j]).abs(), 1e-8)
            back = iterative_fft(it, True)
            for j in range(n):
                self.assertLess((back[j] - a[j]).abs(), 1e-8)


if __name__ == "__main__":
    print(demo(), end="")
