"""Nesting Boxes — CLRS Problem 24-2 (2e–3e) / 22-2 (4e).

d-dimensional boxes nest if some rotation fits strictly inside; sort each box's dimensions and find the longest chain in the nesting DAG.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def nests(a: list[int], b: list[int]) -> bool:
    """Does box a nest inside box b (after some permutation of a's dimensions)?"""
    return all(p < q for p, q in zip(sorted(a), sorted(b)))


def longest_nesting_chain(boxes: list[list[int]]) -> list[int]:
    """Longest sequence B₁ ⊂ B₂ ⊂ … Returns indices of one longest chain, innermost first."""
    n = len(boxes)
    if n == 0:
        return []
    sorted_boxes = [sorted(b) for b in boxes]
    order = sorted(range(n), key=lambda i: sum(sorted_boxes[i]))
    best = [1] * n
    pred = [-1] * n
    for pos, j in enumerate(order):
        sj = sorted_boxes[j]
        for i in order[:pos]:
            if all(p < q for p, q in zip(sorted_boxes[i], sj)) and best[i] + 1 > best[j]:
                best[j] = best[i] + 1
                pred[j] = i
    end = max(range(n), key=lambda i: best[i])
    chain: list[int] = []
    x = end
    while x != -1:
        chain.append(x)
        x = pred[x]
    chain.reverse()
    return chain


def demo() -> str:
    lines: list[str] = []
    boxes = [[1, 5, 3], [4, 2, 6], [7, 8, 9], [2, 6, 4], [3, 7, 5], [10, 2, 2]]
    lines.append(f"boxes {boxes}")
    lines.append(
        f"box 0 nests in box 1? {nests(boxes[0], boxes[1])}   "
        f"box 1 nests in 4? {nests(boxes[1], boxes[4])}   "
        f"box 5 nests in 2? {nests(boxes[5], boxes[2])}"
    )
    chain = longest_nesting_chain(boxes)
    lines.append(f"longest nesting chain (innermost first): {chain} = {[boxes[i] for i in chain]}")
    return "\n".join(lines) + "\n"


def _permutations(d: int) -> list[list[int]]:
    if d == 0:
        return [[]]
    out: list[list[int]] = []
    for p in _permutations(d - 1):
        for pos in range(len(p) + 1):
            q = p[:]
            q.insert(pos, d - 1)
            out.append(q)
    return out


class Tests(unittest.TestCase):
    def test_sorted_comparison_equals_trying_all_rotations(self) -> None:
        r = Rng(360)
        for _ in range(500):
            d = r.range(1, 4)
            a, b = r.vec(d, 1, 8), r.vec(d, 1, 8)
            brute = any(all(a[p[i]] < b[i] for i in range(d)) for p in _permutations(d))
            self.assertEqual(nests(a, b), brute, f"{a} {b}")

    def test_chain_matches_exhaustive_subset_search(self) -> None:
        r = Rng(361)
        for _ in range(80):
            n, d = r.range(1, 8), r.range(1, 3)
            boxes = [r.vec(d, 1, 8) for _ in range(n)]
            chain = longest_nesting_chain(boxes)
            self.assertTrue(all(nests(boxes[chain[i]], boxes[chain[i + 1]]) for i in range(len(chain) - 1)))
            brute = 0
            for m in range(1 << n):
                ids = sorted((i for i in range(n) if (m >> i) & 1), key=lambda i: sum(boxes[i]))
                if all(nests(boxes[ids[k]], boxes[ids[k + 1]]) for k in range(len(ids) - 1)):
                    if m.bit_count() > brute:
                        brute = m.bit_count()
            self.assertEqual(len(chain), brute, f"{boxes}")


if __name__ == "__main__":
    print(demo(), end="")
