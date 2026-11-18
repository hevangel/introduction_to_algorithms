"""Parallel Matrix Multiplication — CLRS 27.2 (3e) / 26.2 (4e) (Editions: 3,4).

P-MAT-VEC and P-SQUARE-MATRIX-MULTIPLY with parallel loops; recursive
divide-and-conquer with spawn.
"""

from __future__ import annotations

import os
from pathlib import Path
import sys
import threading
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Matrix = list[list[int]]


def _workers(n: int) -> int:
    cpus = os.cpu_count() or 2
    return min(max(cpus, 2), 8, max(n, 1))


def _serial(a: Matrix, b: Matrix) -> Matrix:
    n = len(a)
    c = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            aik = a[i][k]
            bk = b[k]
            ci = c[i]
            for j in range(n):
                ci[j] += aik * bk[j]
    return c


def p_mat_vec(a: Matrix, x: list[int]) -> list[int]:
    """P-MAT-VEC: y = A * x with the outer loop run in parallel."""
    n = len(a)
    y = [0] * n
    w = _workers(n)
    chunk = max((n + w - 1) // w, 1)

    def run_chunk(start: int, end: int) -> None:
        for i in range(start, end):
            y[i] = sum(p * q for p, q in zip(a[i], x))

    threads: list[threading.Thread] = []
    for start in range(0, n, chunk):
        end = min(start + chunk, n)
        t = threading.Thread(target=run_chunk, args=(start, end))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()
    return y


def p_matrix_multiply(a: Matrix, b: Matrix) -> Matrix:
    """P-SQUARE-MATRIX-MULTIPLY: parallel-for over rows."""
    n = len(a)
    c = [[0] * n for _ in range(n)]
    w = _workers(n)
    chunk = max((n + w - 1) // w, 1)

    def run_chunk(start: int, end: int) -> None:
        for i in range(start, end):
            ci = c[i]
            ai = a[i]
            for kk in range(n):
                aik = ai[kk]
                bk = b[kk]
                for j in range(n):
                    ci[j] += aik * bk[j]

    threads: list[threading.Thread] = []
    for start in range(0, n, chunk):
        end = min(start + chunk, n)
        t = threading.Thread(target=run_chunk, args=(start, end))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()
    return c


def _block(m: Matrix, r: int, c: int, h: int) -> Matrix:
    return [m[r + i][c : c + h] for i in range(h)]


def p_matrix_multiply_recursive(a: Matrix, b: Matrix) -> Matrix:
    """P-MATRIX-MULTIPLY-RECURSIVE for n a power of two: 8 half-size products as parallel tasks."""
    n = len(a)
    if n <= 32:
        return _serial(a, b)
    h = n // 2

    def quadrants(m: Matrix) -> list[Matrix]:
        return [
            _block(m, 0, 0, h),
            _block(m, 0, h, h),
            _block(m, h, 0, h),
            _block(m, h, h, h),
        ]

    aq, bq = quadrants(a), quadrants(b)
    prods: list[Matrix | None] = [None] * 8

    def run_task(t: int) -> None:
        i, j, k = t // 4, (t // 2) % 2, t % 2
        prods[t] = p_matrix_multiply_recursive(aq[i * 2 + k], bq[k * 2 + j])

    threads = [threading.Thread(target=run_task, args=(t,)) for t in range(8)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    c = [[0] * n for _ in range(n)]
    for t in range(8):
        i, j = t // 4, (t // 2) % 2
        pt = prods[t]
        assert pt is not None
        for r in range(h):
            for col in range(h):
                c[i * h + r][j * h + col] += pt[r][col]
    return c


def demo() -> str:
    lines: list[str] = []
    rng = Rng(5)
    n = 64
    a = [rng.vec(n, -9, 9) for _ in range(n)]
    b = [rng.vec(n, -9, 9) for _ in range(n)]
    want = _serial(a, b)
    lines.append(
        f"n = {n}: parallel-for result equals serial: {str(p_matrix_multiply(a, b) == want).lower()}"
    )
    lines.append(
        f"n = {n}: recursive spawn result equals serial: {str(p_matrix_multiply_recursive(a, b) == want).lower()}"
    )
    x = rng.vec(n, -5, 5)
    lines.append(f"P-MAT-VEC first entries: {p_mat_vec(a, x)[:4]}")
    lines.append(
        "work Theta(n^3); span Theta(n) for the parallel-for version (inner loops serial), Theta(lg n) with parallel inner loops"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_versions_agree(self) -> None:
        r = Rng(150)
        for n in [1, 3, 8, 33, 64]:
            a = [r.vec(n, -9, 9) for _ in range(n)]
            b = [r.vec(n, -9, 9) for _ in range(n)]
            want = _serial(a, b)
            self.assertEqual(p_matrix_multiply(a, b), want, f"n={n}")
            if n & (n - 1) == 0:
                self.assertEqual(p_matrix_multiply_recursive(a, b), want, f"n={n}")

    def test_mat_vec(self) -> None:
        r = Rng(151)
        n = 37
        a = [r.vec(n, -9, 9) for _ in range(n)]
        x = r.vec(n, -9, 9)
        want = [sum(p * q for p, q in zip(row, x)) for row in a]
        self.assertEqual(p_mat_vec(a, x), want)


if __name__ == "__main__":
    print(demo(), end="")
