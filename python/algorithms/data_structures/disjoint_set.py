"""Disjoint-Set Union (Union-Find) — CLRS 21 (3e) / 19 (4e) / 22 (1e, 2e) (Editions: 1,2,3,4).

Linked-list sets with weighted union and disjoint-set forests with union by rank +
path compression.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class DisjointSetForest:
    """Disjoint-set forest: union by rank and path compression give O(m alpha(n))."""

    def __init__(self, n: int) -> None:
        self._p = list(range(n))
        self._rank = [0] * n
        self._sets = n

    def make_set(self) -> int:
        """MAKE-SET: add a new singleton, returns its index."""
        idx = len(self._p)
        self._p.append(idx)
        self._rank.append(0)
        self._sets += 1
        return idx

    def find(self, x: int) -> int:
        """FIND-SET with two-pass path compression."""
        r = x
        while self._p[r] != r:
            r = self._p[r]
        c = x
        while self._p[c] != r:
            nxt = self._p[c]
            self._p[c] = r
            c = nxt
        return r

    def union(self, x: int, y: int) -> bool:
        """UNION by rank. Returns False if already in the same set."""
        rx, ry = self.find(x), self.find(y)
        if rx == ry:
            return False
        if self._rank[rx] > self._rank[ry]:
            self._p[ry] = rx
        else:
            self._p[rx] = ry
            if self._rank[rx] == self._rank[ry]:
                self._rank[ry] += 1
        self._sets -= 1
        return True

    def same_set(self, x: int, y: int) -> bool:
        return self.find(x) == self.find(y)

    def set_count(self) -> int:
        return self._sets


class LinkedListSets:
    """Linked-list representation with weighted-union heuristic (Section 21.2)."""

    def __init__(self, n: int) -> None:
        self._rep = list(range(n))
        self._members = [[i] for i in range(n)]
        self.pointer_updates: int = 0

    def find(self, x: int) -> int:
        return self._rep[x]

    def union(self, x: int, y: int) -> bool:
        a, b = self._rep[x], self._rep[y]
        if a == b:
            return False
        if len(self._members[a]) < len(self._members[b]):
            a, b = b, a
        moved = self._members[b]
        self._members[b] = []
        for m in moved:
            self._rep[m] = a
        self.pointer_updates += len(moved)
        self._members[a].extend(moved)
        return True


def connected_components(n: int, edges: list[tuple[int, int]]) -> tuple[list[int], int]:
    """Section 21.1: connected components of an undirected graph."""
    d = DisjointSetForest(n)
    for u, v in edges:
        d.union(u, v)
    labels = [d.find(v) for v in range(n)]
    return labels, d.set_count()


def demo() -> str:
    lines: list[str] = []
    edges = [(0, 1), (1, 2), (3, 4), (5, 6), (6, 7), (7, 5), (8, 8)]
    labels, count = connected_components(9, edges)
    lines.append(f"edges {edges}")
    lines.append(f"{count} connected components; labels = {labels}")
    f = DisjointSetForest(8)
    lines.append(f"same_set(0,1) before union: {str(f.same_set(0, 1)).lower()}")
    f.union(0, 1)
    f.union(2, 3)
    f.union(1, 3)
    lines.append(
        f"after unions: same_set(0,2) = {str(f.same_set(0, 2)).lower()}, "
        f"same_set(0,4) = {str(f.same_set(0, 4)).lower()}, sets = {f.set_count()}"
    )
    n = 1 << 12
    l = LinkedListSets(n)
    step = 1
    while step < n:
        for i in range(0, n, 2 * step):
            l.union(i, i + step)
        step *= 2
    lines.append(
        f"weighted union of {n} singletons (balanced merges): {l.pointer_updates} pointer updates = n*lg n / 2 = {n * 12 // 2}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_forest_matches_naive_labels(self) -> None:
        r = Rng(84)
        n = 120
        f = DisjointSetForest(n)
        l = LinkedListSets(n)
        label = list(range(n))
        for _ in range(300):
            a, b = r.range(0, n - 1), r.range(0, n - 1)
            la, lb = label[a], label[b]
            self.assertEqual(f.union(a, b), la != lb)
            self.assertEqual(l.union(a, b), la != lb)
            if la != lb:
                label = [la if x == lb else x for x in label]
            c, d = r.range(0, n - 1), r.range(0, n - 1)
            self.assertEqual(f.same_set(c, d), label[c] == label[d])
            self.assertEqual(l.find(c) == l.find(d), label[c] == label[d])
        self.assertEqual(f.set_count(), len(set(label)))

    def test_weighted_union_bound(self) -> None:
        n = 1024
        l = LinkedListSets(n)
        r = Rng(85)
        for _ in range(5000):
            l.union(r.range(0, n - 1), r.range(0, n - 1))
        self.assertLessEqual(l.pointer_updates, n * 10)

    def test_components_and_make_set(self) -> None:
        labels, c = connected_components(6, [(0, 1), (2, 3), (3, 4)])
        self.assertEqual(c, 3)
        self.assertEqual(labels[2], labels[4])
        self.assertNotEqual(labels[0], labels[2])
        f = DisjointSetForest(0)
        a = f.make_set()
        b = f.make_set()
        self.assertTrue(f.union(a, b))
        self.assertEqual(f.set_count(), 1)


if __name__ == "__main__":
    print(demo(), end="")
