"""Strassen's Algorithm — CLRS 4.2 (4e) / 28.2 (1e–3e).

Seven recursive half-size products instead of eight: Θ(n^lg 7) ≈ Θ(n^2.81).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Matrix = list[list[int]]


def _naive(a: Matrix, b: Matrix) -> Matrix:
    n = len(a)
    c = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            aik = a[i][k]
            for j in range(n):
                c[i][j] += aik * b[k][j]
    return c


def _add(a: Matrix, b: Matrix) -> Matrix:
    return [[p + q for p, q in zip(x, y)] for x, y in zip(a, b)]


def _sub(a: Matrix, b: Matrix) -> Matrix:
    return [[p - q for p, q in zip(x, y)] for x, y in zip(a, b)]


def _block(a: Matrix, r: int, c: int, h: int) -> Matrix:
    return [a[r + i][c : c + h] for i in range(h)]


def strassen_pow2(a: Matrix, b: Matrix, cutoff: int) -> Matrix:
    """STRASSEN for n a power of two; falls back to the triple loop below `cutoff`."""
    n = len(a)
    if n <= max(cutoff, 1):
        return _naive(a, b)
    h = n // 2
    a11, a12, a21, a22 = _block(a, 0, 0, h), _block(a, 0, h, h), _block(a, h, 0, h), _block(a, h, h, h)
    b11, b12, b21, b22 = _block(b, 0, 0, h), _block(b, 0, h, h), _block(b, h, 0, h), _block(b, h, h, h)
    p1 = strassen_pow2(a11, _sub(b12, b22), cutoff)
    p2 = strassen_pow2(_add(a11, a12), b22, cutoff)
    p3 = strassen_pow2(_add(a21, a22), b11, cutoff)
    p4 = strassen_pow2(a22, _sub(b21, b11), cutoff)
    p5 = strassen_pow2(_add(a11, a22), _add(b11, b22), cutoff)
    p6 = strassen_pow2(_sub(a12, a22), _add(b21, b22), cutoff)
    p7 = strassen_pow2(_sub(a11, a21), _add(b11, b12), cutoff)
    c11 = _add(_sub(_add(p5, p4), p2), p6)
    c12 = _add(p1, p2)
    c21 = _add(p3, p4)
    c22 = _sub(_sub(_add(p5, p1), p3), p7)
    c = [[0] * n for _ in range(n)]
    for i in range(h):
        c[i][:h] = c11[i]
        c[i][h:] = c12[i]
        c[i + h][:h] = c21[i]
        c[i + h][h:] = c22[i]
    return c


def strassen(a: Matrix, b: Matrix) -> Matrix:
    """Strassen for any square size: pads with zeros up to the next power of two."""
    n = len(a)
    if n == 0:
        return []
    m = 1 << (n - 1).bit_length()

    def pad(x: Matrix) -> Matrix:
        p = [[0] * m for _ in range(m)]
        for i in range(n):
            p[i][:n] = x[i]
        return p

    c = strassen_pow2(pad(a), pad(b), 2)
    return [c[i][:n] for i in range(n)]


def demo() -> str:
    lines: list[str] = []
    a = [[1, 3], [7, 5]]
    b = [[6, 8], [4, 2]]
    lines.append(f"A·B (Strassen) = {strassen(a, b)}")
    lines.append(f"A·B (naive)    = {_naive(a, b)}")
    r = Rng(42)
    big = [r.vec(6, -9, 9) for _ in range(6)]
    lines.append(f"6×6 padded to 8×8 matches naive: {strassen(big, big) == _naive(big, big)}")
    lines.append(f"multiplications: 8^k (definition) vs 7^k (Strassen); lg 7 ≈ {math.log2(7.0):.4f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_naive_for_many_sizes(self) -> None:
        r = Rng(9)
        for n in range(1, 13):
            a = [r.vec(n, -9, 9) for _ in range(n)]
            b = [r.vec(n, -9, 9) for _ in range(n)]
            self.assertEqual(strassen(a, b), _naive(a, b), f"n={n}")

    def test_pure_strassen_no_cutoff(self) -> None:
        r = Rng(10)
        a = [r.vec(16, -3, 3) for _ in range(16)]
        self.assertEqual(strassen_pow2(a, a, 1), _naive(a, a))


if __name__ == "__main__":
    print(demo(), end="")
