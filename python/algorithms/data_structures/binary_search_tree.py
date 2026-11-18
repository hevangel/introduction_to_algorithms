"""Binary Search Trees — CLRS 12.1-12.3 (1e-4e) (Editions: 1,2,3,4).

Inorder walk, search, min/max, successor/predecessor, insert and delete (TRANSPLANT) in O(h).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

NIL = -1


@dataclass
class Node:
    key: int
    left: int = NIL
    right: int = NIL
    p: int = NIL


class Bst:
    """Binary search tree in an arena; nodes keep parent pointers as in CLRS."""

    def __init__(self) -> None:
        self.nodes: list[Node] = []
        self.root: int = NIL
        self._len: int = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def insert(self, key: int) -> int:
        """TREE-INSERT(T, z): O(h). Returns the handle of the new node."""
        z = len(self.nodes)
        self.nodes.append(Node(key=key))
        y, x = NIL, self.root
        while x != NIL:
            y = x
            x = self.nodes[x].left if key < self.nodes[x].key else self.nodes[x].right
        self.nodes[z].p = y
        if y == NIL:
            self.root = z
        elif key < self.nodes[y].key:
            self.nodes[y].left = z
        else:
            self.nodes[y].right = z
        self._len += 1
        return z

    def search(self, key: int) -> int:
        """ITERATIVE-TREE-SEARCH."""
        x = self.root
        while x != NIL and key != self.nodes[x].key:
            x = self.nodes[x].left if key < self.nodes[x].key else self.nodes[x].right
        return x

    def search_recursive(self, x: int, key: int) -> int:
        """TREE-SEARCH (recursive)."""
        if x == NIL or key == self.nodes[x].key:
            return x
        if key < self.nodes[x].key:
            return self.search_recursive(self.nodes[x].left, key)
        return self.search_recursive(self.nodes[x].right, key)

    def minimum(self, x: int) -> int:
        while self.nodes[x].left != NIL:
            x = self.nodes[x].left
        return x

    def maximum(self, x: int) -> int:
        while self.nodes[x].right != NIL:
            x = self.nodes[x].right
        return x

    def successor(self, x: int) -> int:
        """TREE-SUCCESSOR: next key in sorted order (NIL if x is the maximum)."""
        if self.nodes[x].right != NIL:
            return self.minimum(self.nodes[x].right)
        y = self.nodes[x].p
        while y != NIL and x == self.nodes[y].right:
            x = y
            y = self.nodes[y].p
        return y

    def predecessor(self, x: int) -> int:
        if self.nodes[x].left != NIL:
            return self.maximum(self.nodes[x].left)
        y = self.nodes[x].p
        while y != NIL and x == self.nodes[y].left:
            x = y
            y = self.nodes[y].p
        return y

    def inorder_walk(self, x: int, out: list[int]) -> None:
        """INORDER-TREE-WALK (recursive)."""
        if x != NIL:
            self.inorder_walk(self.nodes[x].left, out)
            out.append(self.nodes[x].key)
            self.inorder_walk(self.nodes[x].right, out)

    def keys(self) -> list[int]:
        """Inorder keys using successor links (no recursion, safe for degenerate trees)."""
        v: list[int] = []
        if self.root == NIL:
            return v
        x = self.minimum(self.root)
        while x != NIL:
            v.append(self.nodes[x].key)
            x = self.successor(x)
        return v

    def _transplant(self, u: int, v: int) -> None:
        up = self.nodes[u].p
        if up == NIL:
            self.root = v
        elif u == self.nodes[up].left:
            self.nodes[up].left = v
        else:
            self.nodes[up].right = v
        if v != NIL:
            self.nodes[v].p = up

    def delete(self, z: int) -> None:
        """TREE-DELETE(T, z)."""
        if self.nodes[z].left == NIL:
            self._transplant(z, self.nodes[z].right)
        elif self.nodes[z].right == NIL:
            self._transplant(z, self.nodes[z].left)
        else:
            y = self.minimum(self.nodes[z].right)
            if self.nodes[y].p != z:
                self._transplant(y, self.nodes[y].right)
                self.nodes[y].right = self.nodes[z].right
                self.nodes[self.nodes[y].right].p = y
            self._transplant(z, y)
            self.nodes[y].left = self.nodes[z].left
            self.nodes[self.nodes[y].left].p = y
        self._len -= 1

    def delete_key(self, key: int) -> bool:
        z = self.search(key)
        if z == NIL:
            return False
        self.delete(z)
        return True

    def height(self) -> int:
        """Height = number of edges on the longest root-to-leaf path (-1 for empty)."""
        if self.root == NIL:
            return -1
        level = [self.root]
        h = -1
        while level:
            h += 1
            nxt: list[int] = []
            for x in level:
                if self.nodes[x].left != NIL:
                    nxt.append(self.nodes[x].left)
                if self.nodes[x].right != NIL:
                    nxt.append(self.nodes[x].right)
            level = nxt
        return h


def demo() -> str:
    lines: list[str] = []
    t = Bst()
    for k in [15, 6, 18, 3, 7, 17, 20, 2, 4, 13, 9]:
        t.insert(k)
    w: list[int] = []
    t.inorder_walk(t.root, w)
    lines.append(f"inorder walk      : {w}")
    lines.append(f"height            : {t.height()}")
    mn, mx = t.minimum(t.root), t.maximum(t.root)
    lines.append(f"minimum / maximum : {t.nodes[mn].key} / {t.nodes[mx].key}")
    x = t.search(13)
    lines.append(
        f"successor(13)     : {t.nodes[t.successor(x)].key}, predecessor(13): {t.nodes[t.predecessor(x)].key}"
    )
    t.delete_key(6)
    lines.append(f"after delete(6)   : {t.keys()}")
    t.delete_key(15)
    lines.append(f"after delete(15)  : {t.keys()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_btreeset_under_random_operations(self) -> None:
        t = Bst()
        s: set[int] = set()
        r = Rng(60)
        for _ in range(4000):
            k = r.range(0, 150)
            op = r.range(0, 2)
            if op == 0:
                if k not in s:
                    s.add(k)
                    t.insert(k)
            elif op == 1:
                before = k in s
                s.discard(k)
                self.assertEqual(t.delete_key(k), before)
            else:
                self.assertEqual(t.search(k) != NIL, k in s)
            self.assertEqual(t.keys(), sorted(s))
        self.assertEqual(len(t), len(s))

    def test_successor_predecessor_min_max(self) -> None:
        t = Bst()
        for k in [8, 3, 10, 1, 6, 14]:
            t.insert(k)
        self.assertEqual(t.nodes[t.successor(t.search(6))].key, 8)
        self.assertEqual(t.nodes[t.predecessor(t.search(8))].key, 6)
        self.assertEqual(t.successor(t.search(14)), NIL)
        self.assertEqual(t.predecessor(t.search(1)), NIL)
        self.assertEqual(t.nodes[t.minimum(t.root)].key, 1)
        self.assertEqual(t.nodes[t.maximum(t.root)].key, 14)
        self.assertEqual(t.search_recursive(t.root, 10), t.search(10))

    def test_degenerate_tree_has_linear_height(self) -> None:
        t = Bst()
        for k in range(100):
            t.insert(k)
        self.assertEqual(t.height(), 99)
        self.assertEqual(Bst().height(), -1)


if __name__ == "__main__":
    print(demo(), end="")
