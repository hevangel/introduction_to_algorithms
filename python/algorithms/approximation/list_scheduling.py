"""Scheduling on Parallel Machines (List Scheduling, LPT) — CLRS Problem 35-5 (1e–4e) (1,2,3,4).

Assign jobs to m identical machines to minimise the makespan: Graham's list scheduling is a (2 − 1/m)-approximation, and longest-processing-time-first improves it to 4/3 − 1/(3m).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def list_schedule(jobs: list[int], m: int) -> tuple[int, list[int]]:
    """GRAHAM'S LIST SCHEDULING: give each job to the currently least-loaded machine."""
    load = [0] * m
    assign = [0] * len(jobs)
    for j, p in enumerate(jobs):
        i = min(range(m), key=lambda idx: load[idx])
        load[i] += p
        assign[j] = i
    return (max(load, default=0), assign)


def lpt(jobs: list[int], m: int) -> int:
    """LONGEST PROCESSING TIME FIRST: sort jobs decreasing, then list schedule."""
    return list_schedule(sorted(jobs, reverse=True), m)[0]


def optimal_makespan(jobs: list[int], m: int) -> int:
    """Optimal makespan by exhaustive search with pruning (small inputs only)."""
    sorted_jobs = sorted(jobs, reverse=True)
    best = sum(sorted_jobs) + 1
    load = [0] * m

    def go(i: int) -> None:
        nonlocal best
        if max(load) >= best:
            return
        if i == len(sorted_jobs):
            best = max(load)
            return
        seen: set[int] = set()
        for k in range(m):
            if load[k] in seen:
                continue
            seen.add(load[k])
            load[k] += sorted_jobs[i]
            go(i + 1)
            load[k] -= sorted_jobs[i]

    go(0)
    return best


def demo() -> str:
    m = 3
    bad = [1] * (m * (m - 1)) + [m]
    ls_bad = list_schedule(bad, m)[0]
    opt_bad = optimal_makespan(bad, m)
    jobs = [7, 7, 6, 6, 5, 5, 4, 4, 4]
    return "\n".join([
        f"jobs {bad} on {m} machines",
        f"list scheduling {ls_bad}, LPT {lpt(bad, m)}, optimal {opt_bad} "
        f"(ratio {ls_bad / opt_bad:.3f}; bound 2 − 1/m = {2.0 - 1.0 / m:.3f})",
        f"jobs {jobs}: list {list_schedule(jobs, 4)[0]}, LPT {lpt(jobs, 4)}, optimal {optimal_makespan(jobs, 4)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_graham_bounds_against_the_optimum(self) -> None:
        r = Rng(415)
        for _ in range(300):
            m = r.range(1, 4)
            jobs = [r.range(1, 20) for _ in range(r.range(1, 9))]
            opt = optimal_makespan(jobs, m)
            ls, assign = list_schedule(jobs, m)
            load = [0] * m
            for j, i in enumerate(assign):
                load[i] += jobs[j]
            self.assertEqual(max(load), ls)
            self.assertGreaterEqual(opt, (sum(jobs) + m - 1) // m)
            self.assertGreaterEqual(opt, max(jobs))
            self.assertLessEqual(m * ls, (2 * m - 1) * opt)
            self.assertLessEqual(3 * m * lpt(jobs, m), (4 * m - 1) * opt)

    def test_worst_case_instance_is_tight(self) -> None:
        for m in range(2, 6):
            bad = [1] * (m * (m - 1)) + [m]
            self.assertEqual(list_schedule(bad, m)[0], 2 * m - 1)
            self.assertEqual(optimal_makespan(bad, m), m)


if __name__ == "__main__":
    print(demo(), end="")
