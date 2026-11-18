"""Pointer Jumping (Finding Roots in a Forest) — CLRS 30.1 (1e), Exercises (Editions: 1).

Every node repeatedly replaces its parent by its grandparent: all nodes find their
tree's root in ceil(lg h) synchronous rounds with O(n) processors.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def find_roots(parent: list[int]) -> tuple[list[int], int]:
    """FIND-ROOTS: `parent[v] == v` marks a root.

    Returns (root of each node, number of rounds that changed something).
    """
    p = list(parent)
    rounds = 0
    while True:
        nxt = [p[p[v]] for v in range(len(p))]
        if nxt == p:
            return p, rounds
        p = nxt
        rounds += 1


def demo() -> str:
    lines: list[str] = []
    chain = [max(v - 1, 0) for v in range(16)]
    roots, rounds = find_roots(chain)
    lines.append(f"chain of 16 nodes: roots {roots} after {rounds} rounds (ceil(lg 15) = 4)")
    forest = [0, 0, 1, 2, 4, 4, 5, 6, 8]
    lines.append(f"forest {forest} -> {find_roots(forest)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_finds_every_root_in_logarithmic_rounds(self) -> None:
        r = Rng(434)
        for _ in range(300):
            n = r.range(1, 60)
            parent = [v if (v == 0 or r.f64() < 0.1) else r.range(0, v - 1) for v in range(n)]

            def walk(v: int) -> tuple[int, int]:
                depth = 0
                while parent[v] != v:
                    v = parent[v]
                    depth += 1
                return v, depth

            height = max(walk(v)[1] for v in range(n))
            roots, rounds = find_roots(parent)
            for v in range(n):
                self.assertEqual(roots[v], walk(v)[0])
            self.assertLessEqual(rounds, math.ceil(math.log2(max(height, 1))) + 1)

    def test_chain_needs_exactly_lg_rounds(self) -> None:
        chain = [max(v - 1, 0) for v in range(65)]
        self.assertEqual(find_roots(chain)[1], 6)


if __name__ == "__main__":
    print(demo(), end="")
