"""Longest Common Subsequence — CLRS 15.4 (2e–3e), 14.4 (4e), 16.3 (1e) (1e,2e,3e,4e).

Theta(mn) dynamic program with table reconstruction and Theta(min(m,n))-space length computation.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def lcs_table(x: str | bytes, y: str | bytes) -> list[list[int]]:
    """LCS-LENGTH: the c table, c[i][j] = |LCS(x[:i], y[:j])|."""
    m, n = len(x), len(y)
    c = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(1, m + 1):
        xi = x[i - 1]
        for j in range(1, n + 1):
            if xi == y[j - 1]:
                c[i][j] = c[i - 1][j - 1] + 1
            else:
                c[i][j] = max(c[i - 1][j], c[i][j - 1])
    return c


def lcs(x: str, y: str) -> str:
    """PRINT-LCS: walk the table backwards to build one longest common subsequence."""
    c = lcs_table(x, y)
    i, j = len(x), len(y)
    out: list[str] = []
    while i > 0 and j > 0:
        if x[i - 1] == y[j - 1]:
            out.append(x[i - 1])
            i -= 1
            j -= 1
        elif c[i - 1][j] >= c[i][j - 1]:
            i -= 1
        else:
            j -= 1
    out.reverse()
    return "".join(out)


def lcs_length_two_rows(x: str | bytes, y: str | bytes) -> int:
    """Length only, using two rows: O(min(m,n)) space (Exercise 15.4-4)."""
    if len(x) < len(y):
        x, y = y, x
    prev = [0] * (len(y) + 1)
    for xc in x:
        cur = [0] * (len(y) + 1)
        for j in range(1, len(y) + 1):
            cur[j] = prev[j - 1] + 1 if xc == y[j - 1] else max(prev[j], cur[j - 1])
        prev = cur
    return prev[len(y)]


def lcs_memoized(x: str | bytes, y: str | bytes) -> int:
    """Memoized top-down version."""
    memo: list[list[int | None]] = [[None] * (len(y) + 1) for _ in range(len(x) + 1)]

    def go(i: int, j: int) -> int:
        if i == 0 or j == 0:
            return 0
        cached = memo[i][j]
        if cached is not None:
            return cached
        v = go(i - 1, j - 1) + 1 if x[i - 1] == y[j - 1] else max(go(i - 1, j), go(i, j - 1))
        memo[i][j] = v
        return v

    return go(len(x), len(y))


def demo() -> str:
    lines: list[str] = []
    x, y = "ABCBDAB", "BDCABA"
    c = lcs_table(x, y)
    lines.append(f"X = {x}, Y = {y}")
    lines.append("c table (rows = prefixes of X, columns = prefixes of Y):")
    for row in c:
        lines.append(f"  {row}")
    lines.append(f"LCS = {lcs(x, y)} (length {c[len(x)][len(y)]})")
    lines.append(
        f"two-row length = {lcs_length_two_rows(x, y)}, memoized length = {lcs_memoized(x, y)}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _is_subsequence(self, s: str, t: str) -> bool:
        it = iter(t)
        return all(c in it for c in s)

    def _brute_len(self, x: str, y: str) -> int:
        best = 0
        for m in range(1 << len(x)):
            sub = "".join(x[i] for i in range(len(x)) if (m >> i) & 1)
            if self._is_subsequence(sub, y):
                best = max(best, len(sub))
        return best

    def test_book_example(self) -> None:
        self.assertEqual(len(lcs("ABCBDAB", "BDCABA")), 4)
        self.assertEqual(lcs_length_two_rows(b"ABCBDAB", b"BDCABA"), 4)
        self.assertEqual(lcs("", "abc"), "")

    def test_random_strings_agree_with_brute_force(self) -> None:
        r = Rng(102)
        for _ in range(60):
            n1, n2 = r.range(0, 9), r.range(0, 9)
            x = "".join(chr(ord("a") + r.range(0, 2)) for _ in range(n1))
            y = "".join(chr(ord("a") + r.range(0, 2)) for _ in range(n2))
            l = lcs(x, y)
            self.assertTrue(self._is_subsequence(l, x) and self._is_subsequence(l, y))
            self.assertEqual(len(l), self._brute_len(x, y))
            self.assertEqual(lcs_length_two_rows(x, y), len(l))
            self.assertEqual(lcs_memoized(x, y), len(l))


if __name__ == "__main__":
    print(demo(), end="")
