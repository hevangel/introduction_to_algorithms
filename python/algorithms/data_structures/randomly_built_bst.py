"""Randomly Built Binary Search Trees — CLRS 12.4 (1e–3e) (1,2,3).

Inserting keys in random order gives expected height O(lg n); empirical check against the closed forms.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Bst:
    """Minimal iterative unbalanced BST for random-tree depth/height experiments."""

    def __init__(self) -> None:
        self.keys: list[int] = []
        self.left: list[int] = []
        self.right: list[int] = []
        self.root = -1

    def __len__(self) -> int:
        return len(self.keys)

    def insert(self, k: int) -> None:
        z = len(self.keys)
        self.keys.append(k)
        self.left.append(-1)
        self.right.append(-1)
        if self.root == -1:
            self.root = z
            return
        x = self.root
        while True:
            if k < self.keys[x]:
                if self.left[x] == -1:
                    self.left[x] = z
                    return
                x = self.left[x]
            else:
                if self.right[x] == -1:
                    self.right[x] = z
                    return
                x = self.right[x]

    def height(self) -> int:
        if self.root == -1:
            return 0
        best = 0
        level = [self.root]
        while level:
            best += 1
            nxt: list[int] = []
            for x in level:
                if self.left[x] != -1:
                    nxt.append(self.left[x])
                if self.right[x] != -1:
                    nxt.append(self.right[x])
            level = nxt
        return best


def build_random(n: int, rng: Rng) -> Bst:
    """Build a BST by inserting a uniformly random permutation of 1..=n."""
    keys = list(range(1, n + 1))
    rng.shuffle(keys)
    t = Bst()
    for k in keys:
        t.insert(k)
    return t


def average_depth(t: Bst) -> float:
    """Average node depth (root = depth 0)."""
    total = 0
    level = [t.root] if t.root != -1 else []
    d = 0
    while level:
        total += d * len(level)
        nxt: list[int] = []
        for x in level:
            if t.left[x] != -1:
                nxt.append(t.left[x])
            if t.right[x] != -1:
                nxt.append(t.right[x])
        level = nxt
        d += 1
    return total / len(t)


def expected_average_depth(n: int) -> float:
    """Expected average depth of a random BST: 2(1 + 1/n)·H_n - 4."""
    h = sum(1.0 / i for i in range(1, n + 1))
    return 2.0 * (1.0 + 1.0 / n) * h - 4.0


def expected_height_asymptotic(n: int) -> float:
    """Reed's asymptotic expected height: 4.311·ln n - 1.953·ln ln n."""
    ln = math.log(n)
    return 4.311 * ln - 1.953 * math.log(ln)


def simulate(n: int, trials: int, rng: Rng) -> tuple[float, float]:
    h = 0.0
    d = 0.0
    for _ in range(trials):
        t = build_random(n, rng)
        h += t.height()
        d += average_depth(t)
    return h / trials, d / trials


def demo() -> str:
    lines: list[str] = []
    r = Rng(12)
    lines.append("      n | mean height | Reed 4.311 ln n − 1.953 ln ln n | mean avg depth | 2(1+1/n)H_n − 4 | lg n")
    for n in [100, 1000, 10000]:
        h, d = simulate(n, 20, r)
        lines.append(
            f"{n:>7} | {h:>11.2f} | {expected_height_asymptotic(n):>31.2f} | "
            f"{d:>14.3f} | {expected_average_depth(n):>15.3f} | {math.log2(n):.1f}"
        )
    s_tree = Bst()
    for k in range(200):
        s_tree.insert(k)
    lines.append(f"contrast: inserting 0..200 in sorted order gives height {s_tree.height()}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_average_depth_matches_formula(self) -> None:
        r = Rng(61)
        _, d = simulate(500, 40, r)
        self.assertLess(abs(d - expected_average_depth(500)), 0.5)

    def test_height_is_logarithmic(self) -> None:
        r = Rng(62)
        h, _ = simulate(1000, 20, r)
        self.assertGreater(h, math.log2(1000))
        self.assertLess(h, 4.311 * math.log(1000))

    def test_small_case_exact(self) -> None:
        self.assertAlmostEqual(expected_average_depth(3), 8.0 / 9.0, places=9)


if __name__ == "__main__":
    print(demo(), end="")
