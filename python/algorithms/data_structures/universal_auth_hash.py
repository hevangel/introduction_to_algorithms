"""k-Universal Hashing and Carter–Wegman Authentication — CLRS Problem 11-4 (1e-4e) (Editions: 1,2,3,4).

Construct strongly 2-universal and k-universal degree-(k-1) polynomial hash families over GF(p) and one-time Carter-Wegman message authentication tags.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class PolyHash:
    """Degree-(k-1) polynomial hash over GF(p): h_a(x) = (sum_{j=0}^{k-1} a_j x^j) mod p.

    Uniform choice of a in Z_p^k makes this family k-universal (k-wise independent).
    """

    def __init__(self, p: int, coeffs: list[int], *, reduce: bool = True) -> None:
        if reduce:
            assert p >= 2 and len(coeffs) > 0
            coeffs = [c % p for c in coeffs]
        self.p = p
        self.coeffs = coeffs

    @classmethod
    def new(cls, p: int, coeffs: list[int]) -> PolyHash:
        return cls(p, coeffs)

    @classmethod
    def random(cls, p: int, k: int, rng: Rng) -> PolyHash:
        coeffs = [rng.next_u64() % p for _ in range(k)]
        return cls(p, coeffs, reduce=False)

    def eval(self, x: int) -> int:
        """Horner evaluation of sum_{j=0}^{k-1} a_j x^j mod p."""
        p = self.p
        xm = x % p
        acc = 0
        for c in reversed(self.coeffs):
            acc = (acc * xm + c) % p
        return acc


def strongly_2_universal(p: int, m: int, a: int, b: int, x: int) -> int:
    """Strongly 2-universal hashing into {0, ..., m-1}: ((a*x + b) mod p) mod m."""
    v = (a * (x % p) + b) % p
    return v % m


def cw_authenticate(h: PolyHash, msg: int, pad: int) -> int:
    """Carter-Wegman one-time-pad authenticated tag over GF(p): tag = (h(msg) + pad) mod p."""
    return (h.eval(msg) + pad) % h.p


def cw_verify(h: PolyHash, msg: int, pad: int, tag: int) -> bool:
    return cw_authenticate(h, msg, pad) == tag % h.p


def demo() -> str:
    lines: list[str] = []
    p = 11
    counts = [[0] * 11 for _ in range(11)]
    for a0 in range(p):
        for a1 in range(p):
            h = PolyHash.new(p, [a0, a1])
            counts[h.eval(2)][h.eval(7)] += 1
    all_one = all(c == 1 for row in counts for c in row)
    lines.append(
        f"GF({p}), k=2: across all {p}^2 = {p * p} functions, every pair (h(2), h(7)) hits once? {str(all_one).lower()}"
    )
    rng = Rng(114)
    big_p = 2_147_483_647  # Mersenne prime 2^31 - 1
    key = PolyHash.random(big_p, 3, rng)
    pad = rng.next_u64() % big_p
    tag = cw_authenticate(key, 42_195, pad)
    lines.append(
        f"Carter-Wegman tag (msg=42195, p={big_p}) = {tag}, valid={str(cw_verify(key, 42_195, pad, tag)).lower()}"
    )
    lines.append(f"tampered msg=42196 valid? {str(cw_verify(key, 42_196, pad, tag)).lower()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_exact_pairwise_uniformity_over_gf11(self) -> None:
        p = 11
        for x in range(p):
            for y in range(x + 1, p):
                joint = [[0] * p for _ in range(p)]
                for a0 in range(p):
                    for a1 in range(p):
                        h = PolyHash.new(p, [a0, a1])
                        joint[h.eval(x)][h.eval(y)] += 1
                self.assertTrue(
                    all(c == 1 for row in joint for c in row), f"failed for x={x}, y={y}"
                )

    def test_exact_3_universal_uniformity_over_gf5(self) -> None:
        p = 5
        x, y, z = 1, 2, 4
        joint = [0] * (p * p * p)
        for a0 in range(p):
            for a1 in range(p):
                for a2 in range(p):
                    h = PolyHash.new(p, [a0, a1, a2])
                    idx = h.eval(x) * p * p + h.eval(y) * p + h.eval(z)
                    joint[idx] += 1
        self.assertTrue(all(c == 1 for c in joint))

    def test_carter_wegman_auth_and_forging_uniformity(self) -> None:
        p = 11
        m1, m2 = 3, 8
        pad1, pad2 = 5, 9
        # Given observed (m1, tag1), any guessed tag2 for m2 matches with probability exactly 1/p.
        rng = Rng(77)
        key = PolyHash.random(p, 2, rng)
        t1 = cw_authenticate(key, m1, pad1)
        self.assertTrue(cw_verify(key, m1, pad1, t1))
        self.assertFalse(cw_verify(key, m1, pad1, (t1 + 1) % p))
        consistent_t2 = [0] * 11
        consistent_keys = 0
        for a0 in range(p):
            for a1 in range(p):
                cand = PolyHash.new(p, [a0, a1])
                if cw_verify(cand, m1, pad1, t1):
                    consistent_keys += 1
                    consistent_t2[cw_authenticate(cand, m2, pad2)] += 1
        self.assertEqual(consistent_keys, p)
        self.assertTrue(all(c == 1 for c in consistent_t2))


if __name__ == "__main__":
    print(demo(), end="")
