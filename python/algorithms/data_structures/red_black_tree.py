"""Red-Black Trees — CLRS 13 (1e–4e) (1,2,3,4).

Self-balancing BST: rotations, RB-INSERT and RB-DELETE with fix-up keep height ≤ 2 lg(n+1).
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

NIL = 0
_MIN_I64 = -(1 << 62)
_MAX_I64 = 1 << 62


class Color(Enum):
    RED = auto()
    BLACK = auto()


@dataclass
class _Node:
    key: int
    aux: int
    color: Color
    left: int = NIL
    right: int = NIL
    p: int = NIL
    size: int = 1
    max_aux: int = _MIN_I64


class RedBlackTree:
    """Red-black tree in an arena, following CLRS pseudocode with sentinel T.nil = 0."""

    def __init__(self) -> None:
        nil = _Node(0, _MIN_I64, Color.BLACK, NIL, NIL, NIL, 0, _MIN_I64)
        self.n: list[_Node] = [nil]
        self.root = NIL
        self.free: list[int] = []
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def key(self, x: int) -> int:
        return self.n[x].key

    def aux(self, x: int) -> int:
        return self.n[x].aux

    def size(self, x: int) -> int:
        return self.n[x].size

    def _update(self, x: int) -> None:
        if x == NIL:
            return
        nd = self.n[x]
        nd.size = self.n[nd.left].size + self.n[nd.right].size + 1
        nd.max_aux = max(nd.aux, self.n[nd.left].max_aux, self.n[nd.right].max_aux)

    def _update_path(self, x: int) -> None:
        while x != NIL:
            self._update(x)
            x = self.n[x].p

    def _left_rotate(self, x: int) -> None:
        y = self.n[x].right
        self.n[x].right = self.n[y].left
        if self.n[y].left != NIL:
            self.n[self.n[y].left].p = x
        self.n[y].p = self.n[x].p
        xp = self.n[x].p
        if xp == NIL:
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
        if self.n[y].right != NIL:
            self.n[self.n[y].right].p = x
        self.n[y].p = self.n[x].p
        xp = self.n[x].p
        if xp == NIL:
            self.root = y
        elif x == self.n[xp].right:
            self.n[xp].right = y
        else:
            self.n[xp].left = y
        self.n[y].right = x
        self.n[x].p = y
        self._update(x)
        self._update(y)

    def insert(self, key: int) -> int:
        return self.insert_with_aux(key, key)

    def insert_with_aux(self, key: int, aux: int) -> int:
        node = _Node(key, aux, Color.RED, NIL, NIL, NIL, 1, aux)
        if self.free:
            z = self.free.pop()
            self.n[z] = node
        else:
            z = len(self.n)
            self.n.append(node)
        y = NIL
        x = self.root
        while x != NIL:
            y = x
            x = self.n[x].left if key < self.n[x].key else self.n[x].right
        self.n[z].p = y
        if y == NIL:
            self.root = z
        elif key < self.n[y].key:
            self.n[y].left = z
        else:
            self.n[y].right = z
        self._update_path(y)
        self._insert_fixup(z)
        self._len += 1
        return z

    def _insert_fixup(self, z: int) -> None:
        while self.n[self.n[z].p].color == Color.RED:
            pz = self.n[z].p
            gz = self.n[pz].p
            if pz == self.n[gz].left:
                y = self.n[gz].right
                if self.n[y].color == Color.RED:
                    self.n[pz].color = Color.BLACK
                    self.n[y].color = Color.BLACK
                    self.n[gz].color = Color.RED
                    z = gz
                else:
                    if z == self.n[pz].right:
                        z = pz
                        self._left_rotate(z)
                    pz = self.n[z].p
                    gz = self.n[pz].p
                    self.n[pz].color = Color.BLACK
                    self.n[gz].color = Color.RED
                    self._right_rotate(gz)
            else:
                y = self.n[gz].left
                if self.n[y].color == Color.RED:
                    self.n[pz].color = Color.BLACK
                    self.n[y].color = Color.BLACK
                    self.n[gz].color = Color.RED
                    z = gz
                else:
                    if z == self.n[pz].left:
                        z = pz
                        self._right_rotate(z)
                    pz = self.n[z].p
                    gz = self.n[pz].p
                    self.n[pz].color = Color.BLACK
                    self.n[gz].color = Color.RED
                    self._left_rotate(gz)
        self.n[self.root].color = Color.BLACK

    def find(self, key: int) -> int:
        x = self.root
        while x != NIL and key != self.n[x].key:
            x = self.n[x].left if key < self.n[x].key else self.n[x].right
        return x

    def minimum(self, x: int) -> int:
        while self.n[x].left != NIL:
            x = self.n[x].left
        return x

    def maximum(self, x: int) -> int:
        while self.n[x].right != NIL:
            x = self.n[x].right
        return x

    def successor(self, x: int) -> int:
        if self.n[x].right != NIL:
            return self.minimum(self.n[x].right)
        y = self.n[x].p
        while y != NIL and x == self.n[y].right:
            x = y
            y = self.n[y].p
        return y

    def _transplant(self, u: int, v: int) -> None:
        up = self.n[u].p
        if up == NIL:
            self.root = v
        elif u == self.n[up].left:
            self.n[up].left = v
        else:
            self.n[up].right = v
        self.n[v].p = up

    def delete(self, z: int) -> None:
        assert z != NIL
        y = z
        y_orig = self.n[y].color
        if self.n[z].left == NIL:
            x = self.n[z].right
            start = self.n[z].p
            self._transplant(z, x)
        elif self.n[z].right == NIL:
            x = self.n[z].left
            start = self.n[z].p
            self._transplant(z, x)
        else:
            y = self.minimum(self.n[z].right)
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
        if y_orig == Color.BLACK:
            self._delete_fixup(x)
        self.n[NIL].p = NIL
        self.free.append(z)
        self._len -= 1

    def _delete_fixup(self, x: int) -> None:
        while x != self.root and self.n[x].color == Color.BLACK:
            xp = self.n[x].p
            if x == self.n[xp].left:
                w = self.n[xp].right
                if self.n[w].color == Color.RED:
                    self.n[w].color = Color.BLACK
                    self.n[xp].color = Color.RED
                    self._left_rotate(xp)
                    w = self.n[self.n[x].p].right
                if self.n[self.n[w].left].color == Color.BLACK and self.n[self.n[w].right].color == Color.BLACK:
                    self.n[w].color = Color.RED
                    x = self.n[x].p
                else:
                    if self.n[self.n[w].right].color == Color.BLACK:
                        self.n[self.n[w].left].color = Color.BLACK
                        self.n[w].color = Color.RED
                        self._right_rotate(w)
                        w = self.n[self.n[x].p].right
                    xp = self.n[x].p
                    self.n[w].color = self.n[xp].color
                    self.n[xp].color = Color.BLACK
                    self.n[self.n[w].right].color = Color.BLACK
                    self._left_rotate(xp)
                    x = self.root
            else:
                w = self.n[xp].left
                if self.n[w].color == Color.RED:
                    self.n[w].color = Color.BLACK
                    self.n[xp].color = Color.RED
                    self._right_rotate(xp)
                    w = self.n[self.n[x].p].left
                if self.n[self.n[w].left].color == Color.BLACK and self.n[self.n[w].right].color == Color.BLACK:
                    self.n[w].color = Color.RED
                    x = self.n[x].p
                else:
                    if self.n[self.n[w].left].color == Color.BLACK:
                        self.n[self.n[w].right].color = Color.BLACK
                        self.n[w].color = Color.RED
                        self._left_rotate(w)
                        w = self.n[self.n[x].p].left
                    xp = self.n[x].p
                    self.n[w].color = self.n[xp].color
                    self.n[xp].color = Color.BLACK
                    self.n[self.n[w].left].color = Color.BLACK
                    self._right_rotate(xp)
                    x = self.root
        self.n[x].color = Color.BLACK
        self.n[NIL].color = Color.BLACK

    def delete_key(self, key: int) -> bool:
        z = self.find(key)
        if z == NIL:
            return False
        self.delete(z)
        return True

    def inorder(self) -> list[int]:
        out: list[int] = []
        st: list[int] = []
        x = self.root
        while x != NIL or st:
            while x != NIL:
                st.append(x)
                x = self.n[x].left
            y = st.pop()
            out.append(self.n[y].key)
            x = self.n[y].right
        return out

    def height(self) -> int:
        if self.root == NIL:
            return 0
        h = 0
        level = [self.root]
        while level:
            h += 1
            nxt: list[int] = []
            for x in level:
                if self.n[x].left != NIL:
                    nxt.append(self.n[x].left)
                if self.n[x].right != NIL:
                    nxt.append(self.n[x].right)
            level = nxt
        return h

    def validate(self) -> int:
        if self.n[NIL].color != Color.BLACK:
            raise AssertionError("nil is not black")
        if self.root != NIL and self.n[self.root].color != Color.BLACK:
            raise AssertionError("root is red")

        def rec(x: int, lo: int, hi: int) -> int:
            if x == NIL:
                return 1
            nd = self.n[x]
            if nd.key < lo or nd.key > hi:
                raise AssertionError(f"order violated at {nd.key}")
            if nd.color == Color.RED and (self.n[nd.left].color == Color.RED or self.n[nd.right].color == Color.RED):
                raise AssertionError(f"red-red at {nd.key}")
            for c in (nd.left, nd.right):
                if c != NIL and self.n[c].p != x:
                    raise AssertionError(f"bad parent link below {nd.key}")
            if nd.size != self.n[nd.left].size + self.n[nd.right].size + 1:
                raise AssertionError(f"bad size at {nd.key}")
            if nd.max_aux != max(nd.aux, self.n[nd.left].max_aux, self.n[nd.right].max_aux):
                raise AssertionError(f"bad max at {nd.key}")
            l = rec(nd.left, lo, nd.key)
            r = rec(nd.right, nd.key, hi)
            if l != r:
                raise AssertionError(f"black-height mismatch at {nd.key}")
            return l + (1 if nd.color == Color.BLACK else 0)

        return rec(self.root, _MIN_I64, _MAX_I64)


def demo() -> str:
    lines: list[str] = []
    t = RedBlackTree()
    for k in [41, 38, 31, 12, 19, 8]:
        t.insert(k)
        lines.append(f"insert {k:>2} -> height {t.height()} black-height {t.validate()}")
    lines.append(f"inorder = {t.inorder()}")
    for k in (8, 12, 19):
        t.delete_key(k)
    lines.append(f"after deleting 8, 12, 19: {t.inorder()} (valid: {t.validate() >= 1})")
    big = RedBlackTree()
    for k in range(20_000):
        big.insert(k)
    lines.append(f"20000 ascending inserts: height = {big.height()} (bound 2·lg(n+1) = {2.0 * math.log2(20_001):.1f})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_invariants_hold_through_random_inserts_and_deletes(self) -> None:
        t = RedBlackTree()
        model: list[int] = []
        r = Rng(70)
        for step in range(4000):
            k = r.range(0, 300)
            if r.range(0, 2) < 2:
                t.insert(k)
                model.append(k)
            else:
                removed = t.delete_key(k)
                self.assertEqual(removed, k in model)
                if k in model:
                    model.remove(k)
            if step % 7 == 0:
                t.validate()
        model.sort()
        self.assertEqual(t.inorder(), model)
        self.assertEqual(len(t), len(model))
        t.validate()

    def test_height_bound_on_sorted_input(self) -> None:
        t = RedBlackTree()
        for k in range(2000):
            t.insert(k)
        self.assertLessEqual(t.height(), 2.0 * math.log2(2001))
        for k in range(52):
            self.assertTrue(t.delete_key(k))
            t.validate()

    def test_delete_everything(self) -> None:
        t = RedBlackTree()
        r = Rng(71)
        keys = r.vec(400, 0, 1000)
        for k in keys:
            t.insert(k)
        for k in keys:
            self.assertTrue(t.delete_key(k))
            t.validate()
        self.assertTrue(t.is_empty())
        self.assertEqual(t.root, NIL)

    def test_successor_and_minimum(self) -> None:
        t = RedBlackTree()
        for k in [5, 1, 9, 3, 7]:
            t.insert(k)
        m = t.minimum(t.root)
        self.assertEqual(t.key(m), 1)
        self.assertEqual(t.key(t.successor(m)), 3)
        self.assertEqual(t.key(t.maximum(t.root)), 9)


if __name__ == "__main__":
    print(demo(), end="")
