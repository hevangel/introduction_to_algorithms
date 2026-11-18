"""Amortized Analysis: Stack with MULTIPOP — CLRS 17.1–17.3 (2e–3e) / 16 (4e) / 18 (1e) (1e,2e,3e,4e).

PUSH, POP and MULTIPOP(k) all cost O(1) amortized: a stack object pays for its own removal.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest
from typing import Generic, TypeVar

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

T = TypeVar("T")


class MultipopStack(Generic[T]):
    """Stack with the extra MULTIPOP operation. Costs count elementary push/pop steps."""

    def __init__(self) -> None:
        self._items: list[T] = []
        self.total_cost: int = 0
        self.operations: int = 0

    def __len__(self) -> int:
        return len(self._items)

    def is_empty(self) -> bool:
        return not self._items

    def push(self, x: T) -> None:
        self._items.append(x)
        self.total_cost += 1
        self.operations += 1

    def pop(self) -> T | None:
        self.operations += 1
        self.total_cost += 1
        return self._items.pop() if self._items else None

    def multipop(self, k: int) -> int:
        """MULTIPOP(S, k): pop min(|S|, k) objects, actual cost = number popped."""
        self.operations += 1
        c = 0
        while c < k and self._items:
            self._items.pop()
            c += 1
        self.total_cost += c
        return c


def demo() -> str:
    lines: list[str] = []
    s: MultipopStack[int] = MultipopStack()
    r = Rng(4)
    n = 10_000
    worst_single = 0
    for _ in range(n):
        if r.range(0, 2) in (0, 1):
            s.push(0)
        else:
            c = s.multipop(r.range(1, 50))
            worst_single = max(worst_single, c)
    lines.append(
        f"{s.operations} operations: total actual cost = {s.total_cost}, "
        f"average = {s.total_cost / s.operations:.3f} (≤ 2 by amortized analysis)"
    )
    lines.append(f"most expensive single MULTIPOP = {worst_single}, yet the average is O(1)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_total_cost_at_most_twice_the_operations(self) -> None:
        r = Rng(116)
        for _ in range(20):
            s: MultipopStack[int] = MultipopStack()
            for i in range(2000):
                op = r.range(0, 3)
                if op in (0, 1):
                    s.push(i)
                elif op == 2:
                    s.pop()
                else:
                    s.multipop(r.range(0, 30))
            self.assertLessEqual(s.total_cost, 2 * s.operations, f"{s.total_cost} > 2*{s.operations}")

    def test_multipop_semantics(self) -> None:
        s: MultipopStack[int] = MultipopStack()
        for i in range(5):
            s.push(i)
        self.assertEqual(s.multipop(3), 3)
        self.assertEqual(s.multipop(10), 2)
        self.assertTrue(s.is_empty())
        self.assertIsNone(s.pop())


if __name__ == "__main__":
    print(demo(), end="")
