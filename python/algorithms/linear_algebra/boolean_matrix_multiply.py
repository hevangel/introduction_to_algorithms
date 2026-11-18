"""Boolean Matrix Multiplication — CLRS 31.3 (1e) (1).

Multiply boolean matrices over the OR-AND quasiring by embedding into Z_{n+1} for Strassen's algorithm, plus Four-Russians bit-packing and triangle detection.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _next_power_of_two(n: int) -> int:
    if n <= 1:
        return 1
    return 1 << (n - 1).bit_length()


def _mat_naive(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    n = len(a)
    c = [[0] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            aik = a[i][k]
            if aik:
                row_b = b[k]
                row_c = c[i]
                for j in range(n):
                    row_c[j] += aik * row_b[j]
    return c


def _mat_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    return [[x + y for x, y in zip(ra, rb)] for ra, rb in zip(a, b)]


def _mat_sub(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    return [[x - y for x, y in zip(ra, rb)] for ra, rb in zip(a, b)]


def _mat_block(a: list[list[int]], r: int, c: int, h: int) -> list[list[int]]:
    return [a[r + i][c : c + h] for i in range(h)]


def _strassen_pow2(a: list[list[int]], b: list[list[int]], cutoff: int) -> list[list[int]]:
    n = len(a)
    if n <= max(cutoff, 1):
        return _mat_naive(a, b)
    h = n // 2
    a11 = _mat_block(a, 0, 0, h)
    a12 = _mat_block(a, 0, h, h)
    a21 = _mat_block(a, h, 0, h)
    a22 = _mat_block(a, h, h, h)
    b11 = _mat_block(b, 0, 0, h)
    b12 = _mat_block(b, 0, h, h)
    b21 = _mat_block(b, h, 0, h)
    b22 = _mat_block(b, h, h, h)

    p1 = _strassen_pow2(a11, _mat_sub(b12, b22), cutoff)
    p2 = _strassen_pow2(_mat_add(a11, a12), b22, cutoff)
    p3 = _strassen_pow2(_mat_add(a21, a22), b11, cutoff)
    p4 = _strassen_pow2(a22, _mat_sub(b21, b11), cutoff)
    p5 = _strassen_pow2(_mat_add(a11, a22), _mat_add(b11, b22), cutoff)
    p6 = _strassen_pow2(_mat_sub(a12, a22), _mat_add(b21, b22), cutoff)
    p7 = _strassen_pow2(_mat_sub(a11, a21), _mat_add(b11, b12), cutoff)

    c11 = _mat_add(_mat_sub(_mat_add(p5, p4), p2), p6)
    c12 = _mat_add(p1, p2)
    c21 = _mat_add(p3, p4)
    c22 = _mat_sub(_mat_sub(_mat_add(p5, p1), p3), p7)

    c = [[0] * n for _ in range(n)]
    for i in range(h):
        for j in range(h):
            c[i][j] = c11[i][j]
            c[i][j + h] = c12[i][j]
            c[i + h][j] = c21[i][j]
            c[i + h][j + h] = c22[i][j]
    return c


def bool_matmul_naive(a: list[list[bool]], b: list[list[bool]]) -> list[list[bool]]:
    """Schoolbook Theta(n^3) boolean matrix product (A * B)[i][j] = OR_k (A[i][k] & B[k][j])."""
    n = len(a)
    c = [[False] * n for _ in range(n)]
    for i in range(n):
        for k in range(n):
            if a[i][k]:
                for j in range(n):
                    c[i][j] = c[i][j] or b[k][j]
    return c


def bool_matmul_strassen(a: list[list[bool]], b: list[list[bool]]) -> list[list[bool]]:
    """Embed {0, 1} into Z, compute witness counts via Strassen in O(n^{lg 7}), and threshold > 0."""
    n = len(a)
    if n == 0:
        return []
    m = _next_power_of_two(n)
    ai = [[0] * m for _ in range(m)]
    bi = [[0] * m for _ in range(m)]
    for i in range(n):
        for j in range(n):
            ai[i][j] = int(a[i][j])
            bi[i][j] = int(b[i][j])
    ci = _strassen_pow2(ai, bi, 4)
    return [[ci[i][j] > 0 for j in range(n)] for i in range(n)]


def bool_matmul_four_russians(a: list[list[bool]], b: list[list[bool]]) -> list[list[bool]]:
    """Word-packed / Four-Russians lookup-block multiplication over 64-bit chunks."""
    n = len(a)
    if n == 0:
        return []
    words = (n + 63) // 64
    b_rows = [[0] * words for _ in range(n)]
    for i in range(n):
        for j in range(n):
            if b[i][j]:
                b_rows[i][j // 64] |= 1 << (j % 64)

    block_w = max(1, min(8, int(math.floor(math.log2(n)))))
    c_words = [[0] * words for _ in range(n)]

    k0 = 0
    while k0 < n:
        length = min(n - k0, block_w)
        table_size = 1 << length
        table = [[0] * words for _ in range(table_size)]
        for mask in range(1, table_size):
            lsb = (mask & -mask).bit_length() - 1
            prev = mask ^ (1 << lsb)
            row = b_rows[k0 + lsb]
            for w in range(words):
                table[mask][w] = table[prev][w] | row[w]
        for i in range(n):
            mask = 0
            for bit in range(length):
                if a[i][k0 + bit]:
                    mask |= 1 << bit
            if mask != 0:
                trow = table[mask]
                crow = c_words[i]
                for w in range(words):
                    crow[w] |= trow[w]
        k0 += length

    return [[((c_words[i][j // 64] >> (j % 64)) & 1) == 1 for j in range(n)] for i in range(n)]


def has_triangle(adj: list[list[bool]]) -> tuple[int, int, int] | None:
    """Exercise 31.3-6 (1e): Detect a triangle (i, j, k) in an undirected graph via A^2 & A."""
    n = len(adj)
    a2 = bool_matmul_strassen(adj, adj)
    for i in range(n):
        for j in range(i + 1, n):
            if adj[i][j] and a2[i][j]:
                for k in range(n):
                    if k != i and k != j and adj[i][k] and adj[k][j]:
                        return (i, j, k)
    return None


def demo() -> str:
    lines: list[str] = []
    a = [
        [True, False, True, False],
        [False, True, True, False],
        [False, False, True, True],
        [True, False, False, False],
    ]
    b = [
        [False, True, False, False],
        [True, False, False, True],
        [False, True, True, False],
        [True, False, True, False],
    ]
    c_naive = bool_matmul_naive(a, b)
    c_str = bool_matmul_strassen(a, b)
    c_4r = bool_matmul_four_russians(a, b)
    lines.append(
        f"4x4 boolean product matches (naive == Strassen == Four Russians): {c_naive == c_str and c_naive == c_4r}"
    )
    tri_adj = [
        [False, True, True, False],
        [True, False, True, False],
        [True, True, False, True],
        [False, False, True, False],
    ]
    lines.append(f"triangle detected via A^2 & A: {has_triangle(tri_adj)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_strassen_and_four_russians_match_naive_across_sizes(self) -> None:
        r = Rng(313)
        for n in range(1, 25):
            a = [[r.f64() < 0.4 for _ in range(n)] for _ in range(n)]
            b = [[r.f64() < 0.4 for _ in range(n)] for _ in range(n)]
            naive = bool_matmul_naive(a, b)
            self.assertEqual(bool_matmul_strassen(a, b), naive)
            self.assertEqual(bool_matmul_four_russians(a, b), naive)

    def test_triangle_detection_matches_triple_enumeration(self) -> None:
        r = Rng(314)
        for _ in range(80):
            n = r.range(3, 12)
            adj = [[False] * n for _ in range(n)]
            for i in range(n):
                for j in range(i + 1, n):
                    if r.f64() < 0.3:
                        adj[i][j] = True
                        adj[j][i] = True
            brute = any(
                adj[i][j] and adj[j][k] and adj[i][k]
                for i in range(n)
                for j in range(i + 1, n)
                for k in range(j + 1, n)
            )
            found = has_triangle(adj)
            self.assertEqual(found is not None, brute)
            if found is not None:
                i, j, k = found
                self.assertTrue(adj[i][j] and adj[j][k] and adj[i][k])


if __name__ == "__main__":
    print(demo(), end="")
