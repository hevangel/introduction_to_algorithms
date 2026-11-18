"""Modular Linear Equations — CLRS 31.4 (1e–4e) (1, 2, 3, 4).

Solve ax ≡ b (mod n): d = gcd(a, n) solutions mod n if d | b, none otherwise.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def _extended_euclid(a: int, b: int) -> tuple[int, int, int]:
    if b == 0:
        return (a, 1, 0)
    d, x, y = _extended_euclid(b, a % b)
    return (d, y, x - (a // b) * y)


def solve(a: int, b: int, n: int) -> list[int]:
    """MODULAR-LINEAR-EQUATION-SOLVER(a, b, n): all solutions x in 0..n of a*x ≡ b (mod n)."""
    d, x1, _ = _extended_euclid(a % n, n)
    bn = b % n
    if bn % d != 0:
        return []
    step = n // d
    x0 = (x1 * (bn // d)) % step
    return [(x0 + i * step) % n for i in range(d)]


def demo() -> str:
    lines: list[str] = []
    lines.append(f"14x ≡ 30 (mod 100): {solve(14, 30, 100)}  (gcd = 2, so 2 solutions)")
    lines.append(f"3x ≡ 2 (mod 7):    {solve(3, 2, 7)}  (unique solution)")
    lines.append(f"4x ≡ 3 (mod 8):    {solve(4, 3, 8)}  (no solution: gcd 4 ∤ 3)")
    lines.append(f"6x ≡ 0 (mod 9):    {solve(6, 0, 9)}  (gcd 3 → 3 solutions)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        s = sorted(solve(14, 30, 100))
        self.assertEqual(s, [45, 95])

    def test_exhaustive_check(self) -> None:
        for n in range(1, 40):
            for a in range(n):
                for b in range(n):
                    want = [x for x in range(n) if (a * x - b) % n == 0]
                    got = sorted(solve(a, b, n))
                    self.assertEqual(got, want, f"{a}x ≡ {b} (mod {n})")


if __name__ == "__main__":
    print(demo(), end="")
