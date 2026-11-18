"""Dynamic Binary Search — CLRS Problem 17-2 (1e–4e) (1e,2e,3e,4e).

Keep ceil(lg(n+1)) sorted arrays of sizes 2^i (like a binary counter): SEARCH in O(lg^2 n), INSERT in O(lg n) amortised, DELETE too.
"""

from __future__ import annotations

from bisect import bisect_left
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng, is_sorted


def _merge(a: list[int], b: list[int]) -> list[int]:
    i = j = 0
    out: list[int] = []
    la, lb = len(a), len(b)
    while i < la and j < lb:
        if a[i] <= b[j]:
            out.append(a[i])
            i += 1
        else:
            out.append(b[j])
            j += 1
    if i < la:
        out.extend(a[i:])
    if j < lb:
        out.extend(b[j:])
    return out


class DynamicSet:
    """The logarithmic method: array A_i is empty or sorted with 2^i elements per binary bits of n."""

    def __init__(self) -> None:
        self._arrays: list[list[int]] = []
        self._n: int = 0

    def __len__(self) -> int:
        return self._n

    def is_empty(self) -> bool:
        return self._n == 0

    def sizes(self) -> list[int]:
        return [len(a) for a in self._arrays]

    def search(self, x: int) -> tuple[int, int] | None:
        """SEARCH: binary-search each non-empty array, smallest to largest: O(lg^2 n)."""
        for i, a in enumerate(self._arrays):
            if a:
                p = bisect_left(a, x)
                if p < len(a) and a[p] == x:
                    return i, p
        return None

    def contains(self, x: int) -> bool:
        return self.search(x) is not None

    def insert(self, x: int) -> None:
        """INSERT: carry like a binary counter — merge A_0, A_1, ... until an empty slot is found."""
        carry = [x]
        i = 0
        while True:
            if i == len(self._arrays):
                self._arrays.append([])
            if not self._arrays[i]:
                self._arrays[i] = carry
                break
            carry = _merge(self._arrays[i], carry)
            self._arrays[i].clear()
            i += 1
        self._n += 1

    def delete(self, x: int) -> bool:
        """DELETE: remove x from A_i, refill hole from smallest non-empty array A_j, split rest."""
        found = self.search(x)
        if found is None:
            return False
        i, pos = found
        j = next(idx for idx, a in enumerate(self._arrays) if a)
        self._arrays[i].pop(pos)
        if i != j:
            y = self._arrays[j].pop()
            p = bisect_left(self._arrays[i], y)
            self._arrays[i].insert(p, y)
        rest = self._arrays[j]
        self._arrays[j] = []
        start = 0
        for k in range(j):
            length = 1 << k
            self._arrays[k] = rest[start : start + length]
            start += length
        self._n -= 1
        return True

    def validate(self) -> bool:
        """Invariant: |A_i| in {0, 2^i}, A_i non-empty iff bit i of n is set, and every array is sorted."""
        for i, a in enumerate(self._arrays):
            if not is_sorted(a):
                return False
            expected = (1 << i) if ((self._n >> i) & 1) else 0
            if len(a) != expected:
                return False
        return True


def demo() -> str:
    lines: list[str] = []
    s = DynamicSet()
    for x in (5, 9, 1, 7, 3, 8, 2):
        s.insert(x)
        lines.append(f"insert {x}: array sizes {s.sizes()}")
    lines.append(f"search(7) = {s.search(7)}, search(4) = {s.search(4)}")
    s.delete(9)
    lines.append(f"after delete(9): sizes {s.sizes()}, contains 9? {s.contains(9)}")
    non_empty = sum(1 for x in s.sizes() if x > 0)
    lines.append(f"n = {len(s)} stored in {non_empty} arrays — the binary representation of n")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_a_set_and_keeps_the_binary_counter_invariant(self) -> None:
        s = DynamicSet()
        m: set[int] = set()
        r = Rng(336)
        for _ in range(4000):
            x = r.range(0, 300)
            if r.range(0, 2) in (0, 1):
                if x not in m:
                    m.add(x)
                    s.insert(x)
            else:
                expected = x in m
                if expected:
                    m.remove(x)
                self.assertEqual(s.delete(x), expected)
            self.assertTrue(s.validate())
            self.assertEqual(len(s), len(m))
            self.assertEqual(s.contains(x), x in m)

    def test_insert_sizes_follow_n_in_binary(self) -> None:
        s = DynamicSet()
        for k in range(1, 38):
            s.insert(k * 7 % 101)
            non_empty = sum(1 for x in s.sizes() if x > 0)
            self.assertEqual(non_empty, k.bit_count())


if __name__ == "__main__":
    print(demo(), end="")
