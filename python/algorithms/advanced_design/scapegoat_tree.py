"""Scapegoat Trees — CLRS Problem 17-3 (1e–4e) (1e,2e,3e,4e).

An alpha-weight-balanced BST with no balance fields: rebuild the subtree of the 'scapegoat' when an insertion lands too deep. O(lg n) amortised.
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

ALPHA = 0.7


@dataclass
class _Node:
    key: int
    size: int = 1
    l: _Node | None = None
    r: _Node | None = None


def _size(t: _Node | None) -> int:
    return 0 if t is None else t.size


def _flatten(t: _Node | None, out: list[int]) -> None:
    if t is not None:
        _flatten(t.l, out)
        out.append(t.key)
        _flatten(t.r, out)


def _build(keys: list[int], lo: int, hi: int) -> _Node | None:
    if lo >= hi:
        return None
    m = (lo + hi) // 2
    return _Node(key=keys[m], size=hi - lo, l=_build(keys, lo, m), r=_build(keys, m + 1, hi))


def _rebuild(t: _Node | None) -> _Node | None:
    v: list[int] = []
    _flatten(t, v)
    return _build(v, 0, len(v))


def _depth_limit(n: int) -> int:
    return math.floor(math.log(max(n, 1)) / math.log(1.0 / ALPHA))


@dataclass
class _St:
    inserted: bool = False
    deep: bool = False
    rebuilds: int = 0


def _ins(t: _Node | None, key: int, depth: int, limit: int, st: _St) -> _Node | None:
    if t is None:
        st.inserted = True
        st.deep = depth > limit
        return _Node(key=key, size=1)
    if key == t.key:
        return t
    left = key < t.key
    if left:
        t.l = _ins(t.l, key, depth + 1, limit, st)
    else:
        t.r = _ins(t.r, key, depth + 1, limit, st)
    if not st.inserted:
        return t
    t.size += 1
    if st.deep:
        child = _size(t.l) if left else _size(t.r)
        if child > ALPHA * t.size:
            st.deep = False
            st.rebuilds += 1
            return _rebuild(t)
    return t


def _take_min(t: _Node) -> tuple[_Node | None, int]:
    if t.l is None:
        return t.r, t.key
    rest, m = _take_min(t.l)
    t.l = rest
    t.size = 1 + _size(t.l) + _size(t.r)
    return t, m


def _del(t: _Node | None, key: int) -> tuple[_Node | None, bool]:
    if t is None:
        return None, False
    if key < t.key:
        t.l, removed = _del(t.l, key)
    elif key > t.key:
        t.r, removed = _del(t.r, key)
    else:
        if t.l is None:
            return t.r, True
        if t.r is None:
            return t.l, True
        rest, m = _take_min(t.r)
        t.key = m
        t.r = rest
        removed = True
    t.size = 1 + _size(t.l) + _size(t.r)
    return t, removed


class ScapegoatTree:
    """Scapegoat tree with alpha = 0.7."""

    def __init__(self) -> None:
        self._root: _Node | None = None
        self._n: int = 0
        self._max_n: int = 0
        self.rebuilds: int = 0

    def __len__(self) -> int:
        return self._n

    def is_empty(self) -> bool:
        return self._n == 0

    def insert(self, key: int) -> bool:
        st = _St()
        self._root = _ins(self._root, key, 0, _depth_limit(self._n + 1), st)
        if st.inserted:
            self._n += 1
            self._max_n = max(self._max_n, self._n)
            self.rebuilds += st.rebuilds
        return st.inserted

    def delete(self, key: int) -> bool:
        self._root, removed = _del(self._root, key)
        if removed:
            self._n -= 1
            if self._n < ALPHA * self._max_n:
                self._root = _rebuild(self._root)
                self._max_n = self._n
                self.rebuilds += 1
        return removed

    def contains(self, key: int) -> bool:
        x = self._root
        while x is not None:
            if key == x.key:
                return True
            x = x.l if key < x.key else x.r
        return False

    def height(self) -> int:
        def h(t: _Node | None) -> int:
            return 0 if t is None else 1 + max(h(t.l), h(t.r))

        return h(self._root)

    def max_depth_allowed(self) -> int:
        return _depth_limit(self._max_n) + 1

    def inorder(self) -> list[int]:
        v: list[int] = []
        _flatten(self._root, v)
        return v

    def validate(self) -> bool:
        def go(t: _Node | None, lo: int | None, hi: int | None) -> int | None:
            if t is None:
                return 0
            if (lo is not None and t.key <= lo) or (hi is not None and t.key >= hi):
                return None
            ls = go(t.l, lo, t.key)
            rs = go(t.r, t.key, hi)
            if ls is None or rs is None:
                return None
            s = ls + rs + 1
            return s if s == t.size else None

        return go(self._root, None, None) == self._n


def demo() -> str:
    lines: list[str] = []
    t = ScapegoatTree()
    for k in range(10_000):
        t.insert(k)
    lines.append(
        f"10000 ascending insertions: height {t.height()} "
        f"(≤ log_{{1/α}} n + 1 = {t.max_depth_allowed()}), {t.rebuilds} partial rebuilds"
    )
    for k in range(10_000):
        if k % 3 != 0:
            t.delete(k)
    lines.append(
        f"after deleting two thirds: n = {len(t)}, height {t.height()}, "
        f"total rebuilds {t.rebuilds}, valid = {t.validate()}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_behaves_like_a_set_and_stays_shallow(self) -> None:
        t = ScapegoatTree()
        s: set[int] = set()
        r = Rng(335)
        for step in range(6000):
            k = r.range(0, 400)
            if r.range(0, 5) < 3:
                exp = k not in s
                s.add(k)
                self.assertEqual(t.insert(k), exp)
            else:
                exp = k in s
                s.discard(k)
                self.assertEqual(t.delete(k), exp)
            self.assertEqual(t.contains(k), k in s)
            if step % 50 == 0:
                self.assertTrue(t.validate())
            self.assertLessEqual(
                t.height(),
                t.max_depth_allowed() + 2,
                f"height {t.height()} limit {t.max_depth_allowed()}",
            )
        self.assertEqual(t.inorder(), sorted(s))

    def test_sorted_insertions_trigger_rebuilds_and_logarithmic_height(self) -> None:
        t = ScapegoatTree()
        for k in range(20_000):
            t.insert(k)
        self.assertGreater(t.rebuilds, 0)
        self.assertLessEqual(t.height(), t.max_depth_allowed() + 2)
        self.assertLess(t.height(), 40)
        self.assertTrue(t.validate())


if __name__ == "__main__":
    print(demo(), end="")
