"""Binary Search Trees with Equal Keys — CLRS Problem 12-1 (1e-4e) (Editions: 1,2,3,4).

Four ways to insert duplicates into a BST — always right, alternate sides, keep a
list at the node, choose randomly — and what each costs when n identical keys arrive.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class Strategy(Enum):
    ALWAYS_RIGHT = "always_right"
    ALTERNATE = "alternate"
    COUNT_AT_NODE = "count_at_node"
    RANDOM = "random"


@dataclass
class _Node:
    key: int
    left: int | None = None
    right: int | None = None
    flag: bool = False
    copies: int = 1


class Bst:
    def __init__(self, strategy: Strategy) -> None:
        self._nodes: list[_Node] = []
        self._root: int | None = None
        self._strategy = strategy
        self._rng = Rng(12)

    def insert(self, key: int) -> int:
        """Insert and return the number of nodes visited (depth of new element)."""
        new = len(self._nodes)
        self._nodes.append(_Node(key=key))
        if self._root is None:
            self._root = new
            return 1
        cur = self._root
        depth = 1
        while True:
            node = self._nodes[cur]
            if key < node.key:
                go_left = True
            elif key > node.key:
                go_left = False
            else:
                if self._strategy == Strategy.ALWAYS_RIGHT:
                    go_left = False
                elif self._strategy == Strategy.ALTERNATE:
                    f = node.flag
                    node.flag = not f
                    go_left = f
                elif self._strategy == Strategy.COUNT_AT_NODE:
                    node.copies += 1
                    self._nodes.pop()
                    return depth
                else:
                    go_left = self._rng.range(0, 1) == 0
            nxt = node.left if go_left else node.right
            if nxt is not None:
                cur = nxt
                depth += 1
            else:
                if go_left:
                    node.left = new
                else:
                    node.right = new
                return depth + 1

    def in_order(self) -> list[int]:
        """In-order list of keys (equal keys repeated according to `copies`)."""
        out: list[int] = []
        # Iterative inorder walk to avoid recursion limit on degenerate trees of size 5000
        stack: list[tuple[int, bool]] = []
        if self._root is not None:
            stack.append((self._root, False))
        while stack:
            i, visited = stack.pop()
            nd = self._nodes[i]
            if visited:
                out.extend([nd.key] * nd.copies)
            else:
                if nd.right is not None:
                    stack.append((nd.right, False))
                stack.append((i, True))
                if nd.left is not None:
                    stack.append((nd.left, False))
        return out


def cost_of_identical_keys(strategy: Strategy, n: int) -> int:
    """Total number of nodes visited while inserting n identical keys."""
    t = Bst(strategy)
    return sum(t.insert(7) for _ in range(n))


def demo() -> str:
    lines: list[str] = []
    lines.append("total nodes visited to insert n identical keys:")
    lines.append(
        f"{'n':>6} | {'always right':>12} | {'alternate':>10} | {'count':>10} | {'random':>8}"
    )
    for n in [100, 1000, 5000]:
        lines.append(
            f"{n:>6} | {cost_of_identical_keys(Strategy.ALWAYS_RIGHT, n):>12} | "
            f"{cost_of_identical_keys(Strategy.ALTERNATE, n):>10} | "
            f"{cost_of_identical_keys(Strategy.COUNT_AT_NODE, n):>10} | "
            f"{cost_of_identical_keys(Strategy.RANDOM, n):>8}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_identical_key_costs_match_the_analysis(self) -> None:
        n = 512
        self.assertEqual(cost_of_identical_keys(Strategy.ALWAYS_RIGHT, n), n * (n + 1) // 2)
        self.assertEqual(cost_of_identical_keys(Strategy.COUNT_AT_NODE, n), n)
        lg = math.log2(n)
        alt = float(cost_of_identical_keys(Strategy.ALTERNATE, n))
        self.assertLessEqual(alt, n * (lg + 1.0))
        rnd = float(cost_of_identical_keys(Strategy.RANDOM, n))
        self.assertTrue(n < rnd < n * 4.0 * lg)

    def test_all_strategies_keep_the_inorder_sorted_and_complete(self) -> None:
        r = Rng(445)
        for strategy in Strategy:
            t = Bst(strategy)
            keys = [r.range(0, 20) for _ in range(300)]
            for k in keys:
                t.insert(k)
            self.assertEqual(t.in_order(), sorted(keys), f"{strategy}")


if __name__ == "__main__":
    print(demo(), end="")
