"""Radix Sort — CLRS 8.3 (1e–4e) (1,2,3,4).

Sort d-digit keys with d passes of a stable sort, least-significant digit first.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _stable_by_digit(a: list[int], base: int, exp: int) -> list[int]:
    c = [0] * (base + 1)
    for x in a:
        c[(x // exp) % base + 1] += 1
    for i in range(1, len(c)):
        c[i] += c[i - 1]
    b = [0] * len(a)
    for x in a:
        d = (x // exp) % base
        b[c[d]] = x
        c[d] += 1
    return b


def radix_sort(a: list[int], base: int) -> list[int]:
    """RADIX-SORT(A, d) using stable counting sort on each digit in `base`."""
    assert base >= 2
    mx = max(a, default=0)
    out = list(a)
    exp = 1
    while True:
        out = _stable_by_digit(out, base, exp)
        exp *= base
        if mx // exp == 0:
            break
    return out


def radix_sort_strings(a: list[str]) -> list[str]:
    """Sort ASCII/UTF-8 byte strings by LSD radix sort."""
    length = max((len(s.encode("utf-8")) for s in a), default=0)
    v = [s.encode("utf-8").ljust(length, b"\x00") for s in a]
    for pos in range(length - 1, -1, -1):
        buckets: list[list[bytes]] = [[] for _ in range(256)]
        for item in v:
            buckets[item[pos]].append(item)
        v = [item for b in buckets for item in b]
    return [b.rstrip(b"\x00").decode("utf-8") for b in v]


def demo() -> str:
    lines: list[str] = []
    a = [329, 457, 657, 839, 436, 720, 355]
    lines.append(f"input          : {a}")
    lines.append(f"radix (base 10): {radix_sort(a, 10)}")
    lines.append(f"radix (base 256): {radix_sort(a, 256)}")
    lines.append(f"strings        : {radix_sort_strings(['pear', 'fig', 'apple', 'kiwi', 'banana', 'date'])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_std_sort_for_several_bases(self) -> None:
        r = Rng(34)
        v = r.vec(500, 0, 1_000_000)
        e = sorted(v)
        for base in [2, 10, 16, 256, 1000]:
            self.assertEqual(radix_sort(v, base), e)
        self.assertEqual(radix_sort([], 10), [])
        self.assertEqual(radix_sort([0, 0], 10), [0, 0])

    def test_handles_u64_extremes(self) -> None:
        v = [(1 << 64) - 1, 0, 1 << 63, 5]
        self.assertEqual(radix_sort(v, 256), [0, 5, 1 << 63, (1 << 64) - 1])

    def test_sorts_strings(self) -> None:
        out = radix_sort_strings(["pear", "fig", "apple", "kiwi"])
        self.assertEqual(out, ["apple", "fig", "kiwi", "pear"])


if __name__ == "__main__":
    print(demo(), end="")
