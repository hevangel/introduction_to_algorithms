"""Order-Statistic Trees — CLRS 14.1–14.2 (1e–4e) (1,2,3,4).

Red-black tree augmented with subtree sizes: OS-SELECT and OS-RANK in O(lg n).
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


@dataclass
class _Node:
    key: int
    color: bool = _RED
    left: int = 0
    right: int = 0
    p: int = 0
    size: int = 1


class OsTree:
    """Dynamic order-statistic set built on a size-augmented red-black tree."""

    def __init__(self) -> None:
        self.n: list[_Node] = [_Node(0, color=_BLACK, size=0)]
        self.root = 0
        self.free: list[int] = []
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def _update(self, x: int) -> None:
        if x != 0:
            nd = self.n[x]
            nd.size = self.n[nd.left].size + self.n[nd.right].size + 1

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

    def insert(self, key: int) -> None:
        node = _Node(key)
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
            x = self.n[x].left if key < self.n[x].key else self.n[x].right
        self.n[z].p = y
        if y == 0:
            self.root = z
        elif key < self.n[y].key:
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

    def _find(self, key: int) -> int:
        x = self.root
        while x != 0 and key != self.n[x].key:
            x = self.n[x].left if key < self.n[x].key else self.n[x].right
        return x

    def _minimum(self, x: int) -> int:
        while self.n[x].left != 0:
            x = self.n[x].left
        return x

    def _transplant(self, u: int, v: int) -> None:
        up = self.n[u].p
        if up == 0:
            self.root = v
        elif u == self.n[up].left:
            self.n[up].left = v
        else:
            self.n[up].right = v
        self.n[v].p = up

    def delete(self, key: int) -> bool:
        z = self._find(key)
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

    def select(self, i: int) -> int | None:
        """OS-SELECT(root, i): the i-th smallest key (1-based)."""
        if i <= 0 or i > self._len:
            return None
        x = self.root
        while True:
            r = self.n[self.n[x].left].size + 1
            if i == r:
                return self.n[x].key
            if i < r:
                x = self.n[x].left
            else:
                i -= r
                x = self.n[x].right

    def rank(self, key: int) -> int | None:
        """OS-RANK(T, x): 1-based sorted position of `key`."""
        z = self._find(key)
        if z == 0:
            return None
        r = self.n[self.n[z].left].size + 1
        y = z
        while y != self.root:
            p = self.n[y].p
            if y == self.n[p].right:
                r += self.n[self.n[p].left].size + 1
            y = p
        return r

    def count_le(self, key: int) -> int:
        """Number of stored keys <= key (Exercise 14.1-7 helper)."""
        x, c = self.root, 0
        while x != 0:
            if self.n[x].key <= key:
                c += self.n[self.n[x].left].size + 1
                x = self.n[x].right
            else:
                x = self.n[x].left
        return c

    def validate(self) -> bool:
        if self.n[0].color != _BLACK:
            return False
        if self.root != 0 and self.n[self.root].color != _BLACK:
            return False

        def rec(x: int) -> tuple[int, int] | None:
            if x == 0:
                return (1, 0)
            nd = self.n[x]
            if nd.color == _RED and (self.n[nd.left].color == _RED or self.n[nd.right].color == _RED):
                return None
            l = rec(nd.left)
            r = rec(nd.right)
            if l is None or r is None or l[0] != r[0]:
                return None
            sz = l[1] + r[1] + 1
            if nd.size != sz:
                return None
            return (l[0] + (1 if nd.color == _BLACK else 0), sz)

        return rec(self.root) is not None


def count_inversions(a: list[int]) -> int:
    """Exercise 14.1-7: count inversions in Θ(n lg n) with an order-statistic tree."""
    t = OsTree()
    inv = 0
    for i, x in enumerate(a):
        inv += i - t.count_le(x)
        t.insert(x)
    return inv


def demo() -> str:
    lines: list[str] = []
    t = OsTree()
    for k in [26, 17, 41, 14, 21, 30, 47, 10, 16, 19, 21, 28, 38, 7, 12, 14, 20, 35, 39, 3]:
        t.insert(k)
    lines.append(f"n = {len(t)}")
    for i in [1, 5, 10, 20]:
        lines.append(f"OS-SELECT({i:>2}) = {t.select(i)}")
    for k in [3, 26, 47]:
        lines.append(f"OS-RANK({k:>2})   = {t.rank(k)}")
    t.delete(26)
    lines.append(f"after deleting 26: OS-SELECT(10) = {t.select(10)}, size = {len(t)}")
    lines.append(f"inversions of [2,3,8,6,1] via OS-tree: {count_inversions([2, 3, 8, 6, 1])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_select_and_rank_match_sorted_vector(self) -> None:
        t = OsTree()
        model: list[int] = []
        r = Rng(72)
        for _ in range(1500):
            k = r.range(0, 200)
            if r.range(0, 2) < 2:
                if k not in model:
                    t.insert(k)
                    model.append(k)
                    model.sort()
            else:
                self.assertEqual(t.delete(k), k in model)
                if k in model:
                    model.remove(k)
            if model:
                m = model[len(model) // 2]
                self.assertEqual(t.select(len(model) // 2 + 1), m)
                self.assertEqual(t.rank(m), len(model) // 2 + 1)
        self.assertTrue(t.validate())
        for i, k in enumerate(model):
            self.assertEqual(t.select(i + 1), k)
            self.assertEqual(t.rank(k), i + 1)
        self.assertIsNone(t.select(0))
        self.assertIsNone(t.select(len(model) + 1))

    def test_inversions_match_naive(self) -> None:
        r = Rng(73)
        for n in [0, 1, 10, 100]:
            v = r.vec(n, 0, 30)
            want = sum(1 for i in range(n) for j in range(i + 1, n) if v[i] > v[j])
            self.assertEqual(count_inversions(v), want)


if __name__ == "__main__":
    print(demo(), end="")
