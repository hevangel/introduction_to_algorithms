"""Optimal Binary Search Trees — CLRS 15.5 (2e–3e), 14.5 (4e) (2e,3e,4e).

Minimise expected search cost given key and miss probabilities: O(n^3) DP, O(n^2) with Knuth's monotonicity.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def optimal_bst(p: list[float], q: list[float]) -> tuple[list[list[float]], list[list[int]]]:
    """OPTIMAL-BST(p, q, n): p[1..=n] key probabilities (p[0] unused), q[0..=n] dummy-key probabilities."""
    n = len(q) - 1
    e = [[0.0] * (n + 1) for _ in range(n + 2)]
    w = [[0.0] * (n + 1) for _ in range(n + 2)]
    root = [[0] * (n + 1) for _ in range(n + 2)]
    for i in range(1, n + 2):
        e[i][i - 1] = q[i - 1]
        w[i][i - 1] = q[i - 1]
    for l in range(1, n + 1):
        for i in range(1, n - l + 2):
            j = i + l - 1
            e[i][j] = math.inf
            w[i][j] = w[i][j - 1] + p[j] + q[j]
            for r in range(i, j + 1):
                t = e[i][r - 1] + e[r + 1][j] + w[i][j]
                if t < e[i][j]:
                    e[i][j] = t
                    root[i][j] = r
    return e, root


def optimal_bst_knuth(p: list[float], q: list[float]) -> tuple[list[list[float]], list[list[int]]]:
    """Knuth's O(n^2) refinement: root[i][j-1] <= root[i][j] <= root[i+1][j] (Exercise 15.5-4)."""
    n = len(q) - 1
    e = [[0.0] * (n + 1) for _ in range(n + 2)]
    w = [[0.0] * (n + 1) for _ in range(n + 2)]
    root = [[0] * (n + 1) for _ in range(n + 2)]
    for i in range(1, n + 2):
        e[i][i - 1] = q[i - 1]
        w[i][i - 1] = q[i - 1]
    for l in range(1, n + 1):
        for i in range(1, n - l + 2):
            j = i + l - 1
            e[i][j] = math.inf
            w[i][j] = w[i][j - 1] + p[j] + q[j]
            lo, hi = (i, j) if l == 1 else (root[i][j - 1], root[i + 1][j])
            for r in range(lo, hi + 1):
                t = e[i][r - 1] + e[r + 1][j] + w[i][j]
                if t < e[i][j]:
                    e[i][j] = t
                    root[i][j] = r
    return e, root


def describe(root: list[list[int]], i: int, j: int) -> str:
    """Parenthesised description of the optimal tree, e.g. 'k2(k1(d0,d1),k5(...))'."""
    if j + 1 == i:
        return f"d{j}"
    r = root[i][j]
    return f"k{r}({describe(root, i, r - 1)},{describe(root, r + 1, j)})"


def demo() -> str:
    lines: list[str] = []
    p = [0.0, 0.15, 0.10, 0.05, 0.10, 0.20]
    q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10]
    e, root = optimal_bst(p, q)
    lines.append(f"p = {p[1:]}")
    lines.append(f"q = {q}")
    lines.append(f"expected search cost e[1,5] = {e[1][5]:.2f}")
    lines.append(f"optimal tree: {describe(root, 1, 5)}")
    e2, _ = optimal_bst_knuth(p, q)
    lines.append(f"Knuth's O(n²) variant gives {e2[1][5]:.2f}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        p = [0.0, 0.15, 0.10, 0.05, 0.10, 0.20]
        q = [0.05, 0.10, 0.05, 0.05, 0.05, 0.10]
        e, root = optimal_bst(p, q)
        self.assertLess(abs(e[1][5] - 2.75), 1e-9)
        self.assertEqual(root[1][5], 2)
        self.assertEqual(describe(root, 1, 5), "k2(k1(d0,d1),k5(k4(k3(d2,d3),d4),d5))")

    def test_knuth_matches_cubic(self) -> None:
        r = Rng(103)
        for n in range(1, 13):
            raw = [r.f64() + 0.01 for _ in range(2 * n + 1)]
            s = sum(raw)
            p = [0.0] + [x / s for x in raw[:n]]
            q = [x / s for x in raw[n:]]
            a, _ = optimal_bst(p, q)
            b, _ = optimal_bst_knuth(p, q)
            self.assertLess(abs(a[1][n] - b[1][n]), 1e-9)

    def test_single_key(self) -> None:
        e, root = optimal_bst([0.0, 0.5], [0.25, 0.25])
        self.assertLess(abs(e[1][1] - 1.5), 1e-12)
        self.assertEqual(root[1][1], 1)


if __name__ == "__main__":
    print(demo(), end="")
