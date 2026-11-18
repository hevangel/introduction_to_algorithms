"""String Matching with Finite Automata — CLRS 32.3 (1e–4e) (1, 2, 3, 4).

Build the string-matching automaton δ(q, a) in O(m|Σ|) and scan the text in Θ(n).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def compute_transition_function(p: bytes) -> list[list[int]]:
    """COMPUTE-TRANSITION-FUNCTION(P, Sigma): delta[q][a] for all 256 byte values."""
    m = len(p)
    delta = [[0] * 256 for _ in range(m + 1)]
    if m == 0:
        return delta
    delta[0][p[0]] = 1
    x = 0
    for q in range(1, m + 1):
        delta[q] = list(delta[x])
        if q < m:
            delta[q][p[q]] = q + 1
            x = delta[x][p[q]]
    return delta


def finite_automaton_matcher(t: bytes, p: bytes) -> list[int]:
    """FINITE-AUTOMATON-MATCHER(T, delta, m): scan T once; arrival at state m reports a match."""
    m = len(p)
    delta = compute_transition_function(p)
    if m == 0:
        return list(range(len(t) + 1))
    q = 0
    shifts: list[int] = []
    for i, c in enumerate(t):
        q = delta[q][c]
        if q == m:
            shifts.append(i + 1 - m)
    return shifts


def demo() -> str:
    lines: list[str] = []
    p = b"ababaca"
    delta = compute_transition_function(p)
    lines.append("automaton for P = ababaca (states 0..7); transitions on a, b, c:")
    for q in range(len(p) + 1):
        lines.append(
            f"  state {q}: a→{delta[q][ord('a')]} b→{delta[q][ord('b')]} c→{delta[q][ord('c')]}"
        )
    lines.append(f"T = abababacaba: matches at shifts {finite_automaton_matcher(b'abababacaba', p)}")
    return "\n".join(lines) + "\n"


def _naive(t: bytes, p: bytes) -> list[int]:
    n, m = len(t), len(p)
    if m > n:
        return []
    return [s for s in range(n - m + 1) if t[s : s + m] == p]


class Tests(unittest.TestCase):
    def test_figure_32_7_automaton(self) -> None:
        d = compute_transition_function(b"ababaca")
        want = [(1, 0, 0), (1, 2, 0), (3, 0, 0), (1, 4, 0), (5, 0, 0), (1, 4, 6), (7, 0, 0), (1, 2, 0)]
        for q, w in enumerate(want):
            row = (d[q][ord("a")], d[q][ord("b")], d[q][ord("c")])
            self.assertEqual(row, w, f"state {q}")

    def test_agrees_with_naive(self) -> None:
        r = Rng(221)
        for _ in range(200):
            t = bytes(ord("a") + r.range(0, 2) for _ in range(r.range(0, 50)))
            p = bytes(ord("a") + r.range(0, 2) for _ in range(r.range(1, 6)))
            self.assertEqual(finite_automaton_matcher(t, p), _naive(t, p))


if __name__ == "__main__":
    print(demo(), end="")
