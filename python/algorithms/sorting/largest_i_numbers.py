"""The i Largest Numbers in Sorted Order — CLRS Problem 9-1 (1e–4e).

Three ways to list the i largest of n numbers: sort (n lg n), heap (n + i lg n), order statistic (n + i lg i).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _max_heapify(a: list[int], i: int, heap_size: int) -> None:
    while True:
        l, r = 2 * i + 1, 2 * i + 2
        largest = i
        if l < heap_size and a[l] > a[largest]:
            largest = l
        if r < heap_size and a[r] > a[largest]:
            largest = r
        if largest == i:
            return
        a[i], a[largest] = a[largest], a[i]
        i = largest


def _build_max_heap(a: list[int]) -> None:
    n = len(a)
    for i in range(n // 2 - 1, -1, -1):
        _max_heapify(a, i, n)


def _select(a: list[int], i: int) -> int:
    if len(a) <= 5:
        return sorted(a)[i - 1]
    medians = []
    for k in range(0, len(a), 5):
        g = sorted(a[k : k + 5])
        medians.append(g[(len(g) - 1) // 2])
    pivot = _select(medians, (len(medians) + 1) // 2)
    lt = [x for x in a if x < pivot]
    eq = sum(1 for x in a if x == pivot)
    if i <= len(lt):
        return _select(lt, i)
    if i <= len(lt) + eq:
        return pivot
    gt = [x for x in a if x > pivot]
    return _select(gt, i - len(lt) - eq)


def by_sorting(a: list[int], i: int) -> list[int]:
    """(a) Sort, then take the top i: Θ(n lg n)."""
    return sorted(a, reverse=True)[:i]


def by_heap(a: list[int], i: int) -> list[int]:
    """(b) Build a max-heap (Θ(n)) and call EXTRACT-MAX i times (Θ(i lg n))."""
    h = list(a)
    _build_max_heap(h)
    size = len(h)
    out: list[int] = []
    for _ in range(min(i, len(a))):
        out.append(h[0])
        h[0], h[size - 1] = h[size - 1], h[0]
        size -= 1
        _max_heapify(h, 0, size)
    return out


def by_order_statistic(a: list[int], i: int) -> list[int]:
    """(c) Find the i-th largest with linear-time SELECT, partition, sort top i: Θ(n + i lg i)."""
    if i == 0 or not a:
        return []
    i = min(i, len(a))
    threshold = _select(a, len(a) - i + 1)
    top = [x for x in a if x > threshold]
    ties = i - len(top)
    top.extend([threshold] * ties)
    top.sort(reverse=True)
    return top


def demo() -> str:
    lines: list[str] = []
    r = Rng(4)
    v = r.vec(30, 0, 99)
    lines.append(f"numbers: {v}")
    lines.append(f"top 5 by sorting       : {by_sorting(v, 5)}")
    lines.append(f"top 5 by heap          : {by_heap(v, 5)}")
    lines.append(f"top 5 by order statistic: {by_order_statistic(v, 5)}")
    lines.append(
        "costs for n = 10⁶, i = 20: sort ≈ n lg n = 2·10⁷; heap ≈ n + i lg n = 10⁶; select ≈ n + i lg i = 10⁶"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_three_agree_including_duplicates_and_extremes(self) -> None:
        r = Rng(313)
        for _ in range(200):
            n = r.range(0, 60)
            v = r.vec(n, -10, 10)
            for i in [0, 1, 3, n // 2, n, n + 3]:
                want = by_sorting(v, i)
                self.assertEqual(by_heap(v, i), want, f"heap n={n} i={i}")
                self.assertEqual(by_order_statistic(v, i), want, f"select n={n} i={i}")


if __name__ == "__main__":
    print(demo(), end="")
