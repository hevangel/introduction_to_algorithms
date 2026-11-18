"""Karatsuba Multiplication — CLRS Problem 30-1 (1e–4e) (1, 2, 3, 4).

Multiply two polynomials (or big numbers) with three half-size products instead of four: Θ(n^lg 3) = Θ(n^1.585), the divide-and-conquer step before the FFT.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _next_power_of_two(n: int) -> int:
    return 1 if n <= 1 else 1 << (n - 1).bit_length()


def _kara(a: list[int], b: list[int], mults: list[int]) -> list[int]:
    n = len(a)
    if n == 1:
        mults[0] += 1
        return [a[0] * b[0]]
    h = n // 2
    a0, a1 = a[:h], a[h:]
    b0, b1 = b[:h], b[h:]
    z0 = _kara(a0, b0, mults)
    z2 = _kara(a1, b1, mults)
    sa = [x + y for x, y in zip(a0, a1)]
    sb = [x + y for x, y in zip(b0, b1)]
    z1 = _kara(sa, sb, mults)
    r = [0] * (2 * n - 1)
    for i in range(len(z0)):
        r[i] += z0[i]
        r[i + h] += z1[i] - z0[i] - z2[i]
        r[i + 2 * h] += z2[i]
    return r


def karatsuba(a: list[int], b: list[int]) -> tuple[list[int], int]:
    """Karatsuba polynomial multiplication; returns (product, scalar multiplications used)."""
    if not a or not b:
        return ([], 0)
    n = _next_power_of_two(max(len(a), len(b)))
    pa = list(a) + [0] * (n - len(a))
    pb = list(b) + [0] * (n - len(b))
    mults = [0]
    r = _kara(pa, pb, mults)
    return (r[: len(a) + len(b) - 1], mults[0])


def multiply_decimal(x: str, y: str) -> str:
    """Multiply two non-negative decimal numbers given as strings using Karatsuba."""
    dx = [ord(c) - ord("0") for c in reversed(x)]
    dy = [ord(c) - ord("0") for c in reversed(y)]
    prod, _ = karatsuba(dx, dy)
    carry = 0
    for i in range(len(prod)):
        prod[i] += carry
        carry = prod[i] // 10
        prod[i] %= 10
    while carry > 0:
        prod.append(carry % 10)
        carry //= 10
    while len(prod) > 1 and prod[-1] == 0:
        prod.pop()
    return "".join(str(d) for d in reversed(prod))


def demo() -> str:
    lines: list[str] = []
    p, m = karatsuba([1, 2, 3, 4], [5, 6, 7, 8])
    lines.append(f"(1+2x+3x²+4x³)(5+6x+7x²+8x³) = {p} using {m} multiplications (schoolbook: 16)")
    for k in (4, 8, 12):
        n = 1 << k
        lines.append(
            f"n = {n:>5}: Karatsuba {3**k:>9} multiplications, schoolbook {n * n:>9} (ratio {(n * n) / (3**k):.1f})"
        )
    lines.append(
        f"123456789123456789 × 987654321987654321 = {multiply_decimal('123456789123456789', '987654321987654321')}"
    )
    return "\n".join(lines) + "\n"


def _naive(a: list[int], b: list[int]) -> list[int]:
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] += x * y
    return r


class Tests(unittest.TestCase):
    def test_matches_schoolbook_for_all_shapes(self) -> None:
        rng = Rng(448)
        for _ in range(200):
            la, lb = rng.range(1, 40), rng.range(1, 40)
            a, b = rng.vec(la, -50, 50), rng.vec(lb, -50, 50)
            self.assertEqual(karatsuba(a, b)[0], _naive(a, b))

    def test_uses_exactly_three_to_the_k_multiplications(self) -> None:
        for k in range(9):
            n = 1 << k
            self.assertEqual(karatsuba([1] * n, [2] * n)[1], 3**k)

    def test_decimal_multiplication_matches_int(self) -> None:
        rng = Rng(449)
        for _ in range(200):
            x = rng.next_u64() >> 4
            y = rng.next_u64() >> 4
            self.assertEqual(multiply_decimal(str(x), str(y)), str(x * y))
        self.assertEqual(multiply_decimal("0", "12345"), "0")


if __name__ == "__main__":
    print(demo(), end="")
