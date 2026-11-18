"""Clocked Circuits: Bit-Serial Arithmetic — CLRS 29.4 (1e) (Editions: 1).

Sequential circuits with registers: a one-full-adder serial adder (n cycles) and a
shift-and-add multiplier.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class BitSerialAdder:
    """Bit-serial adder: one full adder plus a carry flip-flop; processes one bit pair per clock tick, LSB first."""

    def __init__(self) -> None:
        self._carry: bool = False
        self.cycles: int = 0

    def tick(self, a: bool, b: bool) -> bool:
        """One clock tick: combinational logic computes (sum, next carry); register latches carry."""
        s = a ^ b ^ self._carry
        nxt = (a and b) or (self._carry and (a ^ b))
        self._carry = nxt
        self.cycles += 1
        return s

    def carry(self) -> bool:
        return self._carry


def serial_add(a: int, b: int, n: int) -> tuple[int, int]:
    """Add two n-bit numbers serially: n + 1 cycles (the last flushes the carry)."""
    adder = BitSerialAdder()
    out = 0
    for i in range(n):
        if adder.tick(((a >> i) & 1) == 1, ((b >> i) & 1) == 1):
            out |= 1 << i
    if adder.tick(False, False):
        out |= 1 << n
    return out, adder.cycles


def shift_add_multiply(a: int, b: int, n: int) -> tuple[int, int]:
    """Sequential shift-and-add multiplier: returns (product, cycles)."""
    acc, multiplicand, multiplier = 0, a, b
    for _ in range(n):
        if multiplier & 1:
            acc += multiplicand
        multiplicand <<= 1
        multiplier >>= 1
    return acc, n


def serial_multiply(a: int, b: int, n: int) -> tuple[int, int]:
    """Fully serial multiplier built from bit-serial adders."""
    acc, cycles = 0, 0
    mask = (1 << (2 * n)) - 1
    for i in range(n):
        if (b >> i) & 1:
            s, c = serial_add(acc, a << i, 2 * n)
            acc = s & mask
            cycles += c
    return acc, cycles


def demo() -> str:
    lines: list[str] = []
    lines.append(f"bit-serial 179 + 94 (8 bits): {serial_add(179, 94, 8)}")
    lines.append(f"shift-and-add 13 x 11 (4 bits): {shift_add_multiply(13, 11, 4)}")
    lines.append(f"fully serial 13 x 11 (4 bits): {serial_multiply(13, 11, 4)}")
    lines.append("  n | serial add cycles | shift-add cycles | serial multiply cycles (worst case)")
    for n in [4, 8, 16]:
        all_ones = (1 << n) - 1
        lines.append(
            f"{n:>3} | {serial_add(all_ones, 1, n)[1]:>17} | "
            f"{shift_add_multiply(all_ones, all_ones, n)[1]:>16} | "
            f"{serial_multiply(all_ones, all_ones, n)[1]:>6}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_serial_add_matches_integer_addition(self) -> None:
        r = Rng(174)
        for n in range(1, 21):
            m = (1 << n) - 1
            x = r.next_u64() & m
            y = r.next_u64() & m
            self.assertEqual(serial_add(x, y, n), (x + y, n + 1))

    def test_multipliers_match(self) -> None:
        r = Rng(175)
        for n in range(1, 11):
            m = (1 << n) - 1
            x = r.next_u64() & m
            y = r.next_u64() & m
            self.assertEqual(shift_add_multiply(x, y, n), (x * y, n))
            self.assertEqual(serial_multiply(x, y, n)[0], x * y)

    def test_register_holds_the_carry_between_cycles(self) -> None:
        a = BitSerialAdder()
        self.assertFalse(a.tick(True, True))
        self.assertTrue(a.carry())
        self.assertTrue(a.tick(False, False))
        self.assertFalse(a.carry())


if __name__ == "__main__":
    print(demo(), end="")
