"""Bitonic Sorting Network — CLRS 27.3–27.4 (1e, 2e).

Half-cleaners turn a bitonic sequence into two bitonic halves; depth lg n·(lg n+1)/2.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Network:
    """Comparison network of (i, j) comparators with i < j."""

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
        t = [0] * self.wires
        for i, j in self.comparators:
            d = max(t[i], t[j]) + 1
            t[i] = d
            t[j] = d
        return max(t, default=0)

    def is_sorting_network(self) -> bool:
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


def _half_cleaner(net: Network, lo: int, n: int) -> None:
    half = n // 2
    for i in range(half):
        net.add(lo + i, lo + i + half)


def _bitonic_sorter(net: Network, lo: int, n: int) -> None:
    if n > 1:
        _half_cleaner(net, lo, n)
        _bitonic_sorter(net, lo, n // 2)
        _bitonic_sorter(net, lo + n // 2, n // 2)


def _merger(net: Network, lo: int, n: int) -> None:
    for i in range(n // 2):
        net.add(lo + i, lo + n - 1 - i)
    _bitonic_sorter(net, lo, n // 2)
    _bitonic_sorter(net, lo + n // 2, n // 2)


def _sorter(net: Network, lo: int, n: int) -> None:
    if n > 1:
        _sorter(net, lo, n // 2)
        _sorter(net, lo + n // 2, n // 2)
        _merger(net, lo, n)


def bitonic_sorting_network(n: int) -> Network:
    """Build the full bitonic sorting network on n = 2^k wires."""
    assert n >= 1 and (n & (n - 1)) == 0
    net = Network(n)
    _sorter(net, 0, n)
    return net


def is_bitonic(a: list[int]) -> bool:
    """Is the sequence bitonic (increasing then decreasing, up to a cyclic shift)?"""
    n = len(a)
    if n == 0:
        return True
    changes = 0
    for i in range(n):
        p, c, nx = a[(i - 1) % n], a[i], a[(i + 1) % n]
        if (c > p and c > nx) or (c < p and c < nx):
            changes += 1
    return changes <= 2


def demo() -> str:
    lines: list[str] = []
    for k in range(1, 6):
        n = 1 << k
        net = bitonic_sorting_network(n)
        lines.append(
            f"n={n:>2}: comparators={net.size():>3} depth={net.depth():>2} (lg n·(lg n+1)/2 = {k * (k + 1) // 2})"
        )
    v = [7, 3, 9, 1, 8, 2, 6, 4]
    bitonic_sorting_network(8).apply(v)
    lines.append(f"sorted by the 8-wire network: {v}")
    lines.append(
        f"is_bitonic([1,4,6,5,2]) = {is_bitonic([1, 4, 6, 5, 2])}   "
        f"is_bitonic([1,5,2,6,3]) = {is_bitonic([1, 5, 2, 6, 3])}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_is_a_sorting_network_for_powers_of_two(self) -> None:
        for k in range(5):
            self.assertTrue(bitonic_sorting_network(1 << k).is_sorting_network(), f"n={1 << k}")

    def test_depth_and_size_formulas(self) -> None:
        for k in range(1, 7):
            n = 1 << k
            net = bitonic_sorting_network(n)
            self.assertEqual(net.depth(), k * (k + 1) // 2)
            self.assertEqual(net.size(), (n // 2) * (k * (k + 1) // 2))

    def test_sorts_random_integers(self) -> None:
        net = bitonic_sorting_network(32)
        r = Rng(38)
        for _ in range(50):
            v = r.vec(32, -500, 500)
            e = sorted(v)
            net.apply(v)
            self.assertEqual(v, e)

    def test_bitonic_predicate(self) -> None:
        self.assertTrue(is_bitonic([1, 3, 5, 4, 2]))
        self.assertTrue(is_bitonic([4, 2, 1, 3, 5]))
        self.assertFalse(is_bitonic([1, 5, 2, 6, 3]))


if __name__ == "__main__":
    print(demo(), end="")
