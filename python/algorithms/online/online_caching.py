"""Online Caching: LRU, FIFO and Belady's Optimum — CLRS 27.3, 15.4 (4e); Problem 16-5 (3e) (3e,4e).

Cache replacement policies compared by cache misses; LRU and FIFO are k-competitive, furthest-in-future is optimal offline.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def lru_misses(k: int, requests: list[int]) -> int:
    """Least-recently-used replacement: evict the page unused for the longest time."""
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


def fifo_misses(k: int, requests: list[int]) -> int:
    """First-in-first-out: evict the page that has been in the cache longest."""
    cache: deque[int] = deque()
    misses = 0
    for p in requests:
        if p not in cache:
            misses += 1
            if len(cache) == k:
                cache.popleft()
            cache.append(p)
    return misses


def fwf_misses(k: int, requests: list[int]) -> int:
    """Flush-when-full: when the cache is full and a miss occurs, empty it entirely."""
    cache: list[int] = []
    misses = 0
    for p in requests:
        if p not in cache:
            misses += 1
            if len(cache) == k:
                cache.clear()
            cache.append(p)
    return misses


def belady_misses(k: int, requests: list[int]) -> int:
    """Belady's offline-optimal policy (Problem 16-5): evict the page whose next request is furthest in the future."""
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


def demo() -> str:
    lines: list[str] = []
    k = 4
    cyc = [i % (k + 1) for i in range(200)]
    lru = lru_misses(k, cyc)
    fifo = fifo_misses(k, cyc)
    fwf = fwf_misses(k, cyc)
    opt = belady_misses(k, cyc)
    lines.append(
        f"cycling through {k + 1} pages with a cache of {k}: "
        f"LRU {lru} misses, FIFO {fifo}, FWF {fwf}, optimal {opt}"
    )
    lines.append(f"→ ratio LRU/OPT = {lru / opt:.2f} ≈ k = {k}: the k-competitive bound is tight")
    r = Rng(9)
    loc = [int((r.f64() ** 3) * 40.0) for _ in range(5000)]
    for k in (4, 8, 16):
        lines.append(
            f"locality-heavy workload, k={k:>2}: "
            f"LRU {lru_misses(k, loc):>4}  FIFO {fifo_misses(k, loc):>4}  "
            f"FWF {fwf_misses(k, loc):>4}  OPT {belady_misses(k, loc):>4}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _optimal_by_search(self, k: int, req: list[int]) -> int:
        def go(t: int, cache: list[int]) -> int:
            if t == len(req):
                return 0
            if req[t] in cache:
                return go(t + 1, cache)
            if len(cache) < k:
                cache.append(req[t])
                r = 1 + go(t + 1, cache)
                cache.pop()
                return r
            best = sys.maxsize
            for i in range(len(cache)):
                old = cache[i]
                cache[i] = req[t]
                best = min(best, 1 + go(t + 1, cache))
                cache[i] = old
            return best

        return go(0, [])

    def test_belady_is_optimal(self) -> None:
        r = Rng(271)
        for _ in range(200):
            k, length = r.range(1, 3), r.range(0, 11)
            req = [r.range(0, 4) for _ in range(length)]
            self.assertEqual(belady_misses(k, req), self._optimal_by_search(k, req), f"k={k} {req}")

    def test_online_policies_are_k_competitive(self) -> None:
        r = Rng(272)
        for _ in range(100):
            k, length = r.range(1, 6), r.range(1, 300)
            req = [r.range(0, 9) for _ in range(length)]
            opt = belady_misses(k, req)
            lru = lru_misses(k, req)
            fifo = fifo_misses(k, req)
            fwf = fwf_misses(k, req)
            self.assertLessEqual(opt, lru)
            self.assertLessEqual(opt, fifo)
            self.assertLessEqual(opt, fwf)
            self.assertLessEqual(lru, k * opt)
            self.assertLessEqual(fifo, k * opt)
            self.assertLessEqual(fwf, k * opt)

    def test_adversarial_cycle_attains_ratio_k(self) -> None:
        k = 5
        req = [i % (k + 1) for i in range(600)]
        self.assertEqual(lru_misses(k, req), 600)
        opt = belady_misses(k, req)
        self.assertLessEqual(opt, 600 // k + k + 1, str(opt))
        self.assertGreater(600 / opt, k - 1.0)


if __name__ == "__main__":
    print(demo(), end="")
