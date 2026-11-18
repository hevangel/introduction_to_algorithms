"""Searching a Sorted Compact List — CLRS Problem 10-3 (1e-4e) (Editions: 1,2,3,4).

A sorted linked list in array form can be searched in expected O(sqrt(n)) probes by
random sampling, beating the Theta(n) scan.
"""

from __future__ import annotations

from bisect import bisect_left
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NIL = -1


class CompactList:
    """Sorted linked list stored in arrays (`key`, `next`) with arbitrary physical order."""

    def __init__(self, key: list[int], next_ptr: list[int], head: int) -> None:
        self._key = key
        self._next = next_ptr
        self._head = head

    @classmethod
    def from_sorted(cls, keys: list[int], rng: Rng) -> CompactList:
        """Store the sorted keys at randomly permuted array positions."""
        n = len(keys)
        pos = list(range(n))
        rng.shuffle(pos)
        key = [0] * n
        next_ptr = [NIL] * n
        for i, k in enumerate(keys):
            key[pos[i]] = k
            if i + 1 < n:
                next_ptr[pos[i]] = pos[i + 1]
        return cls(key, next_ptr, pos[0] if n > 0 else NIL)

    def __len__(self) -> int:
        return len(self._key)

    def is_empty(self) -> bool:
        return len(self._key) == 0

    def search_linear(self, k: int) -> tuple[bool, int]:
        """COMPACT-LIST-SEARCH: walk from the head. Returns (found, probes). Theta(n)."""
        i = self._head
        probes = 0
        key = self._key
        nxt = self._next
        while i != NIL and key[i] < k:
            i = nxt[i]
            probes += 1
        return (i != NIL and key[i] == k, probes + 1)

    def search_randomized(self, k: int, t: int, rng: Rng) -> tuple[bool, int]:
        """COMPACT-LIST-SEARCH': sample t random slots, jump to largest < k, then walk."""
        if self.is_empty():
            return (False, 0)
        i = self._head
        probes = 0
        key = self._key
        nxt = self._next
        n_minus_1 = len(self._key) - 1
        for _ in range(t):
            j = rng.range(0, n_minus_1)
            probes += 1
            if key[i] < key[j] <= k:
                i = j
                if key[i] == k:
                    return (True, probes)
        while i != NIL and key[i] < k:
            i = nxt[i]
            probes += 1
        return (i != NIL and key[i] == k, probes + 1)


def demo() -> str:
    lines: list[str] = []
    r = Rng(3)
    for n in [100, 10_000, 100_000]:
        keys = [2 * x for x in range(n)]
        l = CompactList.from_sorted(keys, r)
        t = int(math.sqrt(n))
        trials = 100
        lin = 0
        rnd = 0
        for _ in range(trials):
            k = 2 * r.range(0, n - 1)
            lin += l.search_linear(k)[1]
            rnd += l.search_randomized(k, t, r)[1]
        lines.append(
            f"n = {n:>7}: linear scan {lin / trials:>8.0f} probes on average; "
            f"randomized with t = sqrt(n) = {t:>4}: {rnd / trials:>6.0f} probes"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_searches_are_correct(self) -> None:
        r = Rng(333)
        for n in [0, 1, 2, 17, 300]:
            keys = [3 * x for x in range(n)]
            l = CompactList.from_sorted(keys, r)
            for q in range(-2, 3 * n + 3):
                idx = bisect_left(keys, q)
                want = idx < len(keys) and keys[idx] == q
                self.assertEqual(l.search_linear(q)[0], want)
                self.assertEqual(l.search_randomized(q, 8, r)[0], want, f"n={n} q={q}")

    def test_randomized_search_probes_about_sqrt_n(self) -> None:
        r = Rng(334)
        n = 40_000
        keys = list(range(n))
        l = CompactList.from_sorted(keys, r)
        lin = sum(l.search_linear(r.range(0, n - 1))[1] for _ in range(300)) / 300.0
        rnd = sum(l.search_randomized(r.range(0, n - 1), 200, r)[1] for _ in range(300)) / 300.0
        self.assertGreater(lin, 15_000.0)
        self.assertLess(rnd, 4.0 * 200.0 + 100.0)


if __name__ == "__main__":
    print(demo(), end="")
