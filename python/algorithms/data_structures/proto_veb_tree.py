"""Proto van Emde Boas Structure — CLRS 20.2 (3e) (Editions: 3).

The first recursive step towards vEB trees: √u clusters plus a summary; every operation costs Θ(√u)-ish recursion, T(u)=2T(√u)+Θ(1).
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class ProtoVeb:
    """Proto-vEB(u) for u = 2^(2^k): u = 2 stores two bits; larger universes hold a summary and
    √u clusters of size √u."""

    def __init__(self, u: int) -> None:
        if not (u >= 2 and u & (u - 1) == 0 and ((u.bit_length() - 1) & (u.bit_length() - 2)) == 0):
            raise ValueError("u must be 2^(2^k)")
        self.u = u
        self.bits = [False, False]
        self.summary: ProtoVeb | None = None
        self.cluster: list[ProtoVeb] = []
        if u > 2:
            r = math.isqrt(u)
            self.summary = ProtoVeb(r)
            self.cluster = [ProtoVeb(r) for _ in range(r)]

    def _sqrt(self) -> int:
        return math.isqrt(self.u)

    def _high(self, x: int) -> int:
        return x // self._sqrt()

    def _low(self, x: int) -> int:
        return x % self._sqrt()

    def _index(self, h: int, l: int) -> int:
        return h * self._sqrt() + l

    def member(self, x: int) -> bool:
        if self.u == 2:
            return self.bits[x]
        return self.cluster[self._high(x)].member(self._low(x))

    def insert(self, x: int) -> None:
        if self.u == 2:
            self.bits[x] = True
        else:
            h, l = self._high(x), self._low(x)
            self.cluster[h].insert(l)
            assert self.summary is not None
            self.summary.insert(h)

    def delete(self, x: int) -> None:
        if self.u == 2:
            self.bits[x] = False
        else:
            h, l = self._high(x), self._low(x)
            self.cluster[h].delete(l)
            if self.cluster[h].minimum() is None:
                assert self.summary is not None
                self.summary.delete(h)

    def minimum(self) -> int | None:
        if self.u == 2:
            if self.bits[0]:
                return 0
            return 1 if self.bits[1] else None
        assert self.summary is not None
        c = self.summary.minimum()
        if c is None:
            return None
        m = self.cluster[c].minimum()
        assert m is not None
        return self._index(c, m)

    def maximum(self) -> int | None:
        if self.u == 2:
            if self.bits[1]:
                return 1
            return 0 if self.bits[0] else None
        assert self.summary is not None
        c = self.summary.maximum()
        if c is None:
            return None
        m = self.cluster[c].maximum()
        assert m is not None
        return self._index(c, m)

    def successor(self, x: int) -> int | None:
        if self.u == 2:
            return 1 if x == 0 and self.bits[1] else None
        h, l = self._high(x), self._low(x)
        off = self.cluster[h].successor(l)
        if off is not None:
            return self._index(h, off)
        assert self.summary is not None
        c = self.summary.successor(h)
        if c is None:
            return None
        m = self.cluster[c].minimum()
        assert m is not None
        return self._index(c, m)

    def predecessor(self, x: int) -> int | None:
        if self.u == 2:
            return 0 if x == 1 and self.bits[0] else None
        h, l = self._high(x), self._low(x)
        off = self.cluster[h].predecessor(l)
        if off is not None:
            return self._index(h, off)
        assert self.summary is not None
        c = self.summary.predecessor(h)
        if c is None:
            return None
        m = self.cluster[c].maximum()
        assert m is not None
        return self._index(c, m)


def demo() -> str:
    lines: list[str] = []
    v = ProtoVeb(16)
    for x in [2, 3, 4, 5, 7, 14, 15]:
        v.insert(x)
    lines.append(f"proto-vEB(16) with {{2,3,4,5,7,14,15}}: min={v.minimum()} max={v.maximum()}")
    lines.append(
        f"successor(5)={v.successor(5)} successor(7)={v.successor(7)} "
        f"predecessor(14)={v.predecessor(14)} member(9)={str(v.member(9)).lower()}"
    )
    v.delete(14)
    lines.append(f"after deleting 14: predecessor(15) = {v.predecessor(15)}")
    lines.append(
        "the recursion T(u) = 2T(√u) + Θ(1) solves to Θ(lg u) for MEMBER/INSERT but successor needs 3 "
        "recursive calls: Θ(lg u · … ) — see the full vEB tree for O(lg lg u)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_sorted_set(self) -> None:
        for u in [2, 4, 16, 256]:
            v = ProtoVeb(u)
            s: set[int] = set()
            r = Rng(323 + u)
            for _ in range(1500):
                x = r.range(0, u - 1)
                if r.range(0, 2) < 2:
                    v.insert(x)
                    s.add(x)
                else:
                    v.delete(x)
                    s.discard(x)
                self.assertEqual(v.member(x), x in s)
                self.assertEqual(v.minimum(), min(s) if s else None)
                self.assertEqual(v.maximum(), max(s) if s else None)
                q = r.range(0, u - 1)
                self.assertEqual(v.successor(q), min((y for y in s if y > q), default=None))
                self.assertEqual(v.predecessor(q), max((y for y in s if y < q), default=None))

    def test_rejects_non_double_power_universes(self) -> None:
        with self.assertRaises(ValueError):
            ProtoVeb(8)
        with self.assertRaises(ValueError):
            ProtoVeb(32)


if __name__ == "__main__":
    print(demo(), end="")
