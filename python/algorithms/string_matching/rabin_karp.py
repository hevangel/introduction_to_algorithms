"""Rabin-Karp Algorithm — CLRS 32.2 (1e–4e) (1,2,3,4).

Compare rolling hashes of the pattern and each text window; verify hash hits. Expected Θ(n + m).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def rabin_karp(t: bytes, p: bytes, d: int, q: int) -> tuple[list[int], int]:
    """RABIN-KARP-MATCHER(T, P, d, q): returns (valid shifts, spurious hits)."""
    n, m = len(t), len(p)
    if m > n:
        return ([], 0)
    h = pow(d, max(0, m - 1), q) if m > 0 else 1
    p0 = 0
    t0 = 0
    for i in range(m):
        p0 = (d * p0 + p[i]) % q
        t0 = (d * t0 + t[i]) % q
    shifts: list[int] = []
    spurious = 0
    for s in range(n - m + 1):
        if p0 == t0:
            if t[s : s + m] == p:
                shifts.append(s)
            else:
                spurious += 1
        if s < n - m:
            t0 = (d * (t0 - t[s] * h) + t[s + m]) % q
    return shifts, spurious


def demo() -> str:
    lines: list[str] = []
    t, p = "2359023141526739921", "31415"
    shifts, spurious = rabin_karp(t.encode(), p.encode(), 10, 13)
    lines.append(f"T = {t}, P = {p}, q = 13: valid shifts {shifts}, spurious hits {spurious}")
    text = b"abracadabra" * 2000
    for q in [101, 1_000_000_007]:
        s, sp = rabin_karp(text, b"cadabra", 256, q)
        lines.append(f"q = {q:>10}: {len(s)} matches in {len(text)} chars, {sp} spurious hits")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_figure(self) -> None:
        s, sp = rabin_karp(b"2359023141526739921", b"31415", 10, 13)
        self.assertEqual(s, [6])
        self.assertEqual(sp, 1)

    def test_agrees_with_naive_on_random_binary_strings(self) -> None:
        r = Rng(220)
        for _ in range(200):
            t = bytes(ord("a") + r.range(0, 1) for _ in range(r.range(0, 60)))
            p = bytes(ord("a") + r.range(0, 1) for _ in range(r.range(1, 5)))
            want = [s for s in range(len(t) - len(p) + 1) if t[s : s + len(p)] == p]
            for q in [7, 101, 1_000_003]:
                self.assertEqual(rabin_karp(t, p, 256, q)[0], want)


if __name__ == "__main__":
    print(demo(), end="")
