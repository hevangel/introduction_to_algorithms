"""Boyer-Moore Algorithm — CLRS 34.5 (1e) (1).

Scan the pattern right to left; bad-character and good-suffix heuristics skip text positions: sublinear in practice.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def last_occurrence(p: bytes) -> list[int]:
    """Bad-character table: last position of each byte in P (-1 if absent)."""
    l = [-1] * 256
    for i, c in enumerate(p):
        l[c] = i
    return l


def good_suffix_shift(p: bytes) -> list[int]:
    """Strong good-suffix shifts: shift[j] is how far to slide after a mismatch at j-1."""
    m = len(p)
    shift = [0] * (m + 1)
    border = [0] * (m + 1)
    i, j = m, m + 1
    border[i] = j
    while i > 0:
        while j <= m and p[i - 1] != p[j - 1]:
            if shift[j] == 0:
                shift[j] = j - i
            j = border[j]
        i -= 1
        j -= 1
        border[i] = j
    j = border[0]
    for i in range(m + 1):
        if shift[i] == 0:
            shift[i] = j
        if i == j:
            j = border[j]
    return shift


def boyer_moore(t: bytes, p: bytes) -> tuple[list[int], int]:
    """BOYER-MOORE-MATCHER: all valid shifts, using both heuristics. Returns (shifts, comparisons)."""
    n, m = len(t), len(p)
    if m == 0:
        return (list(range(n + 1)), 0)
    if m > n:
        return ([], 0)
    last = last_occurrence(p)
    gs = good_suffix_shift(p)
    s = 0
    found: list[int] = []
    cmps = 0
    while s <= n - m:
        j = m
        while j > 0:
            cmps += 1
            if p[j - 1] != t[s + j - 1]:
                break
            j -= 1
        if j == 0:
            found.append(s)
            s += gs[0]
        else:
            bad = max(j - 1 - last[t[s + j - 1]], 1)
            s += max(bad, gs[j])
    return (found, cmps)


def demo() -> str:
    lines: list[str] = []
    t, p = b"here is a simple example, a simple one", b"simple"
    shifts, cmps = boyer_moore(t, p)
    lines.append(f"matches at {shifts} using {cmps} comparisons (text length {len(t)})")
    lines.append(
        f"good-suffix shifts for 'ABYXCDBYXCD'-like pattern 'anpanman': {good_suffix_shift(b'anpanman')}"
    )
    big = ("abcdefghij" * 5000).encode()
    s, c = boyer_moore(big, b"jabcdefghx")
    lines.append(f"sublinear behaviour: {len(s)} matches, {c} comparisons for a text of {len(big)} chars")
    return "\n".join(lines) + "\n"


def _naive(t: bytes, p: bytes) -> list[int]:
    n, m = len(t), len(p)
    if m > n:
        return []
    return [s for s in range(n - m + 1) if t[s : s + m] == p]


class Tests(unittest.TestCase):
    def test_agrees_with_naive_including_periodic_patterns(self) -> None:
        r = Rng(223)
        for _ in range(500):
            alpha = r.range(1, 3)
            t = bytes(ord("a") + r.range(0, alpha - 1) for _ in range(r.range(0, 80)))
            p = bytes(ord("a") + r.range(0, alpha - 1) for _ in range(r.range(1, 8)))
            self.assertEqual(boyer_moore(t, p)[0], _naive(t, p), f"t={t!r} p={p!r}")
        self.assertEqual(boyer_moore(b"aaaaaa", b"aa")[0], [0, 1, 2, 3, 4])

    def test_skips_text_when_pattern_chars_are_absent(self) -> None:
        t = b"x" * 10_000
        s, cmps = boyer_moore(t, b"abcdefgh")
        self.assertEqual(s, [])
        self.assertLess(cmps, len(t) // 4)


if __name__ == "__main__":
    print(demo(), end="")
