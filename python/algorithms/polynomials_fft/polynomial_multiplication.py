"""Polynomial Multiplication via FFT — CLRS 30.1–30.2 (1e–4e) (1, 2, 3, 4).

Multiply two degree-n polynomials in Θ(n lg n): evaluate with FFT, multiply pointwise, interpolate with the inverse FFT.
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
class _Complex:
    re: float
    im: float

    @classmethod
    def cis(cls, theta: float) -> _Complex:
        return cls(math.cos(theta), math.sin(theta))

    def scale(self, k: float) -> _Complex:
        return _Complex(self.re * k, self.im * k)

    def __add__(self, o: _Complex) -> _Complex:
        return _Complex(self.re + o.re, self.im + o.im)

    def __sub__(self, o: _Complex) -> _Complex:
        return _Complex(self.re - o.re, self.im - o.im)

    def __mul__(self, o: _Complex) -> _Complex:
        return _Complex(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)


def _fft(a: list[_Complex], invert: bool = False) -> list[_Complex]:
    def rec(arr: list[_Complex], sign: float) -> list[_Complex]:
        n = len(arr)
        if n == 1:
            return list(arr)
        wn = _Complex.cis(sign * 2.0 * math.pi / n)
        y0 = rec(arr[0::2], sign)
        y1 = rec(arr[1::2], sign)
        y = [_Complex(0.0, 0.0)] * n
        w = _Complex(1.0, 0.0)
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


def multiply_naive(a: list[int], b: list[int]) -> list[int]:
    """Schoolbook Theta(n^2) product of coefficient vectors (lowest degree first)."""
    if not a or not b:
        return []
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            c[i + j] += x * y
    return c


def multiply_fft(a: list[int], b: list[int]) -> list[int]:
    """Theta(n lg n) product using the FFT."""
    if not a or not b:
        return []
    length = len(a) + len(b) - 1
    n = _next_power_of_two(length)
    fa = _fft([_Complex(float(a[i]) if i < len(a) else 0.0, 0.0) for i in range(n)], False)
    fb = _fft([_Complex(float(b[i]) if i < len(b) else 0.0, 0.0) for i in range(n)], False)
    prod = [x * y for x, y in zip(fa, fb)]
    return [int(round(c.re)) for c in _fft(prod, True)[:length]]


def format_poly(c: list[int]) -> str:
    terms: list[str] = []
    for i, v in enumerate(c):
        if v == 0:
            continue
        if i == 0:
            terms.append(f"{v}")
        elif i == 1:
            terms.append(f"{v}x")
        else:
            terms.append(f"{v}x^{i}")
    return "0" if not terms else " + ".join(terms)


def demo() -> str:
    lines: list[str] = []
    a, b = [1, 2, 3], [4, 5, 6, 7]
    lines.append(f"A(x) = {format_poly(a)}")
    lines.append(f"B(x) = {format_poly(b)}")
    lines.append(f"A·B (FFT)   = {format_poly(multiply_fft(a, b))}")
    lines.append(f"A·B (naive) = {format_poly(multiply_naive(a, b))}")

    def digits(n: int) -> list[int]:
        v: list[int] = []
        while n > 0:
            v.append(n % 10)
            n //= 10
        return v

    c = multiply_fft(digits(12345), digits(6789))
    carry = 0
    for i in range(len(c)):
        c[i] += carry
        carry = c[i] // 10
        c[i] %= 10
    while carry > 0:
        c.append(carry % 10)
        carry //= 10
    lines.append(f"12345 × 6789 via digit convolution = {''.join(str(d) for d in reversed(c))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_schoolbook(self) -> None:
        r = Rng(202)
        for _ in range(40):
            na, nb = r.range(1, 40), r.range(1, 40)
            a, b = r.vec(na, -30, 30), r.vec(nb, -30, 30)
            self.assertEqual(multiply_fft(a, b), multiply_naive(a, b))
        self.assertEqual(multiply_fft([], [1]), [])

    def test_book_style_product(self) -> None:
        self.assertEqual(multiply_fft([1, 2, 3], [4, 5, 6, 7]), [4, 13, 28, 34, 32, 21])
        self.assertEqual(multiply_fft([1, 1], [1, 1]), [1, 2, 1])


if __name__ == "__main__":
    print(demo(), end="")
