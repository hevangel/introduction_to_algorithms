"""Minimizing Average Completion Time — CLRS Problem 16-2 (1e–4e) (1e,2e,3e,4e).

Shortest-processing-time-first minimises the average completion time on one machine; with release times and preemption, run the shortest remaining time (SRPT).
"""

from __future__ import annotations

import heapq
from itertools import permutations
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def spt_order(p: list[int]) -> list[int]:
    """Problem 16-2(a): schedule tasks without release times in order of increasing processing time (SPT)."""
    return sorted(range(len(p)), key=lambda i: p[i])


def average_completion(p: list[int], order: list[int]) -> float:
    """Average completion time of running the tasks in the given order, back to back from time 0."""
    t = 0
    total = 0
    for i in order:
        t += p[i]
        total += t
    return total / max(len(order), 1)


def srpt(r: list[int], p: list[int]) -> list[int]:
    """Problem 16-2(c): with release times r[i] and preemption, shortest remaining processing time (SRPT)."""
    n = len(p)
    jobs = sorted(range(n), key=lambda i: r[i])
    nxt = 0
    t = 0
    heap: list[tuple[int, int]] = []
    done = [0] * n
    while nxt < n or heap:
        if not heap:
            t = max(t, r[jobs[nxt]])
        while nxt < n and r[jobs[nxt]] <= t:
            heapq.heappush(heap, (p[jobs[nxt]], jobs[nxt]))
            nxt += 1
        rem, job_id = heapq.heappop(heap)
        until = r[jobs[nxt]] if nxt < n else sys.maxsize
        run = min(rem, until - t) if until > t else rem
        t += run
        if run == rem:
            done[job_id] = t
        else:
            heapq.heappush(heap, (rem - run, job_id))
    return done


def nonpreemptive_optimum(r: list[int], p: list[int]) -> float:
    """Best non-preemptive schedule with release times by exhaustive search."""
    n = len(p)
    best = sys.maxsize
    used = [False] * n

    def go(t: int, total: int, count: int) -> None:
        nonlocal best
        if count == n:
            best = min(best, total)
            return
        for i in range(n):
            if not used[i]:
                used[i] = True
                start = max(t, r[i])
                go(start + p[i], total + start + p[i], count + 1)
                used[i] = False

    go(0, 0, 0)
    return best / max(n, 1)


def demo() -> str:
    lines: list[str] = []
    p = [8, 3, 10, 2, 5]
    spt = spt_order(p)
    lines.append(f"processing times {p}: SPT order {spt}, average completion {average_completion(p, spt):.2f}")
    lines.append(f"given order 0..5 instead: {average_completion(p, [0, 1, 2, 3, 4]):.2f}")
    r = [0, 2, 4, 5, 6]
    done = srpt(r, p)
    lines.append(f"with release times {r}: SRPT completion times {done}, average {sum(done) / 5.0:.2f}")
    lines.append(f"best non-preemptive schedule: {nonpreemptive_optimum(r, p):.2f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_spt_is_optimal_without_release_times(self) -> None:
        r = Rng(345)
        for _ in range(100):
            n = r.range(1, 7)
            p = [r.range(1, 20) for _ in range(n)]
            best = min(average_completion(p, list(o)) for o in permutations(range(n)))
            self.assertLess(abs(average_completion(p, spt_order(p)) - best), 1e-9)
            self.assertLess(abs(sum(srpt([0] * n, p)) / n - best), 1e-9)

    def test_srpt_beats_every_nonpreemptive_schedule_with_release_times(self) -> None:
        r = Rng(346)
        for _ in range(100):
            n = r.range(1, 6)
            rel = [r.range(0, 15) for _ in range(n)]
            p = [r.range(1, 10) for _ in range(n)]
            done = srpt(rel, p)
            self.assertTrue(all(c >= ri + pi for c, ri, pi in zip(done, rel, p)))
            self.assertLessEqual(sum(done) / n, nonpreemptive_optimum(rel, p) + 1e-9)


if __name__ == "__main__":
    print(demo(), end="")
