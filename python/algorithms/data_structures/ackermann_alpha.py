"""The Ackermann Function and its Inverse alpha(n) — CLRS 21.4 (1e-4e) (Editions: 1,2,3,4).

A_k(j) grows faster than any primitive recursive function; alpha(n), its inverse, is <= 4
for every n below 2^2048 — the amortised cost per union-find operation.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

U128_MAX = (1 << 128) - 1


def ackermann(k: int, j: int) -> int | None:
    """A_k(j) of Section 21.4. Returns None if the value exceeds u128."""
    if k == 0:
        v = j + 1
        return v if v <= U128_MAX else None
    if k == 1:
        v = j * 2 + 1
        return v if v <= U128_MAX else None
    if k == 2:
        if j + 1 >= 128:
            return None
        v = (1 << (j + 1)) * (j + 1) - 1
        return v if v <= U128_MAX else None
    x: int | None = j
    for _ in range(j + 1):
        assert x is not None
        x = ackermann(k - 1, x)
        if x is None:
            return None
    return x


def ackermann_by_definition(k: int, j: int) -> int:
    """Literal definition by iteration (no closed forms), usable for tiny arguments only."""
    if k == 0:
        return j + 1
    x = j
    for _ in range(j + 1):
        x = ackermann_by_definition(k - 1, x)
    return x


def alpha(n: int) -> int:
    """alpha(n) = min { k : A_k(1) >= n }."""
    k = 0
    while True:
        v = ackermann(k, 1)
        if v is None or v >= n:
            return k
        k += 1


def demo() -> str:
    lines: list[str] = []
    for k in range(4):
        lines.append(f"A_{k}(1) = {ackermann(k, 1)}")
    lines.append(f"A_4(1) = {ackermann(4, 1)}  (= 2^2048 - 3 ... far beyond u128)")
    for n in [1, 2, 3, 7, 8, 2047, 2048, U128_MAX]:
        lines.append(f"alpha({n}) = {alpha(n)}")
    lines.append(f"A_2(3) = {ackermann(2, 3)}, A_3(2) = {ackermann(3, 2)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_closed_forms_agree_with_the_definition(self) -> None:
        for j in range(6):
            self.assertEqual(ackermann(1, j), ackermann_by_definition(1, j))
            self.assertEqual(ackermann(2, j), ackermann_by_definition(2, j))
        self.assertEqual(ackermann_by_definition(3, 1), 2047)
        self.assertEqual(ackermann(3, 1), 2047)
        self.assertEqual((ackermann(0, 1), ackermann(1, 1), ackermann(2, 1)), (2, 3, 7))

    def test_inverse_thresholds(self) -> None:
        table = [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 1),
            (4, 2),
            (7, 2),
            (8, 3),
            (2047, 3),
            (2048, 4),
            (U128_MAX, 4),
        ]
        for n, a in table:
            self.assertEqual(alpha(n), a, f"alpha({n})")
        self.assertIsNone(ackermann(4, 1))
        self.assertIsNone(ackermann(3, 2))


if __name__ == "__main__":
    print(demo(), end="")
