"""Perfect Hashing (FKS) — CLRS 11.5 (2e, 3e) (Editions: 2,3).

Two-level universal hashing gives worst-case O(1) lookup for a static key set in O(n) space.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_P = (1 << 61) - 1  # Mersenne prime > any 60-bit key


class _H:
    """Universal hash h(k) = ((a·k + b) mod p) mod m."""

    __slots__ = ("a", "b")

    def __init__(self, a: int, b: int) -> None:
        self.a = a
        self.b = b

    @staticmethod
    def random(rng: Rng) -> _H:
        return _H(1 + rng.next_u64() % (_P - 1), rng.next_u64() % _P)

    def apply(self, k: int, m: int) -> int:
        return ((self.a * k + self.b) % _P) % m


class _Bucket:
    __slots__ = ("h", "slots")

    def __init__(self, h: _H, slots: list[int | None]) -> None:
        self.h = h
        self.slots = slots


class PerfectHash:
    """Static perfect hash table: level one splits n keys into n buckets; bucket j gets n_j² slots
    and a collision-free hash."""

    def __init__(self, top: _H, m: int, buckets: list[_Bucket], top_level_retries: int) -> None:
        self.top = top
        self.m = m
        self.buckets = buckets
        self.top_level_retries = top_level_retries

    @staticmethod
    def build(keys: list[int], rng: Rng) -> PerfectHash:
        ks = sorted(set(keys))
        n = max(len(ks), 1)
        retries = 0
        while True:
            h = _H.random(rng)
            groups: list[list[int]] = [[] for _ in range(n)]
            for k in ks:
                groups[h.apply(k, n)].append(k)
            # Theorem 11.9: E[Σ n_j²] < 2n, so by Markov Σ n_j² < 4n with probability >= 1/2
            if sum(len(g) * len(g) for g in groups) < 4 * n:
                top = h
                break
            retries += 1
        buckets: list[_Bucket] = []
        for g in groups:
            size = len(g) * len(g)
            if size == 0:
                buckets.append(_Bucket(_H(1, 0), []))
                continue
            while True:
                bh = _H.random(rng)
                slots: list[int | None] = [None] * size
                ok = True
                for k in g:
                    i = bh.apply(k, size)
                    if slots[i] is not None:
                        ok = False
                        break
                    slots[i] = k
                if ok:
                    buckets.append(_Bucket(bh, slots))
                    break
        return PerfectHash(top, n, buckets, retries)

    def contains(self, k: int) -> bool:
        """Worst-case O(1): two hash evaluations and one comparison."""
        b = self.buckets[self.top.apply(k, self.m)]
        return len(b.slots) > 0 and b.slots[b.h.apply(k, len(b.slots))] == k

    def total_slots(self) -> int:
        return self.m + sum(len(b.slots) for b in self.buckets)


def demo() -> str:
    lines: list[str] = []
    r = Rng(2)
    keys = [r.next_u64() >> 8 for _ in range(1000)]
    ph = PerfectHash.build(keys, r)
    lines.append(f"built a perfect hash for {len(keys)} keys; top-level retries = {ph.top_level_retries}")
    lines.append(f"total slots = {ph.total_slots()} (≤ 5n = 5000 guaranteed by the analysis)")
    lines.append(f"all keys found: {str(all(ph.contains(k) for k in keys)).lower()}")
    lines.append(f"absent key found? {str(ph.contains(12345)).lower()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_exact_membership_and_linear_space(self) -> None:
        r = Rng(58)
        for n in [1, 2, 10, 300, 2000]:
            keys = [r.next_u64() >> 8 for _ in range(n)]
            keyset = set(keys)
            ph = PerfectHash.build(keys, r)
            self.assertTrue(all(ph.contains(k) for k in keys))
            self.assertLessEqual(ph.total_slots(), 5 * n + 5, f"n={n} slots={ph.total_slots()}")
            for _ in range(200):
                q = r.next_u64() >> 8
                self.assertEqual(ph.contains(q), q in keyset)

    def test_empty_and_duplicates(self) -> None:
        r = Rng(59)
        ph = PerfectHash.build([], r)
        self.assertFalse(ph.contains(5))
        ph = PerfectHash.build([3, 3, 3], r)
        self.assertTrue(ph.contains(3) and not ph.contains(4))


if __name__ == "__main__":
    print(demo(), end="")
