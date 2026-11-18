"""Bucket Sort — CLRS 8.4 (1e–4e).

Distribute uniform keys in [0,1) into n buckets, sort each by insertion sort: Θ(n) expected.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def bucket_sort(a: list[float]) -> list[float]:
    """BUCKET-SORT(A): `a` values must lie in `[0, 1)`."""
    n = len(a)
    if n == 0:
        return []
    buckets: list[list[float]] = [[] for _ in range(n)]
    for x in a:
        buckets[int(n * x)].append(x)
    out: list[float] = []
    for b in buckets:
        for j in range(1, len(b)):
            key = b[j]
            i = j
            while i > 0 and b[i - 1] > key:
                b[i] = b[i - 1]
                i -= 1
            b[i] = key
        out.extend(b)
    return out


def max_bucket(a: list[float]) -> int:
    """Size of the largest bucket (a proxy for the extra work done by insertion sort)."""
    n = len(a)
    c = [0] * max(n, 1)
    for x in a:
        c[int(n * x)] += 1
    return max(c, default=0)


def demo() -> str:
    lines: list[str] = []
    a = [0.78, 0.17, 0.39, 0.26, 0.72, 0.94, 0.21, 0.12, 0.23, 0.68]
    lines.append(f"input : {a}")
    lines.append(f"sorted: {bucket_sort(a)}")
    r = Rng(3)
    big = [r.f64() for _ in range(10000)]
    lines.append(f"10000 uniform keys: largest bucket holds {max_bucket(big)} keys (expected small)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_uniform_floats(self) -> None:
        r = Rng(35)
        for n in [0, 1, 2, 17, 500]:
            v = [r.f64() for _ in range(n)]
            e = sorted(v)
            self.assertEqual(bucket_sort(v), e)

    def test_buckets_are_small_for_uniform_input(self) -> None:
        r = Rng(36)
        v = [r.f64() for _ in range(20000)]
        self.assertLess(max_bucket(v), 12)


if __name__ == "__main__":
    print(demo(), end="")
