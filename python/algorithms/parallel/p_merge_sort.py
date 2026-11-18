"""Parallel Merge Sort — CLRS 27.3 (3e) / 26.3 (4e) (Editions: 3,4).

Parallel merge via binary search on the median of the longer array:
span Theta(lg^3 n), parallelism Theta(n / lg^2 n).
"""

from __future__ import annotations

from bisect import bisect_left
from pathlib import Path
import sys
import threading
import unittest
from typing import TypeVar

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

T = TypeVar("T")
CUTOFF = 2048


def _serial_merge(a: list[T], b: list[T], out: list[T], out_start: int) -> None:
    i, j = 0, 0
    la, lb = len(a), len(b)
    for k in range(la + lb):
        if j >= lb or (i < la and a[i] <= b[j]):  # type: ignore[operator]
            out[out_start + k] = a[i]
            i += 1
        else:
            out[out_start + k] = b[j]
            j += 1


def _p_merge_into(a: list[T], b: list[T], out: list[T], out_start: int) -> None:
    if len(a) < len(b):
        a, b = b, a
    if not a:
        return
    if len(a) + len(b) <= CUTOFF:
        _serial_merge(a, b, out, out_start)
        return
    ma = len(a) // 2
    x = a[ma]
    mb = bisect_left(b, x)  # type: ignore[type-var]
    mid = ma + mb
    out[out_start + mid] = x
    t = threading.Thread(target=_p_merge_into, args=(a[:ma], b[:mb], out, out_start))
    t.start()
    _p_merge_into(a[ma + 1 :], b[mb:], out, out_start + mid + 1)
    t.join()


def p_merge(a: list[T], b: list[T], out: list[T]) -> None:
    """P-MERGE: merge sorted `a` and `b` into `out` (len a + len b) recursively in parallel."""
    _p_merge_into(a, b, out, 0)


def _merge_sort_slice(a: list[T], lo: int, hi: int) -> None:
    if hi - lo <= 1:
        return
    a[lo:hi] = sorted(a[lo:hi])  # type: ignore[type-var]


def _p_merge_sort_range(a: list[T], lo: int, hi: int) -> None:
    n = hi - lo
    if n <= CUTOFF:
        _merge_sort_slice(a, lo, hi)
        return
    mid = lo + n // 2
    t = threading.Thread(target=_p_merge_sort_range, args=(a, lo, mid))
    t.start()
    _p_merge_sort_range(a, mid, hi)
    t.join()
    tmp = [a[0]] * n
    _p_merge_into(a[lo:mid], a[mid:hi], tmp, 0)
    a[lo:hi] = tmp


def p_merge_sort(a: list[T]) -> None:
    """P-MERGE-SORT(A, p, r): sort `a` in place."""
    _p_merge_sort_range(a, 0, len(a))


def demo() -> str:
    lines: list[str] = []
    r = Rng(77)
    v = r.vec(200_000, -1_000_000, 1_000_000)
    e = sorted(v)
    p_merge_sort(v)
    lines.append(
        f"sorted 200000 random integers in parallel; equals std sort: {str(v == e).lower()}"
    )
    a = [x * 2 for x in range(10)]
    b = [x * 3 for x in range(8)]
    out = [0] * 18
    p_merge(a, b, out)
    lines.append(f"p_merge of {a} and {b} = {out}")
    lines.append(
        "analysis: work Theta(n lg n), span Theta(lg^3 n) with parallel merge (vs Theta(n) with a serial MERGE)"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_merge_matches_sorted_concatenation(self) -> None:
        r = Rng(152)
        for na, nb in [(0, 0), (0, 5), (7, 0), (3000, 5000), (10_000, 100), (5000, 5000)]:
            a = sorted(r.vec(na, -50, 50))
            b = sorted(r.vec(nb, -50, 50))
            e = sorted(a + b)
            out = [0] * (na + nb)
            p_merge(a, b, out)
            self.assertEqual(out, e, f"{na} {nb}")

    def test_sort_matches_std(self) -> None:
        r = Rng(153)
        for n in [0, 1, 100, 2048, 2049, 50_000]:
            v = r.vec(n, -1000, 1000)
            e = sorted(v)
            p_merge_sort(v)
            self.assertEqual(v, e)


if __name__ == "__main__":
    print(demo(), end="")
