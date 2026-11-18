"""Greedy Set Cover — CLRS 37.3 (1e) / 35.3 (2e–4e) (1,2,3,4).

Repeatedly pick the set covering the most uncovered elements: an H(d)-approximation, H(d) ≤ ln d + 1.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def greedy_set_cover(universe: int, sets: list[list[int]]) -> list[int] | None:
    """GREEDY-SET-COVER(X, F): subsets of {0, ..., universe - 1}. Returns indices or None."""
    uncovered = [True] * universe
    left = universe
    chosen: list[int] = []
    while left > 0:
        if not sets:
            return None
        best, gain = max(
            ((i, sum(1 for e in s if uncovered[e])) for i, s in enumerate(sets)),
            key=lambda x: x[1],
        )
        if gain == 0:
            return None
        for e in sets[best]:
            if uncovered[e]:
                uncovered[e] = False
                left -= 1
        chosen.append(best)
    return chosen


def optimal_cover_size(universe: int, sets: list[list[int]]) -> int | None:
    """Optimal cover size by exhaustive search (exponential)."""
    best: int | None = None
    for mask in range(1 << len(sets)):
        cov = [False] * universe
        for i, s in enumerate(sets):
            if (mask >> i) & 1:
                for e in s:
                    cov[e] = True
        if all(cov):
            cnt = mask.bit_count()
            if best is None or cnt < best:
                best = cnt
    return best


def harmonic(d: int) -> float:
    return sum(1.0 / i for i in range(1, d + 1))


def demo() -> str:
    sets = [[0, 1, 2, 3, 4, 5], [0, 3, 6, 9], [1, 4, 7, 10], [2, 5, 8, 11], [6, 7, 8, 9, 10, 11]]
    bad = [[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13], [0, 1, 2, 3, 7, 8, 9, 10]]
    g_bad = greedy_set_cover(14, bad)
    return "\n".join([
        f"universe 0..12 with {len(sets)} sets",
        f"greedy picks {greedy_set_cover(12, sets)}",
        f"optimal size = {optimal_cover_size(12, sets)}",
        f"adversarial: greedy {len(g_bad) if g_bad is not None else None}, optimal {optimal_cover_size(14, bad)}",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_covers_everything_within_harmonic_bound(self) -> None:
        r = Rng(252)
        checked = 0
        for _ in range(300):
            u, m = r.range(1, 12), r.range(1, 10)
            sets = [[e for e in range(u) if r.f64() < 0.35] for _ in range(m)]
            opt = optimal_cover_size(u, sets)
            g = greedy_set_cover(u, sets)
            self.assertEqual(opt is not None, g is not None)
            if opt is not None and g is not None:
                cov = [False] * u
                for i in g:
                    for e in sets[i]:
                        cov[e] = True
                self.assertTrue(all(cov))
                d = max(len(s) for s in sets)
                self.assertLessEqual(len(g), opt * harmonic(d) + 1e-9)
                checked += 1
        self.assertGreater(checked, 100)

    def test_greedy_can_be_suboptimal(self) -> None:
        bad = [[0, 1, 2, 3, 4, 5, 6], [7, 8, 9, 10, 11, 12, 13], [0, 1, 2, 3, 7, 8, 9, 10]]
        self.assertEqual(optimal_cover_size(14, bad), 2)
        g = greedy_set_cover(14, bad)
        assert g is not None
        self.assertEqual(len(g), 3)
        self.assertIsNone(greedy_set_cover(3, [[0], [1]]))


if __name__ == "__main__":
    print(demo(), end="")
