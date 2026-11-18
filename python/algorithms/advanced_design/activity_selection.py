"""Activity Selection — CLRS 16.1 (1e–3e) / 15.1 (4e) (1e,2e,3e,4e).

Greedy choice (earliest finish time) picks a maximum set of compatible activities; compared with the DP.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

Activity = tuple[int, int]  # (start, finish), half-open [s, f)
_NEG_INF = -(1 << 60)
_POS_INF = 1 << 60


def greedy_activity_selector(acts: list[Activity]) -> list[int]:
    """GREEDY-ACTIVITY-SELECTOR: sort by finish time and take every compatible activity."""
    idx = sorted(range(len(acts)), key=lambda i: (acts[i][1], acts[i][0]))
    chosen: list[int] = []
    last_finish = _NEG_INF
    for i in idx:
        if acts[i][0] >= last_finish:
            chosen.append(i)
            last_finish = acts[i][1]
    return chosen


def recursive_activity_selector(sorted_acts: list[Activity], k: int, out: list[int]) -> None:
    """RECURSIVE-ACTIVITY-SELECTOR over activities already sorted by finish time."""
    m = k
    fk = _NEG_INF if k == 0 else sorted_acts[k - 1][1]
    while m < len(sorted_acts) and sorted_acts[m][0] < fk:
        m += 1
    if m < len(sorted_acts):
        out.append(m)
        recursive_activity_selector(sorted_acts, m + 1, out)


def dp_activity_selection(acts: list[Activity]) -> int:
    """Theta(n^3) dynamic programming of Section 16.1: c[i][j] = size of max compatible subset of S_ij."""
    a = sorted(acts, key=lambda x: x[1])
    n = len(a)

    def fin(i: int) -> int:
        return _NEG_INF if i == 0 else a[i - 1][1]

    def sta(i: int) -> int:
        return _POS_INF if i == n + 1 else a[i - 1][0]

    c = [[0] * (n + 2) for _ in range(n + 2)]
    for length in range(2, n + 2):
        for i in range(0, n + 2 - length):
            j = i + length
            for k in range(i + 1, j):
                if sta(k) >= fin(i) and fin(k) <= sta(j):
                    c[i][j] = max(c[i][j], c[i][k] + c[k][j] + 1)
    return c[0][n + 1]


def min_lecture_halls(acts: list[Activity]) -> int:
    """Exercise 16.1-4: partition activities among the fewest lecture halls (interval-graph colouring)."""
    events: list[tuple[int, int]] = []
    for s, f in acts:
        events.append((s, 1))
        events.append((f, -1))
    events.sort()  # at equal times, finishes (-1) sort before starts (+1)
    cur = 0
    best = 0
    for _, d in events:
        cur += d
        best = max(best, cur)
    return best


def demo() -> str:
    lines: list[str] = []
    acts: list[Activity] = [
        (1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9),
        (6, 10), (8, 11), (8, 12), (2, 14), (12, 16),
    ]
    g = greedy_activity_selector(acts)
    lines.append(f"activities (s,f): {acts}")
    lines.append(f"greedy picks (by finish order): {[acts[i] for i in g]}")
    lines.append(f"greedy size = {len(g)}, dynamic programming size = {dp_activity_selection(acts)}")
    sorted_acts = sorted(acts, key=lambda a: a[1])
    rec: list[int] = []
    recursive_activity_selector(sorted_acts, 0, rec)
    lines.append(f"recursive selector picks {[sorted_acts[i] for i in rec]}")
    lines.append(f"lecture halls needed for all activities: {min_lecture_halls(acts)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        acts = [
            (1, 4), (3, 5), (0, 6), (5, 7), (3, 9), (5, 9),
            (6, 10), (8, 11), (8, 12), (2, 14), (12, 16),
        ]
        self.assertEqual(len(greedy_activity_selector(acts)), 4)
        self.assertEqual(dp_activity_selection(acts), 4)

    def test_greedy_equals_dp_and_is_compatible(self) -> None:
        r = Rng(112)
        for _ in range(80):
            n = r.range(0, 12)
            acts = [(s := r.range(0, 20), s + r.range(1, 8)) for _ in range(n)]
            g = greedy_activity_selector(acts)
            chosen = sorted(acts[i] for i in g)
            self.assertTrue(all(chosen[i][1] <= chosen[i + 1][0] for i in range(len(chosen) - 1)))
            self.assertEqual(len(g), dp_activity_selection(acts))
            sorted_acts = sorted(acts, key=lambda a: a[1])
            rec: list[int] = []
            recursive_activity_selector(sorted_acts, 0, rec)
            self.assertEqual(len(rec), len(g))

    def test_lecture_halls_equal_maximum_overlap(self) -> None:
        self.assertEqual(min_lecture_halls([(0, 3), (3, 6), (6, 9)]), 1)
        self.assertEqual(min_lecture_halls([(0, 5), (1, 6), (2, 7)]), 3)
        self.assertEqual(min_lecture_halls([]), 0)


if __name__ == "__main__":
    print(demo(), end="")
