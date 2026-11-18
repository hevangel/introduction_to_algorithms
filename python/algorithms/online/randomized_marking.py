"""Randomized Marking for Online Paging — CLRS 27.3.2 (4e) (4e).

Epoch-based MARKING (k-competitive) and RANDOMIZED-MARKING (2 H_k = O(lg k) expected competitive ratio against an oblivious adversary).
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def marking_misses(k: int, requests: list[int]) -> int:
    """Deterministic MARKING(k): every cached page starts an epoch unmarked."""
    if k <= 0:
        raise ValueError("k must be > 0")
    cache: list[list] = []  # [page, marked]
    misses = 0
    for p in requests:
        hit = False
        for slot in cache:
            if slot[0] == p:
                slot[1] = True
                hit = True
                break
        if hit:
            continue
        misses += 1
        if len(cache) == k:
            if all(slot[1] for slot in cache):
                for slot in cache:
                    slot[1] = False
            pos = next(i for i, slot in enumerate(cache) if not slot[1])
            cache[pos] = cache[-1]
            cache.pop()
        cache.append([p, True])
    return misses


def randomized_marking_misses(k: int, requests: list[int], seed: int) -> int:
    """RANDOMIZED-MARKING(k): identical epoch marking, choosing unmarked eviction victim uniformly at random."""
    if k <= 0:
        raise ValueError("k must be > 0")
    rng = Rng(seed)
    cache: list[list] = []
    misses = 0
    for p in requests:
        hit = False
        for slot in cache:
            if slot[0] == p:
                slot[1] = True
                hit = True
                break
        if hit:
            continue
        misses += 1
        if len(cache) == k:
            if all(slot[1] for slot in cache):
                for slot in cache:
                    slot[1] = False
            unmarked = [i for i, slot in enumerate(cache) if not slot[1]]
            victim = unmarked[rng.range(0, len(unmarked) - 1)]
            cache[victim] = cache[-1]
            cache.pop()
        cache.append([p, True])
    return misses


def harmonic(k: int) -> float:
    return sum(1.0 / i for i in range(1, k + 1))


def _belady_misses(k: int, requests: list[int]) -> int:
    """Self-contained Belady's offline optimum for competitive comparison."""
    cache: list[int] = []
    misses = 0
    n = len(requests)
    for t, p in enumerate(requests):
        if p in cache:
            continue
        misses += 1
        if len(cache) == k:
            def next_use(q: int) -> int:
                for idx in range(t + 1, n):
                    if requests[idx] == q:
                        return idx
                return n + 1

            victim = max(range(len(cache)), key=lambda i: next_use(cache[i]))
            cache[victim] = cache[-1]
            cache.pop()
        cache.append(p)
    return misses


def _lru_misses(k: int, requests: list[int]) -> int:
    """Self-contained LRU misses for competitive comparison."""
    cache: deque[int] = deque()
    misses = 0
    for p in requests:
        if p in cache:
            cache.remove(p)
        else:
            misses += 1
            if len(cache) == k:
                cache.pop()
        cache.appendleft(p)
    return misses


def demo() -> str:
    lines: list[str] = []
    k = 6
    seq = [i % (k + 1) for i in range(600)]
    opt = _belady_misses(k, seq)
    lru = _lru_misses(k, seq)
    det_m = marking_misses(k, seq)
    avg_rand = sum(randomized_marking_misses(k, seq, s) for s in range(1, 21)) / 20.0
    lines.append(
        f"cyclic (k+1={k + 1}) workload with k={k}: "
        f"OPT={opt}, LRU={lru}, MARKING={det_m}, avg RAND-MARK={avg_rand:.1f}"
    )
    lines.append(
        f"competitive bound 2*H_{k} = {2.0 * harmonic(k):.2f} vs observed rand/OPT = {avg_rand / opt:.2f}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_randomized_marking_beats_deterministic_k_adversary_within_2_hk(self) -> None:
        for k in (4, 6, 8):
            seq = [i % (k + 1) for i in range(800)]
            opt = _belady_misses(k, seq)
            lru = _lru_misses(k, seq)
            self.assertEqual(lru, len(seq))
            det = marking_misses(k, seq)
            self.assertLessEqual(det, k * opt + k)
            avg_rand = sum(randomized_marking_misses(k, seq, 100 + s) for s in range(1, 31)) / 30.0
            self.assertLessEqual(avg_rand, 2.0 * harmonic(k) * opt + k)
            self.assertLess(avg_rand, 0.7 * lru)


if __name__ == "__main__":
    print(demo(), end="")
