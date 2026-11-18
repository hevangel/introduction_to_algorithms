"""Bin Packing: First Fit — CLRS Problem 35-1 (1e–4e) (1,2,3,4).

Pack items of size ≤ capacity into the fewest bins: Next/First/Best Fit and First Fit Decreasing, with the Problem 35-1 guarantees (≤ 2·OPT; FFD ≤ 11/9·OPT + 6/9).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def first_fit(items: list[int], cap: int) -> list[list[int]]:
    """FIRST-FIT: put each item into the lowest-indexed bin where it fits."""
    bins: list[tuple[int, list[int]]] = []
    for x in items:
        placed = False
        for i, (load, contents) in enumerate(bins):
            if load + x <= cap:
                bins[i] = (load + x, contents + [x])
                placed = True
                break
        if not placed:
            bins.append((x, [x]))
    return [b for _, b in bins]


def next_fit(items: list[int], cap: int) -> int:
    """NEXT-FIT: only the most recently opened bin is considered."""
    bins = 0
    load = cap + 1
    for x in items:
        if load + x > cap:
            bins += 1
            load = x
        else:
            load += x
    return bins


def best_fit(items: list[int], cap: int) -> int:
    """BEST-FIT: choose the fullest bin that still has room."""
    loads: list[int] = []
    for x in items:
        candidates = [(loads[i], i) for i in range(len(loads)) if loads[i] + x <= cap]
        if candidates:
            _, idx = max(candidates)
            loads[idx] += x
        else:
            loads.append(x)
    return len(loads)


def first_fit_decreasing(items: list[int], cap: int) -> list[list[int]]:
    """FIRST-FIT-DECREASING: sort items largest first, then First Fit."""
    return first_fit(sorted(items, reverse=True), cap)


def optimal_bins(items: list[int], cap: int) -> int:
    """Optimal number of bins by exhaustive search (small inputs only)."""
    sorted_items = sorted(items, reverse=True)
    best = len(items) + 1
    loads: list[int] = []

    def go(i: int) -> None:
        nonlocal best
        if len(loads) >= best:
            return
        if i == len(sorted_items):
            best = len(loads)
            return
        x = sorted_items[i]
        for b in range(len(loads)):
            if loads[b] + x <= cap:
                loads[b] += x
                go(i + 1)
                loads[b] -= x
        loads.append(x)
        go(i + 1)
        loads.pop()

    go(0)
    return best


def demo() -> str:
    items = [4, 8, 1, 4, 2, 1, 7, 3, 6, 5]
    total = sum(items)
    return "\n".join([
        f"capacity 10, items {items} (sum {total}, lower bound {(total + 9) // 10} bins)",
        f"next fit  : {next_fit(items, 10)} bins",
        f"first fit : {len(first_fit(items, 10))} bins {first_fit(items, 10)}",
        f"best fit  : {best_fit(items, 10)} bins",
        f"FFD       : {len(first_fit_decreasing(items, 10))} bins {first_fit_decreasing(items, 10)}",
        f"optimal   : {optimal_bins(items, 10)} bins",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_guarantees_hold_against_the_exact_optimum(self) -> None:
        r = Rng(414)
        for _ in range(200):
            cap = r.range(5, 20)
            items = [r.range(1, cap) for _ in range(r.range(1, 10))]
            opt = optimal_bins(items, cap)
            ff = len(first_fit(items, cap))
            nf = next_fit(items, cap)
            bf = best_fit(items, cap)
            ffd = len(first_fit_decreasing(items, cap))
            for packing in (first_fit(items, cap), first_fit_decreasing(items, cap)):
                self.assertTrue(all(sum(b) <= cap for b in packing))
                self.assertEqual(sorted(x for b in packing for x in b), sorted(items))
            self.assertGreaterEqual(opt, (sum(items) + cap - 1) // cap)
            self.assertGreaterEqual(ff, opt)
            self.assertGreaterEqual(bf, opt)
            self.assertGreaterEqual(nf, opt)
            self.assertGreaterEqual(ffd, opt)
            self.assertLessEqual(nf, 2 * opt)
            self.assertLessEqual(ff, 2 * opt)
            self.assertLessEqual(bf, 2 * opt)
            self.assertLessEqual(9 * ffd, 11 * opt + 6)

    def test_ffd_beats_first_fit_on_a_classic_input(self) -> None:
        items = [1, 6, 2, 5, 3, 4, 4, 3, 5, 2, 6, 1]
        self.assertLessEqual(len(first_fit_decreasing(items, 7)), len(first_fit(items, 7)))
        self.assertEqual(optimal_bins(items, 7), 6)


if __name__ == "__main__":
    print(demo(), end="")
