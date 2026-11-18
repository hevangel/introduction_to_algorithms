"""Amortized Analysis: Dynamic Tables — CLRS 17.4 (2e–3e) / 16.4 (4e) / 18.4 (1e) (1e,2e,3e,4e).

Doubling on overflow and halving at load 1/4 gives O(1) amortized INSERT and DELETE.
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


class DynamicTable(Generic[T]):
    """A table that grows by doubling and shrinks by halving."""

    def __init__(self) -> None:
        self._items: list[T] = []
        self._size: int = 0
        self.total_cost: int = 0
        self.operations: int = 0
        self.resizes: int = 0

    def __len__(self) -> int:
        return len(self._items)

    def is_empty(self) -> bool:
        return not self._items

    def capacity(self) -> int:
        return self._size

    def load_factor(self) -> float:
        return 1.0 if self._size == 0 else len(self._items) / self._size

    def potential(self) -> float:
        """Potential Phi(T) = 2*num - size if alpha >= 1/2, size/2 - num otherwise."""
        num = float(len(self._items))
        size = float(self._size)
        if self.load_factor() >= 0.5:
            return 2.0 * num - size
        return size / 2.0 - num

    def insert(self, x: T) -> None:
        """TABLE-INSERT."""
        self.operations += 1
        if self._size == 0:
            self._size = 1
        elif len(self._items) == self._size:
            self.total_cost += len(self._items)
            self._size *= 2
            self.resizes += 1
        self._items.append(x)
        self.total_cost += 1

    def delete(self) -> T | None:
        """TABLE-DELETE: contract to half the size when less than a quarter full."""
        self.operations += 1
        if not self._items:
            return None
        x = self._items.pop()
        self.total_cost += 1
        if 4 * len(self._items) < self._size:
            self.total_cost += len(self._items)
            self._size //= 2
            self.resizes += 1
        return x


def demo() -> str:
    lines: list[str] = []
    t: DynamicTable[int] = DynamicTable()
    lines.append("inserting 1..=17: capacity grows 1 → 2 → 4 → 8 → 16 → 32")
    for i in range(1, 18):
        t.insert(i)
        if (i & (i - 1)) == 0 or i == 17:
            lines.append(f"  after insert {i:>2}: capacity {t.capacity():>2}, cumulative cost {t.total_cost:>2}")
    big: DynamicTable[int] = DynamicTable()
    for i in range(100_000):
        big.insert(i)
    lines.append(
        f"100000 inserts: total cost {big.total_cost} = {big.total_cost / 100_000.0:.3f}·n "
        f"(bound 3n), {big.resizes} resizes"
    )
    while len(big) > 100:
        big.delete()
    lines.append(
        f"after deleting down to 100 items: capacity {big.capacity()} "
        f"(load {big.load_factor():.2f}, never below 1/4)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_inserts_cost_less_than_three_per_operation(self) -> None:
        t: DynamicTable[int] = DynamicTable()
        for i in range(10_000):
            t.insert(i)
        self.assertLessEqual(t.total_cost, 3 * t.operations)
        self.assertEqual(t.capacity(), 16384)

    def test_mixed_operations_stay_amortized_constant_and_load_factor_bounded(self) -> None:
        r = Rng(117)
        t: DynamicTable[int] = DynamicTable()
        for rnd in range(30_000):
            if r.range(0, 2) < 1 or t.is_empty():
                t.insert(rnd)
            else:
                t.delete()
            if len(t) > 0:
                self.assertGreaterEqual(t.load_factor(), 0.25 - 1e-12)
                self.assertLessEqual(t.load_factor(), 1.0)
            self.assertGreaterEqual(t.potential(), -1e-9)
        self.assertLessEqual(t.total_cost, 3 * t.operations, f"{t.total_cost} vs {3 * t.operations}")

    def test_potential_is_zero_just_after_resize(self) -> None:
        t: DynamicTable[int] = DynamicTable()
        for i in range(9):
            t.insert(i)
        self.assertEqual(t.capacity(), 16)
        self.assertEqual(t.potential(), 2.0)


if __name__ == "__main__":
    print(demo(), end="")
