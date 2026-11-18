"""AVL Trees — CLRS Problem 13-3 (1e-4e) (Editions: 1,2,3,4).

The original height-balanced BST: |height(left) - height(right)| <= 1 restored by rotations.
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


@dataclass
class _Node:
    key: int
    h: int = 1
    l: _Node | None = None
    r: _Node | None = None


def _h(n: _Node | None) -> int:
    return n.h if n is not None else 0


def _update(n: _Node) -> None:
    n.h = 1 + max(_h(n.l), _h(n.r))


def _balance(n: _Node) -> int:
    return _h(n.l) - _h(n.r)


def _rotate_right(n: _Node) -> _Node:
    l = n.l
    assert l is not None
    n.l = l.r
    _update(n)
    l.r = n
    _update(l)
    return l


def _rotate_left(n: _Node) -> _Node:
    r = n.r
    assert r is not None
    n.r = r.l
    _update(n)
    r.l = n
    _update(r)
    return r


def _rebalance(n: _Node) -> _Node:
    _update(n)
    b = _balance(n)
    if b > 1:
        assert n.l is not None
        if _balance(n.l) < 0:
            n.l = _rotate_left(n.l)
        return _rotate_right(n)
    if b < -1:
        assert n.r is not None
        if _balance(n.r) > 0:
            n.r = _rotate_right(n.r)
        return _rotate_left(n)
    return n


def _insert(n: _Node | None, key: int) -> tuple[_Node, bool]:
    if n is None:
        return _Node(key=key), True
    if key < n.key:
        c, added = _insert(n.l, key)
        n.l = c
    elif key > n.key:
        c, added = _insert(n.r, key)
        n.r = c
    else:
        return n, False
    return _rebalance(n), added


def _take_min(n: _Node) -> tuple[_Node | None, int]:
    if n.l is None:
        return n.r, n.key
    rest, m = _take_min(n.l)
    n.l = rest
    return _rebalance(n), m


def _delete(n: _Node | None, key: int) -> tuple[_Node | None, bool]:
    if n is None:
        return None, False
    if key < n.key:
        c, removed = _delete(n.l, key)
        n.l = c
    elif key > n.key:
        c, removed = _delete(n.r, key)
        n.r = c
    else:
        if n.l is None:
            return n.r, True
        if n.r is None:
            return n.l, True
        rest, m = _take_min(n.r)
        n.key = m
        n.r = rest
        removed = True
    return _rebalance(n), removed


class AvlTree:
    """An AVL tree storing a set of distinct int keys."""

    def __init__(self) -> None:
        self._root: _Node | None = None
        self._len: int = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def height(self) -> int:
        return _h(self._root)

    def insert(self, key: int) -> bool:
        self._root, added = _insert(self._root, key)
        if added:
            self._len += 1
        return added

    def delete(self, key: int) -> bool:
        self._root, removed = _delete(self._root, key)
        if removed:
            self._len -= 1
        return removed

    def contains(self, key: int) -> bool:
        x = self._root
        while x is not None:
            if key == x.key:
                return True
            x = x.l if key < x.key else x.r
        return False

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
        """Verify ordering, stored heights, and balance condition everywhere."""

        def rec(n: _Node | None, lo: int | None, hi: int | None) -> int | None:
            if n is None:
                return 0
            if (lo is not None and n.key <= lo) or (hi is not None and n.key >= hi):
                return None
            a = rec(n.l, lo, n.key)
            b = rec(n.r, n.key, hi)
            if a is None or b is None:
                return None
            if abs(a - b) > 1 or n.h != 1 + max(a, b):
                return None
            return n.h

        return rec(self._root, None, None) is not None


def demo() -> str:
    lines: list[str] = []
    t = AvlTree()
    for k in range(1, 16):
        t.insert(k)
    lines.append(
        f"inserting 1..15 in order gives height {t.height()} (perfectly balanced would be 4)"
    )
    big = AvlTree()
    for k in range(20_000):
        big.insert(k)
    lines.append(
        f"20000 ascending keys: height = {big.height()} (AVL bound 1.4405*lg(n+2) ~= {1.4405 * math.log2(20_002):.1f})"
    )
    t.delete(8)
    lines.append(f"delete(8): {t.inorder()}, valid = {str(t.validate()).lower()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_random_operations_preserve_balance(self) -> None:
        t = AvlTree()
        s: set[int] = set()
        r = Rng(75)
        for step in range(5000):
            k = r.range(0, 250)
            if r.range(0, 2) < 2:
                before = k in s
                s.add(k)
                self.assertEqual(t.insert(k), not before)
            else:
                before = k in s
                s.discard(k)
                self.assertEqual(t.delete(k), before)
            self.assertEqual(t.contains(k), k in s)
            if step % 11 == 0:
                self.assertTrue(t.validate())
        self.assertEqual(t.inorder(), sorted(s))
        self.assertEqual(len(t), len(s))

    def test_height_is_logarithmic_for_sorted_input(self) -> None:
        t = AvlTree()
        for k in range(20_000):
            t.insert(k)
        self.assertLessEqual(t.height(), 1.4405 * math.log2(20_002))
        self.assertTrue(t.validate())


if __name__ == "__main__":
    print(demo(), end="")
