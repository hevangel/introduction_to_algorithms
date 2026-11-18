"""Pairing Heaps — CLRS Companion priority queue (Ch 20 1e-2e / Ch 19 3e) (Editions: 1,2,3).

A self-adjusting heap: meld is O(1), delete-min pairs siblings in two passes (O(lg n) amortised), decrease-key cuts a subtree.
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
    child: int = NIL
    sibling: int = NIL
    prev: int = NIL  # parent if first child, else previous sibling


class PairingHeap:
    """Min pairing heap in an arena; handles are arena indices."""

    def __init__(self) -> None:
        self.n: list[_Node] = []
        self.root = NIL
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def key(self, h: int) -> int:
        return self.n[h].key

    def minimum(self) -> int | None:
        return None if self.root == NIL else self.n[self.root].key

    def _meld(self, a: int, b: int) -> int:
        """MELD two trees: the one with the larger root becomes the first child of the other. O(1)."""
        if a == NIL:
            return b
        if b == NIL:
            return a
        p, c = (a, b) if self.n[a].key <= self.n[b].key else (b, a)
        first = self.n[p].child
        self.n[c].sibling = first
        if first != NIL:
            self.n[first].prev = c
        self.n[c].prev = p
        self.n[p].child = c
        return p

    def insert(self, key: int) -> int:
        self.n.append(_Node(key))
        x = len(self.n) - 1
        self.root = self._meld(self.root, x)
        self._len += 1
        return x

    def union(self, other: PairingHeap) -> int:
        """Meld another heap into this one (its handles are shifted by the returned offset)."""
        off = len(self.n)

        def sh(i: int) -> int:
            return NIL if i == NIL else i + off

        for nd in other.n:
            self.n.append(_Node(nd.key, sh(nd.child), sh(nd.sibling), sh(nd.prev)))
        self.root = self._meld(self.root, sh(other.root))
        self._len += other._len
        return off

    def extract_min(self) -> tuple[int, int] | None:
        """DELETE-MIN: detach the root and combine its children with the two-pass pairing heuristic."""
        r = self.root
        if r == NIL:
            return None
        kids: list[int] = []
        c = self.n[r].child
        while c != NIL:
            nxt = self.n[c].sibling
            self.n[c].sibling = NIL
            self.n[c].prev = NIL
            kids.append(c)
            c = nxt
        # pass 1: meld pairs left to right
        paired: list[int] = []
        for i in range(0, len(kids), 2):
            if i + 1 < len(kids):
                paired.append(self._meld(kids[i], kids[i + 1]))
            else:
                paired.append(kids[i])
        # pass 2: meld right to left
        acc = NIL
        for t in reversed(paired):
            acc = self._meld(t, acc)
        self.root = acc
        if acc != NIL:
            self.n[acc].prev = NIL
        self._len -= 1
        self.n[r].child = NIL
        return (r, self.n[r].key)

    def decrease_key(self, x: int, k: int) -> None:
        """DECREASE-KEY: cut the node (with its subtree) out of its parent's child list and meld it with the root."""
        if k > self.n[x].key:
            raise ValueError("new key is greater than current key")
        self.n[x].key = k
        if x == self.root:
            return
        p, s = self.n[x].prev, self.n[x].sibling
        if self.n[p].child == x:
            self.n[p].child = s
        else:
            self.n[p].sibling = s
        if s != NIL:
            self.n[s].prev = p
        self.n[x].sibling = NIL
        self.n[x].prev = NIL
        self.root = self._meld(self.root, x)


def demo() -> str:
    lines: list[str] = []
    h = PairingHeap()
    hs = [h.insert(k) for k in [8, 3, 5, 9, 1, 7, 2, 6]]
    lines.append(f"minimum = Some({h.minimum()}), size = {len(h)}")
    h.decrease_key(hs[3], 0)  # 9 -> 0
    lines.append(f"after decrease-key(9 -> 0): minimum = Some({h.minimum()})")
    order: list[int] = []
    while (e := h.extract_min()) is not None:
        order.append(e[1])
    lines.append(f"extract-min order: {order}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_heapsort_and_union(self) -> None:
        r = Rng(324)
        a, b = r.vec(200, -100, 100), r.vec(80, -100, 100)
        ha, hb = PairingHeap(), PairingHeap()
        for k in a:
            ha.insert(k)
        for k in b:
            hb.insert(k)
        ha.union(hb)
        out: list[int] = []
        while (e := ha.extract_min()) is not None:
            out.append(e[1])
        self.assertEqual(out, sorted(a + b))

    def test_random_decrease_key_matches_model(self) -> None:
        r = Rng(325)
        h = PairingHeap()
        alive: list[int] = []
        for _ in range(3000):
            v = r.range(0, 3)
            if v in (0, 1):
                alive.append(h.insert(r.range(0, 10_000)))
            elif v == 2 and alive:
                i = r.range(0, len(alive) - 1)
                nk = h.key(alive[i]) - r.range(0, 400)
                h.decrease_key(alive[i], nk)
            elif alive:
                hd, k = h.extract_min()  # type: ignore[misc]
                self.assertEqual(k, min(h.key(a) for a in alive))
                alive = [a for a in alive if a != hd]
            self.assertEqual(len(h), len(alive))
            self.assertEqual(
                h.minimum(), min((h.key(a) for a in alive), default=None)
            )

    def test_errors_and_empty(self) -> None:
        h = PairingHeap()
        self.assertIsNone(h.extract_min())
        x = h.insert(5)
        with self.assertRaises(ValueError):
            h.decrease_key(x, 6)
        h.decrease_key(x, 1)
        self.assertEqual(h.extract_min(), (x, 1))


if __name__ == "__main__":
    print(demo(), end="")
