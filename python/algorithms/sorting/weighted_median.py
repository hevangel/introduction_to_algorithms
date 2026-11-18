"""Weighted Median — CLRS Problem 9-2 (1e–4e) (1,2,3,4).

The element whose lighter side carries < half the total weight, via sorting or linear-time selection.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _select(a: list[int], i: int) -> int:
    cur = list(a)
    while len(cur) > 5:
        meds = [sorted(cur[s : s + 5])[(min(5, len(cur) - s) - 1) // 2] for s in range(0, len(cur), 5)]
        p = _select(meds, (len(meds) + 1) // 2)
        lt = [x for x in cur if x < p]
        eq = sum(1 for x in cur if x == p)
        if i <= len(lt):
            cur = lt
        elif i <= len(lt) + eq:
            return p
        else:
            i -= len(lt) + eq
            cur = [x for x in cur if x > p]
    return sorted(cur)[i - 1]


def weighted_median_sorting(items: list[tuple[int, int]]) -> int:
    """Weighted lower median of (value, weight) pairs in Θ(n lg n)."""
    v = sorted(items)
    total = sum(w for _, w in v)
    cum = 0
    for x, w in v:
        cum += w
        if 2 * cum >= total:
            return x
    raise ValueError("empty input")


def weighted_median_linear(items: list[tuple[int, int]]) -> int:
    """Θ(n) weighted median via worst-case linear selection."""
    cur = list(items)
    total = sum(w for _, w in cur)
    below = 0
    while True:
        vals = [x for x, _ in cur]
        m = _select(vals, (len(vals) + 1) // 2)
        wl = sum(w for x, w in cur if x < m)
        we = sum(w for x, w in cur if x == m)
        if 2 * (below + wl) >= total:
            cur = [(x, w) for x, w in cur if x < m]
        elif 2 * (below + wl + we) >= total:
            return m
        else:
            below += wl + we
            cur = [(x, w) for x, w in cur if x > m]


def demo() -> str:
    lines: list[str] = []
    items = [(1, 10), (2, 5), (3, 20), (4, 5), (5, 15), (6, 5), (7, 40)]
    lines.append(f"(value, weight) = {items}")
    lines.append(f"total weight = {sum(w for _, w in items)}")
    lines.append(f"weighted median (sorting) = {weighted_median_sorting(items)}")
    lines.append(f"weighted median (linear)  = {weighted_median_linear(items)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_linear_matches_sorting(self) -> None:
        r = Rng(43)
        for n in range(1, 60):
            items = [(r.vec(1, -20, 20)[0], r.range(1, 9)) for _ in range(n)]
            self.assertEqual(weighted_median_linear(items), weighted_median_sorting(items))

    def test_equal_weights_give_ordinary_lower_median(self) -> None:
        items = [(x, 1) for x in range(1, 10)]
        self.assertEqual(weighted_median_sorting(items), 5)
        self.assertEqual(weighted_median_linear(items), 5)


if __name__ == "__main__":
    print(demo(), end="")
