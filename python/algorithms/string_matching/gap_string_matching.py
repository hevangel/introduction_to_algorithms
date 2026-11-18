"""String Matching with Gap Characters — CLRS Exercise 32.1-4 (1e–4e) (1, 2, 3, 4).

A pattern like ab*ba*c where * matches any string (including the empty one): greedily match the gap-free pieces left to right in O(n + m) with KMP.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _kmp_first(t: bytes, start: int, p: bytes) -> int | None:
    m = len(p)
    if m == 0:
        return start
    pi = [0] * m
    k = 0
    for q in range(1, m):
        while k > 0 and p[k] != p[q]:
            k = pi[k - 1]
        if p[k] == p[q]:
            k += 1
        pi[q] = k
    q = 0
    for i in range(start, len(t)):
        c = t[i]
        while q > 0 and p[q] != c:
            q = pi[q - 1]
        if p[q] == c:
            q += 1
        if q == m:
            return i + 1 - m
    return None


def gap_match(text: bytes, pattern: bytes, gap: int) -> bool:
    """True if text contains a substring matching pattern where byte `gap` matches any string."""
    pieces = [s for s in pattern.split(bytes([gap])) if s]
    pos = 0
    for piece in pieces:
        idx = _kmp_first(text, pos, piece)
        if idx is None:
            return False
        pos = idx + len(piece)
    return True


def demo() -> str:
    lines: list[str] = []
    t = b"cabccbacbacab"
    for p in ("ab*ba*c", "ba*ab", "c*c*c*c", "a*b*c*d", "cab", "*"):
        lines.append(f"text {t.decode()} contains {p:<8}: {gap_match(t, p.encode(), ord('*'))}")
    return "\n".join(lines) + "\n"


def _glob(p: bytes, s: bytes) -> bool:
    if not p:
        return len(s) == 0
    if p[0] == ord("*"):
        return any(_glob(p[1:], s[i:]) for i in range(len(s) + 1))
    return len(s) > 0 and s[0] == p[0] and _glob(p[1:], s[1:])


def _brute(t: bytes, p: bytes) -> bool:
    return any(_glob(p, t[i:j]) for i in range(len(t) + 1) for j in range(i, len(t) + 1))


class Tests(unittest.TestCase):
    def test_agrees_with_exhaustive_glob_matching(self) -> None:
        r = Rng(402)
        for _ in range(500):
            t = bytes(ord("a") + r.range(0, 2) for _ in range(r.range(0, 14)))
            p = bytes(
                ord("*") if r.range(0, 3) == 0 else ord("a") + r.range(0, 2)
                for _ in range(r.range(1, 7))
            )
            self.assertEqual(gap_match(t, p, ord("*")), _brute(t, p), f"t={t.decode()} p={p.decode()}")

    def test_edge_cases(self) -> None:
        self.assertTrue(gap_match(b"", b"***", ord("*")))
        self.assertFalse(gap_match(b"", b"a", ord("*")))
        self.assertTrue(gap_match(b"xaybz", b"a*b", ord("*")))
        self.assertFalse(gap_match(b"bxa", b"a*b", ord("*")))


if __name__ == "__main__":
    print(demo(), end="")
