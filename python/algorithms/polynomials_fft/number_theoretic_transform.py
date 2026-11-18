"""Number-Theoretic Transform (FFT over a Finite Field) — CLRS Problem 30-6 (1e–4e) (1, 2, 3, 4).

The DFT works over any ring with a principal n-th root of unity: modulo p = 998244353 the transform and convolution are exact integer arithmetic, no rounding errors.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

P = 998_244_353  # 119 * 2^23 + 1, primitive root 3
_G = 3


def _pow_mod(b: int, e: int) -> int:
    return pow(b, e, P)


def _next_power_of_two(n: int) -> int:
    return 1 if n <= 1 else 1 << (n - 1).bit_length()


def ntt(a: list[int], invert: bool = False) -> None:
    """In-place iterative NTT of a power-of-two length array (Cooley-Tukey with bit reversal)."""
    n = len(a)
    assert n > 0 and (n & (n - 1)) == 0 and n <= (1 << 23)
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
        w = _pow_mod(_G, (P - 1) // length)
        if invert:
            w = _pow_mod(w, P - 2)
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
        ninv = _pow_mod(n, P - 2)
        for i in range(n):
            a[i] = (a[i] * ninv) % P


def multiply_mod(a: list[int], b: list[int]) -> list[int]:
    """Product of two polynomials with coefficients in Z_P, exactly, in O(n lg n)."""
    if not a or not b:
        return []
    need = len(a) + len(b) - 1
    size = _next_power_of_two(need)
    fa = list(a) + [0] * (size - len(a))
    fb = list(b) + [0] * (size - len(b))
    ntt(fa, False)
    ntt(fb, False)
    for i in range(size):
        fa[i] = (fa[i] * fb[i]) % P
    ntt(fa, True)
    return fa[:need]


def demo() -> str:
    lines: list[str] = []
    a, b = [1, 2, 3], [4, 5]
    lines.append(f"(1 + 2x + 3x²)(4 + 5x) mod {P} = {multiply_mod(a, b)}")
    r = Rng(5)
    n = 1 << 12
    x = [r.range(0, 999_999) for _ in range(n)]
    y = [r.range(0, 999_999) for _ in range(n)]
    z = multiply_mod(x, y)
    k = 1000
    check = sum((x[i] * y[k - i]) % P for i in range(k + 1) if i < n and k - i < n) % P
    lines.append(
        f"two degree-{n - 1} polynomials with huge coefficients: coefficient {k} of the product = {z[k]} (direct sum {check})"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_convolution_equals_schoolbook_modulo_p(self) -> None:
        r = Rng(419)
        for _ in range(100):
            la, lb = r.range(1, 40), r.range(1, 40)
            a = [r.next_u64() % P for _ in range(la)]
            b = [r.next_u64() % P for _ in range(lb)]
            want = [0] * (la + lb - 1)
            for i in range(la):
                for j in range(lb):
                    want[i + j] = (want[i + j] + a[i] * b[j]) % P
            self.assertEqual(multiply_mod(a, b), want)

    def test_transform_roundtrip_and_root_of_unity(self) -> None:
        r = Rng(420)
        for lg in range(10):
            a = [r.next_u64() % P for _ in range(1 << lg)]
            b = list(a)
            ntt(b, False)
            ntt(b, True)
            self.assertEqual(a, b)
        w = _pow_mod(_G, (P - 1) // 8)
        self.assertEqual((_pow_mod(w, 8), _pow_mod(w, 4)), (1, P - 1))


if __name__ == "__main__":
    print(demo(), end="")
