"""Suffix Arrays & LCP Arrays — CLRS 32.5 (4e) (4).

Sorted array of all suffixes (prefix doubling) with Kasai's LCP array: substring search in O(m lg n).
"""

from __future__ import annotations

import bisect
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def suffix_array(s: bytes) -> list[int]:
    """SUFFIX-ARRAY(T): indices of all suffixes of T in lexicographic order (prefix doubling)."""
    n = len(s)
    sa = list(range(n))
    rank = list(s)
    k = 1
    while k < max(n, 1):
        key = [(rank[i], rank[i + k] + 1 if i + k < n else 0) for i in range(n)]
        sa.sort(key=lambda i: key[i])
        new_rank = [0] * n
        for w in range(1, n):
            new_rank[sa[w]] = new_rank[sa[w - 1]] + (1 if key[sa[w]] != key[sa[w - 1]] else 0)
        rank = new_rank
        if n > 0 and rank[sa[n - 1]] == n - 1:
            break
        k *= 2
    return sa


def lcp_array(s: bytes, sa: list[int]) -> list[int]:
    """LCP array by Kasai et al.: O(n)."""
    n = len(s)
    rank = [0] * n
    for i, p in enumerate(sa):
        rank[p] = i
    lcp = [0] * n
    h = 0
    for i in range(n):
        if rank[i] > 0:
            j = sa[rank[i] - 1]
            while i + h < n and j + h < n and s[i + h] == s[j + h]:
                h += 1
            lcp[rank[i]] = h
            h = max(0, h - 1)
        else:
            h = 0
    return lcp


def search(s: bytes, sa: list[int], p: bytes) -> list[int]:
    """All occurrences of p in s by binary searching the suffix array."""
    class _PrefixView:
        def __init__(self) -> None:
            pass
        def __len__(self) -> int:
            return len(sa)
        def __getitem__(self, idx: int) -> bytes:
            i = sa[idx]
            return s[i : min(i + len(p), len(s))]

    view = _PrefixView()
    lo = bisect.bisect_left(view, p)  # type: ignore[arg-type]
    hi = bisect.bisect_right(view, p)  # type: ignore[arg-type]
    return sorted(sa[lo:hi])


def longest_repeated_substring(s: bytes) -> bytes:
    """Longest repeated substring via the maximum LCP value."""
    sa = suffix_array(s)
    lcp = lcp_array(s, sa)
    if not lcp:
        return b""
    i = max(range(len(s)), key=lambda idx: lcp[idx])
    if lcp[i] == 0:
        return b""
    return s[sa[i] : sa[i] + lcp[i]]


def demo() -> str:
    lines: list[str] = []
    t = b"banana"
    sa = suffix_array(t)
    lcp = lcp_array(t, sa)
    lines.append("T = banana")
    lines.append(" i | SA[i] | LCP[i] | suffix")
    for i in range(len(t)):
        lines.append(f"{i:>2} | {sa[i]:>5} | {lcp[i]:>6} | {t[sa[i]:].decode()}")
    lines.append(f"occurrences of 'ana': {search(t, sa, b'ana')}")
    lines.append(f"longest repeated substring: {longest_repeated_substring(b'abracadabra').decode()!r}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_banana(self) -> None:
        sa = suffix_array(b"banana")
        self.assertEqual(sa, [5, 3, 1, 0, 4, 2])
        self.assertEqual(lcp_array(b"banana", sa), [0, 1, 3, 0, 0, 2])
        self.assertEqual(search(b"banana", sa, b"ana"), [1, 3])
        self.assertEqual(longest_repeated_substring(b"abracadabra"), b"abra")

    def test_matches_sorting_all_suffixes_and_naive_search(self) -> None:
        r = Rng(224)
        for _ in range(200):
            s = bytes(ord("a") + r.range(0, 2) for _ in range(r.range(0, 40)))
            sa = suffix_array(s)
            want = sorted(range(len(s)), key=lambda i: s[i:])
            self.assertEqual(sa, want)
            lcp = lcp_array(s, sa)
            for i in range(1, len(s)):
                a, b = s[sa[i - 1] :], s[sa[i] :]
                k = 0
                while k < len(a) and k < len(b) and a[k] == b[k]:
                    k += 1
                self.assertEqual(lcp[i], k)
            p = bytes(ord("a") + r.range(0, 2) for _ in range(r.range(1, 4)))
            naive = [pos for pos in range(len(s) - len(p) + 1) if s[pos : pos + len(p)] == p]
            self.assertEqual(search(s, sa, p), naive)


if __name__ == "__main__":
    print(demo(), end="")
