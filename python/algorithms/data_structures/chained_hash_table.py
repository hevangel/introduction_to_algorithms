"""Hash Tables with Chaining — CLRS 11.2 (1e-4e) (Editions: 1,2,3,4).

Resolve collisions with linked lists per slot; expected Theta(1+alpha) per operation
under simple uniform hashing.
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
_MASK64 = (1 << 64) - 1


class ChainedHashTable(Generic[V]):
    """Hash table with collision resolution by chaining. Keys are unsigned 64-bit ints."""

    def __init__(self, m: int) -> None:
        self._slots: list[list[tuple[int, V]]] = [[] for _ in range(m)]
        self._n: int = 0

    def _h(self, k: int) -> int:
        return (((k * 0x9E37_79B9_7F4A_7C15) & _MASK64) >> 32) % len(self._slots)

    def __len__(self) -> int:
        return self._n

    def is_empty(self) -> bool:
        return self._n == 0

    def load_factor(self) -> float:
        """Load factor alpha = n/m."""
        return self._n / len(self._slots)

    def insert(self, k: int, v: V) -> None:
        """CHAINED-HASH-INSERT: Theta(1) — insert at head of chain."""
        i = self._h(k)
        self._slots[i].insert(0, (k, v))
        self._n += 1

    def search(self, k: int) -> V | None:
        """CHAINED-HASH-SEARCH: expected Theta(1 + alpha)."""
        for key, val in self._slots[self._h(k)]:
            if key == k:
                return val
        return None

    def delete(self, k: int) -> V | None:
        """CHAINED-HASH-DELETE."""
        chain = self._slots[self._h(k)]
        for idx, (key, val) in enumerate(chain):
            if key == k:
                chain.pop(idx)
                self._n -= 1
                return val
        return None

    def chain_lengths(self) -> list[int]:
        return [len(c) for c in self._slots]


def demo() -> str:
    lines: list[str] = []
    t: ChainedHashTable[int] = ChainedHashTable(8)
    for k in range(1, 21):
        t.insert(k * 37, k)
    lines.append(f"n = {len(t)}, m = 8, load factor alpha = {t.load_factor():.2f}")
    lines.append(f"chain lengths: {t.chain_lengths()}")
    lines.append(f"search(37*5) = {t.search(37 * 5)}, search(1) = {t.search(1)}")
    lines.append(f"delete(37*5) = {t.delete(37 * 5)}, then search = {t.search(37 * 5)}")
    r = Rng(9)
    for alpha in [0.5, 1.0, 2.0, 4.0]:
        m = 1000
        big: ChainedHashTable[None] = ChainedHashTable(m)
        n = int(alpha * m)
        for _ in range(n):
            big.insert(r.next_u64(), None)
        lens = big.chain_lengths()
        mx = max(lens)
        empty = sum(1 for l in lens if l == 0)
        lines.append(f"alpha={alpha:>3}: longest chain = {mx}, empty slots = {empty}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_behaves_like_a_map(self) -> None:
        t: ChainedHashTable[int] = ChainedHashTable(13)
        m: dict[int, int] = {}
        r = Rng(53)
        for i in range(3000):
            k = r.range(0, 200)
            op = r.range(0, 2)
            if op == 0:
                if k not in m:
                    t.insert(k, i)
                    m[k] = i
            elif op == 1:
                self.assertEqual(t.delete(k), m.pop(k, None))
            else:
                self.assertEqual(t.search(k), m.get(k))
        self.assertEqual(len(t), len(m))

    def test_chains_are_short_for_uniform_keys(self) -> None:
        t: ChainedHashTable[None] = ChainedHashTable(1000)
        r = Rng(54)
        for _ in range(1000):
            t.insert(r.next_u64(), None)
        self.assertLessEqual(max(t.chain_lengths()), 10)


if __name__ == "__main__":
    print(demo(), end="")
