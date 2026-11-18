"""Interval Trees — CLRS 14.3 (1e–4e) (1,2,3,4).

Red-black tree keyed by low endpoints and augmented with the maximum high endpoint: INTERVAL-SEARCH in O(lg n).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_RED = True
_BLACK = False
_NEG_INF = -(1 << 62)


@dataclass
class _Node:
    lo: int
    hi: int
    max_hi: int
    color: bool = _RED
    left: int = 0
    right: int = 0
    p: int = 0


def _overlaps(a: tuple[int, int], b: tuple[int, int]) -> bool:
    return a[0] <= b[1] and b[0] <= a[1]


class IntervalTree:
    """Closed intervals [lo, hi] stored in an augmented red-black tree."""

    def __init__(self) -> None:
        self.n: list[_Node] = [_Node(0, _NEG_INF, _NEG_INF, color=_BLACK)]
        self.root = 0
        self.free: list[int] = []
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def _update(self, x: int) -> None:
        if x == 0:
            return
        nd = self.n[x]
        nd.max_hi = max(nd.hi, self.n[nd.left].max_hi, self.n[nd.right].max_hi)

    def _update_path(self, x: int) -> None:
        while x != 0:
            self._update(x)
            x = self.n[x].p

    def _left_rotate(self, x: int) -> None:
        y = self.n[x].right
        self.n[x].right = self.n[y].left
        if self.n[y].left != 0:
            self.n[self.n[y].left].p = x
        self.n[y].p = self.n[x].p
        xp = self.n[x].p
        if xp == 0:
            self.root = y
        elif x == self.n[xp].left:
            self.n[xp].left = y
        else:
            self.n[xp].right = y
        self.n[y].left = x
        self.n[x].p = y
        self._update(x)
        self._update(y)

    def _right_rotate(self, x: int) -> None:
        y = self.n[x].left
        self.n[x].left = self.n[y].right
        if self.n[y].right != 0:
            self.n[self.n[y].right].p = x
        self.n[y].p = self.n[x].p
        xp = self.n[x].p
        if xp == 0:
            self.root = y
        elif x == self.n[xp].right:
            self.n[xp].right = y
        else:
            self.n[xp].left = y
        self.n[y].right = x
        self.n[x].p = y
        self._update(x)
        self._update(y)

    def insert(self, lo: int, hi: int) -> None:
        assert lo <= hi
        node = _Node(lo, hi, hi)
        if self.free:
            z = self.free.pop()
            self.n[z] = node
        else:
            z = len(self.n)
            self.n.append(node)
        y = 0
        x = self.root
        while x != 0:
            y = x
            x = self.n[x].left if lo < self.n[x].lo else self.n[x].right
        self.n[z].p = y
        if y == 0:
            self.root = z
        elif lo < self.n[y].lo:
            self.n[y].left = z
        else:
            self.n[y].right = z
        self._update_path(y)
        self._insert_fixup(z)
        self._len += 1

    def _insert_fixup(self, z: int) -> None:
        while self.n[self.n[z].p].color == _RED:
            pz = self.n[z].p
            gz = self.n[pz].p
            if pz == self.n[gz].left:
                y = self.n[gz].right
                if self.n[y].color == _RED:
                    self.n[pz].color = _BLACK
                    self.n[y].color = _BLACK
                    self.n[gz].color = _RED
                    z = gz
                else:
                    if z == self.n[pz].right:
                        z = pz
                        self._left_rotate(z)
                    pz = self.n[z].p
                    gz = self.n[pz].p
                    self.n[pz].color = _BLACK
                    self.n[gz].color = _RED
                    self._right_rotate(gz)
            else:
                y = self.n[gz].left
                if self.n[y].color == _RED:
                    self.n[pz].color = _BLACK
                    self.n[y].color = _BLACK
                    self.n[gz].color = _RED
                    z = gz
                else:
                    if z == self.n[pz].left:
                        z = pz
                        self._right_rotate(z)
                    pz = self.n[z].p
                    gz = self.n[pz].p
                    self.n[pz].color = _BLACK
                    self.n[gz].color = _RED
                    self._left_rotate(gz)
        self.n[self.root].color = _BLACK

    def _find_exact(self, lo: int, hi: int) -> int:
        def rec(x: int) -> int:
            if x == 0:
                return 0
            if lo < self.n[x].lo:
                return rec(self.n[x].left)
            if lo > self.n[x].lo:
                return rec(self.n[x].right)
            if self.n[x].hi == hi:
                return x
            l = rec(self.n[x].left)
            return l if l != 0 else rec(self.n[x].right)

        return rec(self.root)

    def _transplant(self, u: int, v: int) -> None:
        up = self.n[u].p
        if up == 0:
            self.root = v
        elif u == self.n[up].left:
            self.n[up].left = v
        else:
            self.n[up].right = v
        self.n[v].p = up

    def _minimum(self, x: int) -> int:
        while self.n[x].left != 0:
            x = self.n[x].left
        return x

    def delete(self, lo: int, hi: int) -> bool:
        z = self._find_exact(lo, hi)
        if z == 0:
            return False
        y = z
        y_orig = self.n[y].color
        if self.n[z].left == 0:
            x = self.n[z].right
            start = self.n[z].p
            self._transplant(z, x)
        elif self.n[z].right == 0:
            x = self.n[z].left
            start = self.n[z].p
            self._transplant(z, x)
        else:
            y = self._minimum(self.n[z].right)
            y_orig = self.n[y].color
            x = self.n[y].right
            if self.n[y].p == z:
                self.n[x].p = y
                start = y
            else:
                start = self.n[y].p
                self._transplant(y, x)
                self.n[y].right = self.n[z].right
                self.n[self.n[y].right].p = y
            self._transplant(z, y)
            self.n[y].left = self.n[z].left
            self.n[self.n[y].left].p = y
            self.n[y].color = self.n[z].color
        self._update_path(start)
        if y_orig == _BLACK:
            self._delete_fixup(x)
        self.n[0].p = 0
        self.free.append(z)
        self._len -= 1
        return True

    def _delete_fixup(self, x: int) -> None:
        while x != self.root and self.n[x].color == _BLACK:
            xp = self.n[x].p
            if x == self.n[xp].left:
                w = self.n[xp].right
                if self.n[w].color == _RED:
                    self.n[w].color = _BLACK
                    self.n[xp].color = _RED
                    self._left_rotate(xp)
                    w = self.n[self.n[x].p].right
                if self.n[self.n[w].left].color == _BLACK and self.n[self.n[w].right].color == _BLACK:
                    self.n[w].color = _RED
                    x = self.n[x].p
                else:
                    if self.n[self.n[w].right].color == _BLACK:
                        self.n[self.n[w].left].color = _BLACK
                        self.n[w].color = _RED
                        self._right_rotate(w)
                        w = self.n[self.n[x].p].right
                    xp = self.n[x].p
                    self.n[w].color = self.n[xp].color
                    self.n[xp].color = _BLACK
                    self.n[self.n[w].right].color = _BLACK
                    self._left_rotate(xp)
                    x = self.root
            else:
                w = self.n[xp].left
                if self.n[w].color == _RED:
                    self.n[w].color = _BLACK
                    self.n[xp].color = _RED
                    self._right_rotate(xp)
                    w = self.n[self.n[x].p].left
                if self.n[self.n[w].left].color == _BLACK and self.n[self.n[w].right].color == _BLACK:
                    self.n[w].color = _RED
                    x = self.n[x].p
                else:
                    if self.n[self.n[w].left].color == _BLACK:
                        self.n[self.n[w].right].color = _BLACK
                        self.n[w].color = _RED
                        self._left_rotate(w)
                        w = self.n[self.n[x].p].left
                    xp = self.n[x].p
                    self.n[w].color = self.n[xp].color
                    self.n[xp].color = _BLACK
                    self.n[self.n[w].left].color = _BLACK
                    self._right_rotate(xp)
                    x = self.root
        self.n[x].color = _BLACK
        self.n[0].color = _BLACK

    def search(self, lo: int, hi: int) -> tuple[int, int] | None:
        """INTERVAL-SEARCH(T, i): some interval overlapping [lo, hi], in O(lg n)."""
        x = self.root
        while x != 0 and not _overlaps((self.n[x].lo, self.n[x].hi), (lo, hi)):
            l = self.n[x].left
            x = l if l != 0 and self.n[l].max_hi >= lo else self.n[x].right
        return None if x == 0 else (self.n[x].lo, self.n[x].hi)

    def search_all(self, lo: int, hi: int) -> list[tuple[int, int]]:
        """List all overlapping intervals."""
        out: list[tuple[int, int]] = []

        def rec(x: int) -> None:
            if x == 0:
                return
            l = self.n[x].left
            if l != 0 and self.n[l].max_hi >= lo:
                rec(l)
            iv = (self.n[x].lo, self.n[x].hi)
            if _overlaps(iv, (lo, hi)):
                out.append(iv)
            if self.n[x].lo <= hi:
                rec(self.n[x].right)

        rec(self.root)
        return out

    def validate(self) -> bool:
        if self.n[0].color != _BLACK:
            return False
        if self.root != 0 and self.n[self.root].color != _BLACK:
            return False

        def rec(x: int, lo: int, hi: int) -> int | None:
            if x == 0:
                return 1
            nd = self.n[x]
            if nd.lo < lo or nd.lo > hi:
                return None
            if nd.color == _RED and (self.n[nd.left].color == _RED or self.n[nd.right].color == _RED):
                return None
            if nd.max_hi != max(nd.hi, self.n[nd.left].max_hi, self.n[nd.right].max_hi):
                return None
            lbh = rec(nd.left, lo, nd.lo)
            rbh = rec(nd.right, nd.lo, hi)
            if lbh is None or rbh is None or lbh != rbh:
                return None
            return lbh + (1 if nd.color == _BLACK else 0)

        return rec(self.root, _NEG_INF, (1 << 62)) is not None


def demo() -> str:
    lines: list[str] = []
    t = IntervalTree()
    for lo, hi in [(16, 21), (8, 9), (25, 30), (5, 8), (15, 23), (17, 19), (26, 26), (0, 3), (6, 10), (19, 20)]:
        t.insert(lo, hi)
    lines.append(f"intervals stored: {len(t)}")
    lines.append(f"search [22,25]  -> {t.search(22, 25)}")
    lines.append(f"search [11,14]  -> {t.search(11, 14)}  (no overlap)")
    lines.append(f"all overlapping [14,16] -> {sorted(t.search_all(14, 16))}")
    t.delete(15, 23)
    lines.append(f"after deleting (15,23): search [22,25] -> {t.search(22, 25)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_brute_force(self) -> None:
        r = Rng(74)
        t = IntervalTree()
        model: list[tuple[int, int]] = []
        for step in range(1500):
            if r.range(0, 3) < 2 or not model:
                lo = r.range(0, 200)
                hi = lo + r.range(0, 30)
                t.insert(lo, hi)
                model.append((lo, hi))
            else:
                i = r.range(0, len(model) - 1)
                iv = model[i]
                model[i] = model[-1]
                model.pop()
                self.assertTrue(t.delete(iv[0], iv[1]))
            qlo = r.range(0, 230)
            q = (qlo, qlo + r.range(0, 10))
            want = sorted(iv for iv in model if _overlaps(iv, q))
            got = sorted(t.search_all(q[0], q[1]))
            self.assertEqual(got, want)
            found = t.search(q[0], q[1])
            self.assertEqual(found is not None, bool(want))
            if found is not None:
                self.assertTrue(_overlaps(found, q))
            if step % 50 == 0:
                self.assertTrue(t.validate())

    def test_delete_missing_interval(self) -> None:
        t = IntervalTree()
        t.insert(1, 5)
        self.assertFalse(t.delete(1, 6))
        self.assertTrue(t.delete(1, 5))
        self.assertTrue(t.is_empty())


if __name__ == "__main__":
    print(demo(), end="")
