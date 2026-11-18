"""Treaps — CLRS Problem 13-4 (1e-4e) (Editions: 1,2,3,4).

BST on keys, heap on random priorities: a randomly built tree regardless of insertion order.
"""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_U64_MAX = (1 << 64) - 1


@dataclass
class _Node:
    key: int
    pri: int
    l: _Node | None = None
    r: _Node | None = None


def _rotate_right(n: _Node) -> _Node:
    l = n.l
    assert l is not None
    n.l = l.r
    l.r = n
    return l


def _rotate_left(n: _Node) -> _Node:
    r = n.r
    assert r is not None
    n.r = r.l
    r.l = n
    return r


def _insert(n: _Node | None, key: int, pri: int) -> _Node:
    if n is None:
        return _Node(key=key, pri=pri)
    if key < n.key:
        n.l = _insert(n.l, key, pri)
        if n.l.pri > n.pri:
            return _rotate_right(n)
    elif key > n.key:
        n.r = _insert(n.r, key, pri)
        if n.r.pri > n.pri:
            return _rotate_left(n)
    return n


def _remove_root(n: _Node) -> _Node | None:
    """Rotate the root down until it becomes a leaf, then cut it off."""
    if n.l is None and n.r is None:
        return None
    if n.l is not None and n.r is None:
        return n.l
    if n.l is None and n.r is not None:
        return n.r
    assert n.l is not None and n.r is not None
    if n.l.pri > n.r.pri:
        root = _rotate_right(n)
        assert root.r is not None
        root.r = _remove_root(root.r)
        return root
    root = _rotate_left(n)
    assert root.l is not None
    root.l = _remove_root(root.l)
    return root


def _delete(n: _Node | None, key: int) -> tuple[_Node | None, bool]:
    if n is None:
        return None, False
    if key < n.key:
        c, d = _delete(n.l, key)
        n.l = c
        return n, d
    if key > n.key:
        c, d = _delete(n.r, key)
        n.r = c
        return n, d
    return _remove_root(n), True


class Treap:
    """A treap: a binary search tree on `key` that is simultaneously a max-heap on random `priority`."""

    def __init__(self, seed: int) -> None:
        self._root: _Node | None = None
        self._len = 0
        self._rng = Rng(seed)

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def contains(self, key: int) -> bool:
        x = self._root
        while x is not None:
            if key == x.key:
                return True
            x = x.l if key < x.key else x.r
        return False

    def insert(self, key: int) -> bool:
        if self.contains(key):
            return False
        pri = self._rng.next_u64()
        self._root = _insert(self._root, key, pri)
        self._len += 1
        return True

    def delete(self, key: int) -> bool:
        r, d = _delete(self._root, key)
        self._root = r
        if d:
            self._len -= 1
        return d

    def height(self) -> int:
        def h(n: _Node | None) -> int:
            return 0 if n is None else 1 + max(h(n.l), h(n.r))

        return h(self._root)

    def inorder(self) -> list[int]:
        out: list[int] = []

        def rec(n: _Node | None) -> None:
            if n is not None:
                rec(n.l)
                out.append(n.key)
                rec(n.r)

        rec(self._root)
        return out

    def validate(self) -> bool:
        """Check BST order on keys and heap order on priorities."""

        def rec(n: _Node | None, lo: int | None, hi: int | None, parent_pri: int) -> bool:
            if n is None:
                return True
            if (lo is not None and n.key <= lo) or (hi is not None and n.key >= hi) or n.pri > parent_pri:
                return False
            return rec(n.l, lo, n.key, n.pri) and rec(n.r, n.key, hi, n.pri)

        return rec(self._root, None, None, _U64_MAX)


def demo() -> str:
    lines: list[str] = []
    t = Treap(1)
    for k in range(10_000):
        t.insert(k)
    lines.append(
        f"10000 keys inserted in ascending order: height = {t.height()} "
        f"(a plain BST would have height 9999; 2·lg n ≈ {2.0 * math.log2(10_000):.0f})"
    )
    lines.append(f"valid treap: {str(t.validate()).lower()}")
    for k in range(0, 10_000, 2):
        t.delete(k)
    lines.append(
        f"after deleting even keys: size = {len(t)}, height = {t.height()}, valid = {str(t.validate()).lower()}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_behaves_like_a_set(self) -> None:
        t = Treap(5)
        s: set[int] = set()
        r = Rng(76)
        for _ in range(4000):
            k = r.range(0, 300)
            if r.range(0, 2) < 2:
                self.assertEqual(t.insert(k), k not in s)
                s.add(k)
            else:
                self.assertEqual(t.delete(k), k in s)
                s.discard(k)
            self.assertEqual(t.contains(k), k in s)
        self.assertTrue(t.validate())
        self.assertEqual(t.inorder(), sorted(s))

    def test_sorted_input_still_gives_logarithmic_height(self) -> None:
        t = Treap(6)
        for k in range(5000):
            t.insert(k)
        self.assertLess(t.height(), 60, t.height())


if __name__ == "__main__":
    print(demo(), end="")
