"""Knuth-Morris-Pratt Algorithm — CLRS 32.4 (1e–4e) (1, 2, 3, 4).

The prefix function π lets the matcher never move backwards in the text: Θ(n + m) with O(m) space.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def prefix_function(p: bytes) -> list[int]:
    """COMPUTE-PREFIX-FUNCTION(P): pi[q] = length of longest proper prefix of P[:q+1] that is a suffix."""
    pi = [0] * len(p)
    k = 0
    for q in range(1, len(p)):
        while k > 0 and p[k] != p[q]:
            k = pi[k - 1]
        if p[k] == p[q]:
            k += 1
        pi[q] = k
    return pi


def kmp_matcher(t: bytes, p: bytes) -> list[int]:
    """KMP-MATCHER(T, P): all valid shifts."""
    m = len(p)
    if m == 0:
        return list(range(len(t) + 1))
    pi = prefix_function(p)
    q = 0
    shifts: list[int] = []
    for i, c in enumerate(t):
        while q > 0 and p[q] != c:
            q = pi[q - 1]
        if p[q] == c:
            q += 1
        if q == m:
            shifts.append(i + 1 - m)
            q = pi[q - 1]
    return shifts


def smallest_period(p: bytes) -> int:
    """Smallest period of P: m - pi[m - 1]."""
    return 0 if not p else len(p) - prefix_function(p)[-1]


def demo() -> str:
    lines: list[str] = []
    p = b"ababaca"
    lines.append(f"π for ababaca = {prefix_function(p)}")
    lines.append(f"KMP on T = abababacaba: shifts {kmp_matcher(b'abababacaba', p)}")
    lines.append(f"KMP on T = aaaaaaa, P = aa: shifts {kmp_matcher(b'aaaaaaa', b'aa')}")
    lines.append(f"smallest period of 'abcabcabc' = {smallest_period(b'abcabcabc')}")
    return "\n".join(lines) + "\n"


def _naive(t: bytes, p: bytes) -> list[int]:
    n, m = len(t), len(p)
    if m > n:
        return []
    return [s for s in range(n - m + 1) if t[s : s + m] == p]


class Tests(unittest.TestCase):
    def test_prefix_function_of_figure(self) -> None:
        self.assertEqual(prefix_function(b"ababaca"), [0, 0, 1, 2, 3, 0, 1])
        self.assertEqual(prefix_function(b"aaaa"), [0, 1, 2, 3])
        self.assertEqual(prefix_function(b""), [])

    def test_agrees_with_naive(self) -> None:
        r = Rng(222)
        for _ in range(300):
            t = bytes(ord("a") + r.range(0, 1) for _ in range(r.range(0, 60)))
            p = bytes(ord("a") + r.range(0, 1) for _ in range(r.range(1, 7)))
            self.assertEqual(kmp_matcher(t, p), _naive(t, p))
        self.assertEqual(smallest_period(b"abcabcabc"), 3)
        self.assertEqual(smallest_period(b"abcd"), 4)


if __name__ == "__main__":
    print(demo(), end="")
