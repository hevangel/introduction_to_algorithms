"""van Emde Boas Trees — CLRS 20 (3e) (3).

Priority queue on integers 0..u with insert, delete, member, min/max, successor and predecessor in O(lg lg u).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class VebTree:
    """van Emde Boas tree over {0, ..., u-1} where u >= 2 is a power of two."""

    def __init__(self, u: int) -> None:
        assert u >= 2 and (u & (u - 1)) == 0
        self.u = u
        self.min: int | None = None
        self.max: int | None = None
        if u == 2:
            self._lower_bits = 0
            self.summary: VebTree | None = None
            self.cluster: list[VebTree] = []
        else:
            k = u.bit_length() - 1
            self._lower_bits = k // 2
            lower = 1 << (k // 2)
            upper = 1 << (k - k // 2)
            self.summary = VebTree(upper)
            self.cluster = [VebTree(lower) for _ in range(upper)]

    def _high(self, x: int) -> int:
        return x >> self._lower_bits

    def _low(self, x: int) -> int:
        return x & ((1 << self._lower_bits) - 1)

    def _index(self, h: int, l: int) -> int:
        return (h << self._lower_bits) | l

    def minimum(self) -> int | None:
        return self.min

    def maximum(self) -> int | None:
        return self.max

    def is_empty(self) -> bool:
        return self.min is None

    def member(self, x: int) -> bool:
        if x == self.min or x == self.max:
            return True
        if self.u == 2:
            return False
        return self.cluster[self._high(x)].member(self._low(x))

    def successor(self, x: int) -> int | None:
        if self.u == 2:
            return 1 if x == 0 and self.max == 1 else None
        if self.min is not None and x < self.min:
            return self.min
        h, l = self._high(x), self._low(x)
        cmax = self.cluster[h].max
        if cmax is not None and l < cmax:
            off = self.cluster[h].successor(l)
            return None if off is None else self._index(h, off)
        assert self.summary is not None
        sc = self.summary.successor(h)
        if sc is None:
            return None
        cmin = self.cluster[sc].min
        assert cmin is not None
        return self._index(sc, cmin)

    def predecessor(self, x: int) -> int | None:
        if self.u == 2:
            return 0 if x == 1 and self.min == 0 else None
        if self.max is not None and x > self.max:
            return self.max
        h, l = self._high(x), self._low(x)
        cmin = self.cluster[h].min
        if cmin is not None and l > cmin:
            off = self.cluster[h].predecessor(l)
            return None if off is None else self._index(h, off)
        assert self.summary is not None
        pc = self.summary.predecessor(h)
        if pc is None:
            return self.min if self.min is not None and x > self.min else None
        pcmax = self.cluster[pc].max
        assert pcmax is not None
        return self._index(pc, pcmax)

    def insert(self, x: int) -> None:
        assert 0 <= x < self.u
        if not self.member(x):
            self._insert_new(x)

    def _insert_new(self, x: int) -> None:
        if self.min is None:
            self.min = self.max = x
            return
        if x < self.min:
            x, self.min = self.min, x
        if self.u > 2:
            h, l = self._high(x), self._low(x)
            if self.cluster[h].min is None:
                assert self.summary is not None
                self.summary._insert_new(h)
                self.cluster[h].min = self.cluster[h].max = l
            else:
                self.cluster[h]._insert_new(l)
        assert self.max is not None
        if x > self.max:
            self.max = x

    def delete(self, x: int) -> None:
        if 0 <= x < self.u and self.member(x):
            self._delete_present(x)

    def _delete_present(self, x: int) -> None:
        if self.min == self.max:
            self.min = self.max = None
        elif self.u == 2:
            self.min = self.max = 1 - x
        else:
            assert self.summary is not None
            if x == self.min:
                first = self.summary.min
                assert first is not None
                cmin = self.cluster[first].min
                assert cmin is not None
                x = self._index(first, cmin)
                self.min = x
            h, l = self._high(x), self._low(x)
            self.cluster[h]._delete_present(l)
            if self.cluster[h].min is None:
                self.summary._delete_present(h)
                if x == self.max:
                    sm = self.summary.max
                    if sm is None:
                        self.max = self.min
                    else:
                        scmax = self.cluster[sm].max
                        assert scmax is not None
                        self.max = self._index(sm, scmax)
            elif x == self.max:
                cmax = self.cluster[h].max
                assert cmax is not None
                self.max = self._index(h, cmax)


def demo() -> str:
    lines: list[str] = []
    v = VebTree(16)
    for x in [2, 3, 4, 5, 7, 14, 15]:
        v.insert(x)
    lines.append(f"universe 16, stored {{2,3,4,5,7,14,15}}: min={v.minimum()} max={v.maximum()}")
    lines.append(f"successor(5)={v.successor(5)} successor(7)={v.successor(7)} successor(15)={v.successor(15)}")
    lines.append(f"predecessor(7)={v.predecessor(7)} predecessor(2)={v.predecessor(2)} predecessor(14)={v.predecessor(14)}")
    v.delete(7)
    v.delete(2)
    lines.append(f"after deleting 7 and 2: member(7)={v.member(7)} min={v.minimum()} successor(5)={v.successor(5)}")
    big = VebTree(1 << 16)
    for x in range(0, 65536, 97):
        big.insert(x)
    lines.append(f"u = 2^16: successor(1000) = {big.successor(1000)} found with ~lg lg u = 4 recursive levels")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_set_for_various_universes(self) -> None:
        for k in [1, 2, 3, 4, 5, 8, 10]:
            u = 1 << k
            v = VebTree(u)
            s: set[int] = set()
            r = Rng(90 + k)
            for _ in range(1500):
                x = r.range(0, u - 1)
                if r.range(0, 3) <= 1:
                    v.insert(x)
                    s.add(x)
                else:
                    v.delete(x)
                    s.discard(x)
                self.assertEqual(v.member(x), x in s)
                self.assertEqual(v.minimum(), min(s) if s else None)
                self.assertEqual(v.maximum(), max(s) if s else None)
                q = r.range(0, u - 1)
                succ = min((y for y in s if y > q), default=None)
                pred = max((y for y in s if y < q), default=None)
                self.assertEqual(v.successor(q), succ)
                self.assertEqual(v.predecessor(q), pred)

    def test_empty_tree(self) -> None:
        v = VebTree(16)
        self.assertTrue(v.is_empty())
        self.assertIsNone(v.successor(3))
        self.assertIsNone(v.predecessor(3))
        self.assertFalse(v.member(0))


if __name__ == "__main__":
    print(demo(), end="")
