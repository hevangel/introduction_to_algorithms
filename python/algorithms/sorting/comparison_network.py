"""Comparison Networks & the 0-1 Principle — CLRS 27.1 (1e, 2e).

Fixed sequences of compare-exchange gates; test any network exhaustively on 0-1 inputs.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Network:
    """A comparison network: `comparators[k] = (i, j)` with `i < j`."""

    def __init__(self, wires: int) -> None:
        self.wires = wires
        self.comparators: list[tuple[int, int]] = []

    def add(self, i: int, j: int) -> None:
        assert 0 <= i < j < self.wires
        self.comparators.append((i, j))

    def apply(self, a: list) -> None:
        for i, j in self.comparators:
            if a[i] > a[j]:
                a[i], a[j] = a[j], a[i]

    def size(self) -> int:
        return len(self.comparators)

    def depth(self) -> int:
        """Depth = longest chain of comparators (parallel running time)."""
        t = [0] * self.wires
        for i, j in self.comparators:
            d = max(t[i], t[j]) + 1
            t[i] = d
            t[j] = d
        return max(t, default=0)

    def is_sorting_network(self) -> bool:
        """0-1 principle: a network sorts all inputs iff it sorts all 2ⁿ 0-1 sequences (n ≤ 22)."""
        assert self.wires <= 22
        w = self.wires
        comps = self.comparators
        for bits in range(1 << w):
            v = [(bits >> b) & 1 for b in range(w)]
            for i, j in comps:
                if v[i] > v[j]:
                    v[i], v[j] = v[j], v[i]
            if any(v[k] > v[k + 1] for k in range(w - 1)):
                return False
        return True


def insertion_network(n: int) -> Network:
    """Insertion sort as a comparison network (depth 2n−3, size n(n−1)/2)."""
    net = Network(n)
    for i in range(1, n):
        for j in range(i - 1, -1, -1):
            net.add(j, j + 1)
    return net


def odd_even_transposition(n: int) -> Network:
    """Odd-even transposition network: n rounds of alternating adjacent comparators (depth n)."""
    net = Network(n)
    for rnd in range(n):
        for i in range(rnd % 2, max(n - 1, 0), 2):
            net.add(i, i + 1)
    return net


def demo() -> str:
    lines: list[str] = []
    for n in [4, 6, 8]:
        ins = insertion_network(n)
        oet = odd_even_transposition(n)
        lines.append(
            f"n={n}: insertion net size={ins.size():>2} depth={ins.depth():>2} sorts={ins.is_sorting_network()}   "
            f"odd-even transposition size={oet.size():>2} depth={oet.depth():>2} sorts={oet.is_sorting_network()}"
        )
    bad = Network(3)
    bad.add(0, 1)
    lines.append(f"one comparator on 3 wires sorts? {bad.is_sorting_network()}")
    v = [9, 4, 7, 1]
    insertion_network(4).apply(v)
    lines.append(f"apply to [9,4,7,1] -> {v}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_known_networks_sort(self) -> None:
        for n in range(1, 10):
            self.assertTrue(insertion_network(n).is_sorting_network(), f"insertion {n}")
            self.assertTrue(odd_even_transposition(n).is_sorting_network(), f"oet {n}")

    def test_depth_and_size(self) -> None:
        n = insertion_network(8)
        self.assertEqual(n.size(), 28)
        self.assertEqual(n.depth(), 13)
        self.assertEqual(odd_even_transposition(8).depth(), 8)

    def test_zero_one_principle_agrees_with_integers(self) -> None:
        net = odd_even_transposition(8)
        r = Rng(37)
        for _ in range(200):
            v = r.vec(8, -100, 100)
            e = sorted(v)
            net.apply(v)
            self.assertEqual(v, e)
        bad = Network(4)
        bad.add(0, 1)
        bad.add(2, 3)
        self.assertFalse(bad.is_sorting_network())


if __name__ == "__main__":
    print(demo(), end="")
