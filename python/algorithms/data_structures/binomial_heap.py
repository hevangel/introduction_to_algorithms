"""Binomial Heaps — CLRS 19 (2e) / 20 (1e) (Editions: 1,2).

Mergeable heap built from binomial trees: UNION in O(lg n) by "binary addition" of root lists.
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
class _Node:
    key: int
    degree: int = 0
    p: int = NIL
    child: int = NIL
    sibling: int = NIL


class BinomialHeap:
    """Binomial min-heap in an arena; roots form a singly linked list sorted by increasing degree."""

    def __init__(self) -> None:
        self._n: list[_Node] = []
        self._head: int = NIL
        self._count: int = 0

    def __len__(self) -> int:
        return self._count

    def is_empty(self) -> bool:
        return self._count == 0

    def key(self, h: int) -> int:
        return self._n[h].key

    def minimum(self) -> int | None:
        """BINOMIAL-HEAP-MINIMUM."""
        x = self._head
        best: int | None = None
        while x != NIL:
            k = self._n[x].key
            best = k if best is None else min(best, k)
            x = self._n[x].sibling
        return best

    def root_degrees(self) -> list[int]:
        """Degrees of the root trees (the binary representation of n)."""
        v: list[int] = []
        x = self._head
        while x != NIL:
            v.append(self._n[x].degree)
            x = self._n[x].sibling
        return v

    def _link(self, y: int, z: int) -> None:
        self._n[y].p = z
        self._n[y].sibling = self._n[z].child
        self._n[z].child = y
        self._n[z].degree += 1

    def _merge_lists(self, a: int, b: int) -> int:
        head = NIL
        tail = NIL
        while a != NIL or b != NIL:
            if b == NIL or (a != NIL and self._n[a].degree <= self._n[b].degree):
                pick = a
                a = self._n[a].sibling
            else:
                pick = b
                b = self._n[b].sibling
            if head == NIL:
                head = pick
            else:
                self._n[tail].sibling = pick
            tail = pick
        return head

    def _union_lists(self, h1: int, h2: int) -> int:
        head = self._merge_lists(h1, h2)
        if head == NIL:
            return NIL
        prev = NIL
        x = head
        nxt = self._n[x].sibling
        while nxt != NIL:
            after = self._n[nxt].sibling
            if self._n[x].degree != self._n[nxt].degree or (
                after != NIL and self._n[after].degree == self._n[x].degree
            ):
                prev = x
                x = nxt
            elif self._n[x].key <= self._n[nxt].key:
                self._n[x].sibling = after
                self._link(nxt, x)
            else:
                if prev == NIL:
                    head = nxt
                else:
                    self._n[prev].sibling = nxt
                self._link(x, nxt)
                x = nxt
            nxt = self._n[x].sibling
        return head

    def union(self, other: BinomialHeap) -> int:
        """BINOMIAL-HEAP-UNION(H1, H2): O(lg n). Handles of `other` shift by returned offset."""
        off = len(self._n)

        def shift(i: int) -> int:
            return NIL if i == NIL else i + off

        for nd in other._n:
            self._n.append(
                _Node(
                    key=nd.key,
                    degree=nd.degree,
                    p=shift(nd.p),
                    child=shift(nd.child),
                    sibling=shift(nd.sibling),
                )
            )
        oh = shift(other._head)
        self._head = self._union_lists(self._head, oh)
        self._count += other._count
        return off

    def insert(self, key: int) -> int:
        """BINOMIAL-HEAP-INSERT: union with a one-node heap. Returns node handle."""
        x = len(self._n)
        self._n.append(_Node(key=key))
        self._head = self._union_lists(self._head, x)
        self._count += 1
        return x

    def extract_min(self) -> tuple[int, int] | None:
        """BINOMIAL-HEAP-EXTRACT-MIN: returns (handle, key)."""
        if self._head == NIL:
            return None
        best = self._head
        best_prev = NIL
        prev = NIL
        x = self._head
        while x != NIL:
            if self._n[x].key < self._n[best].key:
                best = x
                best_prev = prev
            prev = x
            x = self._n[x].sibling
        if best_prev == NIL:
            self._head = self._n[best].sibling
        else:
            self._n[best_prev].sibling = self._n[best].sibling

        rev = NIL
        c = self._n[best].child
        while c != NIL:
            nxt = self._n[c].sibling
            self._n[c].sibling = rev
            self._n[c].p = NIL
            rev = c
            c = nxt
        self._n[best].child = NIL
        self._head = self._union_lists(self._head, rev)
        self._count -= 1
        return (best, self._n[best].key)

    def decrease_key(self, x: int, k: int) -> None:
        """BINOMIAL-HEAP-DECREASE-KEY: bubble smaller key up by exchanging keys with ancestors."""
        if k > self._n[x].key:
            raise ValueError("new key is greater than current key")
        self._n[x].key = k
        y = x
        z = self._n[y].p
        while z != NIL and self._n[y].key < self._n[z].key:
            self._n[y].key, self._n[z].key = self._n[z].key, self._n[y].key
            y = z
            z = self._n[y].p

    def delete(self, x: int) -> None:
        """BINOMIAL-HEAP-DELETE: decrease to -infinity and extract."""
        self.decrease_key(x, -(1 << 63))
        self.extract_min()


def demo() -> str:
    lines: list[str] = []
    h = BinomialHeap()
    for k in [12, 7, 25, 15, 28, 33, 41]:
        h.insert(k)
    lines.append(f"7 inserts: root degrees = {h.root_degrees()} (7 = 111_2 -> trees B0, B1, B2)")
    h.insert(1)
    lines.append(f"8th insert: root degrees = {h.root_degrees()} (8 = 1000_2 -> single B3)")
    lines.append(f"minimum = {h.minimum()}")
    other = BinomialHeap()
    for k in [3, 9, 6]:
        other.insert(k)
    h.union(other)
    lines.append(f"after union with {{3,9,6}}: size {len(h)} degrees {h.root_degrees()}")
    out: list[int] = []
    while (item := h.extract_min()) is not None:
        out.append(item[1])
    lines.append(f"extract-min order: {out}")
    return "\n".join(lines) + "\n"


def _binary_degrees(n: int) -> list[int]:
    v: list[int] = []
    bit = 0
    while n > 0:
        if n & 1:
            v.append(bit)
        n >>= 1
        bit += 1
    return v


class Tests(unittest.TestCase):
    def test_root_list_mirrors_binary_representation(self) -> None:
        h = BinomialHeap()
        for k in range(200):
            h.insert((k * 31) % 17)
            self.assertEqual(h.root_degrees(), _binary_degrees(len(h)))

    def test_heapsort_and_union(self) -> None:
        r = Rng(82)
        a, b = r.vec(150, -100, 100), r.vec(77, -100, 100)
        ha, hb = BinomialHeap(), BinomialHeap()
        for k in a:
            ha.insert(k)
        for k in b:
            hb.insert(k)
        ha.union(hb)
        self.assertEqual(ha.root_degrees(), _binary_degrees(227))
        out: list[int] = []
        while (item := ha.extract_min()) is not None:
            out.append(item[1])
        self.assertEqual(out, sorted(a + b))

    def test_decrease_key_and_delete_keep_multiset_semantics(self) -> None:
        r = Rng(83)
        h = BinomialHeap()
        alive: list[int] = []
        for _ in range(2000):
            op = r.range(0, 3)
            if op in (0, 1):
                alive.append(h.insert(r.range(0, 5000)))
            elif op == 2 and alive:
                i = r.range(0, len(alive) - 1)
                h.decrease_key(alive[i], h.key(alive[i]) - r.range(0, 300))
            elif alive:
                item = h.extract_min()
                assert item is not None
                hd, _ = item
                alive = [a for a in alive if a != hd]
            self.assertEqual(len(h), len(alive))
            expected_min = min((h.key(a) for a in alive), default=None)
            self.assertEqual(h.minimum(), expected_min)
        victim = alive.pop(len(alive) // 2)
        h.delete(victim)
        self.assertEqual(len(h), len(alive))


if __name__ == "__main__":
    print(demo(), end="")
