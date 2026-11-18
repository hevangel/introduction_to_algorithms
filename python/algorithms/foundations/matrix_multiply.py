"""Matrix Multiplication (Naive & Recursive) — CLRS 4.2 (1e–4e, as 28.1 in 1e–3e).

Θ(n³) triple loop and the Θ(n³) divide-and-conquer version on n/2 × n/2 blocks.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Matrix = list[list[int]]


def identity(n: int) -> Matrix:
    m = [[0] * n for _ in range(n)]
    for i in range(n):
        m[i][i] = 1
    return m


def square_matrix_multiply(a: Matrix, b: Matrix) -> Matrix:
    """SQUARE-MATRIX-MULTIPLY: C = A·B by the definition, Θ(n³)."""
    n = len(a)
    c = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            total = 0
            for k in range(n):
                total += a[i][k] * b[k][j]
            c[i][j] = total
    return c


def _add_into(c: Matrix, r0: int, c0: int, x: Matrix) -> None:
    for i, row in enumerate(x):
        for j, val in enumerate(row):
            c[r0 + i][c0 + j] += val


def _block(a: Matrix, r0: int, c0: int, h: int) -> Matrix:
    return [a[r0 + i][c0 : c0 + h] for i in range(h)]


def square_matrix_multiply_recursive(a: Matrix, b: Matrix) -> Matrix:
    """SQUARE-MATRIX-MULTIPLY-RECURSIVE (n must be a power of two): eight half-size products."""
    n = len(a)
    if n == 1:
        return [[a[0][0] * b[0][0]]]
    h = n // 2
    c = [[0] * n for _ in range(n)]
    for ci, cj in [(0, 0), (0, h), (h, 0), (h, h)]:
        for k in (0, h):
            p = square_matrix_multiply_recursive(_block(a, ci, k, h), _block(b, k, cj, h))
            _add_into(c, ci, cj, p)
    return c


def demo() -> str:
    lines: list[str] = []
    a = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
    b = identity(4)
    lines.append(f"A·I (naive)     = {square_matrix_multiply(a, b)}")
    c = square_matrix_multiply(a, a)
    lines.append(f"A·A (naive)     = {c}")
    r = square_matrix_multiply_recursive(a, a)
    lines.append(f"A·A (recursive) = {r}")
    lines.append(f"results equal: {c == r}")
    return "\n".join(lines) + "\n"


def _rand_matrix(n: int, r: Rng) -> Matrix:
    return [r.vec(n, -5, 5) for _ in range(n)]


class Tests(unittest.TestCase):
    def test_recursive_matches_naive(self) -> None:
        r = Rng(7)
        for n in [1, 2, 4, 8, 16]:
            a, b = _rand_matrix(n, r), _rand_matrix(n, r)
            self.assertEqual(square_matrix_multiply_recursive(a, b), square_matrix_multiply(a, b))

    def test_identity_is_neutral(self) -> None:
        r = Rng(8)
        a = _rand_matrix(5, r)
        self.assertEqual(square_matrix_multiply(a, identity(5)), a)


if __name__ == "__main__":
    print(demo(), end="")
