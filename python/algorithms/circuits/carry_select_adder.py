"""Carry-Select Adder — CLRS 29.2 (1e) (companion extension) (Editions: 1).

Compute each block's sum twice (carry-in 0 and 1) in parallel and let the real incoming
carry select the right one: delay O(sqrt(n)) with blocks of size sqrt(n).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _ripple(a: list[bool], b: list[bool], cin: bool) -> tuple[list[bool], bool]:
    c = cin
    s: list[bool] = []
    for ai, bi in zip(a, b):
        s.append(ai ^ bi ^ c)
        c = (ai and bi) or (c and (ai ^ bi))
    return s, c


def carry_select_add(a: list[bool], b: list[bool], k: int) -> tuple[list[bool], bool, int]:
    """Carry-select addition with blocks of `k` bits.

    Returns (sum bits, carry out, critical-path delay in gate delays).
    """
    assert len(a) == len(b)
    n = len(a)
    total: list[bool] = []
    carry = False
    delay = 0
    start = 0
    while start < n:
        end = min(start + k, n)
        blk_a, blk_b = a[start:end], b[start:end]
        if start == 0:
            s, c = _ripple(blk_a, blk_b, False)
            total.extend(s)
            carry = c
            delay = 2 * (end - start)
        else:
            s0, c0 = _ripple(blk_a, blk_b, False)
            s1, c1 = _ripple(blk_a, blk_b, True)
            s, c = (s1, c1) if carry else (s0, c0)
            total.extend(s)
            carry = c
            delay = max(delay, 2 * (end - start)) + 1
        start = end
    return total, carry, delay


def to_bits(x: int, n: int) -> list[bool]:
    return [((x >> i) & 1) == 1 for i in range(n)]


def from_bits(b: list[bool]) -> int:
    return sum((1 << i) for i, x in enumerate(b) if x)


def demo() -> str:
    lines: list[str] = []
    n = 32
    a, b = to_bits(0xDEAD_BEEF, n), to_bits(0x1234_5678, n)
    for k in [1, 4, 6, 8, 32]:
        s, c, d = carry_select_add(a, b, k)
        lines.append(
            f"block size {k:>2}: sum = {hex(from_bits(s))}, carry out = {str(c).lower()}, delay = {d} gate delays (ripple would be {2 * n})"
        )
    lines.append(f"0xDEADBEEF + 0x12345678 = {hex(0xDEAD_BEEF + 0x1234_5678)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_integer_addition_for_every_block_size(self) -> None:
        r = Rng(435)
        for _ in range(300):
            n = r.range(1, 40)
            mask = (1 << n) - 1
            x = r.next_u64() & mask
            y = r.next_u64() & mask
            for k in [1, 2, 3, 5, 8, 64]:
                s, c, _ = carry_select_add(to_bits(x, n), to_bits(y, n), k)
                self.assertEqual(from_bits(s) | (int(c) << n), x + y, f"n={n} k={k}")

    def test_delay_beats_ripple_with_square_root_blocks(self) -> None:
        n = 64
        a = to_bits((1 << 64) - 1, n)
        b = to_bits(1, n)
        _, _, d_sqrt = carry_select_add(a, b, 8)
        _, _, d_ripple = carry_select_add(a, b, n)
        self.assertLess(d_sqrt, d_ripple)


if __name__ == "__main__":
    print(demo(), end="")
