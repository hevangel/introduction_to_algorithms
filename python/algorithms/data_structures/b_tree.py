"""B-Trees — CLRS 18 (3e, 4e) / 19 (1e, 2e) (Editions: 1,2,3,4).

Disk-friendly multiway search trees: split-on-the-way-down insertion and full
deletion with merge/borrow.
"""

from __future__ import annotations

from bisect import bisect_left
from dataclasses import dataclass, field
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass
class _Node:
    keys: list[int] = field(default_factory=list)
    children: list[_Node] = field(default_factory=list)
    leaf: bool = True

    def min_key(self) -> int:
        return self.keys[0] if self.leaf else self.children[0].min_key()

    def max_key(self) -> int:
        return self.keys[-1] if self.leaf else self.children[-1].max_key()


def _split_child(x: _Node, i: int, t: int) -> None:
    y = x.children[i]
    z = _Node(keys=y.keys[t:], children=[], leaf=y.leaf)
    median = y.keys[t - 1]
    y.keys = y.keys[: t - 1]
    if not y.leaf:
        z.children = y.children[t:]
        y.children = y.children[:t]
    x.keys.insert(i, median)
    x.children.insert(i + 1, z)


def _insert_nonfull(x: _Node, k: int, t: int) -> None:
    i = bisect_left(x.keys, k)
    if x.leaf:
        x.keys.insert(i, k)
    else:
        if len(x.children[i].keys) == 2 * t - 1:
            _split_child(x, i, t)
            if k > x.keys[i]:
                i += 1
        _insert_nonfull(x.children[i], k, t)


def _merge(x: _Node, i: int) -> None:
    right = x.children.pop(i + 1)
    sep = x.keys.pop(i)
    left = x.children[i]
    left.keys.append(sep)
    left.keys.extend(right.keys)
    left.children.extend(right.children)


def _borrow_from_prev(x: _Node, i: int) -> None:
    sib = x.children[i - 1]
    k = sib.keys.pop()
    c = None if sib.leaf else sib.children.pop()
    sep = x.keys[i - 1]
    x.keys[i - 1] = k
    child = x.children[i]
    child.keys.insert(0, sep)
    if c is not None:
        child.children.insert(0, c)


def _borrow_from_next(x: _Node, i: int) -> None:
    sib = x.children[i + 1]
    k = sib.keys.pop(0)
    c = None if sib.leaf else sib.children.pop(0)
    sep = x.keys[i]
    x.keys[i] = k
    child = x.children[i]
    child.keys.append(sep)
    if c is not None:
        child.children.append(c)


def _delete(x: _Node, k: int, t: int) -> bool:
    i = bisect_left(x.keys, k)
    if i < len(x.keys) and x.keys[i] == k:
        if x.leaf:
            x.keys.pop(i)
            return True
        if len(x.children[i].keys) >= t:
            pred = x.children[i].max_key()
            x.keys[i] = pred
            return _delete(x.children[i], pred, t)
        if len(x.children[i + 1].keys) >= t:
            succ = x.children[i + 1].min_key()
            x.keys[i] = succ
            return _delete(x.children[i + 1], succ, t)
        _merge(x, i)
        return _delete(x.children[i], k, t)
    if x.leaf:
        return False
    if len(x.children[i].keys) < t:
        if i > 0 and len(x.children[i - 1].keys) >= t:
            _borrow_from_prev(x, i)
        elif i < len(x.keys) and len(x.children[i + 1].keys) >= t:
            _borrow_from_next(x, i)
        elif i < len(x.keys):
            _merge(x, i)
        else:
            _merge(x, i - 1)
            i -= 1
    return _delete(x.children[i], k, t)


class BTree:
    """B-tree of minimum degree `t`: every non-root node has between t-1 and 2t-1 keys."""

    def __init__(self, t: int) -> None:
        assert t >= 2
        self.t = t
        self.root = _Node()
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def contains(self, k: int) -> bool:
        """B-TREE-SEARCH."""
        x = self.root
        while True:
            i = bisect_left(x.keys, k)
            if i < len(x.keys) and x.keys[i] == k:
                return True
            if x.leaf:
                return False
            x = x.children[i]

    def insert(self, k: int) -> bool:
        """B-TREE-INSERT: split full nodes on the way down."""
        if self.contains(k):
            return False
        if len(self.root.keys) == 2 * self.t - 1:
            old = self.root
            self.root = _Node(keys=[], children=[old], leaf=False)
            _split_child(self.root, 0, self.t)
        _insert_nonfull(self.root, k, self.t)
        self._len += 1
        return True

    def delete(self, k: int) -> bool:
        """B-TREE-DELETE."""
        removed = _delete(self.root, k, self.t)
        if not self.root.keys and not self.root.leaf:
            self.root = self.root.children.pop(0)
        if removed:
            self._len -= 1
        return removed

    def height(self) -> int:
        h = 0
        x = self.root
        while not x.leaf:
            h += 1
            x = x.children[0]
        return h

    def inorder(self) -> list[int]:
        out: list[int] = []

        def rec(x: _Node) -> None:
            for i, k in enumerate(x.keys):
                if not x.leaf:
                    rec(x.children[i])
                out.append(k)
            if not x.leaf:
                rec(x.children[-1])

        rec(self.root)
        return out

    def validate(self) -> bool:
        """Check key counts, ordering, and uniform leaf depth."""

        def rec(x: _Node, is_root: bool, lo: int | None, hi: int | None) -> int | None:
            if len(x.keys) > 2 * self.t - 1 or (not is_root and len(x.keys) < self.t - 1):
                return None
            if any(x.keys[i] >= x.keys[i + 1] for i in range(len(x.keys) - 1)):
                return None
            if x.keys and lo is not None and x.keys[0] <= lo:
                return None
            if x.keys and hi is not None and x.keys[-1] >= hi:
                return None
            if x.leaf:
                return 0 if not x.children else None
            if len(x.children) != len(x.keys) + 1:
                return None
            depth: int | None = None
            for i, c in enumerate(x.children):
                clo = lo if i == 0 else x.keys[i - 1]
                chi = hi if i == len(x.keys) else x.keys[i]
                d = rec(c, False, clo, chi)
                if d is None:
                    return None
                if depth is not None and depth != d:
                    return None
                depth = d
            return None if depth is None else depth + 1

        if not self.root.keys and self.root.leaf:
            return True
        return rec(self.root, True, None, None) is not None


def demo() -> str:
    lines: list[str] = []
    t = BTree(3)
    for k in range(1, 21):
        t.insert(k)
    lines.append(
        f"B-tree (t=3) with keys 1..=20: height {t.height()}, valid {str(t.validate()).lower()}"
    )
    lines.append(
        f"contains 7? {str(t.contains(7)).lower()}   contains 25? {str(t.contains(25)).lower()}"
    )
    for k in [6, 13, 7, 4, 2, 16]:
        t.delete(k)
    lines.append(f"after deleting 6,13,7,4,2,16: {t.inorder()}")
    lines.append(f"valid {str(t.validate()).lower()}, height {t.height()}")
    big = BTree(50)
    for k in range(20_000):
        big.insert((k * 7919) % 1_000_003)
    lines.append(
        f"t=50 with 20,000 keys: height {big.height()} -> at most {big.height() + 1} node reads per search"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_btreeset_for_several_degrees(self) -> None:
        for t in [2, 3, 5, 8]:
            b = BTree(t)
            s: set[int] = set()
            r = Rng(77 + t)
            for step in range(4000):
                k = r.range(0, 400)
                if r.range(0, 5) < 3:
                    before = k in s
                    s.add(k)
                    self.assertEqual(b.insert(k), not before)
                else:
                    before = k in s
                    s.discard(k)
                    self.assertEqual(b.delete(k), before)
                self.assertEqual(b.contains(k), k in s)
                if step % 13 == 0:
                    self.assertTrue(b.validate(), f"t={t} step={step}")
            self.assertEqual(b.inorder(), sorted(s))
            self.assertEqual(len(b), len(s))

    def test_delete_everything_collapses_height(self) -> None:
        b = BTree(2)
        for k in range(200):
            b.insert(k)
        self.assertGreaterEqual(b.height(), 3)
        for k in range(200):
            self.assertTrue(b.delete(k))
            self.assertTrue(b.validate())
        self.assertTrue(b.is_empty())
        self.assertEqual(b.height(), 0)

    def test_height_bound(self) -> None:
        t = 4
        b = BTree(t)
        n = 5000
        for k in range(n):
            b.insert(k)
        self.assertLessEqual(b.height(), math.log((n + 1) / 2, t) + 1e-9)


if __name__ == "__main__":
    print(demo(), end="")
