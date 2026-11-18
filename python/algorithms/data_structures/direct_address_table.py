"""Direct-Address Tables & Bit Vectors — CLRS 11.1 (1e-4e) (Editions: 1,2,3,4).

An array indexed by key: O(1) search/insert/delete when the universe is small;
bit-vector sets.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest
from typing import Generic, TypeVar

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

V = TypeVar("V")


class DirectAddressTable(Generic[V]):
    """DIRECT-ADDRESS table over the universe U = {0, ..., m-1}."""

    def __init__(self, universe: int) -> None:
        self._slots: list[V | None] = [None] * universe

    def search(self, k: int) -> V | None:
        return self._slots[k]

    def insert(self, k: int, v: V) -> None:
        self._slots[k] = v

    def delete(self, k: int) -> V | None:
        val = self._slots[k]
        self._slots[k] = None
        return val


class BitVectorSet:
    """Exercise 11.1-2: a dynamic set of distinct integers from {0..m-1} stored as a bit vector."""

    def __init__(self, m: int) -> None:
        self._words: list[int] = [0] * ((m + 63) // 64)
        self._m = m

    def insert(self, k: int) -> None:
        self._words[k // 64] |= 1 << (k % 64)

    def delete(self, k: int) -> None:
        self._words[k // 64] &= ~(1 << (k % 64))

    def contains(self, k: int) -> bool:
        return ((self._words[k // 64] >> (k % 64)) & 1) == 1

    def maximum(self) -> int | None:
        """Largest element, found by scanning words from the top (Exercise 11.1-1)."""
        for w in range(len(self._words) - 1, -1, -1):
            word = self._words[w]
            if word != 0:
                return w * 64 + word.bit_length() - 1
        return None

    def __len__(self) -> int:
        return sum(w.bit_count() for w in self._words)

    def is_empty(self) -> bool:
        return len(self) == 0

    def universe(self) -> int:
        return self._m


def demo() -> str:
    lines: list[str] = []
    t: DirectAddressTable[str] = DirectAddressTable(10)
    t.insert(3, "three")
    t.insert(7, "seven")
    lines.append(f"search(3) = {t.search(3)}, search(4) = {t.search(4)}")
    lines.append(f"delete(3) = {t.delete(3)}, search(3) = {t.search(3)}")
    s = BitVectorSet(1000)
    for k in [5, 63, 64, 999, 500]:
        s.insert(k)
    lines.append(
        f"bit-vector set holds {len(s)} elements, maximum = {s.maximum()}, "
        f"universe {s.universe()} bits in {(1000 + 63) // 64} words"
    )
    s.delete(999)
    lines.append(f"after deleting 999, maximum = {s.maximum()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_table_operations(self) -> None:
        t: DirectAddressTable[str] = DirectAddressTable(5)
        self.assertIsNone(t.search(2))
        t.insert(2, "x")
        self.assertEqual(t.search(2), "x")
        self.assertEqual(t.delete(2), "x")
        self.assertIsNone(t.delete(2))

    def test_bit_vector_matches_btreeset(self) -> None:
        s = BitVectorSet(300)
        b: set[int] = set()
        r = Rng(52)
        for _ in range(3000):
            k = r.range(0, 299)
            if r.range(0, 2) == 0:
                s.insert(k)
                b.add(k)
            else:
                s.delete(k)
                b.discard(k)
            self.assertEqual(s.maximum(), max(b) if b else None)
            self.assertEqual(s.contains(k), k in b)
        self.assertEqual(len(s), len(b))


if __name__ == "__main__":
    print(demo(), end="")
