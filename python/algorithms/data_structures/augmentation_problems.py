"""Augmentation Problems: Maximum Overlap & Josephus — CLRS Problems 14-1, 14-2 (1e-4e) (Editions: 1,2,3,4).

The point covered by the most intervals via sorted endpoints, and the Josephus permutation
in O(n lg n) with an order-statistic tree.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass
class _OsNode:
    key: int
    size: int = 1
    red: bool = True
    left: _OsNode | None = None
    right: _OsNode | None = None


def _sz(n: _OsNode | None) -> int:
    return n.size if n is not None else 0


def _is_red(n: _OsNode | None) -> bool:
    return n.red if n is not None else False


def _pull(n: _OsNode) -> None:
    n.size = 1 + _sz(n.left) + _sz(n.right)


def _rot_l(h: _OsNode) -> _OsNode:
    x = h.right
    assert x is not None
    h.right = x.left
    x.left = h
    x.red = h.red
    h.red = True
    _pull(h)
    _pull(x)
    return x


def _rot_r(h: _OsNode) -> _OsNode:
    x = h.left
    assert x is not None
    h.left = x.right
    x.right = h
    x.red = h.red
    h.red = True
    _pull(h)
    _pull(x)
    return x


def _flip(h: _OsNode) -> None:
    h.red = not h.red
    if h.left is not None:
        h.left.red = not h.left.red
    if h.right is not None:
        h.right.red = not h.right.red


def _fix(h: _OsNode) -> _OsNode:
    if _is_red(h.right) and not _is_red(h.left):
        h = _rot_l(h)
    if _is_red(h.left) and _is_red(h.left.left if h.left else None):
        h = _rot_r(h)
    if _is_red(h.left) and _is_red(h.right):
        _flip(h)
    _pull(h)
    return h


def _move_red_left(h: _OsNode) -> _OsNode:
    _flip(h)
    if h.right is not None and _is_red(h.right.left):
        h.right = _rot_r(h.right)
        h = _rot_l(h)
        _flip(h)
    return h


def _move_red_right(h: _OsNode) -> _OsNode:
    _flip(h)
    if h.left is not None and _is_red(h.left.left):
        h = _rot_r(h)
        _flip(h)
    return h


class _OsTree:
    """Compact self-contained order-statistic tree (LLRB) for Josephus."""

    def __init__(self) -> None:
        self.root: _OsNode | None = None

    def insert(self, key: int) -> None:
        def ins(h: _OsNode | None) -> _OsNode:
            if h is None:
                return _OsNode(key=key)
            if key < h.key:
                h.left = ins(h.left)
            elif key > h.key:
                h.right = ins(h.right)
            return _fix(h)

        self.root = ins(self.root)
        self.root.red = False

    def select(self, i: int) -> int | None:
        x = self.root
        k = i
        while x is not None:
            r = _sz(x.left) + 1
            if k == r:
                return x.key
            if k < r:
                x = x.left
            else:
                x = x.right
                k -= r
        return None

    def delete(self, key: int) -> None:
        def min_key(h: _OsNode) -> int:
            while h.left is not None:
                h = h.left
            return h.key

        def del_min(h: _OsNode) -> _OsNode | None:
            if h.left is None:
                return None
            if not _is_red(h.left) and not _is_red(h.left.left):
                h = _move_red_left(h)
            assert h.left is not None
            h.left = del_min(h.left)
            return _fix(h)

        def rem(h: _OsNode, k: int) -> _OsNode | None:
            if k < h.key:
                if h.left is not None:
                    if not _is_red(h.left) and not _is_red(h.left.left):
                        h = _move_red_left(h)
                    if h.left is not None:
                        h.left = rem(h.left, k)
            else:
                if _is_red(h.left):
                    h = _rot_r(h)
                if k == h.key and h.right is None:
                    return None
                if h.right is not None:
                    if not _is_red(h.right) and not _is_red(h.right.left):
                        h = _move_red_right(h)
                    if k == h.key:
                        assert h.right is not None
                        h.key = min_key(h.right)
                        h.right = del_min(h.right)
                    elif h.right is not None:
                        h.right = rem(h.right, k)
            return _fix(h)

        if self.root is None:
            return
        if not _is_red(self.root.left) and not _is_red(self.root.right):
            self.root.red = True
        self.root = rem(self.root, key)
        if self.root is not None:
            self.root.red = False


def max_overlap(intervals: list[tuple[int, int]]) -> tuple[int, int] | None:
    """Problem 14-1: a point of maximum overlap among closed integer intervals and count."""
    ev: list[tuple[int, int]] = []
    for l, r in intervals:
        ev.append((l, 0))
        ev.append((r, 1))
    ev.sort()
    cur = 0
    best: tuple[int, int] | None = None
    for x, kind in ev:
        if kind == 0:
            cur += 1
            if best is None or cur > best[1]:
                best = (x, cur)
        else:
            cur -= 1
    return best


def josephus_os_tree(n: int, m: int) -> list[int]:
    """Problem 14-2(b): Josephus permutation of {1..n} with step m using an order-statistic tree."""
    t = _OsTree()
    for i in range(1, n + 1):
        t.insert(i)
    pos = 0
    out: list[int] = []
    for left in range(n, 0, -1):
        pos = (pos + m - 1) % left
        victim = t.select(pos + 1)
        assert victim is not None
        t.delete(victim)
        out.append(victim)
    return out


def josephus_survivor(n: int, m: int) -> int:
    """Problem 14-2(c): survivor via J(1) = 0, J(k) = (J(k-1) + m) mod k in Theta(n)."""
    j = 0
    for k in range(1, n + 1):
        j = (j + m) % k
    return j + 1


def demo() -> str:
    lines: list[str] = []
    iv = [(1, 5), (3, 8), (4, 6), (7, 10), (9, 12), (2, 4)]
    lines.append(f"intervals {iv}")
    lines.append(f"point of maximum overlap: {max_overlap(iv)}")
    lines.append(f"Josephus(7, 3) elimination order = {josephus_os_tree(7, 3)}")
    lines.append(
        f"survivor of Josephus(41, 3) = {josephus_survivor(41, 3)} (closed form recurrence)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_overlap_matches_brute_force(self) -> None:
        r = Rng(330)
        for _ in range(200):
            iv = [(a, a + r.range(0, 10)) for a in (r.range(0, 30) for _ in range(r.range(1, 15)))]
            res = max_overlap(iv)
            self.assertIsNotNone(res)
            assert res is not None
            _, c = res
            brute = max(sum(1 for l, rr in iv if l <= x <= rr) for x in range(45))
            self.assertEqual(c, brute)
            p, c2 = res
            self.assertEqual(sum(1 for l, rr in iv if l <= p <= rr), c2)
        self.assertIsNone(max_overlap([]))

    def test_josephus_agrees_across_methods(self) -> None:
        self.assertEqual(josephus_os_tree(7, 3), [3, 6, 2, 7, 5, 1, 4])
        for n in range(1, 40):
            for m in range(1, 8):
                order = josephus_os_tree(n, m)
                self.assertEqual(order[-1], josephus_survivor(n, m), f"n={n} m={m}")
                self.assertEqual(sorted(order), list(range(1, n + 1)))


if __name__ == "__main__":
    print(demo(), end="")
