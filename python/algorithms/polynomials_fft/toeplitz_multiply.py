"""Toeplitz Matrix-Vector Product — CLRS Problem 30-2 (2e–4e) (2, 3, 4).

An n×n Toeplitz matrix has constant diagonals; its product with a vector is one polynomial multiplication, so it takes O(n lg n) instead of O(n²).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

P = 998_244_353
_G = 3


def _next_power_of_two(n: int) -> int:
    return 1 if n <= 1 else 1 << (n - 1).bit_length()


def _ntt(a: list[int], invert: bool = False) -> None:
    n = len(a)
    j = 0
    for i in range(1, n):
        bit = n >> 1
        while j & bit:
            j ^= bit
            bit >>= 1
        j ^= bit
        if i < j:
            a[i], a[j] = a[j], a[i]
    length = 2
    while length <= n:
        w = pow(_G, (P - 1) // length, P)
        if invert:
            w = pow(w, P - 2, P)
        for start in range(0, n, length):
            wk = 1
            half = length // 2
            for k in range(half):
                u = a[start + k]
                v = (a[start + k + half] * wk) % P
                a[start + k] = (u + v) % P
                a[start + k + half] = (u + P - v) % P
                wk = (wk * w) % P
        length <<= 1
    if invert:
        ninv = pow(n, P - 2, P)
        for i in range(n):
            a[i] = (a[i] * ninv) % P


def _multiply_mod(a: list[int], b: list[int]) -> list[int]:
    if not a or not b:
        return []
    need = len(a) + len(b) - 1
    size = _next_power_of_two(need)
    fa = list(a) + [0] * (size - len(a))
    fb = list(b) + [0] * (size - len(b))
    _ntt(fa, False)
    _ntt(fb, False)
    for i in range(size):
        fa[i] = (fa[i] * fb[i]) % P
    _ntt(fa, True)
    return fa[:need]


def toeplitz_multiply(t: list[int], x: list[int]) -> list[int]:
    """T·x for Toeplitz matrix T[i][j] = t[i - j + n - 1] (t has 2n - 1 entries) mod P."""
    n = len(x)
    assert len(t) == 2 * n - 1
    c = _multiply_mod(t, x)
    return [c[i + n - 1] % P for i in range(n)]


def toeplitz_multiply_naive(t: list[int], x: list[int]) -> list[int]:
    """The O(n^2) definition, for comparison."""
    n = len(x)
    return [sum(t[i + n - 1 - j] * x[j] for j in range(n)) % P for i in range(n)]


def demo() -> str:
    lines: list[str] = []
    t = [5, 4, 1, 2, 3]
    x = [1, 10, 100]
    lines.append(f"Toeplitz T = [[1,2,3],[4,1,2],[5,4,1]], x = {x}")
    lines.append(f"T·x via polynomial multiplication = {toeplitz_multiply(t, x)}")
    lines.append(f"T·x by the definition             = {toeplitz_multiply_naive(t, x)}")
    r = Rng(3)
    n = 2000
    tt = [r.range(0, 1000) for _ in range(2 * n - 1)]
    xx = [r.range(0, 1000) for _ in range(n)]
    lines.append(f"n = {n}: fast product equals O(n²) product: {toeplitz_multiply(tt, xx) == toeplitz_multiply_naive(tt, xx)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_equals_naive_product(self) -> None:
        r = Rng(421)
        for _ in range(100):
            n = r.range(1, 30)
            t = [r.next_u64() % P for _ in range(2 * n - 1)]
            x = [r.next_u64() % P for _ in range(n)]
            self.assertEqual(toeplitz_multiply(t, x), toeplitz_multiply_naive(t, x))

    def test_identity_like_matrix(self) -> None:
        n = 7
        t = [0] * (2 * n - 1)
        t[n - 1] = 1
        x = list(range(1, n + 1))
        self.assertEqual(toeplitz_multiply(t, x), x)


if __name__ == "__main__":
    print(demo(), end="")
