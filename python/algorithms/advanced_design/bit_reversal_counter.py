"""Bit-Reversed Binary Counter — CLRS Problem 17-1 (1e–4e) (1e,2e,3e,4e).

BIT-REVERSED-INCREMENT enumerates 0...2^k-1 in bit-reversed order with O(1) amortised flips, giving the FFT permutation in Theta(n).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


class BitReversedCounter:
    """A k-bit counter A[0..k) that counts in bit-reversed order by carrying downwards from the top bit."""

    def __init__(self, k: int) -> None:
        self._bits: list[bool] = [False] * k
        self.flips: int = 0

    def value(self) -> int:
        return sum((1 << i) for i, b in enumerate(self._bits) if b)

    def increment(self) -> None:
        i = len(self._bits)
        while i > 0 and self._bits[i - 1]:
            self._bits[i - 1] = False
            self.flips += 1
            i -= 1
        if i > 0:
            self._bits[i - 1] = True
            self.flips += 1


def bit_reversal_permutation(k: int) -> list[int]:
    """The bit-reversal permutation rev_k(0), ..., rev_k(2^k - 1) in Theta(2^k) total time."""
    c = BitReversedCounter(k)
    out: list[int] = []
    for _ in range(1 << k):
        out.append(c.value())
        c.increment()
    return out


def _rev(k: int, bits: int) -> int:
    """Reverse the lowest `bits` bits of k."""
    res = 0
    for i in range(bits):
        if (k >> i) & 1:
            res |= 1 << (bits - 1 - i)
    return res


def demo() -> str:
    lines: list[str] = []
    lines.append(f"bit-reversal permutation for k = 3: {bit_reversal_permutation(3)}")
    lines.append(f"k = 4: {bit_reversal_permutation(4)}")
    for k in (8, 12, 16, 20):
        c = BitReversedCounter(k)
        incs = (1 << k) - 1
        for _ in range(incs):
            c.increment()
        lines.append(
            f"k = {k:>2}: {incs} increments cost {c.flips} bit flips "
            f"({c.flips / incs:.2f} per increment, amortised O(1))"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_enumerates_the_bit_reversal_permutation(self) -> None:
        for k in range(1, 11):
            p = bit_reversal_permutation(k)
            self.assertEqual(p, [_rev(i, k) for i in range(1 << k)], f"k={k}")
        self.assertEqual(bit_reversal_permutation(3), [0, 4, 2, 6, 1, 5, 3, 7])

    def test_amortised_constant_cost(self) -> None:
        for k in (6, 10, 14):
            c = BitReversedCounter(k)
            n = 1 << k
            for _ in range(n - 1):
                c.increment()
            self.assertLess(c.flips, 2 * n, f"k={k} flips={c.flips}")


if __name__ == "__main__":
    print(demo(), end="")
