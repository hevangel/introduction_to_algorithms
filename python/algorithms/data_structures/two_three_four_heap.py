"""2-3-4 Heaps — CLRS Problem 20-1 (1e); Problem 19-1 (2e–3e) (1,2,3).

Store keys only in leaves with subtree minima cached in internal 2-3-4 nodes for O(lg n) minimum, insert, decrease-key, extract-min, delete, and meld.
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
class _Leaf:
    key: int
    parent: int | None = None
    alive: bool = True


@dataclass
class _Internal:
    small: int
    height: int
    parent: int | None
    children: list[int]


_Node = _Leaf | _Internal


class TwoThreeFourHeap:
    """Arena-backed 2-3-4 heap supporting O(lg n) minimum, insert, decrease_key, extract_min, delete, and union."""

    def __init__(self) -> None:
        self.nodes: list[_Node | None] = []
        self.root: int | None = None
        self._len = 0

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def _height_of(self, idx: int) -> int:
        nd = self.nodes[idx]
        assert nd is not None
        return 0 if isinstance(nd, _Leaf) else nd.height

    def _small_of(self, idx: int) -> int:
        nd = self.nodes[idx]
        assert nd is not None
        return nd.key if isinstance(nd, _Leaf) else nd.small

    def _set_parent(self, idx: int, parent: int | None) -> None:
        nd = self.nodes[idx]
        assert nd is not None
        nd.parent = parent

    def _recompute_small(self, idx: int) -> None:
        nd = self.nodes[idx]
        assert nd is not None
        if isinstance(nd, _Internal):
            nd.small = min(self._small_of(c) for c in nd.children)

    def _bubble_small_up(self, cur: int | None) -> None:
        while cur is not None:
            self._recompute_small(cur)
            nd = self.nodes[cur]
            assert nd is not None
            cur = nd.parent

    def _alloc_internal(self, height: int, parent: int | None, children: list[int]) -> int:
        small = min(self._small_of(c) for c in children)
        idx = len(self.nodes)
        for c in children:
            self._set_parent(c, idx)
        self.nodes.append(_Internal(small, height, parent, children))
        return idx

    def _split_four_node(self, idx: int) -> None:
        nd = self.nodes[idx]
        assert isinstance(nd, _Internal) and len(nd.children) == 4
        right_children = nd.children[2:]
        nd.children = nd.children[:2]
        height, parent = nd.height, nd.parent
        self._recompute_small(idx)
        right_id = self._alloc_internal(height, parent, right_children)
        if parent is None:
            self.root = self._alloc_internal(height + 1, None, [idx, right_id])
        else:
            pnd = self.nodes[parent]
            assert isinstance(pnd, _Internal)
            pos = pnd.children.index(idx)
            pnd.children.insert(pos + 1, right_id)
            self._recompute_small(parent)

    def _ensure_root_not_four(self) -> None:
        if self.root is not None:
            nd = self.nodes[self.root]
            if isinstance(nd, _Internal) and len(nd.children) == 4:
                self._split_four_node(self.root)

    def minimum(self) -> int | None:
        return None if self.root is None else self._small_of(self.root)

    def _insert_leaf_id(self, leaf_id: int) -> None:
        if self.root is None:
            self._set_parent(leaf_id, None)
            self.root = leaf_id
            return
        if self._height_of(self.root) == 0:
            self.root = self._alloc_internal(1, None, [self.root, leaf_id])
            return
        self._ensure_root_not_four()
        cur = self.root
        while self._height_of(cur) > 1:
            cnd = self.nodes[cur]
            assert isinstance(cnd, _Internal)
            nxt = min(cnd.children, key=lambda c: len(self.nodes[c].children) if isinstance(self.nodes[c], _Internal) else 0)  # type: ignore[union-attr]
            nnd = self.nodes[nxt]
            if isinstance(nnd, _Internal) and len(nnd.children) == 4:
                self._split_four_node(nxt)
            cur = nxt
        cnd = self.nodes[cur]
        assert isinstance(cnd, _Internal)
        cnd.children.append(leaf_id)
        self._set_parent(leaf_id, cur)
        self._bubble_small_up(cur)

    def insert(self, key: int) -> int:
        leaf_id = len(self.nodes)
        self.nodes.append(_Leaf(key))
        self._len += 1
        self._insert_leaf_id(leaf_id)
        return leaf_id

    def decrease_key(self, handle: int, new_key: int) -> None:
        nd = self.nodes[handle]
        assert isinstance(nd, _Leaf) and nd.alive and new_key <= nd.key
        nd.key = new_key
        self._bubble_small_up(nd.parent)

    def delete(self, handle: int) -> int:
        nd = self.nodes[handle]
        assert isinstance(nd, _Leaf) and nd.alive
        nd.alive = False
        key, cur = nd.key, nd.parent
        self._len -= 1
        if cur is None:
            self.root = None
            return key
        cnd = self.nodes[cur]
        assert isinstance(cnd, _Internal)
        cnd.children = [c for c in cnd.children if c != handle]
        while True:
            cnd = self.nodes[cur]
            assert isinstance(cnd, _Internal)
            deg = len(cnd.children)
            if cur == self.root:
                if deg == 1:
                    only = cnd.children[0]
                    self._set_parent(only, None)
                    self.nodes[cur] = None
                    self.root = only
                else:
                    self._recompute_small(cur)
                return key
            if deg >= 2:
                self._bubble_small_up(cur)
                return key
            gp = cnd.parent
            assert gp is not None
            gnd = self.nodes[gp]
            assert isinstance(gnd, _Internal)
            cur_pos = gnd.children.index(cur)
            sib = gnd.children[cur_pos - 1] if cur_pos > 0 else gnd.children[1]
            snd = self.nodes[sib]
            assert isinstance(snd, _Internal)
            if len(snd.children) >= 3:
                borrowed = snd.children.pop() if cur_pos > 0 else snd.children.pop(0)
                self._set_parent(borrowed, cur)
                if cur_pos > 0:
                    cnd.children.insert(0, borrowed)
                else:
                    cnd.children.append(borrowed)
                self._recompute_small(sib)
                self._bubble_small_up(cur)
                return key
            only = cnd.children[0]
            self._set_parent(only, sib)
            if cur_pos > 0:
                snd.children.append(only)
            else:
                snd.children.insert(0, only)
            gnd.children.pop(cur_pos)
            self.nodes[cur] = None
            self._recompute_small(sib)
            cur = gp

    def extract_min_with_handle(self) -> tuple[int, int] | None:
        if self.root is None:
            return None
        cur = self.root
        while isinstance(self.nodes[cur], _Internal):
            nd = self.nodes[cur]
            assert isinstance(nd, _Internal)
            target = nd.small
            cur = next(c for c in nd.children if self._small_of(c) == target)
        return (cur, self.delete(cur))

    def extract_min(self) -> int | None:
        res = self.extract_min_with_handle()
        return None if res is None else res[1]

    def union(self, other: TwoThreeFourHeap) -> list[int | None]:
        mapping: list[int | None] = [None] * len(other.nodes)
        if other.root is None:
            return mapping
        base = len(self.nodes)
        for old_id, node in enumerate(other.nodes):
            if node is None:
                self.nodes.append(None)
            elif isinstance(node, _Leaf):
                if node.alive:
                    mapping[old_id] = base + old_id
                self.nodes.append(_Leaf(node.key, None if node.parent is None else base + node.parent, node.alive))
            else:
                self.nodes.append(
                    _Internal(
                        node.small,
                        node.height,
                        None if node.parent is None else base + node.parent,
                        [c + base for c in node.children],
                    )
                )
        other_root = other.root + base
        self._len += other._len
        if self.root is None:
            self.root = other_root
            return mapping
        r1 = self.root
        h1, h2 = self._height_of(r1), self._height_of(other_root)
        if h2 == 0:
            self._insert_leaf_id(other_root)
            return mapping
        if h1 == 0:
            self.root = other_root
            self._insert_leaf_id(r1)
            return mapping
        if h1 == h2:
            self.root = self._alloc_internal(h1 + 1, None, [r1, other_root])
            return mapping
        if h1 > h2:
            short_root, short_h = other_root, h2
        else:
            self.root = other_root
            short_root, short_h = r1, h1
        self._ensure_root_not_four()
        cur = self.root
        while self._height_of(cur) > short_h + 1:
            nd = self.nodes[cur]
            assert isinstance(nd, _Internal)
            nxt = nd.children[0]
            nnd = self.nodes[nxt]
            if isinstance(nnd, _Internal) and len(nnd.children) == 4:
                self._split_four_node(nxt)
            cur = nxt
        nd = self.nodes[cur]
        assert isinstance(nd, _Internal)
        nd.children.append(short_root)
        self._set_parent(short_root, cur)
        self._bubble_small_up(cur)
        return mapping

    def check_invariants(self) -> None:
        if self.root is None:
            assert self._len == 0
            return

        def dfs(idx: int, expected_h: int, parent: int | None) -> tuple[int, int]:
            nd = self.nodes[idx]
            assert nd is not None and nd.parent == parent
            if isinstance(nd, _Leaf):
                assert nd.alive and expected_h == 0
                return (nd.key, 1)
            assert nd.height == expected_h and 2 <= len(nd.children) <= 4
            min_k = 1 << 62
            total = 0
            for c in nd.children:
                ck, cnt = dfs(c, expected_h - 1, idx)
                min_k = min(min_k, ck)
                total += cnt
            assert nd.small == min_k
            return (nd.small, total)

        _, count = dfs(self.root, self._height_of(self.root), None)
        assert count == self._len


def demo() -> str:
    lines: list[str] = []
    h1 = TwoThreeFourHeap()
    handles = [h1.insert(k) for k in [24, 15, 38, 7, 19]]
    lines.append(f"h1 after inserts [24,15,38,7,19]: min = {h1.minimum()}")
    h1.decrease_key(handles[2], 3)
    lines.append(f"after decrease_key(38 -> 3)    : min = {h1.minimum()}")
    h2 = TwoThreeFourHeap()
    for k in [11, 2, 30]:
        h2.insert(k)
    h1.union(h2)
    lines.append(f"after union with [11,2,30]     : len = {len(h1)}, min = {h1.minimum()}")
    popped: list[int] = []
    while (v := h1.extract_min()) is not None:
        popped.append(v)
    lines.append(f"extract_min sequence           : {popped}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_randomized_2000_ops_match_oracle(self) -> None:
        r = Rng(1901)
        heap = TwoThreeFourHeap()
        oracle: dict[int, int] = {}
        live: list[int] = []
        for step in range(2000):
            choice = r.range(0, 99)
            if choice < 45 or not live:
                k = r.range(0, 10_000) - 5000
                idx = heap.insert(k)
                oracle[idx] = k
                live.append(idx)
            elif choice < 65:
                idx = live[r.range(0, len(live) - 1)]
                new_k = oracle[idx] - r.range(0, 200)
                heap.decrease_key(idx, new_k)
                oracle[idx] = new_k
            elif choice < 85:
                res = heap.extract_min_with_handle()
                assert res is not None
                ext_id, got = res
                self.assertEqual(got, min(oracle.values()))
                self.assertEqual(oracle.pop(ext_id), got)
                live.remove(ext_id)
            else:
                pos = r.range(0, len(live) - 1)
                idx = live[pos]
                live[pos] = live[-1]
                live.pop()
                self.assertEqual(heap.delete(idx), oracle.pop(idx))
            want_min = min(oracle.values()) if oracle else None
            self.assertEqual(heap.minimum(), want_min)
            if step % 25 == 0:
                heap.check_invariants()
        heap.check_invariants()

    def test_union_at_various_heights_preserves_invariants_and_handles(self) -> None:
        r = Rng(1902)
        for _ in range(30):
            h1, h2 = TwoThreeFourHeap(), TwoThreeFourHeap()
            expected: list[int] = []
            for _ in range(r.range(0, 40)):
                v = r.range(0, 400) - 200
                h1.insert(v)
                expected.append(v)
            h2_handles: list[tuple[int, int]] = []
            for _ in range(r.range(0, 40)):
                v = r.range(0, 400) - 200
                h2_handles.append((h2.insert(v), v))
                expected.append(v)
            mapping = h1.union(h2)
            h1.check_invariants()
            if h2_handles:
                old_h, v = h2_handles[0]
                new_h = mapping[old_h]
                assert new_h is not None
                h1.decrease_key(new_h, v - 100)
                expected[expected.index(v)] = v - 100
                h1.check_invariants()
            got: list[int] = []
            while (x := h1.extract_min()) is not None:
                got.append(x)
            self.assertEqual(got, sorted(expected))


if __name__ == "__main__":
    print(demo(), end="")
