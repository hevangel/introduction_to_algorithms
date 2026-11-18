"""Matrix-Chain Multiplication — CLRS 15.2 (1e–3e: 16.1/15.2), 14.2 (4e) (1e,2e,3e,4e).

Choose the parenthesization minimising scalar multiplications: Theta(n^3) DP.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def matrix_chain_order(p: list[int]) -> tuple[list[list[int]], list[list[int]]]:
    """MATRIX-CHAIN-ORDER: matrix A_i is p[i-1] x p[i]. Returns (m, s) with 1-based indices."""
    n = len(p) - 1
    m = [[0] * (n + 1) for _ in range(n + 1)]
    s = [[0] * (n + 1) for _ in range(n + 1)]
    for l in range(2, n + 1):
        for i in range(1, n - l + 2):
            j = i + l - 1
            m[i][j] = sys.maxsize
            for k in range(i, j):
                q = m[i][k] + m[k + 1][j] + p[i - 1] * p[k] * p[j]
                if q < m[i][j]:
                    m[i][j] = q
                    s[i][j] = k
    return m, s


def optimal_parens(s: list[list[int]], i: int, j: int) -> str:
    """PRINT-OPTIMAL-PARENS."""
    if i == j:
        return f"A{i}"
    return f"({optimal_parens(s, i, s[i][j])}{optimal_parens(s, s[i][j] + 1, j)})"


def recursive_matrix_chain(p: list[int], i: int, j: int) -> int:
    """RECURSIVE-MATRIX-CHAIN: Theta(2^n) — included to show the blow-up."""
    if i == j:
        return 0
    return min(
        recursive_matrix_chain(p, i, k) + recursive_matrix_chain(p, k + 1, j) + p[i - 1] * p[k] * p[j]
        for k in range(i, j)
    )


def memoized_matrix_chain(p: list[int]) -> int:
    """MEMOIZED-MATRIX-CHAIN."""
    n = len(p) - 1
    m = [[sys.maxsize] * (n + 1) for _ in range(n + 1)]

    def look(i: int, j: int) -> int:
        if m[i][j] != sys.maxsize:
            return m[i][j]
        if i == j:
            m[i][j] = 0
        else:
            m[i][j] = min(
                look(i, k) + look(k + 1, j) + p[i - 1] * p[k] * p[j]
                for k in range(i, j)
            )
        return m[i][j]

    return look(1, n)


def demo() -> str:
    lines: list[str] = []
    p = [30, 35, 15, 5, 10, 20, 25]
    m, s = matrix_chain_order(p)
    lines.append(f"dimensions p = {p}")
    lines.append(f"minimum scalar multiplications m[1,6] = {m[1][6]}")
    lines.append(f"optimal parenthesization: {optimal_parens(s, 1, 6)}")
    lines.append(
        f"memoized agrees: {memoized_matrix_chain(p) == m[1][6]}; "
        f"naive recursion agrees: {recursive_matrix_chain(p, 1, 6) == m[1][6]}"
    )
    left_to_right = sum(p[0] * p[k - 1] * p[k] for k in range(2, len(p)))
    lines.append(f"naive left-to-right order would cost {left_to_right}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example(self) -> None:
        p = [30, 35, 15, 5, 10, 20, 25]
        m, s = matrix_chain_order(p)
        self.assertEqual(m[1][6], 15125)
        self.assertEqual(optimal_parens(s, 1, 6), "((A1(A2A3))((A4A5)A6))")

    def test_three_strategies_agree(self) -> None:
        r = Rng(101)
        for n in range(1, 9):
            p = [r.range(1, 20) for _ in range(n + 1)]
            m, _ = matrix_chain_order(p)
            self.assertEqual(m[1][n], memoized_matrix_chain(p))
            self.assertEqual(m[1][n], recursive_matrix_chain(p, 1, n))

    def test_single_matrix_costs_nothing(self) -> None:
        m, s = matrix_chain_order([5, 7])
        self.assertEqual(m[1][1], 0)
        self.assertEqual(optimal_parens(s, 1, 1), "A1")


if __name__ == "__main__":
    print(demo(), end="")
