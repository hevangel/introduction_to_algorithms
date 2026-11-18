"""Breaking a String — CLRS Problem 15-9 (3e) / 14-9 (4e) (3e,4e).

Cut a string of length n at m given positions; each cut costs the length of the piece being cut. Interval DP in O(m^3).
"""

from __future__ import annotations

from itertools import permutations
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def break_string(n: int, cuts: list[int]) -> tuple[int, list[int]]:
    """Minimum total cost and an order of cuts. cuts are positions in 1..n."""
    b = [0] + sorted(cuts) + [n]
    m = len(b)
    cost = [[0] * m for _ in range(m)]
    split = [[0] * m for _ in range(m)]
    for length in range(2, m):
        for i in range(m - length):
            j = i + length
            cost[i][j] = sys.maxsize
            for k in range(i + 1, j):
                q = cost[i][k] + cost[k][j] + (b[j] - b[i])
                if q < cost[i][j]:
                    cost[i][j] = q
                    split[i][j] = k
    order: list[int] = []

    def walk(i: int, j: int) -> None:
        if j - i < 2:
            return
        k = split[i][j]
        order.append(b[k])
        walk(i, k)
        walk(k, j)

    walk(0, m - 1)
    return cost[0][m - 1], order


def cost_of_order(n: int, order: list[int]) -> int:
    """Cost of performing the cuts in the given order on a string of length n."""
    pieces = [(0, n)]
    total = 0
    for c in order:
        idx = next((i for i, (l, r) in enumerate(pieces) if l < c < r), None)
        if idx is None:
            raise ValueError("cut position invalid")
        l, r = pieces.pop(idx)
        total += r - l
        pieces.append((l, c))
        pieces.append((c, r))
    return total


def demo() -> str:
    lines: list[str] = []
    cost, order = break_string(20, [2, 8, 10])
    lines.append(f"length-20 string, cuts at 2, 8, 10: optimal cost {cost} with order {order}")
    lines.append(f"cutting left to right instead costs {cost_of_order(20, [2, 8, 10])}")
    lines.append(f"cutting right to left costs {cost_of_order(20, [10, 8, 2])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        self.assertEqual(break_string(20, [2, 8, 10])[0], 38)

    def test_matches_all_cut_orders(self) -> None:
        r = Rng(341)
        for _ in range(60):
            n = r.range(2, 30)
            cuts = list(range(1, n))
            r.shuffle(cuts)
            cuts = cuts[: r.range(0, min(6, n - 1))]
            c, order = break_string(n, cuts)
            self.assertEqual(cost_of_order(n, order), c)
            brute = min(cost_of_order(n, list(p)) for p in permutations(cuts))
            self.assertEqual(c, brute, f"n={n} cuts={cuts}")


if __name__ == "__main__":
    print(demo(), end="")
