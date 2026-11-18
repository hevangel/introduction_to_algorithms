"""Mergeable Heaps with Linked Lists — CLRS Problem 10-2 (1e-4e) (Editions: 1,2,3,4).

Sorted-list vs unsorted-list mergeable heaps: INSERT/MINIMUM/EXTRACT-MIN/UNION cost trade-offs, with operation counts.
"""

from __future__ import annotations

from bisect import bisect_left
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class SortedListHeap:
    """(a) Sorted list: MINIMUM and EXTRACT-MIN are O(1); INSERT and UNION are O(n). Counts list-node steps."""

    def __init__(self) -> None:
        self.items: list[int] = []  # ascending; index 0 is the head
        self.steps = 0

    def insert(self, x: int) -> None:
        pos = bisect_left(self.items, x)  # first position with items[pos] >= x
        self.steps += pos + 1
        self.items.insert(pos, x)

    def minimum(self) -> int | None:
        self.steps += 1
        return self.items[0] if self.items else None

    def extract_min(self) -> int | None:
        self.steps += 1
        return self.items.pop(0) if self.items else None

    def union(self, other: SortedListHeap) -> None:
        """Merge two sorted lists in O(n1 + n2)."""
        a, b = self.items, other.items
        self.steps += len(a) + len(b)
        out: list[int] = []
        ia = ib = 0
        while ia < len(a) or ib < len(b):
            take_a = ib >= len(b) or (ia < len(a) and a[ia] <= b[ib])
            if take_a:
                out.append(a[ia])
                ia += 1
            else:
                out.append(b[ib])
                ib += 1
        self.items = out

    def __len__(self) -> int:
        return len(self.items)


class UnsortedListHeap:
    """(b) Unsorted list: INSERT and UNION are O(1); MINIMUM and EXTRACT-MIN are O(n)."""

    def __init__(self) -> None:
        self.items: list[int] = []
        self.steps = 0

    def insert(self, x: int) -> None:
        self.steps += 1
        self.items.append(x)

    def minimum(self) -> int | None:
        self.steps += len(self.items)
        return min(self.items) if self.items else None

    def extract_min(self) -> int | None:
        self.steps += len(self.items)
        if not self.items:
            return None
        i = min(range(len(self.items)), key=lambda j: self.items[j])  # first minimum
        last = self.items.pop()
        if i < len(self.items):  # swap_remove
            x = self.items[i]
            self.items[i] = last
            return x
        return last

    def union(self, other: UnsortedListHeap) -> None:
        """O(1) in a linked list (concatenation of two lists); here counted as one step."""
        self.steps += 1
        self.items.extend(other.items)

    def __len__(self) -> int:
        return len(self.items)


def demo() -> str:
    lines: list[str] = []
    r = Rng(1)
    n = 2000
    xs, ys = r.vec(n, 0, 100_000), r.vec(n, 0, 100_000)
    sa, sb = SortedListHeap(), SortedListHeap()
    ua, ub = UnsortedListHeap(), UnsortedListHeap()
    for x in xs:
        sa.insert(x)
        ua.insert(x)
    for y in ys:
        sb.insert(y)
        ub.insert(y)
    ins_s, ins_u = sa.steps + sb.steps, ua.steps + ub.steps
    before_s, before_u = sa.steps, ua.steps
    sa.union(sb)
    ua.union(ub)
    lines.append(
        f"{2 * n} inserts: sorted list {ins_s} steps (~= n^2/4 each), unsorted list {ins_u} steps"
    )
    lines.append(
        f"UNION of two lists of {n}: sorted {sa.steps - before_s} steps, unsorted {ua.steps - before_u} step(s)"
    )
    s0, u0 = sa.steps, ua.steps
    for _ in range(100):
        sa.extract_min()
        ua.extract_min()
    lines.append(
        f"100 EXTRACT-MINs: sorted {sa.steps - s0} steps, unsorted {ua.steps - u0} steps"
    )
    lines.append(
        "neither list gives O(lg n) for everything - that is what binary, binomial and Fibonacci heaps achieve"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_behave_like_a_priority_queue(self) -> None:
        import heapq

        r = Rng(329)
        a, b = SortedListHeap(), UnsortedListHeap()
        model: list[int] = []
        for _ in range(2000):
            if r.range(0, 3) < 2:
                x = r.range(0, 500)
                a.insert(x)
                b.insert(x)
                heapq.heappush(model, x)
            else:
                w = heapq.heappop(model) if model else None
                self.assertEqual(a.extract_min(), w)
                self.assertEqual(b.extract_min(), w)
            want = model[0] if model else None
            self.assertEqual(a.minimum(), want)
            self.assertEqual(b.minimum(), want)

    def test_union_semantics_and_costs(self) -> None:
        a, b = SortedListHeap(), SortedListHeap()
        for x in [5, 1, 9]:
            a.insert(x)
        for x in [4, 2, 10]:
            b.insert(x)
        a.union(b)
        out: list[int] = []
        while (m := a.extract_min()) is not None:
            out.append(m)
        self.assertEqual(out, [1, 2, 4, 5, 9, 10])
        u, v = UnsortedListHeap(), UnsortedListHeap()
        for x in range(1000):
            u.insert(x)
            v.insert(x + 1000)
        s = u.steps
        u.union(v)
        self.assertEqual(u.steps - s, 1)
        self.assertEqual(len(u), 2000)


if __name__ == "__main__":
    print(demo(), end="")
