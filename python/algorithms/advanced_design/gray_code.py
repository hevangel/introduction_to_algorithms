"""Binary Reflected Gray Code — CLRS Problem 16-1 (4e) (4e).

Generate all n-bit words changing one bit per step via recursive reflection, G(i) = i ^ (i >> 1), and amortized O(1) bit-position iteration.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def gray_recursive(n: int) -> list[int]:
    """Recursive reflected construction of the n-bit binary Gray code."""
    if n == 0:
        return [0]
    half = gray_recursive(n - 1)
    msb = 1 << (n - 1)
    return half + [msb | code for code in reversed(half)]


def gray_encode(i: int) -> int:
    """Closed-form i -> Gray(i) = i ^ (i >> 1)."""
    return i ^ (i >> 1)


def gray_decode(g: int) -> int:
    """Inverse Gray(i) -> i via prefix XOR across bit positions."""
    limit = max(64, g.bit_length())
    shift = 1
    while shift < limit:
        g ^= g >> shift
        shift <<= 1
    return g


def gray_transition_positions(n: int) -> tuple[list[int], int]:
    """Bit position toggled at step i -> i + 1 for i = 0..2^n - 2, plus total inspection work."""
    if n == 0:
        return [], 0
    total_steps = (1 << n) - 1
    flips: list[int] = []
    trailing_checks = 0
    for i in range(total_steps):
        x = i + 1
        pos = (x & -x).bit_length() - 1
        trailing_checks += pos + 1
        flips.append(pos)
    return flips, trailing_checks


def demo() -> str:
    lines: list[str] = []
    g4 = gray_recursive(4)
    bits = " ".join(f"{x:04b}" for x in g4)
    lines.append(f"4-bit reflected Gray code ({len(g4)} words): {bits}")
    flips, work = gray_transition_positions(4)
    lines.append(f"toggled bit indices: {flips}")
    lines.append(
        f"total bit inspections across {len(flips)} steps = {work} ({work / (1 << 4):.2f} per step)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_recursive_matches_closed_form_and_hamming_distance_is_1(self) -> None:
        for n in range(11):
            rec = gray_recursive(n)
            self.assertEqual(len(rec), 1 << n)
            for i, val in enumerate(rec):
                self.assertEqual(val, gray_encode(i))
                self.assertEqual(gray_decode(val), i)
            for i in range(len(rec) - 1):
                self.assertEqual((rec[i] ^ rec[i + 1]).bit_count(), 1)
            if n > 0:
                self.assertEqual((rec[0] ^ rec[-1]).bit_count(), 1)

    def test_amortized_cost_is_bounded_by_2_per_step(self) -> None:
        for n in range(1, 13):
            flips, work = gray_transition_positions(n)
            cur = 0
            for step, bit in enumerate(flips):
                cur ^= 1 << bit
                self.assertEqual(cur, gray_encode(step + 1))
            self.assertLessEqual(work, 2 * (1 << n))


if __name__ == "__main__":
    print(demo(), end="")
