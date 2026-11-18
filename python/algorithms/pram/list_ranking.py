"""List Ranking (Pointer Jumping) — CLRS 30.1 (1e) (Editions: 1).

Compute every node's distance to the end of a linked list in O(lg n) PRAM steps by
repeated pointer doubling.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def list_rank(next_ptr: list[int | None]) -> tuple[list[int], int]:
    """PRAM-style synchronous list ranking. `next_ptr[i]` = successor of i (None at tail).

    Returns (distance of each node to the tail, number of parallel rounds used).
    """
    n = len(next_ptr)
    d = [1 if x is not None else 0 for x in next_ptr]
    nxt = list(next_ptr)
    rounds = 0
    while any(x is not None for x in nxt):
        d_old, n_old = list(d), list(nxt)
        for i in range(n):
            j = n_old[i]
            if j is not None:
                d[i] = d_old[i] + d_old[j]
                nxt[i] = n_old[j]
        rounds += 1
    return d, rounds


def list_from_order(order: list[int]) -> list[int | None]:
    """Build the `next` array of a list that visits `order[0], order[1], ...`."""
    nxt: list[int | None] = [None] * len(order)
    for i in range(len(order) - 1):
        nxt[order[i]] = order[i + 1]
    return nxt


def demo() -> str:
    lines: list[str] = []
    order = [3, 6, 0, 4, 1, 5, 2]
    d, rounds = list_rank(list_from_order(order))
    lines.append(f"list order: {order}")
    lines.append(f"distance to tail per node: {d}")
    lines.append(f"synchronous rounds: {rounds} (ceil(lg 7) = 3)")
    for n in [16, 1024, 65536]:
        ord_list = list(range(n))
        r = list_rank(list_from_order(ord_list))[1]
        lines.append(f"n = {n:>5}: {r} rounds (lg n = {n.bit_length() - 1})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_ranks_match_sequential_walk_and_rounds_are_logarithmic(self) -> None:
        r = Rng(160)
        for n in [1, 2, 3, 8, 50, 257]:
            order = list(range(n))
            r.shuffle(order)
            d, rounds = list_rank(list_from_order(order))
            for pos, node in enumerate(order):
                self.assertEqual(d[node], n - 1 - pos)
            self.assertLessEqual(rounds, math.ceil(math.log2(n)), f"n={n} rounds={rounds}")


if __name__ == "__main__":
    print(demo(), end="")
