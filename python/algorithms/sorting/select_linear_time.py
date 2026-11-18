"""Selection in Worst-Case Linear Time — CLRS 9.3 (1e–4e) (1,2,3,4).

Median-of-medians pivot guarantees a 30/70 split, so SELECT runs in Θ(n) worst case.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def select(a: list, i: int):
    """SELECT(A, i): the i-th smallest (1-based) of `a`, worst-case Θ(n)."""
    assert 1 <= i <= len(a)
    cur = list(a)
    while len(cur) > 5:
        medians = []
        for start in range(0, len(cur), 5):
            g = sorted(cur[start : start + 5])
            medians.append(g[(len(g) - 1) // 2])
        pivot = select(medians, (len(medians) + 1) // 2)
        lt = [x for x in cur if x < pivot]
        eq = sum(1 for x in cur if x == pivot)
        if i <= len(lt):
            cur = lt
        elif i <= len(lt) + eq:
            return pivot
        else:
            i -= len(lt) + eq
            cur = [x for x in cur if x > pivot]
    return sorted(cur)[i - 1]


def median(a: list):
    """Lower median via SELECT."""
    return select(a, (len(a) + 1) // 2)


def demo() -> str:
    lines: list[str] = []
    r = Rng(8)
    v = r.vec(25, 0, 99)
    lines.append(f"A      = {v}")
    lines.append(f"sorted = {sorted(v)}")
    for i in [1, 9, 13, 25]:
        lines.append(f"SELECT(A, {i:>2}) = {select(v, i)}")
    lines.append(f"median = {median(v)}")
    lines.append(f"on 100000 sorted keys, median = {median(list(range(100_000)))}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_sorting_including_duplicates(self) -> None:
        r = Rng(42)
        for n in [1, 2, 5, 6, 11, 37, 100]:
            v = r.vec(n, -8, 8)
            s = sorted(v)
            for i in range(1, n + 1):
                self.assertEqual(select(v, i), s[i - 1])

    def test_adversarial_sorted_input_is_fast(self) -> None:
        v = list(range(200_000))
        self.assertEqual(select(v, 100_000), 99_999)


if __name__ == "__main__":
    print(demo(), end="")
