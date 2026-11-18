"""Repetition Factor and the Repetition Matcher — CLRS Problem 32-1 (1e–4e) (1,2,3,4).

ρ(y) is the largest r with y = xʳ; computed from the prefix function. The repetition matcher shifts by ⌈q/(ρ*+1)⌉ after a mismatch at q.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _prefix_function(p: bytes) -> list[int]:
    m = len(p)
    pi = [0] * m
    k = 0
    for q in range(1, m):
        while k > 0 and p[k] != p[q]:
            k = pi[k - 1]
        if p[k] == p[q]:
            k += 1
        pi[q] = k
    return pi


def repetition_factor(y: bytes) -> int:
    """ρ(y): the largest r such that y = x^r for some x."""
    if not y:
        return 0
    n = len(y)
    p = n - _prefix_function(y)[n - 1]
    return n // p if n % p == 0 else 1


def max_prefix_repetition(p: bytes) -> int:
    """ρ*(P) = max over prefixes P_k of ρ(P_k) in one O(m) pass."""
    pi = _prefix_function(p)
    best = 0
    for k in range(1, len(p) + 1):
        per = k - pi[k - 1]
        r = k // per if k % per == 0 else 1
        if r > best:
            best = r
    return best


def repetition_matcher(p: bytes, t: bytes) -> list[int]:
    """REPETITION-MATCHER(P, T): report all shifts s with T[s..s+m] = P."""
    m, n = len(p), len(t)
    out: list[int] = []
    if m == 0 or m > n:
        return out
    k = 1 + max_prefix_repetition(p)
    q = 0
    s = 0
    while s <= n - m:
        if t[s + q] == p[q]:
            q += 1
            if q == m:
                out.append(s)
        if q == m or t[s + q] != p[q]:
            s += max(1, math.ceil(q / k))
            q = 0
    return out


def demo() -> str:
    lines: list[str] = []
    for y in ["abababab", "abcabcabc", "aaaa", "abaab", "a"]:
        lines.append(f"ρ({y}) = {repetition_factor(y.encode())}")
    p = b"abab"
    lines.append(f"ρ*(abab) = {max_prefix_repetition(p)}")
    t = b"abababababcabab"
    lines.append(f"matches of abab in {t.decode()}: {repetition_matcher(p, t)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_repetition_factor_matches_definition(self) -> None:
        r = Rng(400)
        for _ in range(300):
            y = bytes(ord("a") + r.range(0, 1) for _ in range(r.range(1, 12)))
            brute = max(len(y) // d for d in range(1, len(y) + 1) if len(y) % d == 0 and all(y[i] == y[i % d] for i in range(len(y))))
            self.assertEqual(repetition_factor(y), brute)
        self.assertEqual(repetition_factor(b""), 0)

    def test_matcher_agrees_with_naive_search(self) -> None:
        r = Rng(401)
        for _ in range(500):
            alpha = r.range(1, 3)
            p = bytes(ord("a") + r.range(0, alpha) for _ in range(r.range(1, 8)))
            t = bytes(ord("a") + r.range(0, alpha) for _ in range(r.range(0, 40)))
            want = [s for s in range(len(t) - len(p) + 1) if t[s : s + len(p)] == p]
            self.assertEqual(repetition_matcher(p, t), want)


if __name__ == "__main__":
    print(demo(), end="")
