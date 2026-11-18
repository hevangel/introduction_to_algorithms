"""Counting Binary Trees & BST Shapes — CLRS Problem 12-4 (1e-4e) (Editions: 1,2,3,4).

The number of binary trees with n nodes is the Catalan number C(2n, n)/(n+1);
enumerate them and map permutations to their BSTs.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def count_by_recurrence(n: int) -> int:
    """b_n = sum_{k} b_k * b_{n-1-k} with b_0 = 1 (Problem 12-4)."""
    b = [1] * (n + 1)
    for m in range(1, n + 1):
        b[m] = sum(b[k] * b[m - 1 - k] for k in range(m))
    return b[n]


def catalan(n: int) -> int:
    """Closed form C(2n, n)/(n + 1)."""
    c = 1
    for i in range(n):
        c = c * 2 * (2 * i + 1) // (i + 2)
    return c


def all_shapes(n: int) -> list[str]:
    """All binary tree shapes with n nodes as parenthesised strings."""
    if n == 0:
        return [""]
    out: list[str] = []
    for k in range(n):
        for l in all_shapes(k):
            for r in all_shapes(n - 1 - k):
                out.append(f"({l} {r})")
    return out


def bst_shape(keys: list[int]) -> str:
    """The shape of the binary search tree obtained by inserting the keys in this order."""
    if not keys:
        return ""
    root = keys[0]
    rest = keys[1:]
    l = [k for k in rest if k < root]
    r = [k for k in rest if k > root]
    return f"({bst_shape(l)} {bst_shape(r)})"


def permutations(n: int) -> list[list[int]]:
    if n == 0:
        return [[]]
    out: list[list[int]] = []
    for p in permutations(n - 1):
        for pos in range(len(p) + 1):
            q = list(p)
            q.insert(pos, n)
            out.append(q)
    return out


def demo() -> str:
    lines: list[str] = []
    lines.append("n | recurrence | C(2n,n)/(n+1) | distinct BST shapes from the n! permutations")
    for n in range(8):
        perms = permutations(n)
        shapes = {bst_shape(p) for p in perms}
        lines.append(
            f"{n} | {count_by_recurrence(n):>10} | {catalan(n):>13} | {len(shapes):>6} (of {len(perms)} permutations)"
        )
    lines.append(f"3-node shapes: {all_shapes(3)}")
    lines.append(f"asymptotically b_n = 4^n / (sqrt(pi) n^(3/2)) (1 + O(1/n)); b_30 = {catalan(30)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_recurrence_equals_closed_form_and_known_values(self) -> None:
        self.assertEqual(
            [count_by_recurrence(n) for n in range(10)],
            [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862],
        )
        for n in range(30):
            self.assertEqual(count_by_recurrence(n), catalan(n))

    def test_enumeration_and_permutation_mapping(self) -> None:
        for n in range(7):
            shapes = all_shapes(n)
            self.assertEqual(len(shapes), catalan(n))
            self.assertEqual(len(set(shapes)), len(shapes))
            from_perms = {bst_shape(p) for p in permutations(n)}
            self.assertEqual(from_perms, set(shapes))


if __name__ == "__main__":
    print(demo(), end="")
