"""Sorting Variable-Length Items — CLRS Problem 8-3 (1e–4e) (1,2,3,4).

Sort integers of different digit counts in time linear in the total number of digits (group by length, then radix sort), and strings in time linear in their total length (MSD radix sort).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _digit_count(x: int) -> int:
    d = 1
    while x >= 10:
        x //= 10
        d += 1
    return d


def sort_integers_by_digits(a: list[int]) -> list[int]:
    """Problem 8-3(a): sort non-negative integers with varying digit counts in O(total digits)."""
    maxd = max((_digit_count(x) for x in a), default=0)
    groups: list[list[int]] = [[] for _ in range(maxd + 1)]
    for x in a:
        groups[_digit_count(x)].append(x)
    out: list[int] = []
    for d, g in enumerate(groups):
        p = 1
        for _ in range(d):
            buckets: list[list[int]] = [[] for _ in range(10)]
            for x in g:
                buckets[(x // p) % 10].append(x)
            g = [x for b in buckets for x in b]
            p *= 10
        out.extend(g)
    return out


def sort_strings(a: list[bytes]) -> list[bytes]:
    """Problem 8-3(b): sort byte strings lexicographically with MSD radix sort."""
    out: list[bytes] = []

    def rec(items: list[bytes], depth: int) -> None:
        if len(items) <= 1:
            out.extend(items)
            return
        ended: list[bytes] = []
        buckets: list[list[bytes]] = [[] for _ in range(256)]
        for s in items:
            if len(s) == depth:
                ended.append(s)
            else:
                buckets[s[depth]].append(s)
        out.extend(ended)
        for b in buckets:
            if b:
                rec(b, depth + 1)

    rec(list(a), 0)
    return out


def demo() -> str:
    lines: list[str] = []
    nums = [1000, 7, 42, 123456, 9, 100, 99999, 12, 0, 5000]
    lines.append(f"integers {nums}")
    lines.append(f"sorted   {sort_integers_by_digits(nums)}")
    words = [s.encode("utf-8") for s in ["banana", "app", "apple", "", "band", "ban", "apply", "a"]]
    sorted_words = [w.decode("utf-8") for w in sort_strings(words)]
    lines.append(f"strings  {sorted_words}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_integers_match_std_sort(self) -> None:
        r = Rng(441)
        for _ in range(200):
            n = r.range(0, 60)
            a = [r.next_u64() % (10 ** (r.range(0, 12) + 1)) for _ in range(n)]
            self.assertEqual(sort_integers_by_digits(a), sorted(a))

    def test_strings_match_std_sort(self) -> None:
        r = Rng(442)
        for _ in range(200):
            a = [bytes([ord("a") + r.range(0, 2) for _ in range(r.range(0, 8))]) for _ in range(r.range(0, 40))]
            self.assertEqual(sort_strings(a), sorted(a))
        self.assertEqual(sort_strings([b"b", b"", b"ab", b"a"]), [b"", b"a", b"ab", b"b"])


if __name__ == "__main__":
    print(demo(), end="")
