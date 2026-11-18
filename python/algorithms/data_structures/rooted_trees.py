"""Representing Rooted Trees — CLRS 10.4 (1e–4e) (1,2,3,4).

Left-child/right-sibling trees, stack-based and O(1)-space tree traversals.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NIL = -1


class RootedTree:
    """Rooted tree with unbounded branching using the left-child, right-sibling representation."""

    def __init__(self, keys: list[int], left_child: list[int | None], right_sibling: list[int | None]) -> None:
        self.key = keys
        self.left_child = left_child
        self.right_sibling = right_sibling

    @classmethod
    def from_children(cls, keys: list[int], children: list[list[int]]) -> RootedTree:
        n = len(keys)
        lc: list[int | None] = [None] * n
        rs: list[int | None] = [None] * n
        for p, ch in enumerate(children):
            for i in range(len(ch) - 1):
                rs[ch[i]] = ch[i + 1]
            lc[p] = ch[0] if ch else None
        return cls(list(keys), lc, rs)

    def children(self, x: int) -> list[int]:
        v: list[int] = []
        c = self.left_child[x]
        while c is not None:
            v.append(c)
            c = self.right_sibling[c]
        return v

    def preorder_recursive(self, x: int, out: list[int]) -> None:
        out.append(self.key[x])
        for c in self.children(x):
            self.preorder_recursive(c, out)

    def preorder_stack(self, root: int) -> list[int]:
        out: list[int] = []
        st = [root]
        while st:
            x = st.pop()
            out.append(self.key[x])
            for c in reversed(self.children(x)):
                st.append(c)
        return out

    def depth(self, x: int) -> int:
        ch = self.children(x)
        return 1 + max((self.depth(c) for c in ch), default=0)


class BinTree:
    """Binary tree with parent pointers."""

    def __init__(self) -> None:
        self.key: list[int] = []
        self.left: list[int] = []
        self.right: list[int] = []
        self.parent: list[int] = []

    def insert_bst(self, k: int) -> None:
        z = len(self.key)
        self.key.append(k)
        self.left.append(NIL)
        self.right.append(NIL)
        self.parent.append(NIL)
        if z == 0:
            return
        x = 0
        while True:
            side = self.left if k < self.key[x] else self.right
            if side[x] == NIL:
                side[x] = z
                self.parent[z] = x
                return
            x = side[x]

    def inorder_recursive(self, x: int, out: list[int]) -> None:
        if x == NIL:
            return
        self.inorder_recursive(self.left[x], out)
        out.append(self.key[x])
        self.inorder_recursive(self.right[x], out)

    def inorder_constant_space(self) -> list[int]:
        """Exercise 10.4-5: inorder traversal with O(1) extra space using parent pointers and prev."""
        out: list[int] = []
        if not self.key:
            return out
        cur, prev = 0, NIL
        while cur != NIL:
            if prev == self.parent[cur]:
                if self.left[cur] != NIL:
                    prev, cur = cur, self.left[cur]
                    continue
                out.append(self.key[cur])
                nxt = self.right[cur] if self.right[cur] != NIL else self.parent[cur]
                prev, cur = cur, nxt
            elif prev == self.left[cur]:
                out.append(self.key[cur])
                nxt = self.right[cur] if self.right[cur] != NIL else self.parent[cur]
                prev, cur = cur, nxt
            else:
                prev, cur = cur, self.parent[cur]
        return out


def demo() -> str:
    lines: list[str] = []
    t = RootedTree.from_children([10, 20, 30, 40, 50, 60, 70], [[1, 2, 3], [4, 5], [], [6], [], [], []])
    v: list[int] = []
    t.preorder_recursive(0, v)
    lines.append(f"preorder (recursive): {v}")
    lines.append(f"preorder (stack)    : {t.preorder_stack(0)}")
    lines.append(f"children of root    : {t.children(0)}, depth = {t.depth(0)}")
    b = BinTree()
    for k in [8, 3, 10, 1, 6, 14, 4, 7, 13]:
        b.insert_bst(k)
    lines.append(f"inorder, O(1) extra space: {b.inorder_constant_space()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_lcrs_traversals_agree(self) -> None:
        t = RootedTree.from_children([1, 2, 3, 4, 5], [[1, 2], [3, 4], [], [], []])
        a: list[int] = []
        t.preorder_recursive(0, a)
        self.assertEqual(a, [1, 2, 4, 5, 3])
        self.assertEqual(t.preorder_stack(0), a)
        self.assertEqual(t.depth(0), 3)

    def test_constant_space_inorder_matches_recursive(self) -> None:
        r = Rng(51)
        for n in [0, 1, 2, 5, 30, 200]:
            b = BinTree()
            for k in r.vec(n, 0, 1000):
                b.insert_bst(k)
            rec: list[int] = []
            if n > 0:
                b.inorder_recursive(0, rec)
            self.assertEqual(b.inorder_constant_space(), rec)


if __name__ == "__main__":
    print(demo(), end="")
