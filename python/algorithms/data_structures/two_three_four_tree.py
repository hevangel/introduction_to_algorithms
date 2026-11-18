"""2-3-4 Trees — CLRS Problem 18-2 (2e-4e) (Editions: 2,3,4).

B-trees of minimum degree 2: every node has 1–3 keys; top-down insertion splits full 3-key nodes on the way down (a single pass, O(lg n)).
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


class _Node:
    __slots__ = ("keys", "kids")

    def __init__(self, keys: list[int] | None = None, kids: list[_Node] | None = None) -> None:
        self.keys: list[int] = keys if keys is not None else []
        self.kids: list[_Node] = kids if kids is not None else []

    def leaf(self) -> bool:
        return not self.kids

    def split_child(self, i: int) -> None:
        """Split the full (3-key) child i: its middle key moves up into `self`."""
        child = self.kids[i]
        right_keys = child.keys[2:]
        del child.keys[2:]
        if child.leaf():
            right_kids: list[_Node] = []
        else:
            right_kids = child.kids[2:]
            del child.kids[2:]
        mid = child.keys.pop()
        self.keys.insert(i, mid)
        self.kids.insert(i + 1, _Node(right_keys, right_kids))


class Tree234:
    """A 2-3-4 tree of distinct integer keys."""

    def __init__(self) -> None:
        self._root = _Node()
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def contains(self, k: int) -> bool:
        n = self._root
        while True:
            i = bisect_left(n.keys, k)
            if i < len(n.keys) and n.keys[i] == k:
                return True
            if n.leaf():
                return False
            n = n.kids[i]

    def insert(self, k: int) -> bool:
        """Insert k (returns False if already present).

        One top-down pass: every full node met on the way is split first, so a split never
        propagates upward.
        """
        if self.contains(k):
            return False
        if len(self._root.keys) == 3:  # the root is full: the tree grows in height here (and only here)
            old = self._root
            self._root = _Node([], [old])
            self._root.split_child(0)
        n = self._root
        while True:
            if n.leaf():
                n.keys.insert(bisect_left(n.keys, k), k)
                break
            i = bisect_left(n.keys, k)
            if len(n.kids[i].keys) == 3:
                n.split_child(i)
                if k > n.keys[i]:
                    i += 1
            n = n.kids[i]
        self._len += 1
        return True

    def to_vec(self) -> list[int]:
        out: list[int] = []

        def walk(n: _Node) -> None:
            for i, k in enumerate(n.keys):
                if not n.leaf():
                    walk(n.kids[i])
                out.append(k)
            if not n.leaf():
                walk(n.kids[-1])

        walk(self._root)
        return out

    def height(self) -> int:
        h, n = 0, self._root
        while not n.leaf():
            n = n.kids[0]
            h += 1
        return h

    def validate(self) -> bool:
        """Check the 2-3-4 invariants: 1–3 keys per node (except an empty root leaf), sorted keys,
        key/child counts, all leaves at equal depth."""

        def go(n: _Node, is_root: bool, lo: int | None, hi: int | None) -> int | None:
            if not (1 <= len(n.keys) <= 3) and not (is_root and not n.keys and n.leaf()):
                return None
            if any(n.keys[i] >= n.keys[i + 1] for i in range(len(n.keys) - 1)):
                return None
            if lo is not None and n.keys and n.keys[0] <= lo:
                return None
            if hi is not None and n.keys and n.keys[-1] >= hi:
                return None
            if n.leaf():
                return 0
            if len(n.kids) != len(n.keys) + 1:
                return None
            depth: int | None = None
            for i, c in enumerate(n.kids):
                low = lo if i == 0 else n.keys[i - 1]
                high = hi if i == len(n.keys) else n.keys[i]
                d = go(c, False, low, high)
                if d is None:
                    return None
                if depth is not None and depth != d:
                    return None
                depth = d
            return None if depth is None else depth + 1

        return go(self._root, True, None, None) is not None


def demo() -> str:
    lines: list[str] = []
    t = Tree234()
    for k in (10, 20, 30, 40, 50, 60, 70, 80, 90, 5, 15, 25, 35):
        t.insert(k)
    lines.append(f"inserted 13 keys: {t.to_vec()}")
    lines.append(
        f"height {t.height()}, valid 2-3-4 tree: {str(t.validate()).lower()}, "
        f"contains 35: {str(t.contains(35)).lower()}, contains 36: {str(t.contains(36)).lower()}"
    )
    big = Tree234()
    for k in range(20_000):
        big.insert(k)
    lines.append(
        f"20000 sequential keys: height {big.height()} (lg n = {math.log2(20_000):.1f}), "
        f"valid: {str(big.validate()).lower()}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_behaves_like_a_sorted_set(self) -> None:
        r = Rng(444)
        for _ in range(100):
            t, s = Tree234(), set()
            for _ in range(r.range(0, 300)):
                k = r.range(0, 150)
                self.assertEqual(t.insert(k), k not in s)
                s.add(k)
            self.assertEqual(t.to_vec(), sorted(s))
            self.assertEqual(len(t), len(s))
            self.assertTrue(t.validate())
            for k in range(160):
                self.assertEqual(t.contains(k), k in s)

    def test_height_is_logarithmic_for_sorted_input(self) -> None:
        t = Tree234()
        for k in range(5000):
            t.insert(k)
            self.assertTrue(k % 500 != 0 or t.validate())
        h = t.height()
        self.assertTrue(h <= math.ceil(math.log2(5001)) and h >= 5, f"height {h}")


if __name__ == "__main__":
    print(demo(), end="")
