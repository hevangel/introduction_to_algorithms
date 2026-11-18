"""Persistent Red-Black Trees: JOIN and SPLIT — CLRS Problems 13-1, 13-2 (1e–4e) (1,2,3,4).

Immutable red-black trees (old versions stay valid) with O(lg n) JOIN of two trees and SPLIT around a key.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Color(Enum):
    RED = auto()
    BLACK = auto()


@dataclass(frozen=True)
class Node:
    c: Color
    l: Tree
    k: int
    r: Tree
    bh: int


Tree = Node | None


def _bh(t: Tree) -> int:
    return 0 if t is None else t.bh


def _color(t: Tree) -> Color:
    return Color.BLACK if t is None else t.c


def _mk(c: Color, l: Tree, k: int, r: Tree) -> Node:
    return Node(c=c, l=l, k=k, r=r, bh=_bh(l) + (1 if c == Color.BLACK else 0))


def _blacken(t: Tree) -> Tree:
    if t is not None and t.c == Color.RED:
        return _mk(Color.BLACK, t.l, t.k, t.r)
    return t


def _join_right(tl: Tree, k: int, tr: Tree) -> Node:
    if tl is None:
        return _mk(Color.RED, None, k, tr)
    if tl.c == Color.BLACK and tl.bh == _bh(tr):
        return _mk(Color.RED, tl, k, tr)
    nr = _join_right(tl.r, k, tr)
    if tl.c == Color.BLACK and nr.c == Color.RED and _color(nr.r) == Color.RED:
        x = nr
        y = nr.r
        assert y is not None
        return _mk(Color.RED, _mk(Color.BLACK, tl.l, tl.k, x.l), x.k, _mk(Color.BLACK, y.l, y.k, y.r))
    return _mk(tl.c, tl.l, tl.k, nr)


def _join_left(tl: Tree, k: int, tr: Tree) -> Node:
    if tr is None:
        return _mk(Color.RED, tl, k, None)
    if tr.c == Color.BLACK and tr.bh == _bh(tl):
        return _mk(Color.RED, tl, k, tr)
    nl = _join_left(tl, k, tr.l)
    if tr.c == Color.BLACK and nl.c == Color.RED and _color(nl.l) == Color.RED:
        x = nl
        y = nl.l
        assert y is not None
        return _mk(Color.RED, _mk(Color.BLACK, y.l, y.k, y.r), x.k, _mk(Color.BLACK, x.r, tr.k, tr.r))
    return _mk(tr.c, nl, tr.k, tr.r)


def join(tl: Tree, k: int, tr: Tree) -> Tree:
    """RB-JOIN(T1, k, T2): every key of T1 < k < every key of T2."""
    tl = _blacken(tl)
    tr = _blacken(tr)
    if _bh(tl) > _bh(tr):
        t: Tree = _join_right(tl, k, tr)
    elif _bh(tr) > _bh(tl):
        t = _join_left(tl, k, tr)
    else:
        t = _mk(Color.RED, tl, k, tr)
    return _blacken(t)


def split(t: Tree, k: int) -> tuple[Tree, bool, Tree]:
    """RB-SPLIT: (keys < k, whether k was present, keys > k)."""
    if t is None:
        return (None, False, None)
    if k < t.k:
        a, f, b = split(t.l, k)
        return (a, f, join(b, t.k, t.r))
    if k > t.k:
        a, f, b = split(t.r, k)
        return (join(t.l, t.k, a), f, b)
    return (_blacken(t.l), True, _blacken(t.r))


def max_key(t: Tree) -> int | None:
    if t is None:
        return None
    while t.r is not None:
        t = t.r
    return t.k


def join2(a: Tree, b: Tree) -> Tree:
    m = max_key(a)
    if m is None:
        return _blacken(b)
    a2, _, _ = split(a, m)
    return join(a2, m, b)


def insert(t: Tree, k: int) -> Tree:
    a, f, b = split(t, k)
    return t if f else join(a, k, b)


def delete(t: Tree, k: int) -> Tree:
    a, _, b = split(t, k)
    return join2(a, b)


def contains(t: Tree, k: int) -> bool:
    while t is not None:
        if k == t.k:
            return True
        t = t.l if k < t.k else t.r
    return False


def inorder(t: Tree) -> list[int]:
    out: list[int] = []
    def go(x: Tree) -> None:
        if x is not None:
            go(x.l)
            out.append(x.k)
            go(x.r)
    go(t)
    return out


def height(t: Tree) -> int:
    return 0 if t is None else 1 + max(height(t.l), height(t.r))


def validate(t: Tree) -> int:
    if _color(t) != Color.BLACK:
        raise AssertionError("root is red")

    def go(n: Tree, lo: int | None, hi: int | None) -> int:
        if n is None:
            return 0
        if (lo is not None and n.k <= lo) or (hi is not None and n.k >= hi):
            raise AssertionError(f"order violated at {n.k}")
        if n.c == Color.RED and (_color(n.l) == Color.RED or _color(n.r) == Color.RED):
            raise AssertionError(f"red-red at {n.k}")
        a = go(n.l, lo, n.k)
        b = go(n.r, n.k, hi)
        if a != b:
            raise AssertionError(f"black-height mismatch at {n.k}")
        if n.bh != a + (1 if n.c == Color.BLACK else 0):
            raise AssertionError(f"stale bh at {n.k}")
        return n.bh

    return go(t, None, None)


def from_slice(keys: list[int]) -> Tree:
    t: Tree = None
    for k in keys:
        t = insert(t, k)
    return t


def demo() -> str:
    lines: list[str] = []
    t = from_slice(list(range(1, 21)))
    lines.append(f"tree of 1..=20: height {height(t)}, black-height {validate(t)}")
    l, found, r = split(t, 8)
    lines.append(f"split at 8 → left {inorder(l)}, found = {found}, right {inorder(r)}")
    validate(l)
    validate(r)
    lines.append("both halves valid red-black trees: True / True")
    big = from_slice(list(range(100, 110)))
    j = join(l, 50, big)
    lines.append(f"join(left, 50, 100..110) = {inorder(j)}")
    t2 = insert(t, 99)
    lines.append(f"persistence: old version still has {len(inorder(t))} keys, new version has {len(inorder(t2))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_insert_delete_keep_invariants_and_match_a_set(self) -> None:
        r = Rng(321)
        t: Tree = None
        s: set[int] = set()
        for _ in range(600):
            k = r.range(0, 120)
            if r.range(0, 2) < 2:
                t = insert(t, k)
                s.add(k)
            else:
                t = delete(t, k)
                s.discard(k)
            validate(t)
            self.assertEqual(contains(t, k), k in s)
        self.assertEqual(inorder(t), sorted(s))

    def test_split_and_join_roundtrip(self) -> None:
        r = Rng(322)
        for _ in range(100):
            keys = r.vec(r.range(0, 60), 0, 200)
            t = from_slice(keys)
            pivot = r.range(0, 200)
            a, f, b = split(t, pivot)
            validate(a)
            validate(b)
            self.assertEqual(f, pivot in keys)
            self.assertTrue(all(x < pivot for x in inorder(a)) and all(x > pivot for x in inorder(b)))
            back = join(a, pivot, b) if f else join2(a, b)
            if not f:
                self.assertEqual(len(inorder(back)), len(inorder(a)) + len(inorder(b)))
            else:
                self.assertEqual(inorder(back), inorder(t))
            validate(back)

    def test_join_with_wildly_different_heights(self) -> None:
        small = from_slice([2000])
        big = from_slice(list(range(1000)))
        j1 = join(big, 1500, small)
        validate(j1)
        self.assertEqual(len(inorder(j1)), 1002)
        j2 = join(from_slice([-5]), -1, big)
        validate(j2)
        self.assertLessEqual(height(j2), 2.0 * math.log2(1002) + 2.0)
        validate(join(None, 5, None))

    def test_old_versions_are_persistent(self) -> None:
        t1 = from_slice([5, 3, 8, 1, 4])
        t2 = insert(t1, 7)
        t3 = delete(t2, 3)
        self.assertEqual(inorder(t1), [1, 3, 4, 5, 8])
        self.assertEqual(inorder(t2), [1, 3, 4, 5, 7, 8])
        self.assertEqual(inorder(t3), [1, 4, 5, 7, 8])
        for t in (t1, t2, t3):
            validate(t)


if __name__ == "__main__":
    print(demo(), end="")
