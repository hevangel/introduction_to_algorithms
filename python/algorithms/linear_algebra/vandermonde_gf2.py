"""Vandermonde Matrices & GF(2) Linear Permutations — CLRS Problems D-1 & D-2 (3e–4e) (3, 4).

Exact rational determinant matching Π(x_k - x_j) for Vandermonde matrices, plus invertible GF(2) linear matrix bijections on b-bit words.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def vandermonde_matrix(xs: list[int]) -> list[list[int]]:
    """Problem D-1: Build the n x n Vandermonde matrix V_{ij} = x_i^j (0 <= i, j < n)."""
    n = len(xs)
    v = [[0] * n for _ in range(n)]
    for i, x in enumerate(xs):
        p = 1
        for j in range(n):
            v[i][j] = p
            p *= x
    return v


def exact_det(mat: list[list[int]]) -> int:
    """Exact integer determinant via Gaussian elimination in Q."""
    n = len(mat)
    a = [[Fraction(val) for val in row] for row in mat]
    det = Fraction(1)
    for k in range(n):
        piv = next((i for i in range(k, n) if a[i][k] != 0), None)
        if piv is None:
            return 0
        if piv != k:
            a[k], a[piv] = a[piv], a[k]
            det = -det
        pivot = a[k][k]
        det *= pivot
        for i in range(k + 1, n):
            if a[i][k] == 0:
                continue
            factor = a[i][k] / pivot
            for j in range(k, n):
                a[i][j] -= factor * a[k][j]
    assert det.denominator == 1
    return det.numerator


def vandermonde_det_formula(xs: list[int]) -> int:
    """Closed-form Vandermonde product: prod_{0 <= j < k < n} (x_k - x_j)."""
    n = len(xs)
    prod = 1
    for j in range(n):
        for k in range(j + 1, n):
            prod *= xs[k] - xs[j]
    return prod


@dataclass(eq=True)
class Gf2Matrix:
    """Problem D-2: b x b binary matrix over GF(2) acting on b-bit integers as linear permutations."""

    b: int
    rows: list[int]

    def __post_init__(self) -> None:
        assert self.b <= 32 and len(self.rows) == self.b
        mask = (1 << self.b) - 1
        self.rows = [r & mask for r in self.rows]

    def apply(self, x: int) -> int:
        """Apply M * x mod 2 where bits of x form a column vector (bit 0 at LSB)."""
        out = 0
        for i, row in enumerate(self.rows):
            if (row & x).bit_count() & 1:
                out |= 1 << i
        return out

    def inverse(self) -> Gf2Matrix | None:
        """Compute M^{-1} in GL_b(F_2) by Gauss-Jordan elimination over GF(2), or None if singular."""
        b = self.b
        left = list(self.rows)
        right = [1 << i for i in range(b)]
        for col in range(b):
            piv = next((r for r in range(col, b) if (left[r] >> col) & 1), None)
            if piv is None:
                return None
            left[col], left[piv] = left[piv], left[col]
            right[col], right[piv] = right[piv], right[col]
            for r in range(b):
                if r != col and ((left[r] >> col) & 1):
                    left[r] ^= left[col]
                    right[r] ^= right[col]
        return Gf2Matrix(b, right)

    @classmethod
    def bit_reversal(cls, b: int) -> Gf2Matrix:
        return cls(b, [1 << (b - 1 - i) for i in range(b)])

    @classmethod
    def gray_encode_matrix(cls, b: int) -> Gf2Matrix:
        """Matrix for Gray encoding G(x) = x ^ (x >> 1)."""
        rows: list[int] = []
        for i in range(b):
            r = 1 << i
            if i + 1 < b:
                r |= 1 << (i + 1)
            rows.append(r)
        return cls(b, rows)


def demo() -> str:
    lines: list[str] = []
    xs = [1, 2, 4, 7]
    v = vandermonde_matrix(xs)
    det_ge = exact_det(v)
    det_f = vandermonde_det_formula(xs)
    lines.append(f"Vandermonde xs={xs}: exact_det = {det_ge}, product formula = {det_f}")

    g = Gf2Matrix.gray_encode_matrix(4)
    g_inv = g.inverse()
    assert g_inv is not None
    mapped = [g.apply(x) for x in range(16)]
    back = [g_inv.apply(y) for y in mapped]
    lines.append(f"GF(2) 4-bit Gray matrix output: {mapped}")
    lines.append(f"GF(2) inverse recovers identity? {back == list(range(16))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_vandermonde_determinant_matches_product_formula_exactly(self) -> None:
        r = Rng(3001)
        for n in range(1, 7):
            for _ in range(10):
                xs = [r.range(0, 12) - 6 for _ in range(n)]
                v = vandermonde_matrix(xs)
                self.assertEqual(exact_det(v), vandermonde_det_formula(xs))

    def test_gf2_invertible_matrices_induce_bijections_and_singular_ones_collide(self) -> None:
        r = Rng(3002)
        for b in range(1, 8):
            rev = Gf2Matrix.bit_reversal(b)
            self.assertEqual(rev.inverse(), rev)
            gray = Gf2Matrix.gray_encode_matrix(b)
            gray_inv = gray.inverse()
            assert gray_inv is not None

            dom = 1 << b
            seen = [False] * dom
            for x in range(dom):
                y = gray.apply(x)
                self.assertEqual(y, x ^ (x >> 1))
                self.assertEqual(gray_inv.apply(y), x)
                seen[y] = True
            self.assertTrue(all(seen))

        b = 5
        dom = 1 << b
        for _ in range(40):
            rows = [r.next_u64() & 31 for _ in range(b)]
            m = Gf2Matrix(b, rows)
            imgs = [m.apply(x) for x in range(dom)]
            unique_imgs = set(imgs)
            inv = m.inverse()
            if inv is not None:
                self.assertEqual(len(unique_imgs), dom)
                for x in range(dom):
                    self.assertEqual(inv.apply(m.apply(x)), x)
            else:
                self.assertLess(len(unique_imgs), dom)


if __name__ == "__main__":
    print(demo(), end="")
