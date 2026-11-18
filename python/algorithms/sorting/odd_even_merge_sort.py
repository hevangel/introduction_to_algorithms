"""Odd-Even Merging Network — CLRS Problem 28-2 (1e) / Problem 27-2 (2e).

Batcher's odd-even merge sort: merge by recursively merging odd and even subsequences.
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


def _oddeven_merge(net: Network, lo: int, hi: int, r: int) -> None:
    step = r * 2
    if step < hi - lo:
        _oddeven_merge(net, lo, hi, step)
        _oddeven_merge(net, lo + r, hi, step)
        i = lo + r
        while i + r <= hi:
            net.add(i, i + r)
            i += step
    else:
        net.add(lo, lo + r)


def _sort_range(net: Network, lo: int, hi: int) -> None:
    if hi > lo:
        mid = lo + (hi - lo) // 2
        _sort_range(net, lo, mid)
        _sort_range(net, mid + 1, hi)
        _oddeven_merge(net, lo, hi, 1)


def odd_even_merge_sort_network(n: int) -> Network:
    """Batcher's odd-even merge sorting network for n = 2^k wires."""
    assert n >= 1 and (n & (n - 1)) == 0
    net = Network(n)
    if n > 1:
        _sort_range(net, 0, n - 1)
    return net


def odd_even_merging_network(n: int) -> Network:
    """A network that only merges: wires 0..n/2 and n/2..n are each sorted on input."""
    assert n >= 2 and (n & (n - 1)) == 0
    net = Network(n)
    _oddeven_merge(net, 0, n - 1, 1)
    return net


def _bitonic_sorting_network(n: int) -> Network:
    def half_cleaner(net: Network, lo: int, m: int) -> None:
        for i in range(m // 2):
            net.add(lo + i, lo + i + m // 2)

    def bitonic_sorter(net: Network, lo: int, m: int) -> None:
        if m > 1:
            half_cleaner(net, lo, m)
            bitonic_sorter(net, lo, m // 2)
            bitonic_sorter(net, lo + m // 2, m // 2)

    def merger(net: Network, lo: int, m: int) -> None:
        for i in range(m // 2):
            net.add(lo + i, lo + m - 1 - i)
        bitonic_sorter(net, lo, m // 2)
        bitonic_sorter(net, lo + m // 2, m // 2)

    def sorter(net: Network, lo: int, m: int) -> None:
        if m > 1:
            sorter(net, lo, m // 2)
            sorter(net, lo + m // 2, m // 2)
            merger(net, lo, m)

    net = Network(n)
    sorter(net, 0, n)
    return net


def demo() -> str:
    lines: list[str] = []
    lines.append("   n | odd-even merge sort (size, depth) | bitonic (size, depth)")
    for k in range(1, 6):
        n = 1 << k
        a = odd_even_merge_sort_network(n)
        b = _bitonic_sorting_network(n)
        lines.append(f"{n:>4} | ({a.size():>3}, {a.depth():>2})                          | ({b.size():>3}, {b.depth():>2})")
    v = [1, 4, 6, 9, 2, 3, 7, 8]
    odd_even_merging_network(8).apply(v)
    lines.append(f"merging two sorted halves [1,4,6,9] [2,3,7,8] -> {v}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_full_sorter_passes_zero_one_test(self) -> None:
        for k in range(5):
            self.assertTrue(odd_even_merge_sort_network(1 << k).is_sorting_network(), f"n={1 << k}")

    def test_uses_fewer_comparators_than_bitonic(self) -> None:
        for k in range(2, 7):
            n = 1 << k
            a = odd_even_merge_sort_network(n)
            self.assertEqual(a.depth(), k * (k + 1) // 2)
            self.assertLess(a.size(), _bitonic_sorting_network(n).size())

    def test_merging_network_merges_sorted_halves(self) -> None:
        r = Rng(39)
        net = odd_even_merging_network(16)
        for _ in range(100):
            v = r.vec(16, -50, 50)
            v[:8] = sorted(v[:8])
            v[8:] = sorted(v[8:])
            e = sorted(v)
            net.apply(v)
            self.assertEqual(v, e)


if __name__ == "__main__":
    print(demo(), end="")
